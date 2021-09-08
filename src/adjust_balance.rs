use crate::*;

// This struct makes the assumption that the provided source is stereo.
// If it isn't, you need to use `IntoChannels::new(n, 2, source)` first.

pub struct AdjustBalance<S: Iterator<Item=f32>> {
    balance: Balance,
    source: S,
    strategy: fn(&mut Self) -> Option<f32>,

    // This field is only used in left_channel_only and right_channel_only.
    counter: usize,

    // This field is only used in blend_channels.
    next_sample: Option<f32>,
}

pub enum Balance {
    Static { left_volume: f32, right_volume: f32, inv_left: f32, inv_right: f32 },
    Dynamic { balance: DynamicFloat },
}

impl<S: Iterator<Item=f32>> AdjustBalance<S> {
    pub fn new<B: IntoBalance>(balance: B, source: S) -> Self {
        let initial_value = balance.get();
        let balance = balance.into();

        let strategy = match &balance {
            Balance::Static { .. } if initial_value == 0.  => Self::left_channel_only,
            Balance::Static { .. } if initial_value == 0.5 => Self::noop,
            Balance::Static { .. } if initial_value == 1.  => Self::right_channel_only,
            _                                              => Self::blend_channels,
        };

        Self { balance, source, strategy, counter: 0, next_sample: None }
    }

    fn left_channel_only(&mut self) -> Option<f32> {
        let sample = self.source.next();
        let is_left = self.counter % 2 == 0;

        self.counter += 1;
        if is_left { sample } else { Some(0.) }
    }

    fn noop(&mut self) -> Option<f32> {
        self.source.next()
    }

    fn right_channel_only(&mut self) -> Option<f32> {
        let sample = self.source.next();
        let is_right = self.counter % 2 == 1;

        self.counter += 1;
        if is_right { sample } else { Some(0.) }
    }

    fn blend_channels(&mut self) -> Option<f32> {
        if let Some(s) = self.next_sample.take() { return Some(s); }

        let left_sample = self.source.next()?;
        let right_sample = self.source.next()?;

        let (left_volume, right_volume, inv_left, inv_right) = self.balance.volumes();

        let new_left_sample = left_sample * left_volume + right_sample * inv_right;
        let new_right_sample = right_sample * right_volume + left_sample * inv_left;

        self.next_sample = Some(new_right_sample);
        Some(new_left_sample)
    }
}

impl<S: Iterator<Item=f32>> Iterator for AdjustBalance<S> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        (self.strategy)(self)
    }
}

impl Balance {
    pub fn volumes(&self) -> (f32, f32, f32, f32) {
        match self {
            Self::Static { left_volume, right_volume, inv_left, inv_right } => {
                (*left_volume, *right_volume, *inv_left, *inv_right)
            },
            Self::Dynamic { balance } => {
                calculate_left_right_volumes(balance.get())
            },
        }
    }
}

pub trait IntoBalance: MaybeDynamic<f32> {
    fn into(self) -> Balance;
}

impl IntoBalance for f32 {
    fn into(self) -> Balance {
        let (left_volume, right_volume, inv_left, inv_right) = calculate_left_right_volumes(self);
        Balance::Static { left_volume, right_volume, inv_left, inv_right }
    }
}

impl IntoBalance for DynamicFloat {
    fn into(self) -> Balance {
        Balance::Dynamic { balance: self }
    }
}

fn calculate_left_right_volumes(balance: f32) -> (f32, f32, f32, f32) {
    let left_volume = ((1. - balance) * 2.).clamp(0., 1.);
    let right_volume = (balance * 2.).clamp(0., 1.);

    let inv_left = 1. - left_volume;
    let inv_right = 1. - right_volume;

    (left_volume, right_volume, inv_left, inv_right)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_silences_the_right_samples_when_the_balance_is_fully_left() {
        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = AdjustBalance::new(0., input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 0., 3., 0., 5., 0.]);
    }

    #[test]
    fn it_does_not_change_the_samples_when_the_balance_is_in_the_middle() {
        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = AdjustBalance::new(0.5, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 2., 3., 4., 5., 6.]);
    }

    #[test]
    fn it_silences_the_left_samples_when_the_balance_is_fully_right() {
        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = AdjustBalance::new(1., input).collect::<Vec<_>>();
        assert_eq!(output, vec![0., 2., 0., 4., 0., 6., 0.]);

        // This yields an extra 0. but it shouldn't matter.
    }

    #[test]
    fn it_blends_the_samples_relative_to_how_far_the_balance_is_towards_each_side() {
        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = AdjustBalance::new(0.01, input).collect::<Vec<_>>();
        assert_eq!(output, vec![2.96, 0.04, 6.92, 0.08, 10.88, 0.12]);
    }
}

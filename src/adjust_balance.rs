use crate::*;

// This struct makes the assumption that the provided source is stereo.
// If it isn't, you need to use `IntoChannels::new(n, 2, source)` first.

pub struct AdjustBalance<B: MaybeDynamic<f32>, S: Iterator<Item=f32>> {
    balance: B,
    source: S,
    volumes: (f32, f32, f32, f32),
    next_sample: Option<f32>,
    prev_balance: f32,
}

impl<B: MaybeDynamic<f32>, S: Iterator<Item=f32>> AdjustBalance<B, S> {
    pub fn new(balance: B, source: S) -> Self {
        let prev_balance = balance.get();
        let volumes = Self::left_right_volumes(prev_balance);

        Self { balance, source, volumes, next_sample: None, prev_balance }
    }

    fn left_right_volumes(balance: f32) -> (f32, f32, f32, f32) {
        let left_volume = ((1. - balance) * 2.).clamp(0., 1.);
        let right_volume = (balance * 2.).clamp(0., 1.);

        let inv_left = 1. - left_volume;
        let inv_right = 1. - right_volume;

        (left_volume, right_volume, inv_left, inv_right)
    }
}

impl<B: MaybeDynamic<f32>, S: Iterator<Item=f32>> Iterator for AdjustBalance<B, S> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if let Some(s) = self.next_sample.take() { return Some(s); }

        let left_sample = self.source.next()?;
        let right_sample = self.source.next()?;

        let balance = self.balance.get();
        let balance_changed = balance != self.prev_balance;

        if balance_changed { self.volumes = Self::left_right_volumes(balance); self.prev_balance = balance; }
        let (left_volume, right_volume, inv_left, inv_right) = self.volumes;

        let new_left_sample = left_sample * left_volume + right_sample * inv_right;
        let new_right_sample = right_sample * right_volume + left_sample * inv_left;

        self.next_sample = Some(new_right_sample);
        Some(new_left_sample)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_adds_the_right_samples_into_the_left_when_the_balance_is_fully_left() {
        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = AdjustBalance::new(0., input).collect::<Vec<_>>();
        assert_eq!(output, vec![3., 0., 7., 0., 11., 0.]);
    }

    #[test]
    fn it_does_not_change_the_samples_when_the_balance_is_in_the_middle() {
        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = AdjustBalance::new(0.5, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 2., 3., 4., 5., 6.]);
    }

    #[test]
    fn it_adds_the_left_samples_into_the_right_when_the_balance_is_fully_right() {
        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = AdjustBalance::new(1., input).collect::<Vec<_>>();
        assert_eq!(output, vec![0., 3., 0., 7., 0., 11.]);
    }

    #[test]
    fn it_blends_the_samples_relative_to_how_far_the_balance_is_towards_each_side() {
        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = AdjustBalance::new(0.01, input).collect::<Vec<_>>();
        assert_eq!(output, vec![2.96, 0.04, 6.92, 0.08, 10.88, 0.12]);
    }
}

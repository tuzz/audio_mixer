use crate::*;

pub struct IntoSampleRate<S: Iterator<Item=f32>> {
    sample_rates: SampleRates,
    channels: usize,
    source: S,
    strategy: fn(&mut Self) -> Option<f32>,
    position: f64,
    after_index: usize,

    // These fields are only used in sample_based_linear_interpolation.
    sample_before: f32,
    sample_after: f32,

    // These fields are only used in frame_based_linear_interpolation.
    frame_before: Vec<f32>,
    frame_after: Vec<f32>,
    output_samples: Vec<f32>,
    sample_count: usize,
}

pub enum SampleRates {
    Static { scale: f32 },
    Dynamic { from: DynamicUsize, to: f32 },
}

impl<S: Iterator<Item=f32>> IntoSampleRate<S> {
    pub fn new<T: IntoSampleRates>(from: T, to: usize, channels: usize, source: S) -> Self {
        let from_rate = from.get();
        let sample_rates = from.sample_rates(to);

        let strategy = match (from_rate, to, channels, &sample_rates) {
            (a, b, _, SampleRates::Static { .. }) if a == b => Self::noop,
            (_, _, 1, _) => Self::sample_based_linear_interpolation,
            (_, _, _, _) => Self::frame_based_linear_interpolation,
        };

        Self {
            sample_rates,
            channels,
            source,
            strategy,
            position: 0.,
            sample_before: 0.,
            sample_after: 0.,
            frame_before: vec![0.; channels],
            frame_after: vec![0.; channels],
            after_index: 0,
            sample_count: 0,
            output_samples: vec![0.; channels],
        }
    }

    fn noop(&mut self) -> Option<f32> {
        self.source.next()
    }

    // Linearly interpolate between neighboring samples to derive samples at the
    // faster/slower sample rate. This produces pretty good results and isn't too
    // difficult to implement but won't be as good as others, e.g Sinc interpolation.
    fn sample_based_linear_interpolation(&mut self) -> Option<f32> {
        // If the sample rates match, we don't need to do any conversion.
        // Keep hold of from rather than loading the atomic again later.
        let (noop, from) = self.sample_rates.noop();
        if noop { return self.source.next(); }

        // Calculate the index in the source iterator for the current sample count.
        // This will probably be somewhere between two indexes (the ratio t).
        let (index, t) = (self.position as usize, self.position.fract() as f32);

        // Fast-forward in the source so we are between index and index + 1.
        while index >= self.after_index {
            swap(&mut self.sample_before, &mut self.sample_after);

            if let Some(sample) = self.source.next() {
                self.sample_after = sample;
            } else {
                return None;
            }

            self.after_index += 1;
        }

        // Linearly interpolate between the neighboring samples using the ratio t.
        let delta = self.sample_after - self.sample_before;
        let sample = self.sample_before + t * delta;

        self.position += self.sample_rates.scale(from) as f64;
        Some(sample)
    }

    // This approach is the same as the one above except it interpolates
    // frame-by-frame (all channels at once) and therefore needs to use vectors
    // and pre-compute some values. It'll be slower than the one above.
    fn frame_based_linear_interpolation(&mut self) -> Option<f32> {
        self.sample_count += 1;

        // If the sample rates match, we don't need to do any conversion.
        // Keep hold of from rather than loading the atomic again later.
        let (noop, from) = self.sample_rates.noop();
        if noop { return self.source.next(); }

        // Return the samples from the output_samples buffer (computed below).
        // Handle the case when we transition from noop and the output_samples buffer is empty.
        let channel = self.sample_count % self.channels;
        if channel != 0 { return self.output_samples.get(channel).copied().or(Some(0.)); }

        let (index, t) = (self.position as usize, self.position.fract() as f32);

        while index >= self.after_index {
            swap(&mut self.frame_before, &mut self.frame_after);

            for i in 0..self.channels {
                if let Some(sample) = self.source.next() {
                    self.frame_after[i] = sample;
                } else {
                    return None;
                }
            }

            self.after_index += 1;
        }

        // Store these values in output_samples which acts as a kind of buffer.
        for i in 0..self.channels {
            let delta = self.frame_after[i] - self.frame_before[i];
            self.output_samples[i] = self.frame_before[i] + t * delta;
        }

        self.position += self.sample_rates.scale(from) as f64;
        Some(self.output_samples[0])
    }
}

impl<S: Iterator<Item=f32>> Iterator for IntoSampleRate<S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.strategy)(self)
    }
}

impl SampleRates {
    pub fn noop(&self) -> (bool, f32) {
        match self {
            Self::Static { .. } => (false, 0.),
            Self::Dynamic { from, to } => { let f = from.get() as f32; (f == *to, f) },
        }
    }

    pub fn scale(&self, from: f32) -> f32 {
        match self {
            Self::Static { scale } => *scale,
            Self::Dynamic { to, .. } => from / *to,
        }
    }
}

pub trait IntoSampleRates: MaybeDynamic<usize> {
    fn sample_rates(self, to: usize) -> SampleRates;
}

impl IntoSampleRates for usize {
    fn sample_rates(self, to: usize) -> SampleRates {
        SampleRates::Static { scale: self as f32 / to as f32 }
    }
}

impl IntoSampleRates for DynamicUsize {
    fn sample_rates(self, to: usize) -> SampleRates {
        SampleRates::Dynamic { from: self, to: to as f32 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_upsample_a_mono_source() {
        let input = [1., 2., 3.].into_iter();
        let output = IntoSampleRate::new(1, 2, 1, input).collect::<Vec<_>>();
        assert_eq!(output, vec![0., 0.5, 1., 1.5, 2., 2.5]);

        // We don't do anything special for the first and last samples to keep
        // the algorithm above simple. That means we start interpolating from 0
        // and don't actually produce the very last sample, which isn't ideal.
        //
        // In practice, this shouldn't matter because the audible duration of
        // these samples for a 48KHz sound is ~0.000021 seconds.

        let input = [7., 5., 3.].into_iter();
        let output = IntoSampleRate::new(1, 2, 1, input).collect::<Vec<_>>();
        assert_eq!(output, vec![0., 3.5, 7., 6., 5., 4.]);
    }

    #[test]
    fn it_can_upsample_a_non_mono_source() {
        let input = [1., 7., 2., 5., 3., 3.].into_iter(); // Same as above but interlaced.
        let output = IntoSampleRate::new(1, 2, 2, input).collect::<Vec<_>>();
        assert_eq!(output, vec![0., 0., 0.5, 3.5, 1., 7., 1.5, 6., 2., 5., 2.5, 4.]);
    }

    #[test]
    fn it_can_downsample_a_mono_source() {
        let input = [1., 2., 3., 4., 5., 6., 7.].into_iter();
        let output = IntoSampleRate::new(3, 2, 1, input).collect::<Vec<_>>();
        assert_eq!(output, vec![0., 1.5, 3., 4.5, 6.]);

        let input = [13., 11., 9., 7., 5., 3., 1.].into_iter();
        let output = IntoSampleRate::new(3, 2, 1, input).collect::<Vec<_>>();
        assert_eq!(output, vec![0., 12., 9., 6., 3.]);
    }

    #[test]
    fn it_can_downsample_a_stereo_source() {
        let input = [1., 13., 2., 11., 3., 9., 4., 7., 5., 5., 6., 3., 7., 1.].into_iter();
        let output = IntoSampleRate::new(3, 2, 2, input).collect::<Vec<_>>();
        assert_eq!(output, vec![0., 0., 1.5, 12., 3., 9., 4.5, 6., 6., 3.]);
    }

    #[test]
    fn it_can_dynamically_change_the_input_rate_to_control_the_pitch() {
        let input = [1., 2., 3., 4., 5., 6., 7., 8.].into_iter();
        let input_rate = DynamicUsize::new(1);

        let mut output = IntoSampleRate::new(input_rate.clone(), 1, 1, input);

        assert_eq!(output.next(), Some(0.));
        assert_eq!(output.next(), Some(1.));
        assert_eq!(output.next(), Some(2.));

        input_rate.set(2);

        assert_eq!(output.next(), Some(3.));
        assert_eq!(output.next(), Some(5.));
        assert_eq!(output.next(), Some(7.));
        assert_eq!(output.next(), None);
    }
}

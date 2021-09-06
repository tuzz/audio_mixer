use std::mem::swap;

pub struct IntoSampleRate<S: Iterator<Item=f32>> {
    scale: f32,
    channels: usize,
    source: S,
    strategy: fn(&mut IntoSampleRate<S>) -> Option<f32>,
    sample_count: usize,
    after_index: usize,

    // These fields are only used in sample_based_linear_interpolation.
    sample_before: f32,
    sample_after: f32,

    // These fields are only used in frame_based_linear_interpolation.
    frame_before: Vec<f32>,
    frame_after: Vec<f32>,
    output_samples: Vec<f32>,
    channel_index: usize,
}

impl<S: Iterator<Item=f32>> IntoSampleRate<S> {
    pub fn new(from: usize, to: usize, channels: usize, source: S) -> Self {
        let strategy = match (from, to, channels) {
            (a, b, _) if a == b => Self::noop,
            (_, _, 1)           => Self::sample_based_linear_interpolation,
            (_, _, _)           => Self::frame_based_linear_interpolation,
        };

        Self {
            scale: from as f32 / to as f32,
            channels,
            source,
            strategy,
            sample_count: 0,
            sample_before: 0.,
            sample_after: 0.,
            frame_before: vec![0.; channels],
            frame_after: vec![0.; channels],
            after_index: 0,
            channel_index: 0,
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
        // Calculate the index in the source iterator for the current sample count.
        // This will probably be somewhere between two indexes (the ratio t).
        let position = self.sample_count as f32 * self.scale;
        let (index, t) = (position as usize, position.fract());

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

        self.sample_count += 1;
        Some(sample)
    }

    // This approach is the same as the one above except it interpolates
    // frame-by-frame (all channels at once) and therefore needs to use vectors
    // and pre-compute some values. It'll be slower than the one above.
    fn frame_based_linear_interpolation(&mut self) -> Option<f32> {
        let channel = self.channel_index;
        self.channel_index = (self.channel_index + 1) % self.channels;

        // Return the samples from the output_samples buffer (computed below).
        if channel != 0 { return Some(self.output_samples[channel]); }

        let position = self.sample_count as f32 * self.scale;
        let (index, t) = (position as usize, position.fract());

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

        self.sample_count += 1;
        Some(self.output_samples[0])
    }
}

impl<S: Iterator<Item=f32>> Iterator for IntoSampleRate<S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.strategy)(self)
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
}

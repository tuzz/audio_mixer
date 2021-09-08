pub struct IntoChannels<S: Iterator<Item=f32>> {
    from: usize,
    to: usize,
    source: S,
    strategy: fn(&mut Self) -> Option<f32>,
    counter: usize,
    previous: f32,
}

impl<S: Iterator<Item=f32>> IntoChannels<S> {
    pub fn new(from: usize, to: usize, source: S) -> Self {
        let strategy = match (from, to) {
            (a, b) if a == b => Self::noop,
            (1, _)           => Self::duplicate,
            (_, 1)           => Self::combine,
            (a, b) if a < b  => Self::pad,
            (a, b) if a > b  => Self::discard,
            _                => unreachable!(),
        };

        Self { from, to, source, strategy, counter: 0, previous: -1. }
    }

    // Don't incur any performance overhead when not changing channels.
    fn noop(&mut self) -> Option<f32> {
        self.source.next()
    }

    // Duplicate the mono sample across all channels (e.g. from=1, to=2).
    fn duplicate(&mut self) -> Option<f32> {
        let sample = if self.counter == 0 {
            self.source.next().map(|s| { self.previous = s; s })
        } else {
            Some(self.previous)
        };

        self.counter = (self.counter + 1) % self.to;
        sample
    }

    // Combine channel samples into one mono sample (e.g. from=2, to=1).
    fn combine(&mut self) -> Option<f32> {
        let mut sum = match self.source.next() { Some(s) => s, _ => return None };

        for _ in 1..self.from {
            sum += self.source.next().unwrap_or(0.);
        }

        Some(sum)
    }

    // Pad additional output channels with silence (e.g. from=2, to=3).
    fn pad(&mut self) -> Option<f32> {
        let sample = if self.counter < self.from {
            self.source.next().or_else(|| (self.counter != 0).then_some(0.))
        } else {
            Some(0.)
        };

        self.counter = (self.counter + 1) % self.to;
        sample
    }

    // Discard additional input channels (e.g. from=3, to=2).
    fn discard(&mut self) -> Option<f32> {
        loop {
            let sample = self.source.next();
            let keep = self.counter < self.to;

            self.counter = (self.counter + 1) % self.from;
            if keep { return sample; }
        }
    }
}

impl<S: Iterator<Item=f32>> Iterator for IntoChannels<S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.strategy)(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_does_nothing_when_converting_from_and_to_the_same_number_of_channels() {
        let input = [1., 2., 3.].into_iter();
        let output = IntoChannels::new(1, 1, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 2., 3.]);

        let input = [1., 2., 3.].into_iter();
        let output = IntoChannels::new(5, 5, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 2., 3.]);
    }

    #[test]
    fn it_combines_samples_into_mono_samples_when_converting_to_one_channel() {
        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = IntoChannels::new(2, 1, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1. + 2.,      3. + 4.,    5. + 6.]);

        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = IntoChannels::new(3, 1, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1. + 2. + 3.,        4. + 5. + 6.]);

        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = IntoChannels::new(4, 1, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1. + 2. + 3. + 4.,        5. + 6.]);
    }

    #[test]
    fn it_duplicates_each_sample_across_all_channels_when_converting_from_mono() {
        let input = [1., 2.].into_iter();
        let output = IntoChannels::new(1, 2, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 1., 2., 2.]);

        let input = [1., 2.].into_iter();
        let output = IntoChannels::new(1, 3, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 1., 1., 2., 2., 2.]);
    }

    #[test]
    fn it_pads_additional_output_channels_with_zeroes_so_they_are_silent() {
        let input = [1., 2., 3., 4.].into_iter();
        let output = IntoChannels::new(2, 3, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 2., 0., 3., 4., 0.]);

        let input = [1., 2., 3., 4., 5.].into_iter();
        let output = IntoChannels::new(3, 5, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 2., 3., 0., 0., 4., 5., 0., 0., 0.]);
    }

    #[test]
    fn it_discards_additional_input_channels_if_there_arent_enough_output_channels() {
        let input = [1., 2., 3., 4., 5., 6.].into_iter();
        let output = IntoChannels::new(3, 2, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 2., 4., 5.]);

        let input = [1., 2., 3., 4., 5.].into_iter();
        let output = IntoChannels::new(3, 2, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 2., 4., 5.]);

        let input = [1., 2., 3., 4., 5.].into_iter();
        let output = IntoChannels::new(4, 2, input).collect::<Vec<_>>();
        assert_eq!(output, vec![1., 2., 5.]);
    }
}

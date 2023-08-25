use crate::*;

// This is based on the implementation from the rodio crate:
// https://github.com/RustAudio/rodio/blob/a6f50364b0dbe8869519b2e79c65c4a606aadccd/src/source/blt.rs
//
// It differs in that it works for an arbitrary number of channels, uses
// precomputed coefficients and allows the threshold value to change over time.
//
// We don't currently support MaybeDynamic for sample_rate for better
// performance but support could be added later using a strategy pattern.

pub struct LowPassFilter<S: Iterator<Item=f32>, F: M, C: M> {
    threshold_frequency: F,
    channels: C,
    source: S,
    coefficients: LowPassCoefficients,
    index: usize,
    previous: Vec<[f32; 4]>,
    counter: usize,
    max: usize,
}

pub trait M = MaybeDynamic<usize>;

impl<S: Iterator<Item=f32>, F: M, C: M> LowPassFilter<S, F, C> {
    pub fn new(threshold_frequency: F, channels: C, sample_rate: usize, source: S, coefficients: LowPassCoefficients) -> Self {
        let previous = vec![[0.; 4]; 128]; // In case the number of channels changes.

        let index = coefficients.index_for_sample_rate(sample_rate);
        let max = coefficients.for_sample_rate_index(index).len() - 1;

        Self { threshold_frequency, channels, source, coefficients, index, max, previous, counter: 0 }
    }
}

impl<S: Iterator<Item=f32>, F: M, C: M> Iterator for LowPassFilter<S, F, C> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let threshold = self.threshold_frequency.get();
        if threshold >= self.max { return self.source.next(); }

        let sample = self.source.next()?;
        let scoped = self.coefficients.for_sample_rate_index(self.index);

        if let Some(coefficients) = scoped[self.threshold_frequency.get()] {
            let channel = self.counter % self.channels.get();

            let [b0, b1, b2, a1, a2] = coefficients;
            let [x_n1, x_n2, y_n1, y_n2] = &mut self.previous[channel];

            let output = b0 * sample + b1 * *x_n1 + b2 * *x_n2 - a1 * *y_n1 - a2 * *y_n2;

            *y_n2 = *y_n1;
            *x_n2 = *x_n1;
            *y_n1 = output;
            *x_n1 = sample;

            self.counter += 1;
            Some(output)
        } else {
            self.counter += 1;
            Some(sample) // noop
        }
    }
}

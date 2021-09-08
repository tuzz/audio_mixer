use crate::*;

// This is based on the implementation from the rodio crate:
// https://github.com/RustAudio/rodio/blob/a6f50364b0dbe8869519b2e79c65c4a606aadccd/src/source/blt.rs
//
// It differs in that it works for an arbitrary number of channels, uses
// precomputed coefficients and allows the threshold value to change over time.

pub struct LowPassFilter<S: Iterator<Item=f32>, F: M, R: M, C: M> {
    threshold_frequency: F,
    channels: C,
    sample_rate: R,
    source: S,
    previous: Vec<[f32; 4]>,
    counter: usize,
}

pub trait M = MaybeDynamic<usize>;

impl<S: Iterator<Item=f32>, F: M, R: M, C: M> LowPassFilter<S, F, R, C> {
    pub fn new(threshold_frequency: F, channels: C, sample_rate: R, source: S) -> Self {
        let previous = vec![[0.; 4]; 1024]; // In case the number of channels changes.

        Self { threshold_frequency, channels, sample_rate, source, previous, counter: 0 }
    }
}

impl<S: Iterator<Item=f32>, F: M, R: M, C: M> Iterator for LowPassFilter<S, F, R, C> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.source.next()?;

        if let Some(map) = LOW_PASS_COEFFICIENTS.get() {
            let hash_key = (self.threshold_frequency.get(), self.sample_rate.get());

            if let Some(coefficients) = map.get(&hash_key) {
                let channel = self.counter % self.channels.get();

                let [b0, b1, b2, a1, a2] = coefficients;
                let [x_n1, x_n2, y_n1, y_n2] = &mut self.previous[channel];

                let result = b0 * sample + b1 * *x_n1 + b2 * *x_n2 - a1 * *y_n1 - a2 * *y_n2;

                *y_n2 = *y_n1;
                *x_n2 = *x_n1;
                *y_n1 = result;
                *x_n1 = sample;

                self.counter += 1;
                return Some(result);
            }
        }

        self.counter += 1;
        Some(sample)
    }
}

use crate::*;

#[derive(Clone)]
pub struct LowPassCoefficients {
    precomputed: Precomputed,
}

type Precomputed = Arc<Vec<(SampleRate, Vec<Option<Coefficients>>)>>;
type SampleRate = usize;
type Coefficients = [f32; 5];

impl LowPassCoefficients {
    pub fn new<S: Iterator<Item=usize>>(sample_rates: S, max_threshold_frequency: usize) -> Self {
        let iter = sample_rates.map(|r| (r, Self::coefficients_for_each_threshold_frequency(r, max_threshold_frequency)));

        Self { precomputed: Arc::new(iter.collect()) }
    }

    pub fn index_for_sample_rate(&self, sample_rate: usize) -> usize {
        self.precomputed.iter().position(|(r, _)| *r == sample_rate).unwrap()
    }

    pub fn for_sample_rate_index(&self, sample_rate_index: usize) -> &Vec<Option<Coefficients>> {
        &self.precomputed[sample_rate_index].1
    }

    pub fn clone_arc(&self) -> Self {
        self.clone()
    }

    pub fn coefficients_for_each_threshold_frequency(sample_rate: usize, max_threshold_frequency: usize) -> Vec<Option<Coefficients>> {
        // LowPassFilter diverges to infinite values if the sample rate is less
        // than twice the threshold frequency due to the Nyquist limit. Don't
        // generate coefficients and forward the original sample in this case.
        let minimum_sample_rate = max_threshold_frequency as usize * 2 + 10; // Give it some leeway.

        if sample_rate < minimum_sample_rate {
            (0..=max_threshold_frequency).map(|_| None).collect()
        } else {
            (0..=max_threshold_frequency).map(|f| Some(Self::coefficients(sample_rate, f))).collect()
        }
    }

    fn coefficients(sample_rate: usize, frequency: usize) -> Coefficients {
        let w0 = 2.0 * PI * frequency as f32 / sample_rate as f32;
        let q = 0.5;

        let alpha = w0.sin() / (2.0 * q);
        let b1 = 1.0 - w0.cos();
        let b0 = b1 / 2.0;
        let b2 = b0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * w0.cos();
        let a2 = 1.0 - alpha;

        [b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0]
    }
}

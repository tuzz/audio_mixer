use crate::*;

// The precomputed coefficients are stored in a SyncOnceCell so that the audio
// thread can access them without needing a Mutex or other RAII guards. However,
// that means we can only initialize them once.
//
// We could store the coefficients in the struct and pass the struct to the
// LowPassFilter to get around this problem but I think this is cleaner.

pub struct LowPassCoefficients;

pub static LOW_PASS_COEFFICIENTS: OnceLock<HashMap<HashKey, Coefficients>> = OnceLock::new();

type HashKey = (ThresholdFrequency, SampleRate); type ThresholdFrequency = usize;
type SampleRate = usize;
type Coefficients = [f32; 5];

impl LowPassCoefficients {
    pub fn precompute<I1, I2>(threshold_frequencies: I1, sample_rates: I2)
        where I1: Iterator<Item=usize>,
              I2: Iterator<Item=usize> + Clone,
    {
        LOW_PASS_COEFFICIENTS.get_or_init(|| {
            threshold_frequencies.flat_map(|threshold_frequency| {
                // LowPassFilter diverges to infinite values if the sample rate is
                // less than twice the threshold frequency. It's probably something
                // to do with Nyquist limit. Therefore, skip generating coefficients
                // in these cases seeing as they won't be used by the filter.
                let frequency = threshold_frequency as f32;
                let minimum_sample_rate = frequency * 2. + 10.; // Give it some leeway.

                sample_rates.clone().filter_map(move |sample_rate| {
                    let rate = sample_rate as f32;
                    if rate < minimum_sample_rate { return None; }

                    let w0 = 2.0 * PI * frequency / rate;
                    let q = 0.5;

                    let alpha = w0.sin() / (2.0 * q);
                    let b1 = 1.0 - w0.cos();
                    let b0 = b1 / 2.0;
                    let b2 = b0;
                    let a0 = 1.0 + alpha;
                    let a1 = -2.0 * w0.cos();
                    let a2 = 1.0 - alpha;

                    let hash_key = (threshold_frequency, sample_rate);
                    let coefficients = [b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0];

                    Some((hash_key, coefficients))
                })
            }).collect()
        });
    }
}

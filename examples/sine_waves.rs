use audio_mixer::AudioMixer;
use std::f32::consts::PI;
use std::{thread::sleep, time::Duration};

fn main() {
    let mixer = AudioMixer::for_default_device().unwrap();
    let sample_rate = mixer.sample_rate();
    let channels = mixer.channels();

    // Each sine wave plays for 2 seconds.
    let duration = 2 * sample_rate * channels;

    let sine1 = SineWave::new(110., sample_rate, duration);
    let sine2 = SineWave::new(220., sample_rate, duration);
    let sine3 = SineWave::new(440., sample_rate, duration);
    let sine4 = SineWave::new(880., sample_rate, duration);

    // Play the sound waves simultaneously, staggered by 500ms.
    mixer.add(sine1);
    sleep(Duration::from_millis(500));

    mixer.add(sine2);
    sleep(Duration::from_millis(500));

    mixer.add(sine3);
    sleep(Duration::from_millis(500));

    mixer.add(sine4);

    // Wait until all sine waves have finished playing.
    mixer.wait();
}

struct SineWave {
    frequency: f32,
    sample_rate: f32,
    sample_clock: f32,
    sample_count: usize,
    duration: usize,
}

impl SineWave {
    pub fn new(frequency: f32, sample_rate: usize, duration: usize) -> Self {
        Self { frequency, sample_rate: sample_rate as f32, sample_clock: 0., sample_count: 0, duration }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    // The source is removed from the AudioMixer if/when the iterator returns None.
    fn next(&mut self) -> Option<Self::Item> {
        self.sample_count += 1;
        if self.sample_count > self.duration { return None; }

        self.sample_clock = (self.sample_clock + 1.) % self.sample_rate;
        Some((self.sample_clock * self.frequency * 2. * PI / self.sample_rate).sin())
    }
}

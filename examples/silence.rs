use audio_mixer::AudioMixer;

fn main() {
  let mixer = AudioMixer::for_default_device().unwrap();

  mixer.add(Silence);
  mixer.add(Silence);

  mixer.wait(); // Wait until all sources have finished (optional).
}

struct Silence;

impl Iterator for Silence {
  type Item = f32;

  fn next(&mut self) -> Option<Self::Item> {
      Some(0.) // Return your audio sample here, or None when finished.
  }
}

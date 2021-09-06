## AudioMixer

An efficient, cross-platform Rust crate that mixes together audio from different
sources, such as sound files and sine waves. It has an extremely minimal
interface (iterators) and is written with performance in mind.

I wrote this crate because I initially tried to use rodio and ran into lots of
problems with performance and with audio playing in the wrong channels. I haven't
published this crate yet but you can install from the GitHub URL if you wish.

## How to use

```rust
use audio_mixer::AudioMixer;

fn main() {
  let mixer = AudioMixer::default();

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
```

Check out the other examples for programs that actually make sounds.

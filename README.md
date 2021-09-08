## AudioMixer

An efficient, cross-platform Rust crate for mixing together audio from different
sources, such as sound files and sine waves. It has a minimal interface (iterators),
is easily extended and was written with performance in mind.

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

  mixer.wait(); // Wait until all sources have finished playing (optional).
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

## Conversions

The crate includes `IntoChannels` and `IntoSampleRate` structs to help with
channel and sample rate conversions. These are needed, for example, when trying
to play a 44100Hz mono source on a 48000Hz stereo output device. Without these,
the audio will play in the wrong channels and/or at the wrong speed.

## Ogg decoding

The crate supports ogg decoding (via the lewton crate). You need to enable the
`ogg` crate feature to use the `OggDecoder` struct. It should be easy to
implement your own decoders for other formats, provided you can produce an
iterator of channel-interlaced samples. There are plenty of crates available
that should be able to help.

See [examples/ogg_file.rs](examples/ogg_file.rs) for an example that combines
`OggDecoder`, `IntoChannels` and `IntoSampleRate`.

```
cargo run --example ogg_file --features ogg
```

## Reusing buffers

If you want to play a sound multiple times, it makes sense to write this sound
into a buffer first so you can reuse it, rather than reading it from the file
system and decoding it each time. The crate provides a `ReusableBuffer` struct
to help with this. See [examples/buffer_reuse.rs](examples/buffer_reuse.rs).

## Dynamic controls

If you want to change some parameter of an iterator while it is being read
(e.g. change the pitch or volume) there are `DynamicUsize` and `DynamicFloat`
structs to help with this. See [examples/dynamic_controls.rs](examples/dynamic_controls.rs).

## License

MIT

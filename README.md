## AudioMixer

An efficient, cross-platform Rust crate for mixing together audio from different
sources, such as sound files and sine waves. It has a minimal interface (iterators),
is easily extended and was written with performance in mind.

I wrote this crate because I initially tried to use rodio and ran into lots of
problems with performance and with audio playing in the wrong channels. I haven't
published this crate yet but you can install it from the GitHub URL.

## How to use

```rust
use audio_mixer::AudioMixer;

fn main() {
  let mixer = AudioMixer::for_default_device().unwrap();

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

## How to extend

The crate contains a few useful things (see below), but it's also easy to extend
it with your own iterators. For example, here's how you would write an iterator
to reverse the left/right channels of a stereo source.

```rust
struct ReverseStereo<S: Iterator<Item=f32>> {
    stereo_source: S,
    left_sample: Option<f32>,
}

impl<S: Iterator<Item=f32>> ReverseStereo<S> {
    pub fn new(stereo_source: S) -> Self {
        Self { stereo_source, left_sample: None }
    }
}

impl<S: Iterator<Item=f32>> Iterator for ReverseStereo<S> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if let Some(right_sample) = self.left_sample.take() {
            Some(right_sample)
        } else {
            // Samples are channel-interlaced so this works by stashing the left
            // sample on self and yielding the right one in its place. The
            // iterator then yields the stashed sample on the next call.

            self.left_sample = self.stereo_source.next();
            let right_sample = self.stereo_source.next();

            right_sample
        }
    }
}
```

See [examples/reverse_stereo.rs](examples/reverse_stereo.rs) for a working
version of the above code.

## Conversions

The crate includes `IntoChannels` and `IntoSampleRate` structs to help with
channel and sample rate conversions. These are needed, for example, when trying
to play a 44100Hz mono source on a 48000Hz stereo output device. Without these,
the audio will play in the wrong channels and/or at the wrong speed.

Many of the iterators in the crate use a
[strategy pattern](https://en.wikipedia.org/wiki/Strategy_pattern) so that
unnecessary processing doesn't take place. For example, if you convert 2
channels into 2 channels, the samples will simply be forwarded on (a "no op").
Therefore, you don't need to check these conditions yourself before
deciding whether an iterator is needed.

## Ogg and Wav decoding

The crate supports ogg and wav decoding (via the lewton and hound crates). You
need to enable the `ogg` or `wav` crate features respectively to use the
`OggDecoder` or `WavDecoder` structs.

It should be easy to implement your own decoders for other formats, provided you
can produce an iterator of channel-interlaced samples. There are plenty of
crates available that should be able to help.

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

The `ReusableBuffer` also allows you to seek forwards and backwards through a
sound or start it playing from a specific point. See
[examples/buffer_seeking.rs](examples/buffer_seeking.rs).

## Dynamic controls

If you want to change some parameter of an iterator while it is being read
(e.g. change the pitch or volume) there are `DynamicUsize` and `DynamicFloat`
structs to help with this. See [examples/dynamic_controls.rs](examples/dynamic_controls.rs).

## Low-pass filtering

The crate provides a `LowPassFilter` which is perhaps the most complicated of
the available filters. It requires that you tell it which threshold frequencies
and sample rates you want to filter in advance to keep expensive computations
out of the time-sensitive audio thread. See [examples/low_pass.rs](examples/low_pass.rs).

## Controlling playback

The crate provides `PausableAudio` and `StoppableAudio`. These allow audio to be
paused temporarily and stopped completely via a dynamic control. Their
implementations are iterator-based like everything else.

The `StoppableAudio` iterator can also be used as an 'observer' to determine
when a source has finished playing. See
[examples/pausing_playback.rs](examples/pausing_playback.rs) and
[examples/stopping_playback.rs](examples/stopping_playback.rs).

## Recording playback

The crate provides `audio_mixer.start_recording` which allows you to capture the
stream of samples sent to the audio device. These samples are provided to a
closure along with some metadata such as how long the audio has been playing.
See [examples/recording_audio.rs](examples/recording_audio.rs)

You could use this feature, for example, to record audio into a file or to
stream over the network. This feature is really intended to 'listen in' on
what's playing on the device rather than as an offline processing pipeline
since it works in real-time with an audio device.

## Optimizations

The crate provides three iterators that might help you to reduce load:

- `PauseWhenMuted`: Pauses while the volume is muted. Resumes when the volume is unmuted.
- `SkipWhenMuted`: Pauses while muted. Seeks to its expected position when unmuted.
- `StopWhenMuted`: Stops immediately when muted. The iterator fuses and returns None.

These iterators can save unnecessary work in the iterator chain when the audio
is muted. For example, they can be used to bypass sample rate conversions and
low-pass filtering which are relatively expensive operations. The
`SkipWhenMuted` iterator makes use of the seeking capability of `ReusableBuffer`. See
[examples/pausing_when_muted.rs](examples/pausing_when_muted.rs) and
[examples/skipping_when_muted.rs](examples/skipping_when_muted.rs).

## Ideas for improvement

- Fix SkipWhenMuted not handling channels properly
- Make cpal an optional dependency so that you can mix audio together without
  also having to play it on a device

## License

MIT

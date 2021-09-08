use audio_mixer::*;
use std::{io::Cursor, thread::sleep, time::Duration};

// This example is the same as examples/ogg_file.rs except it dynamically
// controls the pitch of the audio while it is being played.
//
// The DynamicUsize and DynamicFloat structs are wrappers for Arc<Atomic...>
// which are shared between the main thread and the audio thread.
//
// This pattern can be used whenever you need to change something about the audio
// while it is playing, for example, its volume or pan it from left to right.

fn main() {
  let decoder = OggDecoder::new(Cursor::new(include_bytes!("./ogg_file.ogg")));
  let mixer = AudioMixer::default();

  let in_channels = decoder.channels();
  let out_channels = mixer.channels();

  let in_rate = decoder.sample_rate();
  let out_rate = mixer.sample_rate();

  // Start the rate off so that it matches the rate of the input source. If we
  // increase this rate, we'll consume samples faster from the source.
  let rate = DynamicUsize::new(in_rate);

  let source1 = IntoSampleRate::new(rate.clone(), out_rate, in_channels, decoder);
  let source2 = IntoChannels::new(in_channels, out_channels, source1);

  mixer.add(source2);

  // Double the rate every 0.5 seconds.
  while mixer.is_playing() {
      sleep(Duration::from_millis(500));
      rate.set(rate.get() * 2);
  }
}

use audio_mixer::*;
use std::{io::Cursor, thread::sleep, time::Duration};

// This example is the same as examples/ogg_file.rs except it dynamically
// controls the balance of the audio while it is being played.
//
// This example makes the assumption that ogg_file.ogg is a stereo source
// (it is) but we could use the IntoChannels iterator first if it wasn't.
//
// See examples/dynamic_controls.rs for more explanation of dynamic controls.

fn main() {
  let decoder = OggDecoder::new(Cursor::new(include_bytes!("./ogg_file.ogg")));
  let mixer = AudioMixer::default();

  let in_channels = decoder.channels();
  let out_channels = mixer.channels();

  let in_rate = decoder.sample_rate();
  let out_rate = mixer.sample_rate();

  let balance = DynamicFloat::new(0.); // Start on the left (0.5 is the middle).

  let panned = AdjustBalance::new(balance.clone(), decoder);
  let source1 = IntoSampleRate::new(in_rate, out_rate, in_channels, panned);
  let source2 = IntoChannels::new(in_channels, out_channels, source1);

  mixer.add(source2);

  while mixer.is_playing() {
      sleep(Duration::from_millis(15));

      // The atomics have special-purpose functions for adding/subtracting/etc.
      // Therefore, use fetch_add instead of balance.set(balance.get() + 0.01)
      // which will be slightly more performant (not that it really matters).
      balance.value.fetch_add(0.01, std::sync::atomic::Ordering::Relaxed);
  }
}

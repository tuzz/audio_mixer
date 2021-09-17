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
  let cursor = Cursor::new(include_bytes!("./ogg_file.ogg"));
  let decoder = OggDecoder::new(cursor).unwrap();
  let mixer = AudioMixer::for_default_device().unwrap();

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
      balance.add(0.01);
  }
}

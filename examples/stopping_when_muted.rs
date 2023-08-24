use audio_mixer::*;
use std::{io::Cursor, thread::sleep, time::Duration};

// This example demonstrates the StopWhenMuted optimization. This iterator stops
// playback of the source immediately when it is muted by returning None.
//
// If you want to resume when un-muted again, see PauseWhenMuted.
// If you want to seek ahead when un-muted again, see SkipWhenMuted.

fn main() {
  let cursor = Cursor::new(include_bytes!("./ogg_file.ogg"));
  let decoder = OggDecoder::new(cursor).unwrap();
  let mixer = AudioMixer::for_default_device().unwrap();

  let in_channels = decoder.channels();
  let out_channels = mixer.channels();

  let in_rate = decoder.sample_rate();
  let out_rate = mixer.sample_rate();

  let volume = DynamicFloat::new(1.);

  let source1 = IntoSampleRate::new(in_rate, out_rate, in_channels, decoder);
  let source2 = IntoChannels::new(in_channels, out_channels, source1);
  let source3 = StopWhenMuted::new(volume.clone(), source2);

  mixer.add(source3);
  sleep(Duration::from_millis(1500));

  println!("Muting the sound which stops it immediately.");
  volume.set(0.);
}

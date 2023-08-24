use audio_mixer::*;
use std::{io::Cursor, thread::sleep, time::Duration};

// This example demonstrates the PauseWhenMuted optimization. This iterator does
// not advance its source iterator when the volume is muted which can save
// unnecessary work in the iterator chain when nothing would be audible anyway.
//
// If you want to seek ahead when un-muted again, see SkipWhenMuted.
// If you want to stop immediately when muted, see StopWhenMuted.

fn main() {
  let cursor = Cursor::new(include_bytes!("./ogg_file.ogg"));
  let decoder = OggDecoder::new(cursor).unwrap();

  let mixer = AudioMixer::for_default_device().unwrap();

  let in_channels = decoder.channels();
  let out_channels = mixer.channels();

  let in_rate = decoder.sample_rate() * 2; // Play this example at double the pitch.
  let out_rate = mixer.sample_rate();

  let volume = DynamicFloat::new(0.);

  let source1 = AdjustVolume::new(volume.clone(), decoder);
  let source2 = IntoSampleRate::new(in_rate, out_rate, in_channels, source1);
  let source3 = IntoChannels::new(in_channels, out_channels, source2);

  // Add the optimization right at the end of the chain of iterators so that
  // it bypasses work performed by those earlier in the chain.
  let source4 = PauseWhenMuted::new(volume.clone(), out_channels, source3);

  println!("Playing while muted");
  mixer.add(source4);

  sleep(Duration::from_millis(1000));
  println!("Un-muting after 1 second");
  volume.set(1.);

  mixer.wait();
  println!("Playback was paused so should have started from the beginning.")
}

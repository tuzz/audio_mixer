use audio_mixer::*;
use std::{io::Cursor, thread::sleep, time::Duration};

// This example is the same as examples/ogg_file.rs except it dynamically
// toggles whether the playback is paused using a dynamic control.
//
// See examples/dynamic_controls.rs for more explanation of dynamic controls.
//
// When paused, then PausableAudio iterator emits silence, i.e. Some(0.). This
// ensures the iterator isn't removed from AudioMixer before it has finished.
//
// The AudioMixer's is_playing() method isn't affected by whether iterators are
// paused. The 'pausing' concept is entirely contained within PausableAudio.

fn main() {
  let cursor = Cursor::new(include_bytes!("./ogg_file.ogg"));
  let decoder = OggDecoder::new(cursor).unwrap();
  let mixer = AudioMixer::for_default_device().unwrap();

  let in_channels = decoder.channels();
  let out_channels = mixer.channels();

  let in_rate = decoder.sample_rate();
  let out_rate = mixer.sample_rate();

  let paused = DynamicBool::new(false);

  let source1 = IntoSampleRate::new(in_rate, out_rate, in_channels, decoder);
  let source2 = IntoChannels::new(in_channels, out_channels, source1);
  let source3 = PausableAudio::new(paused.clone(), out_channels, source2);

  mixer.add(source3);

  while mixer.is_playing() {
      sleep(Duration::from_millis(1000));
      paused.set(!paused.get());
  }
}

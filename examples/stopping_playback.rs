use audio_mixer::*;
use std::{io::Cursor, thread::sleep, time::Duration};

// This example is the same as examples/ogg_file.rs except it explicitly stops
// the audio part-way through which ends the iterator. If you want to resume
// playback again later, use PausableAudio instead of StoppableAudio.
//
// The iterator can also be used to determine when the audio has stopped via the
// same dynamic control. It is automatically set to true by StoppableAudio when
// the source iterator returns None. After that, StoppableAudio behaves like
// Iterator::fuse and always returns None from that point forward.
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

  let stopped1 = DynamicBool::new(false);
  let stopped2 = DynamicBool::new(false);

  let source1 = IntoSampleRate::new(in_rate, out_rate, in_channels, decoder);
  let source2 = IntoChannels::new(in_channels, out_channels, source1);
  let reusable = ReusableBuffer::new(0, source2.collect());

  let source4 = StoppableAudio::new(stopped1.clone(), reusable.reuse_from(0));
  let source5 = StoppableAudio::new(stopped2.clone(), reusable.reuse_from(0));

  mixer.add(source4);
  sleep(Duration::from_millis(1500));

  println!("Stopping the sound.");
  stopped1.set(true);
  sleep(Duration::from_millis(1000));

  println!("Playing again until completion.");
  mixer.add(source5);

  while !stopped2.get() {}
  println!("The iterator automatically set stopped=true.");
}

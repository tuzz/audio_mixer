use audio_mixer::*;
use std::{io::Cursor, thread::sleep, time::Duration};

// This example demonstrates the top-level AudioMixer::pause() method. This is
// more efficient than using the PauseWhenMuted for each individual source.
//
// New sources can be added while all others are paused and the new sources will
// play as normal. Sources can be resumed again by calling AudioMixer::play()
// or calling AudioMixer::wait() which will block until all sources have finished.
//
//
//
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

  let source1 = IntoSampleRate::new(in_rate, out_rate, in_channels, decoder);
  let source2 = IntoChannels::new(in_channels, out_channels, source1);
  let source3 = ReusableBuffer::new(0, source2);

  assert!(!mixer.is_playing());

  println!("Playing the first source.");
  mixer.add(source3.reuse_from(0));
  assert!(mixer.is_playing());

  sleep(Duration::from_millis(1000));
  println!("Playing the second source.");
  mixer.add(source3.reuse_from(0));
  assert!(mixer.is_playing());

  sleep(Duration::from_millis(1000));
  println!("Pausing all sources.");
  mixer.pause();
  assert!(!mixer.is_playing());

  sleep(Duration::from_millis(1000));
  println!("Playing the third source while other sources are paused.");
  mixer.add(source3);
  assert!(mixer.is_playing());

  sleep(Duration::from_millis(4000));
  println!("Resuming all sources (via AudioMixer::play).");
  mixer.play();
  assert!(mixer.is_playing());

  sleep(Duration::from_millis(1000));
  println!("Pausing all sources.");
  mixer.pause();
  assert!(!mixer.is_playing());

  sleep(Duration::from_millis(2000));
  println!("Resuming all sources (via AudioMixer::wait).");
  mixer.wait();
  assert!(!mixer.is_playing());
}

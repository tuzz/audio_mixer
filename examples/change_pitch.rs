use audio_mixer::{AudioMixer, IntoChannels, IntoSampleRate, OggDecoder};
use std::{io::Cursor, thread::sleep, time::Duration};
use std::{sync::{Arc, atomic::{AtomicUsize, Ordering::Relaxed}}};

// This example is the same as examples/ogg_file.rs except it dynamically
// controls the pitch of the audio while it is being played.
//
// It uses IntoSampleRate::dynamic instead of IntoSampleRate::new which accepts
// an Arc<AtomicUsize> instead of a usize. This lets you change the rate while
// the iterator is being consumed.
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
  let rate = Arc::new(AtomicUsize::new(in_rate));

  let source1 = IntoSampleRate::dynamic(Arc::clone(&rate), out_rate, in_channels, decoder);
  let source2 = IntoChannels::new(in_channels, out_channels, source1);

  mixer.add(source2);

  // Double the rate every 0.5 seconds.
  while mixer.is_playing() {
      sleep(Duration::from_millis(500));

      let current = rate.load(Relaxed);
      rate.store(current * 2, Relaxed)
  }
}

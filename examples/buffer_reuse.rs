use audio_mixer::{AudioMixer, IntoChannels, IntoSampleRate, OggDecoder, ReusableBuffer};
use std::{io::Cursor, thread::sleep, time::Duration};

// This example shows how to reuse a buffer to avoid having to keep decoding the
// same ogg data every time you want to play a sound.
//
// The ReusableBuffer struct doesn't let you change the data once it's been
// created so that the audio thread can access it without a Mutex.

fn main() {
  let decoder = OggDecoder::new(Cursor::new(include_bytes!("./ogg_file.ogg")));
  let mixer = AudioMixer::default();

  let in_channels = decoder.channels();
  let out_channels = mixer.channels();

  let in_rate = decoder.sample_rate();
  let out_rate = mixer.sample_rate();

  let source1 = IntoSampleRate::new(in_rate, out_rate, in_channels, decoder);
  let source2 = IntoChannels::new(in_channels, out_channels, source1);

  let buffer = ReusableBuffer::new(source2.collect());

  // Play the same sound 5 times, staggered by 0.5 seconds.
  for _ in 0..5 {
      mixer.add(buffer.reuse()); // Or buffer.clone()
      sleep(Duration::from_millis(1000));
  }

  mixer.wait();
}

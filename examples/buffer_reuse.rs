use audio_mixer::{AudioMixer, IntoChannels, IntoSampleRate, OggDecoder, ReusableBuffer};
use std::{io::Cursor, thread::sleep, time::Duration};

// This example shows how to reuse a buffer to avoid having to keep decoding the
// same ogg data every time you want to play a sound.
//
// The ReusableBuffer struct writes to an internal buffer Vec the first time it
// is used and then iterates over this buffer on subsequent uses.

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

  let buffer = ReusableBuffer::new(0, source2.collect());

  // Play the same sound 5 times, staggered by 0.5 seconds.
  for _ in 0..5 {
      mixer.add(buffer.reuse_from(0));
      sleep(Duration::from_millis(1000));
  }

  mixer.wait();
}

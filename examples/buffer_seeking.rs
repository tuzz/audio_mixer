use audio_mixer::*;
use std::{io::Cursor, thread::sleep, time::Duration};

// This example shows how to seek to a specific point in a buffer so that audio
// plays from that point. It pases dynamic controls to the ReusableBuffer::new
// and ReusableBuffer::reuse_buffer functions then modifies them.
//
// See examples/dynamic_controls.rs for more explanation of dynamic controls.
//
// The 'seek' parameter is the number of samples to seek to in the buffer, so if
// the audio plays at 48KHz and has 2 channels, you'd need to seek to 96_000 to
// start the audio playing from the 1 second mark. The seek parameter can be
// controlled but it is also incremented by ReusableBuffer::next().
//
// Keep in mind that ReusableBuffer calls .next() on its source iterator if it
// hasn't populated that far into its internal buffer yet. If you seek ahead a
// long way, this can cause a lot of calls on the source iterator which might
// take a while, e.g. when seeking 1 minute into the decoder for an Ogg file. In
// this case, you might want to call buffer.next() immediately after creating
// the buffer to perform this work ahead of time.
//
// Also, if a buffer is added to a mixer and it reaches the end at any point by
// return a sample of None, the mixer will remove it and seeking it back to the
// start won't play it again. Instead, reuse the buffer and add it again.

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

  let samples_in_1_second = mixer.sample_rate() * mixer.channels();

  let seek1 = DynamicUsize::new(0);
  let seek2 = DynamicUsize::new(samples_in_1_second); // Start buffer2 at 1 second.

  let buffer1 = ReusableBuffer::new(seek1.clone(), source2.collect());
  let buffer2 = buffer1.reuse_from(seek2.clone());

  // Seek buffer1 to the beginning after it has played for 1 second.
  mixer.add(buffer1);
  sleep(Duration::from_millis(1000));
  seek1.set(0);
  mixer.wait();

  // Seek buffer2 to the 1 second mark after it has played for 1 second.
  mixer.add(buffer2);
  sleep(Duration::from_millis(1000));
  seek2.set(samples_in_1_second);
  mixer.wait();
}

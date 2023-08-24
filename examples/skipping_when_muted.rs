use audio_mixer::*;
use std::{io::Cursor, thread::sleep, time::Duration};

// This example demonstrates the SkipWhenMuted optimization. This iterator does
// not advance its source iterator when the volume is muted which can save
// unnecessary work in the iterator chain when nothing would be audible anyway.
//
// When the volume is un-muted again, it skips forward using the seek parameter
// of a reusable buffer so that playback resumes as though it had been playing
// while muted. For a simpler version that doesn't do this, see PauseWhenMuted.
//
// The seek_ratio is needed if the number of channels and sample rate differs
// between the point when the ReusableBuffer iterator is used and the point when
// the SkipWhenMuted iterator is used. For every 1 sample advanced later in the
// chain, it needs to know how far to advance the ReusableBuffer.
//
// The peek parameter is optional (0 to disable) and instructs SkipWhenMuted to
// peek at its source iterator every n output samples to see if it has finished.
// This can help reduce load in AudioMixer because otherwise the SkipWhenMuted
// iterator would never finish if it stays muted forever. If this happens a lot,
// these zombied iterators can start to slow AudioMixer down.
//
// If you want to resume when un-muted again, see PauseWhenMuted.
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
  let seek = DynamicUsize::new(0);

  // Important: This needs to be (in / out) rather than (out / in) to skip to the right place.
  // Try switching these around and notice it doesn't seek far enough when un-muted.
  let seek_ratio = (in_channels * in_rate) as f32 / (out_channels * out_rate) as f32;
  //let seek_ratio = (out_channels * out_rate) as f32 / (in_channels * in_rate) as f32;

  // See the comment at the top of this file. Peek every 1 second.
  let peek = out_channels * out_rate;

  let source1 = AdjustVolume::new(volume.clone(), decoder);
  let source2 = IntoSampleRate::new(in_rate * 2, out_rate, in_channels, source1);
  let source3 = IntoChannels::new(in_channels, out_channels, source2);

  // Add the optimization right at the end of the chain of iterators so that
  // it bypasses work performed by those earlier in the chain.
  let source4 = SkipWhenMuted::new(volume.clone(), seek, seek_ratio, peek, out_channels, source3);

  println!("Playing while muted");
  mixer.add(source4);

  sleep(Duration::from_millis(1000));
  println!("Un-muting after 1 second");
  volume.set(1.);

  mixer.wait();
  println!("Playback was skipped so should have started part-way through.");
}

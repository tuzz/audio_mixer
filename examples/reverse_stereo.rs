use audio_mixer::{AudioMixer, IntoChannels, IntoSampleRate, OggDecoder};
use std::io::Cursor;

// This example is discussed in the README. This makes the assumption that the
// reverse_stereo.ogg file is a stereo source (it is) but if it wasn't we'd
// first need to use the IntoChannels iterator to convert it into one.

fn main() {
  let cursor = Cursor::new(include_bytes!("./reverse_stereo.ogg"));
  let decoder = OggDecoder::new(cursor).unwrap();
  let mixer = AudioMixer::for_default_device().unwrap();

  let in_channels = decoder.channels();
  let out_channels = mixer.channels();

  let in_rate = decoder.sample_rate();
  let out_rate = mixer.sample_rate();

  let reversed = ReverseStereo::new(decoder);
  let source1 = IntoSampleRate::new(in_rate, out_rate, in_channels, reversed);
  let source2 = IntoChannels::new(in_channels, out_channels, source1);

  mixer.add(source2);
  mixer.wait();
}

struct ReverseStereo<S: Iterator<Item=f32>> {
    stereo_source: S,
    left_sample: Option<f32>,
}

impl<S: Iterator<Item=f32>> ReverseStereo<S> {
    pub fn new(stereo_source: S) -> Self {
        Self { stereo_source, left_sample: None }
    }
}

impl<S: Iterator<Item=f32>> Iterator for ReverseStereo<S> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if let Some(right_sample) = self.left_sample.take() {
            Some(right_sample)
        } else {
            // Samples are channel-interlaced so this works by stashing the left
            // sample on self and yielding the right one in its place. The
            // iterator then yields the stashed sample on the next call.

            self.left_sample = self.stereo_source.next();
            let right_sample = self.stereo_source.next();

            right_sample
        }
    }
}

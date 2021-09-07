use audio_mixer::{AudioMixer, IntoChannels, IntoSampleRate, OggDecoder};
use std::io::Cursor;

// The ogg file used in this example is stereo and has a sample rate of 44100 Hz.
// The output device might have a different number of channels and sample rate.
//
// The 'Into' structs make these compatible with each other so that the audio
// plays at the right speed and on the expected channels.
//
// The rodio crate does this conversion for you but this crate is minimal so you
// must apply these 'Into' conversions yourself (if you want).

fn main() {
  let decoder = OggDecoder::new(Cursor::new(include_bytes!("./ogg_file.ogg")));
  let mixer = AudioMixer::default();

  let in_channels = decoder.channels();
  let out_channels = mixer.channels();

  let in_rate = decoder.sample_rate();
  let out_rate = mixer.sample_rate();

  let source1 = IntoSampleRate::new(in_rate, out_rate, in_channels, decoder);
  let source2 = IntoChannels::new(in_channels, out_channels, source1);

  mixer.add(source2);
  mixer.wait();
}

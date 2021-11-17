use audio_mixer::{AudioMixer, IntoChannels, IntoSampleRate, WavDecoder};
use hound::{WavWriter, WavSpec, SampleFormat};
use std::io::Cursor;

// This example reads from a wav file, plays it at half pitch through the
// default audio device and records the result to a wav file.
//
// The AudioMixer::start_recording method takes a closure. Whenever the audio
// device requests the next chunk of audio to be played, this chunk is passed
// into the closure with some extra metadata (e.g. elapsed_time). This example
// then uses the hound crate to write this uncompressed audio to a wav file.
//
// AudioMixer does not implement audio recording as an iterator, instead opting
// to handle it specially as a first-class method. This is because the point at
// which .next() is called on the iterators isn't necessarily the point when
// these samples are played on the audio device. AudioMixer tries to give an
// accurate representation of the real playback time in the elapsed_time field.

fn main() {
  let cursor = Cursor::new(include_bytes!("./wav_file.wav"));
  let decoder = WavDecoder::new(cursor).unwrap();
  let mixer = AudioMixer::for_default_device().unwrap();

  let in_channels = decoder.channels();
  let out_channels = mixer.channels();

  let in_rate = decoder.sample_rate() / 2; // Play this example at half the pitch.
  let out_rate = mixer.sample_rate();

  let source1 = IntoSampleRate::new(in_rate, out_rate, in_channels, decoder);
  let source2 = IntoChannels::new(in_channels, out_channels, source1);

  let mut writer = WavWriter::create("haunting.wav", WavSpec {
      channels: out_channels as u16,
      sample_rate: out_rate as u32,
      sample_format: SampleFormat::Float,
      bits_per_sample: 32,
  }).unwrap();

  mixer.start_recording(Box::new(move |audio_frame| {
      println!("{:?}: Recording frame {} which contains {} samples", audio_frame.elapsed_time, audio_frame.frame_number, audio_frame.audio_data.len());

      for sample in audio_frame.audio_data.iter() {
          writer.write_sample(*sample).unwrap();
      }
  }));

  mixer.add(source2);
  mixer.wait();

  println!("Written to haunting.wav");
}

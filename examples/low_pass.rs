use audio_mixer::*;
use std::{io::Cursor, thread::sleep, time::Duration};

// This example uses a LowPassFilter to filter out frequencies above a given
// threshold value. The threshold frequency is changed over time using a dynamic
// control - a pattern that is explained more in examples/dynamic_controls.rs.
//
// In order to use LowPassFilter, you first have to precompute some mathematical
// coefficients that it needs. These calculations are relatively expensive
// because they use sine and cosine functions as well as division so this
// implementation requires they be computed in advance so that the audio thread
// is kept as lean as possible. We certainly could compute these on-demand but
// this should keep processing times down and reduce the likelihood of stutter.
//
// If no coefficient is available for the current threshold frequency and sample
// rate, LowPassFilter will "no op" and return the original sample.
//
// To precompute coefficients, you need to tell it which frequencies you'd like
// to use as the max threshold value, here we're using 20_000 which is the
// audible range of frequencies for humans. Additionally, you need to tell it
// which sample rates will be filtered, which will usually match the sample rate
// of the output device (mixer.sample_rate()).
//
// Each set of coefficients is five f32 values, so the precomputed coefficients
// below consume 5 * 32 * 20_000 = 3,200,000 bits of memory (391KB) which isn't
// much but it's worth keeping in mind if you're precomputing for a large range
// of sample rates as well. Once precomputed, the coefficients can be reused
// for all LowPassFilter iterators by calling coefficients.clone_arc().

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

  // Make the sound loop so we can hear the threshold frequency change over time.
  // Iterators make this really easy because they come with lots of great methods.
  let looping = source2.collect::<Vec<_>>().into_iter().cycle();

  // Start with the threshold frequency at 0 which filters out everything.
  let threshold = DynamicUsize::new(0);

  let coefficients = LowPassCoefficients::new([out_rate].into_iter(), 20_000);
  let source3 = LowPassFilter::new(threshold.clone(), out_channels, out_rate, looping, coefficients.clone_arc());

  mixer.add(source3);

  for i in 0..2_000 {
      sleep(Duration::from_millis(10));

      // Gradually ramp up the threshold, then gradually ramp it back down again.
      let step = if i < 1000 { 10 } else { -10 };
      let new_value = threshold.get() as i32 + step;

      println!("Playing frequencies below {}", new_value);
      threshold.set(new_value as usize);
  }
}

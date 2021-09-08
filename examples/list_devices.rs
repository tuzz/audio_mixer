use audio_mixer::AudioMixer;
use cpal::traits::DeviceTrait;

// AudioMixer provides a method to list the available output devices. These
// should have names associated with them but possibly not.

fn main() {
    let devices = AudioMixer::output_devices();
    let plural = if devices.len() > 1 { "s" } else { "" };

    println!("\nDetected {} output device{} for the current system:", devices.len(), plural);

    for device in &devices {
        println!("  - {:?}", device.name());
    }

    println!();

    // You can build the audio mixer for a specific device with:
    // AudioMixer::for_device(&devices[0]);
}

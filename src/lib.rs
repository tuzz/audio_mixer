#![feature(drain_filter)]
#![feature(bool_to_option)]

mod adjust_volume;
mod audio_mixer;
mod dynamic_controls;
mod into_channels;
mod into_sample_rate;
mod reusable_buffer;

pub use adjust_volume::AdjustVolume;
pub use audio_mixer::AudioMixer;
pub use dynamic_controls::{DynamicUsize, DynamicFloat, MaybeDynamic};
pub use into_channels::IntoChannels;
pub use into_sample_rate::IntoSampleRate;
pub use reusable_buffer::ReusableBuffer;

#[cfg(feature = "ogg")] mod ogg_decoder;
#[cfg(feature = "ogg")] pub use ogg_decoder::*;

use std::mem::swap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use atomic_float::AtomicF32;

use cpal::{default_host, Device, Sample, SampleFormat, Stream, SupportedStreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

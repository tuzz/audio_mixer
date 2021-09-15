#![feature(drain_filter)]
#![feature(bool_to_option)]
#![feature(once_cell)]
#![feature(trait_alias)]

mod adjust_balance;
mod adjust_volume;
mod audio_mixer;
mod dynamic_controls;
mod into_channels;
mod into_sample_rate;
mod low_pass_coefficients;
mod low_pass_filter;
mod pausable_audio;
mod reusable_buffer;
mod stoppable_audio;

pub use adjust_balance::AdjustBalance;
pub use adjust_volume::AdjustVolume;
pub use audio_mixer::AudioMixer;
pub use dynamic_controls::{DynamicBool, DynamicUsize, DynamicFloat, MaybeDynamic};
pub use into_channels::IntoChannels;
pub use into_sample_rate::IntoSampleRate;
pub use low_pass_coefficients::{LowPassCoefficients, LOW_PASS_COEFFICIENTS};
pub use low_pass_filter::LowPassFilter;
pub use pausable_audio::PausableAudio;
pub use reusable_buffer::ReusableBuffer;
pub use stoppable_audio::StoppableAudio;

#[cfg(feature = "ogg")] mod ogg_decoder;
#[cfg(feature = "ogg")] pub use ogg_decoder::*;

use std::collections::HashMap;
use std::f32::consts::PI;
use std::lazy::SyncOnceCell;
use std::mem::swap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::sleep;
use std::time::Duration;

use atomic_float::AtomicF32;

use cpal::{Stream, SupportedStreamConfig, DefaultStreamConfigError};
use cpal::{available_hosts, default_host, host_from_id};
use cpal::{Device, Sample, SampleFormat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#![feature(extract_if)]
#![feature(trait_alias)]

mod adjust_balance;
mod adjust_volume;
mod audio_mixer;
mod audio_recorder;
mod dynamic_controls;
mod into_channels;
mod into_sample_rate;
mod low_pass_coefficients;
mod low_pass_filter;
mod pausable_audio;
mod pause_when_muted;
mod reusable_buffer;
mod skip_when_muted;
mod stop_when_muted;
mod stoppable_audio;

pub use adjust_balance::AdjustBalance;
pub use adjust_volume::AdjustVolume;
pub use audio_mixer::AudioMixer;
pub use audio_recorder::{AudioRecorder, AudioFrame};
pub use dynamic_controls::{DynamicBool, DynamicUsize, DynamicFloat, MaybeDynamic};
pub use into_channels::IntoChannels;
pub use into_sample_rate::IntoSampleRate;
pub use low_pass_coefficients::LowPassCoefficients;
pub use low_pass_filter::LowPassFilter;
pub use pausable_audio::PausableAudio;
pub use pause_when_muted::PauseWhenMuted;
pub use reusable_buffer::ReusableBuffer;
pub use skip_when_muted::SkipWhenMuted;
pub use stop_when_muted::StopWhenMuted;
pub use stoppable_audio::StoppableAudio;
pub use cpal;

#[cfg(feature = "ogg")] mod ogg_decoder;
#[cfg(feature = "ogg")] pub use ogg_decoder::*;
#[cfg(feature = "ogg")] pub use lewton;

#[cfg(feature = "wav")] mod wav_decoder;
#[cfg(feature = "wav")] pub use wav_decoder::*;
#[cfg(feature = "wav")] pub use hound;

use std::borrow::Cow;
use std::f32::consts::PI;
use std::mem::{swap, transmute};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use atomic_float::AtomicF32;

use cpal::{Stream, SupportedStreamConfig, DefaultStreamConfigError};
use cpal::{available_hosts, default_host, host_from_id};
use cpal::{Device, Sample, SampleFormat, OutputCallbackInfo, StreamInstant};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use cpal::{default_host, Device, Sample, SampleFormat, Stream, SupportedStreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{sync::{Arc, Mutex}, thread::sleep, time::Duration};

pub struct AudioMixer {
    channels: usize,
    sample_rate: usize,
    inner: Arc<Mutex<Inner>>,
    _stream: Stream,
}

struct Inner {
    channels: usize,
    sample_count: usize,
    pending: Vec<Box<dyn Iterator<Item=f32> + Send>>,
    playing: Vec<Box<dyn Iterator<Item=f32> + Send>>,
}

impl AudioMixer {
    pub fn for_device(device: &Device) -> Self {
        let config = device.default_output_config().unwrap();

        let channels = config.channels() as usize;
        let sample_rate = config.sample_rate().0 as usize;

        let inner = Arc::new(Mutex::new(Inner {
            channels,
            sample_count: 0,
            pending: vec![],
            playing: vec![],
        }));

        let _stream = match config.sample_format() {
            SampleFormat::F32 => Self::build_stream::<f32>(device, config, inner.clone()),
            SampleFormat::I16 => Self::build_stream::<i16>(device, config, inner.clone()),
            SampleFormat::U16 => Self::build_stream::<u16>(device, config, inner.clone()),
        };

        Self { channels, sample_rate, inner, _stream }
    }

    pub fn add<S: Iterator<Item=f32> + Send + 'static>(&self, source: S) -> &Self {
        self.inner.lock().unwrap().pending.push(Box::new(source)); self
    }

    pub fn wait(&self) {
        loop {
            if let Ok(inner) = self.inner.try_lock() {
                if inner.pending.is_empty() && inner.playing.is_empty() {
                    break;
                }
            }

            sleep(Duration::from_millis(10));
        }
    }

    pub fn is_playing(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        !inner.pending.is_empty() || !inner.playing.is_empty()
    }

    pub fn channels(&self) -> usize {
        self.channels
    }

    pub fn sample_rate(&self) -> usize {
        self.sample_rate
    }

    fn build_stream<S: Sample>(device: &Device, config: SupportedStreamConfig, inner: Arc<Mutex<Inner>>) -> Stream {
        let config = &config.into();

        let stream = device.build_output_stream::<S, _, _>(config, move |out, _| {
            let mut inner = inner.lock().unwrap();
            out.iter_mut().for_each(|o| *o = Sample::from(&inner.next().unwrap()));
        }, |error| {
            eprintln!("output stream error: {}", error);
        }).unwrap();

        stream.play().unwrap();
        stream
    }
}

impl Default for AudioMixer {
    fn default() -> Self {
        Self::for_device(&default_host().default_output_device().unwrap())
    }
}

impl Iterator for Inner {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let in_sync = self.sample_count % self.channels == 0;
        if in_sync { self.playing.append(&mut self.pending); }

        let mut total = 0.;

        self.playing.drain_filter(|s| s.next().map(|f| total += f).is_none());
        self.sample_count += 1;

        Some(total)
    }
}

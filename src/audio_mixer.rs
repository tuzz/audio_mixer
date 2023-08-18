use crate::*;

pub struct AudioMixer {
    channels: usize,
    sample_rate: usize,
    inner: Arc<Mutex<Inner>>,
    _stream: Stream,
}

struct Inner {
    channels: usize,
    sample_rate: usize,
    sample_count: usize,
    pending: Vec<Box<dyn Iterator<Item=f32> + Send>>,
    playing: Vec<Box<dyn Iterator<Item=f32> + Send>>,
    recorder: Option<AudioRecorder>,
}

impl AudioMixer {
    pub fn for_default_device() -> Result<Self, DefaultStreamConfigError> {
        if let Some(device) = default_host().default_output_device() {
            Self::for_device(&device)
        } else {
            Err(DefaultStreamConfigError::DeviceNotAvailable)
        }
    }

    pub fn for_device(device: &Device) -> Result<Self, DefaultStreamConfigError> {
        let config = device.default_output_config()?;

        let channels = config.channels() as usize;
        let sample_rate = config.sample_rate().0 as usize;

        let inner = Arc::new(Mutex::new(Inner {
            channels,
            sample_rate,
            sample_count: 0,
            pending: vec![],
            playing: vec![],
            recorder: None,
        }));

        let _stream = match config.sample_format() {
            SampleFormat::F32 => Self::build_stream::<f32>(device, config, inner.clone()),
            SampleFormat::I16 => Self::build_stream::<i16>(device, config, inner.clone()),
            SampleFormat::U16 => Self::build_stream::<u16>(device, config, inner.clone()),
        };

        Ok(Self { channels, sample_rate, inner, _stream })
    }

    pub fn set_device(&self, device: &Device) -> Result<Self, DefaultStreamConfigError> {
        let mut inner = self.inner.lock().unwrap();

        if inner.recorder.is_some() {
            panic!("Please stop recording before calling AudioMixer::set_device");
        }

        let config = device.default_output_config()?;
        let channels = config.channels() as usize;

        while inner.sample_count % channels != 0 { inner.next(); }

        Self::for_device(device)
    }

    pub fn output_devices() -> Vec<cpal::Device> {
        let mut output_devices = vec![];

        for host_id in available_hosts() {
            let host = match host_from_id(host_id) { Ok(h) => h, _ => continue };
            let devices = match host.devices() { Ok(d) => d, _ => continue };

            for device in devices {
                let configs = match device.supported_output_configs() { Ok(o) => o, _ => continue };
                let has_output = configs.into_iter().next().is_some();

                if has_output { output_devices.push(device); }
            }
        }

        output_devices
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

    pub fn start_recording(&self, process_function: Box<dyn FnMut(crate::AudioFrame)>) {
        self.inner.lock().unwrap().recorder = Some(AudioRecorder::new(process_function));
    }

    pub fn stop_recording(&self) {
        self.inner.lock().unwrap().recorder = None;
    }

    fn build_stream<S: Sample>(device: &Device, config: SupportedStreamConfig, inner: Arc<Mutex<Inner>>) -> Stream {
        let config = &config.into();

        let stream = device.build_output_stream::<S, _, _>(config, move |out, info| {
            let mut inner = inner.lock().unwrap();
            out.iter_mut().for_each(|o| *o = Sample::from(&inner.next().unwrap()));

            let (channels, sample_rate) = (inner.channels, inner.sample_rate);
            inner.recorder.as_mut().map(|r| r.record(out, info, channels, sample_rate));
        }, |error| {
            eprintln!("output stream error: {}", error);
        }).unwrap();

        stream.play().unwrap();
        stream
    }
}

impl Iterator for Inner {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let in_sync = self.sample_count % self.channels == 0;
        if in_sync { self.playing.append(&mut self.pending); }

        let mut total = 0.;

        self.playing.retain_mut(|s| s.next().map(|f| total += f).is_some());
        self.sample_count += 1;

        Some(total)
    }
}

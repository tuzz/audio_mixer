use crate::*;

pub struct AudioRecorder {
    start_time: Option<StreamInstant>,
    process_function: Box<dyn FnMut(AudioFrame)>,
    frame_number: usize,
}

impl AudioRecorder {
    pub fn new(process_function: Box<dyn FnMut(AudioFrame)>) -> Self {
        Self { start_time: None, process_function, frame_number: 0 }
    }

    pub fn record<S: Sample>(&mut self, samples: &[S], info: &OutputCallbackInfo, channels: usize, sample_rate: usize) {
        self.frame_number += 1;

        let audio_data = into_f32_samples(samples);
        let frame_number = self.frame_number;

        let start_time = self.start_time.get_or_insert_with(|| info.timestamp().callback);
        let current_time = info.timestamp().playback;
        let elapsed_time = current_time.duration_since(start_time).unwrap();

        (self.process_function)(AudioFrame {
            audio_data, channels, sample_rate, frame_number, elapsed_time,
        });
    }
}

fn into_f32_samples<S: Sample>(samples: &[S]) -> Cow<[f32]> {
    if let SampleFormat::F32 = S::FORMAT {
        Cow::Borrowed(unsafe { transmute::<&[S], &[f32]>(samples) })
    } else {
        Cow::Owned(samples.iter().map(|s| s.to_f32()).collect())
    }
}

#[derive(Debug)]
pub struct AudioFrame<'a> {
    pub audio_data: Cow<'a, [f32]>,

    pub channels: usize,
    pub sample_rate: usize,

    pub frame_number: usize,
    pub elapsed_time: Duration,
}

unsafe impl Send for AudioRecorder {}
unsafe impl Sync for AudioRecorder {}

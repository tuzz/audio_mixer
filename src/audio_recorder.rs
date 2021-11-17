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

        let audio_data = samples.iter().map(|s| s.to_f32()).collect();
        let frame_number = self.frame_number;

        let start_time = self.start_time.get_or_insert_with(|| info.timestamp().callback);
        let current_time = info.timestamp().playback;
        let elapsed_time = current_time.duration_since(start_time).unwrap();

        (self.process_function)(AudioFrame {
            audio_data, channels, sample_rate, frame_number, elapsed_time,
        });
    }
}

#[derive(Debug)]
pub struct AudioFrame {
    pub audio_data: Vec<f32>,

    pub channels: usize,
    pub sample_rate: usize,

    pub frame_number: usize,
    pub elapsed_time: Duration,
}

unsafe impl Send for AudioRecorder {}
unsafe impl Sync for AudioRecorder {}

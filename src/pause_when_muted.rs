use crate::*;

pub struct PauseWhenMuted<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> {
    volume: V,
    channels: usize,
    source: S,
    is_muted: bool,
    num_paused: usize,
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> PauseWhenMuted<V, S> {
    pub fn new(volume: V, channels: usize, source: S) -> Self {
        let is_muted = volume.get() <= 0.;

        Self { volume, channels, source, is_muted, num_paused: 0 }
    }
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> Iterator for PauseWhenMuted<V, S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let previously_muted = self.is_muted;
        let currently_muted = self.volume.get() <= 0.;

        let unpause = previously_muted && !currently_muted;
        self.is_muted = currently_muted;

        if unpause {
            while self.num_paused % self.channels != 0 {
                if let None = self.source.next() { return None; }
                self.num_paused += 1;
            }
        }

        if currently_muted {
            self.num_paused += 1;
            Some(0.)
        } else {
            self.source.next()
        }
    }
}

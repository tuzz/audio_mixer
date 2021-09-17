use crate::*;

pub struct SkipWhenMuted<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> {
    volume: V,
    source: S,
    seek: DynamicUsize,
    seek_ratio: f32,
    is_muted: bool,
    num_skipped: usize,
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> SkipWhenMuted<V, S> {
    pub fn new(volume: V, seek: DynamicUsize, seek_ratio: f32, source: S) -> Self {
        let is_muted = volume.get() <= 0.;

        Self { volume, source, seek_ratio, seek, is_muted, num_skipped: 0 }
    }
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> Iterator for SkipWhenMuted<V, S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let previously_muted = self.is_muted;
        let currently_muted = self.volume.get() <= 0.;

        let seek_ahead = previously_muted && !currently_muted;
        self.is_muted = currently_muted;

        if seek_ahead {
            let seek_by = (self.num_skipped as f32 * self.seek_ratio) as usize;

            self.seek.set(self.seek.get() + seek_by);
            self.num_skipped = 0;
        }

        if currently_muted {
            self.num_skipped += 1;
            Some(0.)
        } else {
            self.source.next()
        }
    }
}

use crate::*;

pub struct SkipWhenMuted<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> {
    volume: V,
    source: S,
    seek: DynamicUsize,
    seek_ratio: f32,
    peek: usize,
    channels: usize,
    strategy: fn(&mut Self) -> Option<f32>,
    is_muted: bool,
    num_skipped: usize,
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> SkipWhenMuted<V, S> {
    pub fn new(volume: V, seek: DynamicUsize, seek_ratio: f32, peek: usize, channels: usize, source: S) -> Self {
        let strategy = if peek == 0 { Self::skip_without_peeking } else { Self::skip_with_peeking };
        let is_muted = volume.get() <= 0.;

        Self { volume, source, seek_ratio, seek, peek, channels, strategy, is_muted, num_skipped: 0 }
    }

    fn skip_without_peeking(&mut self) -> Option<f32> {
        let previously_muted = self.is_muted;
        let currently_muted = self.volume.get() <= 0.;

        let seek_ahead = previously_muted && !currently_muted;
        self.is_muted = currently_muted;

        if seek_ahead {
            let mut seek_by = (self.num_skipped as f32 * self.seek_ratio) as usize;
            seek_by -= seek_by % self.channels;

            self.seek.add(seek_by);
            self.num_skipped = 0;
        }

        if currently_muted {
            self.num_skipped += 1;
            Some(0.)
        } else {
            self.source.next()
        }
    }

    fn skip_with_peeking(&mut self) -> Option<f32> {
        let previously_muted = self.is_muted;
        let currently_muted = self.volume.get() <= 0.;

        let peek_now = currently_muted && self.num_skipped % self.peek == 0;
        let seek_ahead = previously_muted && !currently_muted || peek_now;
        self.is_muted = currently_muted;

        if seek_ahead {
            let mut seek_by = (self.num_skipped as f32 * self.seek_ratio) as usize;
            seek_by -= seek_by % self.channels;

            self.seek.add(seek_by);
            self.num_skipped = 0;

            if peek_now {
                for _ in 0..self.channels {
                    if self.source.next().is_none() { return None; }
                }
            }
        }

        if currently_muted {
            self.num_skipped += 1;
            Some(0.)
        } else {
            self.source.next()
        }
    }
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> Iterator for SkipWhenMuted<V, S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.strategy)(self)
    }
}

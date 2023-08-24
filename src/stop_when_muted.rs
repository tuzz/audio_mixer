use crate::*;

pub struct StopWhenMuted<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> {
    volume: V,
    source: S,
    strategy: fn(&mut Self) -> Option<f32>,
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> StopWhenMuted<V, S> {
    pub fn new(volume: V, source: S) -> Self {
        let strategy = match (V::is_static(), volume.get() <= 0.) {
            (true, true)  => Self::always_emit_none,
            (true, false) => Self::noop,
            (false, _)    => Self::check_when_muted_or_ended_then_fuse,
        };

        Self { volume, source, strategy }
    }

    fn always_emit_none(&mut self) -> Option<f32> {
        None
    }

    fn noop(&mut self) -> Option<f32> {
        self.source.next()
    }

    fn check_when_muted_or_ended_then_fuse(&mut self) -> Option<f32> {
        let sample = self.source.next();

        if sample.is_none() || self.volume.get() <= 0. {
            self.strategy = Self::always_emit_none; // Behaves like Iterator::fuse.
        }

        sample
    }
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> Iterator for StopWhenMuted<V, S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.strategy)(self)
    }
}

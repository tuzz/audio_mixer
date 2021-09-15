use crate::*;

pub struct StoppableAudio<X: MaybeDynamic<bool>, S: Iterator<Item=f32>> {
    stopped: X,
    source: S,
    strategy: fn(&mut Self) -> Option<f32>,
}

impl<X: MaybeDynamic<bool>, S: Iterator<Item=f32>> StoppableAudio<X, S> {
    pub fn new(stopped: X, source: S) -> Self {
        let strategy = match (X::is_static(), stopped.get()) {
            (true, true)  => Self::always_emit_none,
            (true, false) => Self::noop,
            (false, _)    => Self::check_when_stopped_or_ended_then_fuse,
        };

        Self { stopped, source, strategy }
    }

    fn always_emit_none(&mut self) -> Option<f32> {
        None
    }

    fn noop(&mut self) -> Option<f32> {
        self.source.next()
    }

    fn check_when_stopped_or_ended_then_fuse(&mut self) -> Option<f32> {
        let sample = self.source.next();

        if sample.is_none() || self.stopped.get() {
            self.stopped.set(true);
            self.strategy = Self::always_emit_none; // Behaves like Iterator::fuse.
        }

        sample
    }
}

impl<X: MaybeDynamic<bool>, S: Iterator<Item=f32>> Iterator for StoppableAudio<X, S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.strategy)(self)
    }
}

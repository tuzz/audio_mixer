use crate::*;

pub struct PausableAudio<P: MaybeDynamic<bool>, S: Iterator<Item=f32>> {
    is_paused: bool,
    should_pause: P,
    channels: usize,
    source: S,
    strategy: fn(&mut Self) -> Option<f32>,
    counter: usize,
}

impl<P: MaybeDynamic<bool>, S: Iterator<Item=f32>> PausableAudio<P, S> {
    pub fn new(should_pause: P, channels: usize, source: S) -> Self {
        let strategy = match (P::is_static(), should_pause.get()) {
            (true, true)  => Self::always_emit_silence,
            (true, false) => Self::noop,
            (false, _)    => Self::emit_silence_when_paused,
        };

        Self { is_paused: should_pause.get(), should_pause, channels, source, strategy, counter: 0 }
    }

    fn always_emit_silence(&mut self) -> Option<f32> {
        Some(0.)
    }

    fn noop(&mut self) -> Option<f32> {
        self.source.next()
    }

    fn emit_silence_when_paused(&mut self) -> Option<f32> {
        if self.counter == 0 && self.is_paused != self.should_pause.get() {
            self.is_paused = !self.is_paused;
        }

        self.counter = (self.counter + 1) % self.channels;
        if self.is_paused { Some(0.) } else { self.source.next() }
    }
}

impl<P: MaybeDynamic<bool>, S: Iterator<Item=f32>> Iterator for PausableAudio<P, S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.strategy)(self)
    }
}

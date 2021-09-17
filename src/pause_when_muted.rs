use crate::*;

pub struct PauseWhenMuted<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> {
    volume: V,
    source: S,
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> PauseWhenMuted<V, S> {
    pub fn new(volume: V, source: S) -> Self {
        Self { volume, source }
    }
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> Iterator for PauseWhenMuted<V, S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.volume.get() <= 0. {
            Some(0.)
        } else {
            self.source.next()
        }
    }
}

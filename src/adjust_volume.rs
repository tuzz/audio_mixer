use crate::*;

pub struct AdjustVolume<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> {
    volume: V,
    source: S,
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> AdjustVolume<V, S> {
    pub fn new(volume: V, source: S) -> Self {
        Self { volume, source }
    }
}

impl<V: MaybeDynamic<f32>, S: Iterator<Item=f32>> Iterator for AdjustVolume<V, S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(|s| s * self.volume.get())
    }
}

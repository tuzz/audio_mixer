use crate::*;

pub struct ReusableBuffer<K: MaybeDynamic<usize>, S: Iterator<Item=f32>> {
    seek: K,
    inner: Arc<RwLock<Inner<S>>>,
}

struct Inner<S: Iterator<Item=f32>> {
    source: S,
    buffer: Vec<f32>,
}

impl<K: MaybeDynamic<usize>, S: Iterator<Item=f32>> ReusableBuffer<K, S> {
    pub fn new(seek: K, source: S) -> Self {
        let inner = Inner { source, buffer: vec![] };

        Self { seek, inner: Arc::new(RwLock::new(inner)) }
    }

    pub fn reuse_from<L: MaybeDynamic<usize>>(&self, seek: L) -> ReusableBuffer<L, S> {
        ReusableBuffer { seek, inner: Arc::clone(&self.inner) }
    }
}

impl<K: MaybeDynamic<usize>, S: Iterator<Item=f32>> Iterator for ReusableBuffer<K, S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let seek = self.seek.get();
        let inner = self.inner.read().unwrap();

        let sample = inner.buffer.get(seek).copied();
        let sample = sample.or_else(|| {
            drop(inner);

            let mut inner = self.inner.write().unwrap();
            let num_missing = seek - inner.buffer.len() + 1;

            for _ in 0..num_missing {
                if let Some(sample) = inner.source.next() {
                    inner.buffer.push(sample);
                } else {
                    return None;
                }
            }

            Some(inner.buffer[seek])
        });

        self.seek.add(1);
        sample
    }
}

use crate::*;

pub struct ReusableBuffer<S: MaybeDynamic<usize>> {
    seek: S,
    source: Arc<Vec<f32>>,
    counter: usize,
    strategy: fn(&mut Self) -> Option<f32>,
}

impl<S: MaybeDynamic<usize>> ReusableBuffer<S> {
    pub fn new(seek: S, source: Vec<f32>) -> Self {
        let source = Arc::new(source);
        let counter = seek.get();
        let strategy = if S::is_static() { Self::without_seeking } else { Self::with_seeking };

        Self { seek, source, counter, strategy }
    }

    pub fn reuse_from<K: MaybeDynamic<usize>>(&self, seek: K) -> ReusableBuffer<K> {
        let source = Arc::clone(&self.source);
        let counter = seek.get();
        let strategy = if K::is_static() { ReusableBuffer::without_seeking } else { ReusableBuffer::with_seeking };

        ReusableBuffer { source, seek, counter, strategy }
    }

    pub fn len(&self) -> usize {
        self.source.len()
    }

    pub fn with_seeking(&mut self) -> Option<f32> {
        if let Some(sample) = self.source.get(self.seek.get()) {
            self.seek.add(1);
            Some(*sample)
        } else {
            // Don't switch strategy in case seek is set lower.
            None
        }
    }

    pub fn without_seeking(&mut self) -> Option<f32> {
        if let Some(sample) = self.source.get(self.counter) {
            self.counter += 1;
            Some(*sample)
        } else {
            self.strategy = Self::always_emit_none;
            None
        }
    }

    pub fn always_emit_none(&mut self) -> Option<f32> {
        None
    }
}

impl<S: MaybeDynamic<usize>> Iterator for ReusableBuffer<S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.strategy)(self)
    }
}

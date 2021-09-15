use crate::*;

pub struct ReusableBuffer<S: Iterator<Item=f32>> {
    inner: Arc<RwLock<Inner<S>>>,
    index: usize,
}

struct Inner<S: Iterator<Item=f32>> {
    source: S,
    buffer: Vec<f32>,
}

impl<S: Iterator<Item=f32>> ReusableBuffer<S> {
    pub fn new(source: S) -> Self {
        let inner = Inner { source, buffer: vec![] };

        Self { inner: Arc::new(RwLock::new(inner)), index: 0 }
    }

    pub fn reuse(&self) -> Self {
        self.clone()
    }
}

impl<S: Iterator<Item=f32>> Iterator for ReusableBuffer<S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner.read().unwrap();
        let sample = inner.buffer.get(self.index).copied();

        drop(inner);
        let sample = sample.or_else(|| {
            let mut inner = self.inner.write().unwrap();
            inner.source.next().map(|s| { inner.buffer.push(s); s })
        });

        self.index += 1;
        sample
    }
}

impl<S: Iterator<Item=f32>> Clone for ReusableBuffer<S> {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner), index: 0 }
    }
}

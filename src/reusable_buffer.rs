use std::sync::Arc;

// Sometimes it's useful to write samples into a buffer so that you can reuse
// that buffer a number of times, e.g. to play the same sound multiple times.
//
// Otherwise, you'd have to read data from a file and decode it every time you
// wanted to play a sound which causes unnecessary work and IO operations.
//
// This struct lets you collect the intermediate results of a pipeline of
// iterators into a Vec then subsequently iterate over that Vec multiple times.

pub struct ReusableBuffer {
    buffer: Arc<Vec<f32>>,
    index: usize,
}

impl ReusableBuffer {
    pub fn new(data: Vec<f32>) -> Self {
        Self { buffer: Arc::new(data), index: 0 }
    }

    pub fn reuse(&self) -> Self {
        self.clone()
    }
}

impl Clone for ReusableBuffer {
    fn clone(&self) -> Self {
        Self { buffer: Arc::clone(&self.buffer), index: 0 }
    }
}

impl Iterator for ReusableBuffer {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.buffer.get(self.index).cloned();

        self.index += 1;
        sample
    }
}

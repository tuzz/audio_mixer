use crate::*;

#[derive(Clone)]
pub struct DynamicUsize {
    pub value: Arc<AtomicUsize>,
}

impl DynamicUsize {
    pub fn new(initial_value: usize) -> Self {
        Self { value: Arc::new(AtomicUsize::new(initial_value)) }
    }

    pub fn get(&self) -> usize {
        self.value.load(Ordering::Relaxed)
    }

    pub fn set(&self, new_value: usize) {
        self.value.store(new_value, Ordering::Relaxed);
    }
}

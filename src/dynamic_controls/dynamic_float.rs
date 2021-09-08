use crate::*;

#[derive(Clone)]
pub struct DynamicFloat {
    pub value: Arc<AtomicF32>,
}

impl DynamicFloat {
    pub fn new(initial_value: f32) -> Self {
        Self { value: Arc::new(AtomicF32::new(initial_value)) }
    }

    pub fn get(&self) -> f32 {
        self.value.load(Ordering::Relaxed)
    }

    pub fn set(&self, new_value: f32) {
        self.value.store(new_value, Ordering::Relaxed);
    }
}

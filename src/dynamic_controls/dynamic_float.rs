use crate::*;

#[derive(Clone, Debug, Default)]
pub struct DynamicFloat {
    value: Arc<AtomicF32>,
}

impl DynamicFloat {
    pub fn new(initial_value: f32) -> Self {
        let value = Arc::new(AtomicF32::new(initial_value));

        Self { value }
    }

    pub fn get(&self) -> f32 {
        self.value.load(Ordering::Relaxed)
    }

    pub fn set(&self, new_value: f32) {
        self.value.store(new_value, Ordering::Relaxed);
    }

    pub fn add(&self, amount: f32) {
        self.value.fetch_add(amount, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_gets_and_sets_values() {
        let dynamic = DynamicFloat::new(123.);
        assert_eq!(dynamic.get(), 123.);

        dynamic.set(456.);
        assert_eq!(dynamic.get(), 456.);
    }
}

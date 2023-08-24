use crate::*;

#[derive(Clone, Debug, Default)]
pub struct DynamicUsize {
    value: Arc<AtomicUsize>,
}

impl DynamicUsize {
    pub fn new(initial_value: usize) -> Self {
        let value = Arc::new(AtomicUsize::new(initial_value));

        Self { value }
    }

    pub fn get(&self) -> usize {
        self.value.load(Ordering::Relaxed)
    }

    pub fn set(&self, new_value: usize) {
        self.value.store(new_value, Ordering::Relaxed);
    }

    pub fn add(&self, amount: usize) {
        self.value.fetch_add(amount, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_gets_and_sets_values() {
        let dynamic = DynamicUsize::new(123);
        assert_eq!(dynamic.get(), 123);

        dynamic.set(456);
        assert_eq!(dynamic.get(), 456);
    }
}

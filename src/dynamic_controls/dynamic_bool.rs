use crate::*;

#[derive(Clone, Debug, Default)]
pub struct DynamicBool {
    value: Arc<AtomicBool>,
}

impl DynamicBool {
    pub fn new(initial_value: bool) -> Self {
        let value = Arc::new(AtomicBool::new(initial_value));

        Self { value }
    }

    pub fn get(&self) -> bool {
        self.value.load(Ordering::Relaxed)
    }

    pub fn set(&self, new_value: bool) {
        self.value.store(new_value, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_gets_and_sets_values() {
        let dynamic = DynamicBool::new(true);
        assert_eq!(dynamic.get(), true);

        dynamic.set(false);
        assert_eq!(dynamic.get(), false);
    }
}

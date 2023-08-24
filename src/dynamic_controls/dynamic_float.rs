use crate::*;

#[derive(Clone, Debug)]
pub struct DynamicFloat {
    value: Arc<AtomicF32>,
    cache: f32,

    num_calls_to_cache_for: usize,
    num_calls: usize,
}

impl DynamicFloat {
    pub fn new(initial_value: f32) -> Self {
        let value = Arc::new(AtomicF32::new(initial_value));
        let cache = initial_value;

        let cache_time = <Self as MaybeDynamic<f32>>::default_cache_time();
        let num_calls_to_cache_for = cache_time + 1;

        Self { value, cache, num_calls_to_cache_for, num_calls: 0 }
    }

    pub fn set_cache_time(mut self, cache_time: usize) -> Self {
        self.num_calls_to_cache_for = cache_time + 1;
        self
    }

    pub fn get(&mut self) -> f32 {
        let invalidate = self.num_calls % self.num_calls_to_cache_for == 0;
        if invalidate { self.cache = self.value.load(Ordering::Relaxed); }

        self.num_calls += 1;
        self.cache
    }

    pub fn set(&mut self, new_value: f32) {
        self.value.store(new_value, Ordering::Relaxed);
        self.cache = new_value;
    }

    pub fn add(&mut self, amount: f32) -> f32 {
        self.cache = self.value.fetch_add(amount, Ordering::Relaxed);
        self.cache
    }
}

impl Default for DynamicFloat {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_gets_and_sets_values() {
        let mut dynamic = DynamicFloat::new(123.);
        assert_eq!(dynamic.get(), 123.);

        dynamic.set(456.);
        assert_eq!(dynamic.get(), 456.);
    }
}

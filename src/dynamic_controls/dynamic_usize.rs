use crate::*;

#[derive(Clone, Debug)]
pub struct DynamicUsize {
    value: Arc<AtomicUsize>,
    cache: usize,

    num_calls_to_cache_for: usize,
    num_calls: usize,
}

impl DynamicUsize {
    pub fn new(initial_value: usize) -> Self {
        let value = Arc::new(AtomicUsize::new(initial_value));
        let cache = initial_value;

        let cache_time = <Self as MaybeDynamic<usize>>::default_cache_time();
        let num_calls_to_cache_for = cache_time + 1;

        Self { value, cache, num_calls_to_cache_for, num_calls: 0 }
    }

    pub fn set_cache_time(mut self, cache_time: usize) -> Self {
        self.num_calls_to_cache_for = cache_time + 1;
        self
    }

    pub fn get(&mut self) -> usize {
        let invalidate = self.num_calls % self.num_calls_to_cache_for == 0;
        if invalidate { self.cache = self.value.load(Ordering::Relaxed); }

        self.num_calls += 1;
        self.cache
    }

    pub fn set(&mut self, new_value: usize) {
        self.value.store(new_value, Ordering::Relaxed);
        self.cache = new_value;
    }

    pub fn add(&mut self, amount: usize) -> usize {
        self.cache = self.value.fetch_add(amount, Ordering::Relaxed);
        self.cache
    }
}

impl Default for DynamicUsize {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_gets_and_sets_values() {
        let mut dynamic = DynamicUsize::new(123);
        assert_eq!(dynamic.get(), 123);

        dynamic.set(456);
        assert_eq!(dynamic.get(), 456);
    }
}

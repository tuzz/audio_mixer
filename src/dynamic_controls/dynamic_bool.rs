use crate::*;

#[derive(Clone, Debug)]
pub struct DynamicBool {
    value: Arc<AtomicBool>,
    cache: bool,

    num_calls_to_cache_for: usize,
    num_calls: usize,
}

impl DynamicBool {
    pub fn new(initial_value: bool) -> Self {
        let value = Arc::new(AtomicBool::new(initial_value));
        let cache = initial_value;

        let cache_time = <Self as MaybeDynamic<bool>>::default_cache_time();
        let num_calls_to_cache_for = cache_time + 1;

        Self { value, cache, num_calls_to_cache_for, num_calls: 0 }
    }

    pub fn set_cache_time(mut self, cache_time: usize) -> Self {
        self.num_calls_to_cache_for = cache_time + 1;
        self
    }

    pub fn get(&mut self) -> bool {
        let invalidate = self.num_calls % self.num_calls_to_cache_for == 0;
        if invalidate { self.cache = self.value.load(Ordering::Relaxed); }

        self.num_calls += 1;
        self.cache
    }

    pub fn set(&mut self, new_value: bool) {
        self.value.store(new_value, Ordering::Relaxed);
        self.cache = new_value;
    }
}

impl Default for DynamicBool {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_gets_and_sets_values() {
        let mut dynamic = DynamicBool::new(true);
        assert_eq!(dynamic.get(), true);

        dynamic.set(false);
        assert_eq!(dynamic.get(), false);
    }
}

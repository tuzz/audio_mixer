use crate::*;

#[derive(Clone)]
pub struct DynamicUsize {
    inner: Arc<Inner>,
    seen: usize,
}

struct Inner {
    value: AtomicUsize,
    current: AtomicUsize,
}

impl DynamicUsize {
    pub fn new(initial_value: usize) -> Self {
        let value = AtomicUsize::new(initial_value);
        let current = AtomicUsize::new(0);
        let inner = Arc::new(Inner { value, current });

        Self { inner, seen: 0 }
    }

    pub fn get(&self) -> usize {
        self.inner.value.load(Ordering::Relaxed)
    }

    pub fn set(&self, new_value: usize) {
        let value = self.inner.value.load(Ordering::Relaxed);
        if new_value == value { return; }

        self.inner.value.store(new_value, Ordering::Relaxed);
        self.inner.current.fetch_add(1, Ordering::Relaxed);
    }

    pub fn handle_change<F: FnMut(usize)>(&mut self, mut handler_function: F) {
        let current = self.inner.current.load(Ordering::Relaxed);
        if self.seen == current { return; }

        handler_function(self.inner.value.load(Ordering::Relaxed));
        self.seen = current;
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

    #[test]
    fn it_calls_the_handler_if_the_value_has_changed() {
        let mut dynamic = DynamicUsize::new(123);
        let mut calls = 0;

        dynamic.handle_change(|_| calls += 1); assert_eq!(calls, 0);
        dynamic.handle_change(|_| calls += 1); assert_eq!(calls, 0);

        dynamic.set(456);

        dynamic.handle_change(|_| calls += 1); assert_eq!(calls, 1);
        dynamic.handle_change(|_| calls += 1); assert_eq!(calls, 1);

        dynamic.set(456); // Did not change.

        dynamic.handle_change(|_| calls += 1); assert_eq!(calls, 1);
        dynamic.handle_change(|_| calls += 1); assert_eq!(calls, 1);
    }

    #[test]
    fn it_shares_the_value_across_clones_but_tracks_changes_separately() {
        let mut dynamic1 = DynamicUsize::new(123);
        let mut dynamic2 = dynamic1.clone();

        dynamic1.set(456);

        assert_eq!(dynamic1.get(), 456);
        assert_eq!(dynamic2.get(), 456);

        let mut calls1 = 0;
        let mut calls2 = 0;

        dynamic1.handle_change(|_| calls1 += 1); assert_eq!(calls1, 1);
        dynamic2.handle_change(|_| calls2 += 1); assert_eq!(calls2, 1);
    }
}

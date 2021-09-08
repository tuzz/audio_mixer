use crate::*;

// TODO: consider adding a changed field that is set whenever the atomic's value
// is changed so that the iterators can decide whether or not they need to
// recompute derived values. At present, this isn't needed because the iterators
// are simple and checking the flag would probably be more costly than just
// recomputing the values.
//
// If this flag was added, we could also add an on_change convenience method
// that is called in the iterators, which clears the changed flag afterwards.
// However, this would mean we couldn't reuse the same DynamicFloat in multiple
// iterators because we'd mistakenly reset the flag before all iterators have
// seen it. In that case, we could use generational indexing instead.

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

use crate::*;

pub trait MaybeDynamic<T>: Clone + Default {
    fn get(&mut self) -> T;
    fn set(&mut self, new_value: T);
    fn add(&mut self, amount: T) -> T;
    fn is_dynamic() -> bool { false }
    fn is_static() -> bool { !Self::is_dynamic() }
    fn default_cache_time() -> usize { 441 } // ~10ms based on 44100Hz audio.
    fn set_cache_time(self, cache_time: usize) -> Self;
}

impl MaybeDynamic<usize> for usize {
    fn get(&mut self) -> usize { *self }
    fn set(&mut self, new_value: usize) { *self = new_value; }
    fn add(&mut self, amount: usize) -> usize { *self += amount; *self }
    fn set_cache_time(self, _cache_time: usize) -> Self { panic!(); }
}

impl MaybeDynamic<usize> for DynamicUsize {
    fn get(&mut self) -> usize { self.get() }
    fn set(&mut self, new_value: usize) { DynamicUsize::set(self, new_value); }
    fn add(&mut self, amount: usize) -> usize { DynamicUsize::add(self, amount) }
    fn is_dynamic() -> bool { true }
    fn set_cache_time(self, cache_time: usize) -> Self { self.set_cache_time(cache_time) }
}

impl MaybeDynamic<f32> for f32 {
    fn get(&mut self) -> f32 { *self }
    fn set(&mut self, new_value: f32) { *self = new_value; }
    fn add(&mut self, amount: f32) -> f32 { *self += amount; *self }
    fn set_cache_time(self, _cache_time: usize) -> Self { panic!(); }
}

impl MaybeDynamic<f32> for DynamicFloat {
    fn get(&mut self) -> f32 { self.get() }
    fn set(&mut self, new_value: f32) { DynamicFloat::set(self, new_value); }
    fn add(&mut self, amount: f32) -> f32 { DynamicFloat::add(self, amount) }
    fn is_dynamic() -> bool { true }
    fn set_cache_time(self, cache_time: usize) -> Self { self.set_cache_time(cache_time) }
}

impl MaybeDynamic<bool> for bool {
    fn get(&mut self) -> bool { *self }
    fn set(&mut self, new_value: bool) { *self = new_value; }
    fn add(&mut self, _amount: bool) -> bool { panic!(); }
    fn set_cache_time(self, _cache_time: usize) -> Self { panic!(); }
}

impl MaybeDynamic<bool> for DynamicBool {
    fn get(&mut self) -> bool { self.get() }
    fn set(&mut self, new_value: bool) { DynamicBool::set(self, new_value); }
    fn add(&mut self, _amount: bool) -> bool { panic!(); }
    fn is_dynamic() -> bool { true }
    fn set_cache_time(self, cache_time: usize) -> Self { self.set_cache_time(cache_time) }
}

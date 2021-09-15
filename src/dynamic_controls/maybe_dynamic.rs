use crate::*;

pub trait MaybeDynamic<T> {
    fn get(&self) -> T;
    fn set(&self, _new_value: T) { panic!(); }
    fn handle_change<F: FnMut(T)>(&mut self, _f: F) {}
    fn is_dynamic() -> bool { false }
    fn is_static() -> bool { !Self::is_dynamic() }
}

impl MaybeDynamic<usize> for usize {
    fn get(&self) -> usize { *self }
}

impl MaybeDynamic<usize> for DynamicUsize {
    fn get(&self) -> usize { self.get() }
    fn set(&self, new_value: usize) { self.set(new_value); }
    fn handle_change<F: FnMut(usize)>(&mut self, f: F) { self.handle_change(f); }
    fn is_dynamic() -> bool { true }
}

impl MaybeDynamic<f32> for f32 {
    fn get(&self) -> f32 { *self }
}

impl MaybeDynamic<f32> for DynamicFloat {
    fn get(&self) -> f32 { self.get() }
    fn set(&self, new_value: f32) { self.set(new_value); }
    fn handle_change<F: FnMut(f32)>(&mut self, f: F) { self.handle_change(f); }
    fn is_dynamic() -> bool { true }
}

impl MaybeDynamic<bool> for bool {
    fn get(&self) -> bool { *self }
}

impl MaybeDynamic<bool> for DynamicBool {
    fn get(&self) -> bool { self.get() }
    fn set(&self, new_value: bool) { self.set(new_value); }
    fn handle_change<F: FnMut(bool)>(&mut self, f: F) { self.handle_change(f); }
    fn is_dynamic() -> bool { true }
}

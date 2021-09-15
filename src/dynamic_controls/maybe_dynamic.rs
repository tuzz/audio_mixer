use crate::*;

pub trait MaybeDynamic<T> {
    fn get(&self) -> T;
    fn handle_change<F: FnMut(T)>(&mut self, _f: F) {}
}

impl MaybeDynamic<usize> for usize {
    fn get(&self) -> usize { *self }
}

impl MaybeDynamic<usize> for DynamicUsize {
    fn get(&self) -> usize { self.get() }
    fn handle_change<F: FnMut(usize)>(&mut self, f: F) { self.handle_change(f); }
}

impl MaybeDynamic<f32> for f32 {
    fn get(&self) -> f32 { *self }
}

impl MaybeDynamic<f32> for DynamicFloat {
    fn get(&self) -> f32 { self.get() }
    fn handle_change<F: FnMut(f32)>(&mut self, f: F) { self.handle_change(f); }
}

use crate::*;

pub trait MaybeDynamic<T>: Clone + Default {
    fn get(&self) -> T;
    fn set(&mut self, new_value: T);
    fn add(&mut self, amount: T);
    fn is_dynamic() -> bool { false }
    fn is_static() -> bool { !Self::is_dynamic() }
}

impl MaybeDynamic<usize> for usize {
    fn get(&self) -> usize { *self }
    fn set(&mut self, new_value: usize) { *self = new_value; }
    fn add(&mut self, amount: usize) { *self += amount; }
}

impl MaybeDynamic<usize> for DynamicUsize {
    fn get(&self) -> usize { self.get() }
    fn set(&mut self, new_value: usize) { DynamicUsize::set(self, new_value); }
    fn add(&mut self, amount: usize) { DynamicUsize::add(self, amount); }
    fn is_dynamic() -> bool { true }
}

impl MaybeDynamic<f32> for f32 {
    fn get(&self) -> f32 { *self }
    fn set(&mut self, new_value: f32) { *self = new_value; }
    fn add(&mut self, amount: f32) { *self += amount; }
}

impl MaybeDynamic<f32> for DynamicFloat {
    fn get(&self) -> f32 { self.get() }
    fn set(&mut self, new_value: f32) { DynamicFloat::set(self, new_value); }
    fn add(&mut self, amount: f32) { DynamicFloat::add(self, amount); }
    fn is_dynamic() -> bool { true }
}

impl MaybeDynamic<bool> for bool {
    fn get(&self) -> bool { *self }
    fn set(&mut self, new_value: bool) { *self = new_value; }
    fn add(&mut self, _amount: bool) { panic!() }
}

impl MaybeDynamic<bool> for DynamicBool {
    fn get(&self) -> bool { self.get() }
    fn set(&mut self, new_value: bool) { DynamicBool::set(self, new_value); }
    fn add(&mut self, _amount: bool) { panic!() }
    fn is_dynamic() -> bool { true }
}

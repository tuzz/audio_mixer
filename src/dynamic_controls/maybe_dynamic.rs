use crate::*;

pub trait MaybeDynamic<T> {
    fn get(&self) -> T;
}

impl MaybeDynamic<usize> for usize {
    fn get(&self) -> usize { *self }
}

impl MaybeDynamic<usize> for DynamicUsize {
    fn get(&self) -> usize { self.get() }
}

impl MaybeDynamic<f32> for f32 {
    fn get(&self) -> f32 { *self }
}

impl MaybeDynamic<f32> for DynamicFloat {
    fn get(&self) -> f32 { self.get() }
}

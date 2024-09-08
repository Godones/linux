mod rref;
use core::{
    alloc::Layout,
    any::{type_name_of_val, TypeId},
};
/// A trait for types that can be shared between domains.
///
/// # Safety
/// This trait is unsafe because it is not safe to share all types between domains.
pub unsafe auto trait RRefable {}

impl<T> !RRefable for *mut T {}
impl<T> !RRefable for *const T {}
impl<T> !RRefable for &T {}
impl<T> !RRefable for &mut T {}
impl<T> !RRefable for [T] {}

pub trait TypeIdentifiable {
    fn type_id() -> TypeId;
}

impl<T: 'static> TypeIdentifiable for T {
    fn type_id() -> TypeId {
        TypeId::of::<T>()
    }
}

pub trait CustomDrop {
    fn custom_drop(&mut self);
}

impl<T: RRefable> CustomDrop for T {
    default fn custom_drop(&mut self) {
        // log::warn!("default for {}", type_name_of_val(self));
    }
}
impl<T: RRefable> CustomDrop for Option<T> {
    fn custom_drop(&mut self) {
        if let Some(val) = self {
            val.custom_drop();
        }
    }
}

impl<T: RRefable, const N: usize> CustomDrop for [T; N] {
    fn custom_drop(&mut self) {
        for el in self.iter_mut() {
            el.custom_drop();
        }
    }
}

pub trait SharedData {
    fn move_to(&self, new_domain_id: u64) -> u64;
}

impl<T: RRefable> SharedData for T {
    default fn move_to(&self, _new_domain_id: u64) -> u64 {
        0
    }
}

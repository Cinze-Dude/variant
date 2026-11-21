use crate::variant::Variant;
use std::any::{Any, TypeId};

impl<T: Any, U: Any> Variant<T, U> {
    pub fn from_any<X: Any>(value: X) -> Option<Self> {
        let value = Box::new(value) as Box<dyn Any>;

        if TypeId::of::<X>() == TypeId::of::<T>() {
            Some(Variant::First(*value.downcast::<T>().unwrap()))
        } else if TypeId::of::<X>() == TypeId::of::<U>() {
            Some(Variant::Second(*value.downcast::<U>().unwrap()))
        } else {
            None
        }
    }
}

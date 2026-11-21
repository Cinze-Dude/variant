use crate::variant::Variant::{self, First, Second};

impl<T, U> Variant<T, U> {
    pub fn replace_first(&mut self, value: T) -> Option<T> {
        if let First(v) = self {
            Some(std::mem::replace(v, value))
        } else {
            None
        }
    }

    pub fn replace_second(&mut self, value: U) -> Option<U> {
        if let Second(v) = self {
            Some(std::mem::replace(v, value))
        } else {
            None
        }
    }
}

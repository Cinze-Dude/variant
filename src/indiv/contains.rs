use crate::variant::Variant::{self, First, Second};

impl<T, U> Variant<T, U> {
    pub fn contains_first(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        if let First(x) = &self {
            x == &value
        } else {
            false
        }
    }

    pub fn contains_second(&self, value: U) -> bool
    where
        U: PartialEq,
    {
        if let Second(x) = &self {
            x == &value
        } else {
            false
        }
    }
}

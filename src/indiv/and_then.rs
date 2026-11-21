use crate::variant::Variant::{self, First, Second};

impl<T, U> Variant<T, U> {
    pub fn and_then_first<F, O>(self, op: F) -> Variant<O, U>
    where
        F: FnOnce(T) -> Variant<O, U>,
    {
        match self {
            First(x) => op(x),
            Second(x) => Second(x),
        }
    }

    pub fn and_then_second<F, O>(self, op: F) -> Variant<T, O>
    where
        F: FnOnce(U) -> Variant<T, O>,
    {
        match self {
            First(x) => First(x),
            Second(x) => op(x),
        }
    }
}

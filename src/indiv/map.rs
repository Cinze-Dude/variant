use crate::variant::Variant::{self, First, Second};

impl<T, U> Variant<T, U> {
    pub fn map_first<O, F>(self, op: F) -> Variant<O, U>
    where
        F: FnOnce(T) -> O,
    {
        match self {
            First(x) => First(op(x)),
            Second(x) => Second(x),
        }
    }

    pub fn map_first_or<O, F>(self, op: F, default: U) -> Variant<O, U>
    where
        F: FnOnce(T) -> O,
    {
        match self {
            First(x) => First(op(x)),
            Second(_) => Second(default),
        }
    }

    pub fn map_first_or_else<O, F, D>(self, op: F, default: D) -> Variant<O, U>
    where
        F: FnOnce(T) -> O,
        D: FnOnce() -> U,
    {
        match self {
            First(x) => First(op(x)),
            Second(_) => Second(default()),
        }
    }

    pub fn map_second<O, F>(self, op: F) -> Variant<T, O>
    where
        F: FnOnce(U) -> O,
    {
        match self {
            First(x) => First(x),
            Second(x) => Second(op(x)),
        }
    }

    pub fn map_second_or<O, F>(self, op: F, default: T) -> Variant<T, O>
    where
        F: FnOnce(U) -> O,
    {
        match self {
            First(_) => First(default),
            Second(x) => Second(op(x)),
        }
    }

    pub fn map_second_or_else<O, F, D>(self, op: F, default: D) -> Variant<T, O>
    where
        F: FnOnce(U) -> O,
        D: FnOnce() -> T,
    {
        match self {
            First(_) => First(default()),
            Second(x) => Second(op(x)),
        }
    }
}

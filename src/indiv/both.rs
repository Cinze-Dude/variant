use crate::variant::Variant::{self, First, Second};

impl<T, U> Variant<T, U> {
    pub fn collapse(self) -> Option<T>
    where
        U: Into<T>,
    {
        match self {
            First(v) => Some(v),
            Second(v) => Some(v.into()),
        }
    }

    pub fn flip(self) -> Variant<U, T> {
        match self {
            First(x) => Second(x),
            Second(x) => First(x),
        }
    }

    pub fn fold<F, S, O>(self, f: F, s: S) -> O
    where
        F: FnOnce(T) -> O,
        S: FnOnce(U) -> O,
    {
        match self {
            First(x) => f(x),
            Second(x) => s(x),
        }
    }

    pub fn as_ref(&self) -> Variant<&T, &U> {
        match self {
            First(x) => Variant::First(x),
            Second(x) => Variant::Second(x),
        }
    }
}

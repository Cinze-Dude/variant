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

    pub fn as_ref(&self) -> Variant<&T, &U> {
        match self {
            First(x) => Variant::First(x),
            Second(x) => Variant::Second(x),
        }
    }
}

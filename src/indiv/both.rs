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
}

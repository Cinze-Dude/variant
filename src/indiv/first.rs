use crate::variant::Variant::{self, First, Second};

impl<T, U> Variant<T, U> {
    pub fn into_first(self) -> Option<T> {
        if let First(v) = self { Some(v) } else { None }
    }

    pub fn as_first(&self) -> Option<&T> {
        if let First(v) = self { Some(v) } else { None }
    }

    pub fn as_mut_first(&mut self) -> Option<&mut T> {
        if let First(v) = self { Some(v) } else { None }
    }

    pub fn is_first(&self) -> bool {
        matches!(self, First(_))
    }

    pub fn unwrap_first(self) -> T {
        self.into_first().unwrap()
    }

    pub fn unwrap_first_or(self, value: T) -> T {
        self.into_first().unwrap_or(value)
    }

    pub fn unwrap_first_or_default(self) -> T
    where
        T: Default,
    {
        self.into_first().unwrap_or_default()
    }

    pub fn unwrap_first_or_else<F>(self, op: F) -> T
    where
        F: Fn() -> T,
    {
        self.into_first().unwrap_or_else(op)
    }

    pub fn map_first<O, F>(self, op: F) -> Variant<O, U>
    where
        F: FnOnce(T) -> O,
    {
        match self {
            First(x) => First(op(x)),
            Second(x) => Second(x),
        }
    }
}

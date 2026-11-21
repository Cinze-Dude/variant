use crate::variant::Variant::{self, First, Second};

impl<T, U> Variant<T, U> {
    pub fn into_second(self) -> Option<U> {
        if let Second(v) = self { Some(v) } else { None }
    }

    pub fn as_second(&self) -> Option<&U> {
        if let Second(v) = self { Some(v) } else { None }
    }

    pub fn as_mut_second(&mut self) -> Option<&mut U> {
        if let Second(v) = self { Some(v) } else { None }
    }

    pub fn is_second(&self) -> bool {
        matches!(self, Second(_))
    }

    pub fn unwrap_second(self) -> U {
        self.into_second().unwrap()
    }

    pub fn unwrap_second_or(self, value: U) -> U {
        self.into_second().unwrap_or(value)
    }

    pub fn unwrap_second_or_default(self) -> U
    where
        U: Default,
    {
        self.into_second().unwrap_or_default()
    }

    pub fn unwrap_second_or_else<F>(self, op: F) -> U
    where
        F: FnOnce() -> U,
    {
        self.into_second().unwrap_or_else(op)
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

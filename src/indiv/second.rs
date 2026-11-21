use crate::variant::Variant::{self, First, Second};

impl<T, U> Variant<T, U> {
    pub fn into_second(self) -> Option<U> {
        if let Second(v) = self { Some(v) } else { None }
    }

    pub fn unwrap_second(self) -> U {
        self.into_second().unwrap()
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

    pub fn map_second<O, F>(self, op: F) -> Variant<T, O>
    where
        F: Fn(U) -> O,
    {
        match self {
            First(x) => First(x),
            Second(x) => Second(op(x)),
        }
    }
}

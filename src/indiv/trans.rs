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
}

use crate::variant::Variant::{self, First, Second};

impl<T, U> Variant<T, U> {
    // First Functions
    pub fn into_first(self) -> Option<T> {
        if let First(v) = self { Some(v) } else { None }
    }

    pub fn unwrap_first(self) -> T {
        self.into_first().unwrap()
    }

    pub fn as_first(&self) -> Option<&T> {
        if let First(v) = self { Some(v) } else { None }
    }

    pub fn is_first(&self) -> bool {
        matches!(self, First(_))
    }

    // Second Functions
    pub fn into_second(self) -> Option<U> {
        if let Second(v) = self { Some(v) } else { None }
    }

    pub fn unwrap_second(self) -> U {
        self.into_second().unwrap()
    }

    pub fn as_second(&self) -> Option<&U> {
        if let Second(v) = self { Some(v) } else { None }
    }

    pub fn is_second(&self) -> bool {
        matches!(self, Second(_))
    }
}

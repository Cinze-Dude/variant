use crate::variant::Variant::{self, First};

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

    pub fn as_mut_first(&mut self) -> Option<&mut T> {
        if let First(v) = self { Some(v) } else { None }
    }

    pub fn is_first(&self) -> bool {
        matches!(self, First(_))
    }
}

use crate::variant::Variant::{self, First, Second};

impl<T, U> Variant<T, U> {
    pub fn from_result(r: Result<T, U>) -> Self {
        match r {
            Ok(k) => First(k),
            Err(e) => Second(e),
        }
    }

    pub fn into_result(self) -> Result<T, U> {
        match self {
            First(k) => Ok(k),
            Second(e) => Err(e),
        }
    }
}

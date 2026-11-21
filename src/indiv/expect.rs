use crate::variant::Variant::{self, First, Second};

impl<T, U> Variant<T, U> {
    pub fn expect_first(self, msg: &str) -> T {
        self.into_first().expect(msg)
    }

    pub fn expect_second(self, msg: &str) -> U {
        self.into_second().expect(msg)
    }

    pub fn inspect_first<F>(self, op: F) -> Self
    where
        F: FnOnce(&T),
    {
        if let First(ref x) = self {
            op(x)
        }
        self
    }

    pub fn inspect_second<F>(self, op: F) -> Self
    where
        F: FnOnce(&U),
    {
        if let Second(ref x) = self {
            op(x)
        }
        self
    }
}

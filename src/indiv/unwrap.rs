use crate::variant::Variant;

impl<T, U> Variant<T, U> {
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
        F: FnOnce() -> T,
    {
        self.into_first().unwrap_or_else(op)
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
}

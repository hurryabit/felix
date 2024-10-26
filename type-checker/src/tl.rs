use std::marker::PhantomData;

pub enum False {}

pub enum True {}

pub enum Unknown {}

#[derive(Clone)]
pub struct None<T> {
    phantom: PhantomData<T>,
}

#[allow(non_snake_case)]
pub fn None<T>() -> None<T> {
    None {
        phantom: PhantomData,
    }
}

#[derive(Clone)]
pub struct Some<T>(pub T);

impl<T> Some<T> {
    pub fn unwrap(self) -> T {
        self.0
    }
}

pub trait Bool {
    type Option<T>: AsOption<Inner = T>;
}

impl Bool for False {
    type Option<T> = None<T>;
}

impl Bool for True {
    type Option<T> = Some<T>;
}

impl Bool for Unknown {
    type Option<T> = Option<T>;
}

pub trait AsOption {
    type Inner;

    fn as_option(self) -> Option<Self::Inner>;
}

impl<T> AsOption for None<T> {
    type Inner = T;

    fn as_option(self) -> Option<Self::Inner> {
        Option::None
    }
}

impl<T> AsOption for Some<T> {
    type Inner = T;

    fn as_option(self) -> Option<Self::Inner> {
        Option::Some(self.0)
    }
}

impl<T> AsOption for Option<T> {
    type Inner = T;

    fn as_option(self) -> Option<Self::Inner> {
        self
    }
}

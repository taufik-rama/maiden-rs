#![doc = include_str!("../README.md")]

pub trait Replace {
    fn replace_with(&mut self, other: Self);
}

impl<T> Replace for Option<T>
where
    T: Replace,
{
    fn replace_with(&mut self, other: Self) {
        if let Some(other) = other {
            self.as_mut().map(|val| val.replace_with(other));
        }
    }
}

#[cfg(feature = "derive")]
pub use replace_derive::*;

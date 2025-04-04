//! A trait to convert any error into a `Wrapp`.

use crate::Wrapp;

/// A trait to convert any error into a `Wrapp`.
pub trait IntoWrapp<E>
where
    E: std::error::Error,
{
    /// Convert the error into a `Wrapp`.
    /// If rust can infer the type, you should use the `Into::into` method instead.
    fn into_wrapp(self) -> Wrapp<E>;
}

impl<E> IntoWrapp<E> for E
where
    E: std::error::Error + 'static,
{
    fn into_wrapp(self) -> Wrapp<E> {
        Wrapp::new(self)
    }
}

impl<E> From<E> for Wrapp<E>
where
    E: std::error::Error + 'static,
{
    fn from(value: E) -> Self {
        value.into_wrapp()
    }
}

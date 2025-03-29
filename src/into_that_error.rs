use crate::ThatError;

pub trait IntoThatError<E>
where
    E: std::error::Error,
{
    fn into_that_error(self) -> ThatError<E>;
}

impl<E> IntoThatError<E> for E
where
    E: std::error::Error,
{
    fn into_that_error(self) -> ThatError<E> {
        ThatError::new(self)
    }
}

impl<E> From<E> for ThatError<E>
where
    E: std::error::Error,
{
    fn from(value: E) -> Self {
        value.into_that_error()
    }
}

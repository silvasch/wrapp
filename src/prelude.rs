pub use crate::{IntoThatError, ThatError};

pub type Result<T, E> = std::result::Result<T, ThatError<E>>;

pub use crate::{IntoWrapp, Wrapp};

/// A type alias for `Result<T, Wrapp<E>>`.
pub type Result<T, E> = std::result::Result<T, Wrapp<E>>;

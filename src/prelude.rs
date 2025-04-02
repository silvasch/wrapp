pub use crate::{IntoWrapp, Wrapp};

pub type Result<T, E> = std::result::Result<T, Wrapp<E>>;

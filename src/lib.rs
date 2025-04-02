#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Add additional functionality to Rust errors using a wrapper.
//!
//! # Example
//!
//! ```rust
//! use wrapp::prelude::*;
//!
//! #[derive(Debug)]
//! enum MyError {
//!    ReadFile,
//! }
//!
//! impl std::fmt::Display for MyError {
//!    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!       match self {
//!         MyError::ReadFile => write!(f, "could not read file"),
//!      }
//!   }
//! }
//!
//! impl std::error::Error for MyError {}
//!
//! if let Err(e) = std::fs::read_to_string("non_existent_file.txt")
//!     .map_err(|e| MyError::ReadFile.into_wrapp().with_source(Box::new(e))) {
//!    eprintln!("{}", e.full_display());
//! }
//! ```
mod into_wrapp;
pub use into_wrapp::IntoWrapp;

pub mod prelude;

mod wrapp;
pub use wrapp::Wrapp;

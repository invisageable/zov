pub mod error;

/// The result type of the whole compiler.
pub type Result<T> = anyhow::Result<T, error::Error>;

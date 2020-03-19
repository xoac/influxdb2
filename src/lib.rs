//! Generate `README.md` with `cargo readme > README.md`

// #![deny(missing_docs)]

pub mod bucket;
mod client;
pub mod error;
mod id;
mod precision;
mod timestamp;
pub mod write;

pub use client::Client;
pub use precision::Precision;

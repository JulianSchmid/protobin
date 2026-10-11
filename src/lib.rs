#![doc = include_str!("../README.md")]
#![warn(clippy::undocumented_unsafe_blocks, clippy::missing_panics_doc)]

/// Datatypes for message building.
pub mod builders;

/// Datatypes for message decoding.
pub mod decode;

/// Low level "wire" data types for encoding & decoding.
pub mod wire;

mod field_number;
pub use field_number::*;

mod field_number_into_error;
pub use field_number_into_error::*;

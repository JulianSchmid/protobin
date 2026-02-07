#![doc = include_str!("../README.md")]

/// Datatypes for message building.
pub mod builders;

/// Datatypes for message decoding.
pub mod decode;

/// Low level "wire" data types for encoding & decoding.
pub mod wire;

mod field_number;
pub use field_number::*;

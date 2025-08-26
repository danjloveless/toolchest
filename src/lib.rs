#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![deny(unsafe_code)]

//! # toolchest - Essential Utility Collection for Rust
//!
//! A comprehensive collection of utility functions that complement itertools.
//! While itertools handles collection manipulation, toolchest provides everything else.
//!
//! ## Quick Start
//!
//! ```rust
//! use toolchest::prelude::*;
//!
//! // String manipulation
//! let snake = strings::to_snake_case("HelloWorld");
//!
//! // Math utilities  
//! let clamped = math::clamp(15, 0, 10);
//!
//! // Type checking
//! let is_empty = types::is_empty::<Vec<i32>>(&vec![]);
//! ```

#[cfg(feature = "std")]
pub mod strings;

#[cfg(feature = "std")]
pub mod math;

#[cfg(feature = "std")]
pub mod deep;

#[cfg(feature = "std")]
pub mod functions;

pub mod types;

#[cfg(feature = "std")]
pub mod collections;
#[cfg(feature = "std")]
pub mod encoding;
#[cfg(feature = "std")]
pub mod hash;
#[cfg(feature = "std")]
pub mod io;
pub mod prelude;
#[cfg(feature = "std")]
pub mod random;
#[cfg(feature = "std")]
pub mod time;
#[cfg(feature = "std")]
pub mod validation;

// Re-export commonly used items at crate root
#[cfg(feature = "std")]
pub use strings::{to_camel_case, to_kebab_case, to_snake_case};

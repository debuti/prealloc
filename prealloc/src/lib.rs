#![cfg_attr(not(feature = "std"), no_std)]

pub use prealloc_impl::prealloc_from_config;
// Re-export the macros
pub use paste::*;

#[cfg(not(feature = "std"))]
pub use spin::*;

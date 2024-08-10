//! [![github]](https://github.com/debuti/prealloc)&ensp;[![crates-io]](https://crates.io/crates/prealloc)&ensp;[![docs-rs]](https://docs.rs/prealloc)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! Some design patterns would request memory only at the initialization stage. The usage of these patterns,
//! specially on embedded and real-time systems where the heap usage is forbidden, is hard to implement on
//! constrained languages like Rust.
//!
//! This crate provides a macro to generate all the boilerplate needed to provide heap-like storage
//! while using static memory as the storage backend. The exposed interface allows you to manage objects
//! in this preallocated static memory, mimicking the behavior of dynamic memory allocation
//! without relying on a heap. You can safely dispatch and use items from this preallocated pool, ensuring
//! that you have tight control over memory usage with zero runtime allocation overhead.
//!
//! <br>
//!
//! # Defining the configuration
//!
//! Create a JSON configuration file (ex. `config.json`) that defines the items, their types, and counts:
//! ```json
//! [
//!     {
//!         "name": "ExampleUsage",
//!         "type": "MyEnum",
//!         "size": 5
//!     }
//! ]
//! ```
//!
//! # Accesing the preallocated memory
//!
//! ```ignore
//! use prealloc::prealloc_from_config;
//!
//! prealloc_from_config!("path/to/config.json");
//!
//! #[derive(Debug)]
//! #[allow(dead_code)]
//! pub enum MyEnum {
//!     L(u32),
//!     R,
//! }
//!
//! impl Drop for MyEnum {
//!     fn drop(&mut self) {
//!         println!("Dropping MyEnum located at {self:p}");
//!     }
//! }
//!
//! fn main() {
//!     for idx in 0..=5 {
//!         // The dispatch_static macro retrieves one preallocated element and initializes it.
//!         if let Some(item) = dispatch_static!(ExampleUsage, MyEnum::L(33)) {
//!             println!("Retrieved {idx}: {item:p}");
//!         } else {
//!             println!("ExampleUsage depleted");
//!         }
//!     }
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub use prealloc_impl::prealloc_from_config;

#[doc(hidden)]
pub use paste::*;

#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub use spin::*;

// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

//! The core Move VM logic.
//!
//! It is a design goal for the Move VM to be independent of the Diem blockchain, so that
//! other blockchains can use it as well. The VM isn't there yet, but hopefully will be there
//! soon.

#![no_std]
#[macro_use]
extern crate alloc;
#[allow(unused_imports)]
pub(crate) mod no_std {
    pub use alloc::borrow::ToOwned;
    pub use alloc::boxed::Box;
    pub use alloc::string::String;
    pub use alloc::string::ToString;
    pub use alloc::vec::Vec;
}

#[cfg(test)]
extern crate std;
pub mod data_cache;
mod interpreter;
mod loader;
pub mod logging;
pub mod move_vm;
pub mod native_extensions;
pub mod native_functions;
mod runtime;
pub mod session;
#[cfg(test)]
#[macro_use]
mod tracing;
pub mod config;

// Only include debugging functionality in debug builds
//#[cfg(any(debug_assertions, feature = "debugging"))]
#[cfg(test)]
mod debug;

#[cfg(test)]
mod unit_tests;

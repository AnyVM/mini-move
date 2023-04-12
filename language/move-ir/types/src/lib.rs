// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//! Base types for the Move IR.

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
pub mod ast;
pub mod location;
pub mod spec_language_ast;

// SPDXLicenseIdentifier: GPL2.0

//! The `kernel` prelude.
//!
//! These are the most common items used by Rust code in the kernel,
//! intended to be imported by all Rust code, for convenience.
//!
//! # Examples
//!
//! ```
//! use kernel::prelude::*;
//! ```

use core::alloc::Layout;
#[doc(no_inline)]
pub use core::pin::Pin;

pub use crate::alloc::{flags::*, Box, KBox, KVBox, KVVec, KVec, VBox, VVec, Vec};

#[doc(no_inline)]
pub use macros::{module, pin_data, pinned_drop, vtable, Zeroable};

pub use super::build_assert;

// `super::std_vendor` is hidden, which makes the macro inline for some reason.
#[doc(no_inline)]
pub use super::dbg;
pub use super::{pr_alert, pr_crit, pr_debug, pr_emerg, pr_err, pr_info, pr_notice, pr_warn};

pub use super::{init, pin_init, try_init, try_pin_init};

pub use super::static_assert;

pub use super::error::{code::*, Error, Result};

pub use super::{str::CStr, ThisModule};

pub use super::init::{InPlaceInit, InPlaceWrite, Init, PinInit};

pub use super::current;

pub use alloc::borrow::ToOwned;
pub use alloc::boxed::Box as AllocBox;

/// The Rust allocation error handler.
#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    panic!("Out of memory!");
}

/// The Rust allocation error handler.
#[no_mangle]
pub fn __rust_alloc_error_handler(_size: usize, _align: usize) -> ! {
    panic!("Out of memory!");
}

// This symbol is emitted by rustc next to __rust_alloc_error_handler.
// Its value depends on the Zoom={panic,abort} compiler option.
#[no_mangle]
static __rust_alloc_error_handler_should_panic: u8 = 1;
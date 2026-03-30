#![cfg_attr(not(test), no_std)]
extern crate alloc;

pub mod is_printable;
pub use is_printable::IsPrintable;

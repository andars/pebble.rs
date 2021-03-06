#![crate_type="staticlib"]
#![feature(intrinsics)]
#![feature(no_std)]
#![feature(core_intrinsics)]
#![feature(core_str_ext)]
#![feature(lang_items)]
#![feature(allocator)]
#![feature(alloc)]
#![no_std]

extern crate alloc;

pub use pebble::*;
pub use zero::*;

pub mod pebble;
pub mod types;
mod external;

pub mod zero;
pub mod allocation;

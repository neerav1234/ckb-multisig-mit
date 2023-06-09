//! Generated by capsule
//!
//! `main.rs` is used to define rust lang items and modules.
//! See `entry.rs` for the `main` function. 
//! See `error.rs` for the `Error` type.

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(lang_items)]
#![feature(panic_info_message)]
// define modules
mod entry;
mod error;
mod try1;

use ckb_std::default_alloc;

ckb_std::entry!(program_entry);
default_alloc!();

/// program entry
fn program_entry() -> i8 {
    // Call main function and return error code
    match try1::main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}

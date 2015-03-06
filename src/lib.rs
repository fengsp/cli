// Rust command line utility.
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

//! Cli is a Rust crate for creating beautiful command line applications.

#![crate_name = "cli"]
#![crate_type = "lib"]
#![deny(non_camel_case_types)]

extern crate getopts;

/* public api */
pub use core::Command;

mod core;
mod types;
mod utils;

// Rust command line utility.
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

//! Cli is a Rust crate for creating beautiful command line applications.

#![crate_name = "cli"]
#![crate_type = "lib"]
#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "http://fengsp.github.io/cli/")]
#![deny(non_camel_case_types)]

extern crate getopts;

/* public api */
pub use utils::sprintln;
pub use term::{
    Style,
    Color,
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        White,
    prompt,
    confirm,
};

mod utils;
mod term;

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
#![feature(collections, convert, libc)]
extern crate libc;
extern crate time;
extern crate getopts;
extern crate tempdir;

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
    get_terminal_size,
    print_via_pager,
    isatty,
    clear,
    ProgressBar,
    Editor,
};

mod core;
mod types;
mod utils;
mod term;
mod formatting;

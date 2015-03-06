// This module implements useful utilities.
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::io;

pub fn print(message: String) {
    print!("{}", message);
}


pub fn style(text: String, fg: &str, bg: &str, bold: bool, underline: bool,
             blink: bool, reverse: bool, reset: bool) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push(27 as u8);
    bytes.push('[' as u8);
    bytes.push(32 as u8);
    bytes.push('m' as u8);
    bytes.push_all(text.as_bytes());
    io::stdout().write(bytes);
}

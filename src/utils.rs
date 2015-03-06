// This module implements useful utilities.
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.


pub fn print(message: String) {
    print!("{}", message);
}


pub fn style(text: String, fg: &str, bg: &str, bold: bool, underline: bool,
             blink: bool, reverse: bool, reset: bool) -> String {
    let mut bits: Vec<u8> = Vec::new();
    "style".to_string()
}

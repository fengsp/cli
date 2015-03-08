// This module implements useful utilities.
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use term::{Color, Style};


/// Styles the message with ANSI styles and println it.
pub fn sprintln(message: String, fg: Color, bg: Color, bold: bool, dim: bool,
                underline: bool, blink: bool, reverse: bool) {
    let mut text = Style::new(message);
    text.fg(fg);
    text.bg(bg);
    text.bold(bold);
    text.dim(dim);
    text.underline(underline);
    text.blink(blink);
    text.reverse(reverse);
    println!("{}", text);
}

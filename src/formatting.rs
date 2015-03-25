// This module implements useful format related utilities.
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std;
use std::cmp::max;


/// This helps with formatting text-based help pages.
pub struct HelpFormatter {
    width: usize,
    indent_increment: usize,
    current_indent: usize,
    buffer: Vec<String>,
}

impl HelpFormatter {
    /// Create one new help formatter.
    ///
    /// - `indent_increment` - the additional increment for each level.
    /// - `width` - the width for the text.
    ///
    pub fn new(width: usize, indent_increment: usize) -> HelpFormatter {
        HelpFormatter {
            width: width,
            indent_increment: indent_increment,
            current_indent: 0,
            buffer: Vec::new(),
        }
    }

    /// Increases the indentation.
    fn indent(&mut self) {
        self.current_indent = self.current_indent + self.indent_increment;
    }

    /// Decreases the indentation.
    fn dedent(&mut self) {
        self.current_indent = self.current_indent - self.indent_increment;
    }

    /// Writes a string into the internal buffer.
    pub fn write(&mut self, s: String) {
        self.buffer.push(s);
    }

    /// Writes a usage line.
    pub fn write_usage(&mut self, name: &str, args: String, prefix: &str) {
        let prefix = format!("{:>2$}{}", prefix, name, self.current_indent);
        let prefix_len = prefix.len();
        self.write(prefix);
        let text_width = max(self.width - self.current_indent - prefix_len, 10);
        let mut indent = String::from_str(" ");
        for _ in range(0, prefix_len) {
            indent.push_str(" ");
        }
        self.write(wrap_text(args, text_width, " ", indent.as_slice()));
        self.write(String::from_str("\n"));
    }
}


/// A helper function that wraps text.
/// TODO
fn wrap_text(text: String, width: usize, initial_indent: &str, subsequent_indent: &str) -> String {
    return text;
}

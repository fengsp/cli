// This module implements useful format related utilities.
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std;
use std::cmp::max;
use std::slice::SliceConcatExt;


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
    pub fn indent(&mut self) {
        self.current_indent = self.current_indent + self.indent_increment;
    }

    /// Decreases the indentation.
    pub fn dedent(&mut self) {
        self.current_indent = self.current_indent - self.indent_increment;
    }

    /// Writes a string into the internal buffer.
    pub fn write(&mut self, s: String) {
        self.buffer.push(s);
    }

    /// Writes a paragraph into the internal buffer.
    pub fn write_paragraph(&mut self) {
        if !self.buffer.is_empty() {
            self.buffer.push(String::from_str("\n"));
        }
    }

    /// Writes a heading.
    pub fn write_heading(&mut self, heading: &str) {
        let current_indent = self.current_indent;
        self.write(format!("{:>2$}{}\n", "", heading, current_indent));
    }

    /// Writes a definition list into the buffer.
    pub fn write_dl(&mut self, rows: Vec<(String, String)>) {
        let col_spacing: usize = 2;
        let current_indent = self.current_indent;
        for &(ref first, ref second) in rows.iter() {
            self.write(format!("{:>2$}{}", "", first, current_indent));
            self.write(format!("{:>2$}{}\n", "", second, col_spacing));
        }
    }

    /// Helper function that writes a paragraph, a heading, and indent.
    pub fn enter_section(&mut self, name: &str) {
        self.write_paragraph();
        self.write_heading(name);
        self.indent();
    }

    /// Leave section indent state.
    pub fn exit_section(&mut self) {
        self.dedent();
    }

    /// Writes re-indented text into the buffer.
    pub fn write_text(&mut self, text: String) {
        let text_width = max(self.width - self.current_indent, 10);
        let mut indent = String::new();
        for _ in (0..self.current_indent) {
            indent.push_str(" ");
        }
        self.write(wrap_text(text, text_width, &indent, &indent));
        self.write(String::from_str("\n"));
    }

    /// Writes a usage line.
    pub fn write_usage(&mut self, name: &str, args: String, prefix: &str) {
        let prefix = format!("{:>2$}{}", prefix, name, self.current_indent);
        let prefix_len = prefix.len();
        self.write(prefix);
        let text_width = max(self.width - self.current_indent - prefix_len, 10);
        let mut indent = String::from_str(" ");
        for _ in (0..prefix_len) {
            indent.push_str(" ");
        }
        self.write(wrap_text(args, text_width, " ", &indent));
        self.write(String::from_str("\n"));
    }

    /// Get buffer contents.
    pub fn getvalue(&self) -> String {
        self.buffer.concat()
    }
}


/// A helper function that wraps text.
/// TODO
fn wrap_text(text: String, width: usize, initial_indent: &str, subsequent_indent: &str) -> String {
    return text;
}

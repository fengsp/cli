// This module implements useful format related utilities.
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std;


/// This helps with formatting text-based help pages.
pub struct HelpFormatter {
    width: isize,
    indent_increment: isize,
    current_indent: isize,
}

impl HelpFormatter {
    /// Create one new help formatter.
    ///
    /// - `indent_increment` - the additional increment for each level.
    /// - `width` - the width for the text.
    ///
    pub fn new(width: isize, indent_increment: isize) -> HelpFormatter {
        HelpFormatter {
            width: width,
            indent_increment: indent_increment,
            current_indent: 0,
        }
    }
}

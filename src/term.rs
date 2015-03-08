// This module implements terminal related stuff.
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::old_io;
use std::fmt;

pub use self::Color::{
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
};


pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}

impl Color {
    /// Get ANSI foreground color str.
    pub fn get_fg_str(&self) -> &str {
        match *self {
            Black   => "\x1b[30m",
            Red     => "\x1b[31m",
            Green   => "\x1b[32m",
            Yellow  => "\x1b[33m",
            Blue    => "\x1b[34m",
            Magenta => "\x1b[35m",
            Cyan    => "\x1b[36m",
            White   => "\x1b[37m",
        }
    }

    /// Get ANSI background color str.
    pub fn get_bg_str(&self) -> &str {
        match *self {
            Black   => "\x1b[40m",
            Red     => "\x1b[41m",
            Green   => "\x1b[42m",
            Yellow  => "\x1b[43m",
            Blue    => "\x1b[44m",
            Magenta => "\x1b[45m",
            Cyan    => "\x1b[46m",
            White   => "\x1b[47m",
        }
    }
}


/// Styles a text with ANSI styles.  This styling is self contained which means
/// that at the end of the string a reset code is issued.  Examples:
///
/// ```rust,no_run
/// use cli::{Style, Red};
///
/// let mut text = Style::new(String::from_str("hello"));
/// text.fg(Red);
/// println!("{}", text);
/// ```
///
pub struct Style {
    text: String,
    fg: Option<Color>,
    bg: Option<Color>,
    bold: Option<bool>,
    dim: Option<bool>,
    underline: Option<bool>,
    blink: Option<bool>,
    reverse: Option<bool>,
}

impl Style {
    /// Create one new styled string with ansi codes.
    pub fn new(text: String) -> Style {
        Style {
            text: text,
            fg: None,
            bg: None,
            bold: None,
            dim: None,
            underline: None,
            blink: None,
            reverse: None,
        }
    }

    /// Sets the foreground color.
    pub fn fg(&mut self, color: Color) {
        self.fg = Some(color);
    }

    /// Sets the background color.
    pub fn bg(&mut self, color: Color) {
        self.bg = Some(color);
    }

    /// Enable or disable bold mode.
    pub fn bold(&mut self, bold: bool) {
        self.bold = Some(bold);
    }

    /// Enable or disable dim mode.
    pub fn dim(&mut self, dim: bool) {
        self.dim = Some(dim);
    }

    /// Enable or disable underline.
    pub fn underline(&mut self, underline: bool) {
        self.underline = Some(underline);
    }

    /// Enable or disable blinking.
    pub fn blink(&mut self, blink: bool) {
        self.blink = Some(blink);
    }

    /// Enable or disable inverse rendering.
    pub fn reverse(&mut self, reverse: bool) {
        self.reverse = Some(reverse);
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.fg {
            Some(ref fg) => {
                try!(f.write_str(fg.get_fg_str()));
            },
            None => ()
        }
        match self.bg {
            Some(ref bg) => {
                try!(f.write_str(bg.get_bg_str()));
            },
            None => ()
        }
        if self.bold.is_some() {
            if self.bold.unwrap() {
                try!(f.write_str("\x1b[1m"));
            } else {
                try!(f.write_str("\x1b[22m"));
            }
        }
        if self.dim.is_some() {
            if self.dim.unwrap() {
                try!(f.write_str("\x1b[2m"));
            } else {
                try!(f.write_str("\x1b[22m"));
            }
        }
        if self.underline.is_some() {
            if self.underline.unwrap() {
                try!(f.write_str("\x1b[4m"));
            } else {
                try!(f.write_str("\x1b[24m"));
            }
        }
        if self.blink.is_some() {
            if self.blink.unwrap() {
                try!(f.write_str("\x1b[5m"));
            } else {
                try!(f.write_str("\x1b[25m"));
            }
        }
        if self.reverse.is_some() {
            if self.reverse.unwrap() {
                try!(f.write_str("\x1b[7m"));
            } else {
                try!(f.write_str("\x1b[27m"));
            }
        }
        try!(f.write_str(self.text.as_slice()));
        // Currently we always reset.
        try!(f.write_str("\x1b[0m"));

        Ok(())
    }
}


fn build_prompt_text(text: &str, suffix: &str, show_default: bool,
                     default: Option<String>) -> String {
    let prompt_text: String;
    if default.is_some() && show_default {
        prompt_text = format!("{} [{}]", text, default.unwrap());
    } else {
        prompt_text = text.to_string();
    }
    prompt_text + suffix
}


fn get_prompt_input(prompt_text: &str, hide_input: bool) -> String {
    print!("{}", prompt_text);
    let input = old_io::stdin().read_line().ok().expect("Failed to read line");
    return input.trim().to_string();
}


/// Prompts a user for input.
///
/// - `text` - the text to show for the prompt.
/// - `default` - the default value to use if no input happens.
/// - `hide_input` - the input value will be hidden
/// - `confirmation` - asks for confirmation for the value
/// - `prompt_suffix` - a suffix that should be added to the prompt
/// - `show_default` - shows or hides the default value
///
pub fn prompt(text: &str, default: Option<String>, hide_input: bool, confirmation: bool,
              prompt_suffix: &str, show_default: bool) -> String {
    let prompt_text = build_prompt_text(text, prompt_suffix, show_default, default.clone());

    let mut prompt_input: String;
    loop {
        prompt_input = get_prompt_input(prompt_text.as_slice(), hide_input);
        if prompt_input != String::from_str("") {
            break
        } else if default.is_some() {
            return default.unwrap();
        }
    }

    if !confirmation {
        return prompt_input;
    }
    let mut confirm_input: String;
    loop {
        confirm_input = get_prompt_input("Repeat for confirmation: ", hide_input);
        if confirm_input != String::from_str("") {
            break
        }
    }
    if prompt_input == confirm_input {
        return prompt_input;
    } else {
        panic!("Error: the two entered values do not match");
    }
}

// This module implements a number of types..
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use getopts;


/// Command params type.
pub type Params = Vec<String>;


/// Command callback func type.
pub type CommandCallback = fn(Params);


/// Options are usually optional values on the command line.
#[derive(Clone)]
pub struct Options {
    short_name: &'static str,
    long_name: &'static str,
    help: &'static str,
    is_flag: bool,
    is_bool_flag: bool,
    multiple: bool,
    required: bool,
    default: Option<&'static str>,
}

impl Options {
    pub fn new(s_name: &'static str, l_name: &'static str, help: &'static str, is_flag: bool, is_bool_flag: bool,
               multiple: bool, required: bool, default: Option<&'static str>) -> Options {
        Options {
            short_name: s_name,
            long_name: l_name,
            help: help,
            is_flag: is_flag,
            is_bool_flag: is_bool_flag,
            multiple: multiple,
            required: required,
            default: default,
        }
    }

    pub fn add_to_parser(&self, parser: &mut getopts::Options) {
        if self.is_flag {
            if !self.is_bool_flag {
                parser.optflagopt(self.short_name, self.long_name, self.help, self.long_name);
            } else if self.multiple {
                parser.optflagmulti(self.short_name, self.long_name, self.help);
            } else {
                parser.optflag(self.short_name, self.long_name, self.help);
            }
        } else {
            if self.required {
                parser.reqopt(self.short_name, self.long_name, self.help, self.long_name);
            } else if self.multiple {
                parser.optmulti(self.short_name, self.long_name, self.help, self.long_name);
            } else {
                parser.optopt(self.short_name, self.long_name, self.help, self.long_name);
            }
        }
    }
}


/// Arguments are positional parameters to a command.
pub struct Argument {
    name: &'static str,
    required: bool,
    default: Option<&'static str>,
}

impl Argument {
    pub fn new(name: &'static str, required: bool, default: Option<&'static str>) -> Argument {
        Argument {
            name: name,
            required: required,
            default: default,
        }
    }

    pub fn add_to_parser(&self, parser: &mut getopts::Options) {
    }

    pub fn get_usage_piece(&self) -> String {
        match self.required {
            true => format!("{}", self.name),
            false => format!("[{}]", self.name),
        }
    }
}

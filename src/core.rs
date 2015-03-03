// Core types.
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::os;
use std::path::Path;

use types::{Params, CommandCallback};


/// The command is the basic type of command line applications in cli.  This
/// handles command line parsing.
pub struct Command {
    name: String,  // The name of the command to use
    callback: CommandCallback,  // The callback to execute
    params: Params,  // The parameters to register with this command
    help: String,  // The help message to use for this command
    epilog: String,  // Printed at the end of the help page
    short_help: String,  // The short help to use for this command, this is shown on the command listing of the parent command
    add_help_option: bool,  // Whether this command registers "--help", default to be true
}


impl Command {
    pub fn new(name: &str, callback: CommandCallback) -> Command {
        Command {
            name: name.to_string(),
            callback: callback,
            params: Vec::new(),
            help: String::new(),
            epilog: String::new(),
            short_help: String::new(),
            add_help_option: true,
        }
    }

    pub fn get_usage(&self) -> String {
    }

    pub fn get_help(&self) -> String {
    }

    /// This invokes the command with given arguments.
    pub fn invoke(&self, pragram_name: String, args: Vec<String>) {
        let args = self.parse_args(args);
        self.callback(
    }

    /// Create the parser and parses the arguments.
    fn parse_args(&self, args: Vec<String>) {
    }

    /// This is the way to run one command application.
    pub fn run(&self) {
        let mut args = os::args();
        let program_path = Path::new(args.remove(0).as_slice());
        let program_name = program_path.file_name().unwrap().to_str().unwrap();
        self.invoke(program_name.to_string(), args);
    }
}

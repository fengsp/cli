// Core types.
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.

use std::os;
use std::path::Path;

use getopts;

use types::{Params, CommandCallback};
use types::{Options, Argument};


/// The command is the basic type of command line applications in cli.  This
/// handles command line parsing.
pub struct Command {
    name: String,  // The name of the command to use
    callback: CommandCallback,  // The callback to execute
    options: Vec<Options>,  // The options to register with this command
    arguments: Vec<Argument>,  // The arguments to register with this command
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
            options: Vec::new(),
            arguments: Vec::new(),
            help: String::new(),
            epilog: String::new(),
            short_help: String::new(),
            add_help_option: true,
        }
    }

    /// Attaches an option to the command.
    pub fn option(&mut self, short_name: &'static str, long_name: &'static str, help: &'static str,
                  is_flag: bool, is_bool_flag: bool, multiple: bool,
                  required: bool, default: Option<&'static str>) {
        let option = Options::new(short_name, long_name, help, is_flag,
                                  is_bool_flag, multiple, required, default);
        self.options.push(option);
    }

    /// Attaches an argument to the command.
    pub fn argument(&mut self, name: &'static str, required: bool, default: Option<&'static str>) {
        let argument = Argument::new(name, required, default);
        self.arguments.push(argument);
    }

    pub fn get_usage(&self) {
    }

    pub fn get_help(&self) -> String {
        "help".to_string()
    }

    /// Returns the help option.
    fn get_help_option(&self) -> Options {
        let help_option_names = vec!["h", "help"];
        let show_help = |params: Params| {
            print!("{}", self.get_help());
        };
        return Options::new(help_option_names[0], help_option_names[1],
                            "Show this message and exit.", true, true, false, false, None);
    }

    /// This invokes the command with given arguments.
    pub fn invoke(&self, pragram_name: String, args: Vec<String>) {
        let args = self.parse_args(args);
        let callback = self.callback;
        callback(args);
    }

    /// Creates the underlying option parser for this command.
    fn make_parser(&self) -> getopts::Options {
        let mut parser = getopts::Options::new();
        for option in self.options.iter() {
            option.add_to_parser(&mut parser);
        }
        let help_option = self.get_help_option();
        help_option.add_to_parser(&mut parser);
        return parser;
    }

    /// Create the parser and parses the arguments.
    fn parse_args(&self, args: Vec<String>) -> Vec<String> {
        args
    }

    /// This is the way to run one command application.
    pub fn run(&self) {
        let mut args = os::args();
        let program = args.remove(0);
        let program_path = Path::new(program.as_slice());
        let program_name = program_path.file_name().unwrap().to_str().unwrap();
        self.invoke(program_name.to_string(), args);
    }
}

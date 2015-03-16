extern crate cli;

use std::old_io::Timer;
use std::time::duration::Duration;
use cli::Editor;


fn main() {
    let editor = Editor::new("vim");
    let text = String::from_str("hello");
    let edited = editor.edit(text, ".txt");
    println!("{}", edited);
}

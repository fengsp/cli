extern crate cli;

// use std::io::timer::Timer;
// use std::time::duration::Duration;
use cli::Editor;


fn main() {
    let editor = Editor::new("vim");
    let text = "hello".to_string();
    let edited = editor.edit(text, ".txt");
    println!("{}", edited);
}

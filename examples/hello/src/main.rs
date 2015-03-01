extern crate cli;

use cli::Cli;


/// Simple program that greets NAME.
fn hello(name: String) {
    println!("Hello {}", name);
}


fn main() {
    let mut app = Cli::new();
    app.command(hello);
    app.option("--name", "Your name", "The person to greet.");
    app.run();
}

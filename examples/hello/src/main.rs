extern crate cli;

use std::old_io::Timer;
use std::time::duration::Duration;
use cli::ProgressBar;


fn main() {
    let interval = Duration::milliseconds(100);
    let mut timer = Timer::new().unwrap();
    let mut bar = ProgressBar::new(40, "Test");
    bar.begin();
    for i in range(0, 50) {
        timer.sleep(interval);
        bar.next();
    }
    bar.end();
}

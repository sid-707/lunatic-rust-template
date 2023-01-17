use std::time::Duration;
use lunatic::{sleep, spawn_link};
        
fn main() {
    spawn_link!(|| println!("Hello, World! I'm a process."));
    sleep(Duration::from_millis(100));
}

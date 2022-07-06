use std::thread;
use std::time::Duration;

/// Script for testing x11 connection handling in Linux.
fn main() {
    let mut counter = 0;
    loop {
        use display_info::DisplayInfo;
        // This opens a connection to X which is never closed
        let displays = DisplayInfo::all();
        // Eventually this will fail with a message: Maximum number of clients reached (regarding X)
        counter += 1;
        println!("Called DisplayInfo::all() {counter} times.");
        thread::sleep(Duration::from_millis(30));
    }
}

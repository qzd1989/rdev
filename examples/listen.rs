use rdev::{Event, listen, set_is_main_thread, stop_listen};
use std::{thread, time::Duration};
fn main() {
    dbg!("hello");
    thread::spawn(|| {
        // only use on tauri
        // #[cfg(target_os = "macos")]
        // set_is_main_thread(false);

        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });
    thread::sleep(Duration::from_secs(1000));
    stop_listen();
    thread::sleep(Duration::from_secs(1000));
}

fn callback(event: Event) {
    println!("My callback {:?}", event);
}

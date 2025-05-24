use rdev::{Event, listen, set_is_main_thread, stop_listen};
use std::{thread, time::Duration};
fn main() {
    thread::spawn(|| {
        #[cfg(target_os = "macos")]
        set_is_main_thread(false);

        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });
    thread::sleep(Duration::from_secs(5));
    stop_listen();
    thread::sleep(Duration::from_secs(10000));
}

fn callback(event: Event) {
    println!("My callback {:?}", event);
}

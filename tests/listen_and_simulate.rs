use rdev::{Button, EventType, Key, listen, simulate};
use serial_test::serial;
use std::error::Error;
use std::iter::Iterator;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn sim_then_listen(events: &mut dyn Iterator<Item = EventType>) -> Result<(), Box<dyn Error>> {
    let (send, recv) = channel();
    // spawn new thread because listen blocks
    let _listener = thread::spawn(move || {
        listen(move |event| send.send(event).unwrap()).expect("Could not listen");
    });
    let second = Duration::from_millis(1000);
    thread::sleep(second);

    for event in events {
        simulate(&event)?;
        let received_event = recv
            .recv_timeout(second)
            .unwrap_or_else(|_| panic!("{}", "No events to receive {event:?}"));
        assert_eq!(received_event.event_type, event);
    }
    Ok(())
}

#[test]
#[serial]
fn test_listen_and_simulate() -> Result<(), Box<dyn Error>> {
    // wait for user input from keyboard to stop
    // (i.e. the return/enter keypress to run test command)
    thread::sleep(Duration::from_millis(50));
    // On wayland we need to open libuinput and the hooks take
    // some time to install, this forces the handle to get installed
    simulate(&EventType::MouseMove { x: 0.0, y: 0.0 })?;
    thread::sleep(Duration::from_millis(50));

    let events = vec![
        //TODO: fix sending shift keypress events on linux
        EventType::KeyPress(Key::ShiftLeft),
        EventType::KeyRelease(Key::ShiftLeft),
        EventType::KeyPress(Key::KeyS),
        EventType::KeyRelease(Key::KeyS),
        EventType::ButtonPress(Button::Right),
        EventType::ButtonRelease(Button::Right),
        EventType::Wheel {
            delta_x: 0,
            delta_y: 1,
        },
        EventType::Wheel {
            delta_x: 0,
            delta_y: -1,
        },
    ]
    .into_iter();
    let click_events = (1..480).map(|pixel| EventType::MouseMove {
        x: pixel as f64,
        y: pixel as f64,
    });
    let mut events = events.chain(click_events);
    sim_then_listen(&mut events)
}

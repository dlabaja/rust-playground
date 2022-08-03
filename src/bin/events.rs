use event_listener::Event;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let event = Arc::new(Event::new());

    //listener
    thread::spawn({
        let event = event.clone();
        move || {
            loop {
                event.listen().wait();
                println!("ping");
            }
        }
    });

    //notifier
    thread::sleep(Duration::from_secs(1));
    event.notify(usize::MAX);
    thread::sleep(Duration::from_micros(1));
    event.notify(usize::MAX);
}
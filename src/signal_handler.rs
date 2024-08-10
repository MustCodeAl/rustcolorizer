use std::sync::{Arc, atomic::AtomicBool};
use signal_hook::{consts::signal::SIGINT, iterator::Signals};

pub fn setup_signal_handler(term_now: Arc<AtomicBool>) {
    let mut signals = Signals::new(&[SIGINT]).expect("Error setting up signal handler");

    std::thread::spawn(move || {
        for _ in signals.forever() {
            term_now.store(true, std::sync::atomic::Ordering::Relaxed);
            println!("Interrupt signal received, stopping execution...");
        }
    });
}
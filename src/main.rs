use libc::{sighandler_t, signal};
use std::sync::atomic::AtomicBool;

static STOP_SIGNAL: AtomicBool = AtomicBool::new(true);

fn main() {
    let handler_ptr = signal_handler;
    let handler_addr = handler_ptr as sighandler_t;

    unsafe {
        signal(libc::SIGINT, handler_addr);
    }

    println!("Idle.");
    while STOP_SIGNAL.load(std::sync::atomic::Ordering::Relaxed) {
        std::hint::spin_loop();
    }
}

/// Windows will execute this function in its own thread, so this effectively turns a single-threaded application into a multi-threaded one.
extern "C" fn signal_handler(sig: i32) {
    println!("Signal: {sig}.");
    STOP_SIGNAL.store(false, std::sync::atomic::Ordering::Relaxed);
}

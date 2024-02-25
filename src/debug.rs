extern crate chrono;

pub fn print_debug_message(message: &str) {
    println!(
        "[ {} ] {}",
        chrono::Local::now().format("%H:%M:%S"),
        message,
    );
}

pub fn print_debug_error<T: std::fmt::Debug>(error: T) {
    println!(
        "[ {} ] [ ERROR ] {:?}",
        chrono::Local::now().format("%H:%M:%S"),
        error,
    );
}
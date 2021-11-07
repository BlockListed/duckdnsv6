#[macro_export]
macro_rules! send_debug {
    ($x:expr) => {
        println!("{} {}", "[DEBUG]".color(Color::Green), $x);
    };
}

#[macro_export]
macro_rules! handle_error {
    ($x:expr) => {
        match $x {
            Ok(data) => data,
            Err(err) => {
                eprintln!("{} {}", "[ERROR]".color(Color::Red), err.bold());
                std::process::exit(1);
            }
        }
    };
}

#[macro_export]
macro_rules! handle_warn {
    ($x:expr) => {
        match $x {
            Ok(data) => data,
            Err(err) => {
                eprintln!("{} {}", "[WARN]".color(Color::YELLOW), err.bold());
                None
            }
        }
    };
}
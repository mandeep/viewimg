/// Wrap eprintln!() and std::process::exit into a single macro
///
/// Reference: https://www.reddit.com/r/rust/comments/7vveid/unwrapping_with_the_exact_error_message_from_err/
#[macro_export]
macro_rules! exit {
    ($fmt:expr) => ({eprint!(concat!($fmt, "\n")); ::std::process::exit(1)});
    ($fmt:expr, $($arg:tt)*) => ({
            eprint!(concat!($fmt, "\n"), $($arg)*);
            ::std::process::exit(1)
    });
}

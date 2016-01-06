/// Write to console if the debug option is set (with newline)
#[macro_export]
macro_rules! debugln {
    ($e:expr, $($arg:tt)*) => ({
        if $e.options.debug {
            println!($($arg)*);
        }
    });
}

/// Write to console if the debug option is set
#[macro_export]
macro_rules! debug {
    ($e:expr, $($arg:tt)*) => ({
        if $e.options.debug {
            print!($($arg)*);
        }
    });
}

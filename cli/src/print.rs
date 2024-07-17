/// A wrapper around print! that will not panic on EPIPE.
/// Useful for avoiding spurious panics when piping to head(1).
#[macro_export]
macro_rules! print_nopipe {
    // Ignore failure when printing an empty line.
    () => {
        {
            use std::io::Write;
            match write!(std::io::stdout()) {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => (),
                Err(e) => panic!("{e}"),
            }
        }
    };
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            match write!(std::io::stdout(), $($arg)*) {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => (),
                Err(e) => panic!("{e}"),
            }
        }
    };
}

/// A wrapper around println! that will not panic on EPIPE.
/// Useful for avoiding spurious panics when piping to head(1).
#[macro_export]
macro_rules! println_nopipe {
    // Ignore failure when printing an empty line.
    () => {
        {
            use std::io::Write;
            match writeln!(std::io::stdout()) {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => (),
                Err(e) => panic!("{e}"),
            }
        }
    };
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            match writeln!(std::io::stdout(), $($arg)*) {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => (),
                Err(e) => panic!("{e}"),
            }
        }
    };
}

/// A wrapper around eprint! that will not panic on EPIPE.
/// Useful for avoiding spurious panics when piping to head(1).
#[macro_export]
macro_rules! eprint_nopipe {
    // Ignore failure when printing an empty line.
    () => {
        {
            use std::io::Write;
            match write!(std::io::stderr()) {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => (),
                Err(e) => panic!("{e}"),
            }
        }
    };
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            match write!(std::io::stderr(), $($arg)*) {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => (),
                Err(e) => panic!("{e}"),
            }
        }
    };
}

/// A wrapper around eprintln! that will not panic on EPIPE.
/// Useful for avoiding spurious panics when piping to head(1).
#[macro_export]
macro_rules! eprintln_nopipe {
    // Ignore failure when printing an empty line.
    () => {
        {
            use std::io::Write;
            match writeln!(std::io::stderr()) {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => (),
                Err(e) => panic!("{e}"),
            }
        }
    };
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            match writeln!(std::io::stderr(), $($arg)*) {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => (),
                Err(e) => panic!("{e}"),
            }
        }
    };
}

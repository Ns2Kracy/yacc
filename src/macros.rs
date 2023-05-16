#![allow(unused_macros)]

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        {
            let console = console::Term::stdout();
            console
                .write_line(&format!("{} {}", console::style("error: ").red(), format!($($arg)*)))
                .unwrap();
            std::process::exit(1);
        }
    };
}

#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {
        {
            let console = console::Term::stdout();
            console
                .write_line(&format!("{} {}", console::style("info: ").green(), format!($($arg)*)))
                .unwrap();
        }
    };
}

#[macro_export]
macro_rules! print_warn {
    ($($arg:tt)*) => {
        {
            let console = console::Term::stdout();
            console
                .write_line(&format!("{} {}", console::style("warning: ").yellow(), format!($($arg)*)))
                .unwrap();
        }
    };
}

#[macro_export]
macro_rules! print_output {
    ($($arg:tt)*) => {
        {
            let console = console::Term::stdout();
            console
                .write_line(&format!("{}", format!($($arg)*)))
                .unwrap();
        }
    };
}

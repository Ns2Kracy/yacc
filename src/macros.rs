#![allow(unused_macros)]

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        {
            let console = console::Term::stdout();
            console
                .write_line(&format!("{} {}", console::style("[ ERROR ]").red(), format!($($arg)*)))
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
                .write_line(&format!("{} {}", console::style("[ INFO ]").green(), format!($($arg)*)))
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
                .write_line(&format!("{} {}", console::style("[ WARN ]").yellow(), format!($($arg)*)))
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

#[macro_export]
macro_rules! print_ok {
    ($($arg:tt)*) => {
        {
            let console = console::Term::stdout();
            console
                .write_line(&format!("{} {}", console::style("[ OK ]").green(), format!($($arg)*)))
                .unwrap();
        }
    };
}

mod cli;
mod commands;
mod consts;
mod utils;

#[macro_use]
mod macros;

fn main() -> anyhow::Result<(), anyhow::Error> {
    if !cfg!(target_os = "linux") {
        print_error!("Only support Linux.");
    }
    cli::run()
}

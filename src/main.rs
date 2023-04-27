mod cli;
mod commands;
mod consts;
mod utils;

#[macro_use]
mod macros;

fn main() -> anyhow::Result<(), anyhow::Error> {
    cli::run()
}

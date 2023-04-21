mod cli;
mod commands;
mod config;
mod consts;
mod utils;

#[macro_use]
mod macros;

fn main() {
    let cli = cli::parse_cli();
    cli.subcommand.call(&cli.config);
}

mod cli;
mod commands;
mod config;
mod logger;

fn main() {
    logger::init_tracing_subscriber();
    let _cli = cli::parse_cli();
}

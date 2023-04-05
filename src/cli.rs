use clap::Parser;

use crate::{commands, config::Config};

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[clap(name = "install")]
    Install(commands::install::Install),
    #[clap(name = "uninstall")]
    Uninstall(commands::uninstall::Uninstall),
}

impl SubCommand {}

#[derive(Parser, Debug)]
#[clap(name="scom", version=env!("CARGO_PKG_VERSION"), author=env!("CARGO_PKG_AUTHORS"))]
pub struct Cli {
    #[clap(flatten)]
    pub config: Config,
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

pub trait Command {
    fn handle(&self, config: Config);

    fn error(&self, error: String) {
        tracing::error!("{}", error);
    }
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}

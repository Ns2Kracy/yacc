use crate::{commands, config::Config, print_error};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name="yacc", version=env!("CARGO_PKG_VERSION"), author=env!("CARGO_PKG_AUTHORS"))]
pub struct Cli {
    #[clap(flatten)]
    pub config: Config,
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

pub trait Command {
    type Error: std::error::Error;

    fn call(&self, config: &Config) -> anyhow::Result<(), Self::Error>;

    fn error(&self, error: String) {
        print_error!("{}", error);
    }

    fn handle(&self, config: &Config) {
        match self.call(config) {
            Ok(()) => (),
            Err(e) => {
                self.error(format!("Error: {}", e));
            }
        }
    }
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[clap(name = "install")]
    Install(commands::install::Install),

    #[clap(name = "uninstall")]
    Uninstall(commands::uninstall::Uninstall),

    #[clap(name = "status")]
    Status(commands::status::Status),

    #[clap(name = "start")]
    Start(commands::start::Start),

    #[clap(name = "stop")]
    Stop(commands::stop::Stop),

    #[clap(name = "restart")]
    Restart(commands::start::Start),

    #[clap(name = "enable")]
    Enable(commands::enable::Enable),

    #[clap(name = "disable")]
    Disable(commands::disable::Disable),
}

impl SubCommand {
    pub fn call(&self, config: &Config) {
        match self {
            SubCommand::Install(cmd) => cmd.handle(config),
            SubCommand::Uninstall(cmd) => cmd.handle(config),
            SubCommand::Status(cmd) => cmd.handle(config),
            SubCommand::Start(cmd) => cmd.handle(config),
            SubCommand::Stop(cmd) => cmd.handle(config),
            SubCommand::Restart(cmd) => cmd.handle(config),
            SubCommand::Enable(cmd) => cmd.handle(config),
            SubCommand::Disable(cmd) => cmd.handle(config),
        }
    }
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
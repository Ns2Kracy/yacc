use crate::commands;
use anyhow::Error;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name="yacc", version=env!("CARGO_PKG_VERSION"), author=env!("CARGO_PKG_AUTHORS"))]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[clap(name = "install")]
    Install(commands::install::Args),

    #[clap(name = "uninstall")]
    Uninstall(commands::uninstall::Args),

    #[clap(name = "status")]
    Status(commands::status::Args),

    #[clap(name = "start")]
    Start(commands::start::Args),

    #[clap(name = "stop")]
    Stop(commands::stop::Args),

    #[clap(name = "restart")]
    Restart(commands::start::Args),

    #[clap(name = "enable")]
    Enable(commands::enable::Args),

    #[clap(name = "disable")]
    Disable(commands::disable::Args),
}

pub fn run() -> anyhow::Result<(), Error> {
    let cmd = Cli::parse();

    let _ = match cmd.subcommand {
        SubCommand::Install(cmd) => commands::install::run(cmd),
        SubCommand::Uninstall(cmd) => commands::uninstall::run(cmd),
        SubCommand::Status(cmd) => commands::status::run(cmd),
        SubCommand::Start(cmd) => commands::start::run(cmd),
        SubCommand::Stop(cmd) => commands::stop::run(cmd),
        SubCommand::Restart(cmd) => commands::start::run(cmd),
        SubCommand::Enable(cmd) => commands::enable::run(cmd),
        SubCommand::Disable(cmd) => commands::disable::run(cmd),
    };
    Ok(())
}

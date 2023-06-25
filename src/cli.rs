use crate::commands;
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

    #[clap(name = "update")]
    Update(commands::update::Args),
}

pub async fn run() -> anyhow::Result<(), anyhow::Error> {
    let cmd = Cli::parse();

    let _ = match cmd.subcommand {
        SubCommand::Install(cmd) => commands::install::run(cmd).await,
        SubCommand::Uninstall(cmd) => commands::uninstall::run(cmd).await,
        SubCommand::Update(cmd) => commands::update::run(cmd).await,
    };
    Ok(())
}

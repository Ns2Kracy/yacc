mod cli;
mod commands;
mod consts;
mod utils;

#[macro_use]
mod macros;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    cli::run().await
}

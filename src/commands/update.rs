use anyhow::Error;

#[derive(clap::Parser, Debug, Default)]
pub struct Args {
    #[clap(short, default_value = "false")]
    alpha: bool,
}

pub fn run(_cmd: Args) -> anyhow::Result<(), Error> {
    todo!()
}

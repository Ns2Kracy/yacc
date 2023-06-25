use crate::print_output;

#[derive(clap::Parser, Debug, Default)]
pub struct Args {
    #[clap(short, default_value = "false")]
    alpha: bool,
}

pub async fn run(_cmd: Args) -> anyhow::Result<(), anyhow::Error> {
    print_output!("Update CasaOS successfully.");
    Ok(())
}

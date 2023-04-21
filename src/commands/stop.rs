#[derive(clap::Parser, Debug, Default)]
pub struct Stop {
    #[clap(short, long)]
    pub all: bool,

    #[clap(short, long)]
    pub service: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to stop service {0}")]
    ServiceStopError(String),

    #[error("Failed to stop all services: {0}")]
    StopAllError(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Permission denied")]
    PermissionDenied,
}

impl crate::cli::Command for Stop {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}

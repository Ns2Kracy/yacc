#[derive(clap::Parser, Debug, Default)]
pub struct Start {
    #[clap(short, long)]
    pub all: bool,

    #[clap(short, long)]
    pub service: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to start service {0}")]
    ServiceStartError(String),

    #[error("Failed to start all services: {0}")]
    StartAllError(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Permission denied")]
    PermissionDenied,
}

impl crate::cli::Command for Start {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}

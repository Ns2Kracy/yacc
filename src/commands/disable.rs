#[derive(clap::Parser, Debug, Default)]
pub struct Disable {
    #[clap(short, long)]
    pub all: bool,

    #[clap(short, long)]
    pub service: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to disable service {0}")]
    ServiceDisableError(String),

    #[error("Failed to disable all services: {0}")]
    DisableAllError(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Permission denied")]
    PermissionDenied,
}

impl crate::cli::Command for Disable {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}

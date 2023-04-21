use crate::utils::systemd::systemctl;

#[derive(clap::Parser, Debug, Default)]
pub struct Restart {
    #[clap(short, long)]
    pub all: bool,

    #[clap(short, long)]
    pub service: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to restart service: {0}")]
    RestartFailed(String),

    #[error("Failed to restart all services: {0}")]
    RestartAllFailed(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Permission denied")]
    PermissionDenied,
}

impl crate::cli::Command for Restart {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}

#[derive(clap::Parser, Debug, Default)]
pub struct Status {
    #[clap(short, long)]
    pub all: bool,

    #[clap(short, long)]
    pub service: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to get status of service {0}")]
    ServiceStatusError(String),

    #[error("Failed to get status of all services: {0}")]
    StatusAllError(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Permission denied")]
    PermissionDenied,
}

impl crate::cli::Command for Status {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}

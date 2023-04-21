#[derive(clap::Parser, Debug, Default)]
pub struct Uninstall {}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to uninstall service {0}")]
    ServiceUninstallError(String),

    #[error("Failed to uninstall all services: {0}")]
    UninstallAllError(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

impl crate::cli::Command for Uninstall {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}

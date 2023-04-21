#[derive(clap::Parser, Debug, Default)]
pub struct Update {
    #[clap(short, long)]
    pub alpha: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to download new version: {0}")]
    DownloadError(String),

    #[error("Failed to extract new version: {0}")]
    ExtractError(String),

    #[error("Migration failed: {0}")]
    MigrationFailed(String),

    #[error("Failed to update service {0}")]
    ServiceUpdateError(String),

    #[error("Permission denied")]
    PermissionDenied,
}

impl crate::cli::Command for Update {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}

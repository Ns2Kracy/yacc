#[derive(clap::Parser, Debug, Default)]
pub struct Install {
    #[clap(short, long)]
    pub version: Option<String>,

    #[clap(short, long)]
    pub latest: bool,

    #[clap(short, long)]
    pub alpha: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to download the latest release, please check your internet connection")]
    DownloadError,

    #[error("Failed to extract the downloaded release")]
    ExtractError,

    #[error("Failed to install the downloaded release")]
    InstallError,

    #[error("Failed to remove the downloaded release")]
    RemoveError,

    #[error("Permission denied")]
    PermissionDenied,
}

impl crate::cli::Command for Install {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}

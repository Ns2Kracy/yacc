use clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug, Default)]
pub struct Status {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to get service status")]
    GetServiceStatusFailed,
}

impl crate::cli::Command for Status {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}
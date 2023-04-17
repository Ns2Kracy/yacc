use clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug, Default)]
pub struct Install {}

#[derive(Error, Debug)]
pub enum Error {}

impl crate::cli::Command for Install {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}
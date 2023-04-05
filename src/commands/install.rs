use crate::cli::Command;
use clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct Install {}

#[derive(Error, Debug)]
pub enum Error {}

impl Command for Install {
    type Error = Error;

    fn call(&self, config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}

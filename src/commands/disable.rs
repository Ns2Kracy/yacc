use clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug, Default)]
pub struct Disable {}

#[derive(Error, Debug)]
pub enum Error {}

impl crate::cli::Command for Disable {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}
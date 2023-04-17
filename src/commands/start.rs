use clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug, Default)]
pub struct Start {}

#[derive(Error, Debug)]
pub enum Error {}

impl crate::cli::Command for Start {
    type Error = Error;

    fn call(&self, _config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}
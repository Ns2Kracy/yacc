use clap::Parser;
use thiserror::Error;

use crate::cli::Command;

#[derive(Parser, Debug)]
pub struct Uninstall {}

#[derive(Error, Debug)]
pub enum Error {}

impl Command for Uninstall {
    type Error = Error;

    fn call(&self, config: &crate::config::Config) -> anyhow::Result<(), Self::Error> {
        todo!()
    }
}

use anyhow::Result;

use crate::cli::Cli;

#[derive(Debug)]
pub struct App {
    cli: Cli,
}

impl App {
    pub fn new() -> App {
        App { cli: Cli::new() }
    }
}

impl App {
    pub fn run(self) -> Result<()> {
        todo!()
    }
}

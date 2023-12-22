use anyhow::Result;

use crate::app::App;

mod app;
mod cli;

fn main() -> Result<()> {
    App::new().run()
}

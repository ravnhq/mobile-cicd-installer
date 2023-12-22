use anyhow::Result;

use crate::app::App;

mod app;
mod cli;
mod util;

fn main() -> Result<()> {
    App::new().run()
}

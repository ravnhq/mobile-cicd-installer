use clap::Parser;
use crate::cli::Cli;

mod cli;

fn main() {
    let cli = Cli::parse();
    let platforms = cli.get_platforms();

    println!("{:?}", platforms);
}

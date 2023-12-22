use crate::cli::Cli;

mod cli;

fn main() {
    let cli = Cli::new();
    let platforms = cli.get_platforms();
    let _ = cli.should_copy_github_workflow();
    let _ = cli.should_copy_fastlane();
    let _ = cli.should_configure_cocoapods();

    println!("{:?}", platforms);
}

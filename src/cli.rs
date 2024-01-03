use std::borrow::Cow;
use std::ops::Not;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use atty::Stream;
use clap::{CommandFactory, Parser, ValueEnum};
use clap::error::ErrorKind;
use inquire::{Confirm, MultiSelect};
use inquire::list_option::ListOption;
use inquire::validator::Validation;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, ValueEnum)]
pub enum Platform {
    #[value(name = "android")]
    Android,
    #[value(name = "ios")]
    Ios,
    #[value(name = "all")]
    All,
}

impl Platform {
    pub fn as_platforms(self) -> Vec<Platform> {
        match self {
            Platform::All => vec![Platform::Android, Platform::Ios],
            platform => vec![platform]
        }
    }

    pub fn name(&self) -> String {
        let name = match self {
            Platform::Android => "android",
            Platform::Ios => "ios",
            Platform::All => "all",
        };

        name.to_string()
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    interactive: bool,

    #[arg(short, long)]
    destination: Option<PathBuf>,

    #[arg(short, long, value_enum)]
    platform: Option<Platform>,

    #[arg(long)]
    skip_fastlane: bool,

    #[arg(long)]
    copy_github_workflow: bool,

    #[arg(long)]
    uses_cocoapods: bool,

    #[arg(short, long)]
    force_copy: bool,
}

impl Cli {
    pub fn new() -> Cli {
        if atty::isnt(Stream::Stdin) {
            println!("Terminal is not interactive, -i/--interactive won't take effect")
        }

        Cli::parse()
    }
}

impl Cli {
    pub fn is_interactive(&self) -> bool {
        atty::is(Stream::Stdin) && self.interactive
    }

    pub fn should_force_copy(&self) -> bool {
        self.force_copy
    }

    pub fn get_destination(&self) -> Result<Cow<Path>> {
        match &self.destination {
            Some(path) => Ok(Cow::Borrowed(path.as_path())),
            None => Ok(Cow::Owned(std::env::current_dir()?)),
        }
    }

    pub fn get_platforms(&self) -> Result<Vec<Platform>> {
        if self.is_interactive() {
            let options = vec!["Android", "iOS"];
            let platforms = MultiSelect::new("Select the platforms to configure:", options)
                .with_validator(|values: &[ListOption<&&str>]| {
                    if values.is_empty() {
                        Ok(Validation::Invalid("Select at least one platform".into()))
                    } else {
                        Ok(Validation::Valid)
                    }
                })
                .prompt()?;

            let platforms: Vec<_> = platforms
                .into_iter()
                .map(|p| Platform::from_str(p, true).map_err(|_| anyhow!("Failed to parse")))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(platforms)
        } else if let Some(platform) = self.platform {
            Ok(platform.as_platforms())
        } else {
            Cli::command()
                .error(ErrorKind::MissingRequiredArgument, "Missing platform")
                .exit()
        }
    }

    pub fn should_copy_fastlane(&self) -> Result<bool> {
        if self.is_interactive() {
            let answer = Confirm::new("Copy fastlane configuration files?")
                .with_default(true)
                .prompt()?;

            Ok(answer)
        } else {
            Ok(self.skip_fastlane.not())
        }
    }

    pub fn should_copy_github_workflow(&self) -> Result<bool> {
        if self.is_interactive() {
            let answer = Confirm::new("Copy starter GitHub workflow?")
                .with_default(false)
                .prompt()?;

            Ok(answer)
        } else {
            Ok(self.copy_github_workflow)
        }
    }

    pub fn should_configure_cocoapods(&self) -> Result<bool> {
        if self.is_interactive() {
            let answer = Confirm::new("Configure Cocoapods?")
                .with_default(false)
                .prompt()?;

            Ok(answer)
        } else {
            Ok(self.uses_cocoapods)
        }
    }
}


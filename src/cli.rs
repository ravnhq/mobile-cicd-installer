use clap::{CommandFactory, Parser, ValueEnum};
use clap::error::ErrorKind;
use anyhow::{anyhow, Result};
use inquire::list_option::ListOption;
use inquire::MultiSelect;
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
    fn as_platforms(self) -> Vec<Platform> {
        match self {
            Platform::All => vec![Platform::Android, Platform::Ios],
            platform => vec![platform]
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    interactive: bool,

    #[arg(short, long, value_enum)]
    platform: Option<Platform>,

    #[arg(long)]
    skip_fastlane: bool,

    #[arg(long)]
    copy_github_workflow: bool,

    #[arg(long)]
    uses_cocoapods: bool,
}


impl Cli {
    pub fn get_platforms(&self) -> Result<Vec<Platform>> {
        if self.interactive {
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
}

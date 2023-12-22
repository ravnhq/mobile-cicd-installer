use std::path::PathBuf;
use std::time::SystemTime;

use anyhow::{anyhow, Result};
use inquire::Confirm;
use shells::sh;

use crate::cli::Cli;
use crate::log;

#[derive(Debug)]
pub struct App {
    cli: Cli,
    repo_dir: PathBuf,
}

impl App {
    fn get_repo_dir() -> PathBuf {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        std::env::temp_dir().join(format!("ravn_mobile_ci_cd_{now}"))
    }

    pub fn new() -> Self {
        let cli = Cli::new();
        let repo_dir = Self::get_repo_dir();

        Self { cli, repo_dir }
    }
}

impl App {
    pub fn run(self) -> Result<()> {
        self.download_repo()?;

        // Ask first, better UX
        let platforms = self.cli.get_platforms()?;

        self.copy_fastlane_wrapper()?;
        self.copy_ruby_files()?;

        Ok(())
    }
}

impl App {
    const VERSION_URL: &'static str = "https://raw.githubusercontent.com/ravnhq/mobile-cicd/main/.version";

    fn download_repo(&self) -> Result<()> {
        log!("Downloading required files...");

        let (code, version, _) = sh!("curl -s {}", Self::VERSION_URL);
        if code != 0 {
            return Err(anyhow!("Failed to get current version"));
        }

        let version = version.trim();
        let repo_dir = self.repo_dir.display();
        let (code, _, _) = sh!("git clone --branch {version} --depth 1 https://github.com/ravnhq/mobile-cicd {repo_dir}");
        if code != 0 {
            return Err(anyhow!("Failed to download repository"));
        }

        Ok(())
    }

    fn copy_fastlane_wrapper(&self) -> Result<()> {
        let src = self.repo_dir.join("fastlanew");
        let dst = self.cli.get_destination()?;
        std::fs::copy(src, dst.join("fastlanew"))?;

        Ok(())
    }

    fn copy_ruby_files(&self) -> Result<()> {
        let dst = self.cli.get_destination()?;
        let src = self.repo_dir.join(".ruby-version");

        std::fs::copy(src, dst.join(".ruby-version"))?;

        let src = self.repo_dir.join("Gemfile");
        let dst = dst.join("Gemfile");

        if dst.exists() {
            if self.cli.is_interactive() {
                let answer = Confirm::new("File 'Gemfile' already exists, overwrite it?")
                    .with_default(true)
                    .prompt()?;

                if !answer {
                    return Err(anyhow!("File exists but couldn't overwrite it"));
                }
            } else if !self.cli.should_force_copy() {
                return Err(anyhow!("File exists but couldn't overwrite it"));
            }
        }

        std::fs::copy(src, &dst)?;

        Ok(())
    }
}

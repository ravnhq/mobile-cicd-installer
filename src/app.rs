use std::io::Write;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{anyhow, Result};
use inquire::Confirm;
use shells::sh;

use crate::cli::{Cli, Platform};
use crate::{log, util};

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
        let selected_platforms = self.cli.get_platforms()?;
        let ignored_platforms: Vec<_> = Platform::All
            .as_platforms()
            .into_iter()
            .filter(|p| !selected_platforms.contains(p))
            .collect();

        self.copy_fastlane_wrapper()?;
        self.copy_ruby_files()?;
        self.copy_fastlane_files(&ignored_platforms)?;
        self.copy_github_workflow(&ignored_platforms)?;
        self.configure_platforms(&selected_platforms)?;

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

    fn backup_fastlane_files(&self) -> Result<()> {
        let dst = self.cli.get_destination()?;
        if !dst.join("fastlane").exists() {
            return Ok(());
        }

        let msg = if dst.join("fastlane.old").exists() {
            "Found an existing backup, do you want to replace it with a new one?"
        } else {
            "Do you want to backup your existing fastlane configuration files?"
        };

        let answer = if self.cli.is_interactive() {
            Confirm::new(msg)
                .with_default(false)
                .prompt()?
        } else {
            false
        };

        if !answer {
            return Ok(());
        }

        let src = dst.join("fastlane");
        let dst = dst.join("fastlane.old");
        util::fs::copy_recursively(src, dst)?;
        log!("Created backup 'fastlane.old', consolidate your files before removing it");

        Ok(())
    }

    fn remove_platform_regions(&self, paths: &[impl AsRef<Path>], platforms: &[Platform]) -> Result<()> {
        for platform_to_remove in platforms {
            for path in paths {
                util::io::remove_region(path, &platform_to_remove.name())?;
            }
        }

        Ok(())
    }

    fn copy_fastlane_files(&self, ignored_platforms: &[Platform]) -> Result<()> {
        if !self.cli.should_copy_fastlane()? {
            return Ok(());
        }

        self.backup_fastlane_files()?;

        let src = self.repo_dir.join("fastlane");
        let dst = self.cli.get_destination()?.join("fastlane");
        util::fs::copy_recursively(src, &dst)?;

        let paths = vec![dst.join("Appfile"), dst.join("Fastfile")];
        self.remove_platform_regions(&paths, ignored_platforms)?;

        Ok(())
    }

    fn copy_github_workflow(&self, ignored_platforms: &[Platform]) -> Result<()> {
        if !self.cli.should_copy_github_workflow()? {
            return Ok(());
        }

        let src = self.repo_dir.join("github/main.yml");
        let dst = self.cli.get_destination()?.join(".github/workflows/main.yml");

        if dst.exists() {
            let answer = Confirm::new("Replace existing main.yml workflow?")
                .with_default(false)
                .prompt()?;

            if !answer {
                return Ok(());
            }
        }

        if let Some(parent) = dst.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        std::fs::copy(src, &dst)?;
        self.remove_platform_regions(&[dst], ignored_platforms)?;

        Ok(())
    }

    fn configure_platforms(&self, platforms: &[Platform]) -> Result<()> {
        if platforms.contains(&Platform::Ios) {
            self.configure_cocoapods()?;
        }

        Ok(())
    }

    fn configure_cocoapods(&self) -> Result<()> {
        if !self.cli.should_configure_cocoapods()? {
            return Ok(());
        }

        let dst = self.cli.get_destination()?.join("Gemfile");
        let contents = std::io::read_to_string(File::open(dst.as_path())?)?;
        let mut output = Vec::new();

        for line in contents.lines() {
            output.push(line);

            if line.contains("gem 'fastlane'") {
                output.push("gem 'cocoapods'");
            }
        }

        let mut file = File::create(dst)?;
        for line in output {
            writeln!(file, "{line}")?;
        }

        Ok(())
    }
}

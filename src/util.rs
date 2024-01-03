#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use colorful::{Color, Colorful};
        let prompt = ">".color(Color::LightGreen);
        println!("{prompt} {}", format!($($arg)*));
    }};
}

pub mod fs {
    use std::path::Path;
    use anyhow::Result;
    use fs_extra::dir::CopyOptions;

    pub fn copy_recursively(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
        let options = CopyOptions::new()
            .overwrite(true)
            .copy_inside(true);

        let dst = dst.as_ref();
        if dst.exists() {
            if dst.is_dir() {
                std::fs::remove_dir_all(dst)?;
            } else {
                std::fs::remove_file(dst)?;
            }
        }

        fs_extra::dir::copy(src, dst, &options)?;
        Ok(())
    }
}

pub mod io {
    use std::io::Write;
    use std::fs::File;
    use std::path::Path;
    use anyhow::Result;

    pub fn remove_region(path: impl AsRef<Path>, region: &str) -> Result<()> {
        let contents = std::io::read_to_string(File::open(path.as_ref())?)?;
        let mut output = Vec::new();
        let mut skip = false;

        for line in contents.lines() {
            if line.contains(&format!("# region {region}")) {
                skip = true
            }

            if !skip {
                output.push(line);
            }

            if line.contains(&format!("# endregion {region}")) {
                skip = false
            }
        }

        let mut file = File::create(path.as_ref())?;
        for line in output {
            writeln!(file, "{line}")?;
        }

        Ok(())
    }
}

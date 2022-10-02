use std::{env::temp_dir, path::PathBuf};

use common_domain::error::{Error, Result};
use fs_extra::dir::CopyOptions;

macro_rules! new_lang {
    ($lang:literal, $base:literal => $bp:ident, $cp: ident) => {
        new_lang!($lang => $bp);
        new_lang!($base => $bp, $cp);
    };
    ($base:literal => $bp:ident, $cp: ident) => {
        pub async fn $cp(target: &PathBuf) -> Result<PathBuf> {
            let path = $bp();
            copy_project(&path, target).await?;

            Ok(path.join($base))
        }
    };
    ($lang:literal => $bp:ident) => {
        pub fn $bp() -> PathBuf {
            base_projects_path().join($lang)
        }
    };
}

pub fn base_projects_path() -> PathBuf {
    temp_dir().join("tessar").join("base")
}

async fn copy_project(from: &PathBuf, to: &PathBuf) -> Result<()> {
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    fs_extra::dir::copy(from, to, &options).map_err(|e| Error::unknown(&e.to_string()))?;
    Ok(())
}

new_lang!("dart", "bin" => base_dart_project_path, create_dart_project);

use std::{
    env,
    path::{Path, PathBuf},
    process,
};

use crate::{
    cargo_content::CargoContent,
    error::{DynError, Error},
};

const CARGO_FILE: &str = "Cargo.toml";

pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

pub fn install_cross() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = process::Command::new(cargo)
        .current_dir(project_root())
        .args(&["install", "cross"])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::FailedToInstallCross)?
    }
}

pub fn run_cargo_build(target: &str, use_cross: bool) -> Result<(), DynError> {
    let cargo = if use_cross {
        env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
    } else {
        "cross".to_owned()
    };
    let status = process::Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release", "--target", target])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::BuildFailed)?
    }
}

pub fn get_project_names(path: PathBuf) -> Vec<String> {
    if !is_dir(&path) {
        return vec![];
    }
    let cargo_path = path.join(CARGO_FILE);
    if let Some(cargo) = get_cargo_content(&cargo_path) {
        return vec![cargo.package.name];
    }

    let mut result = Vec::new();

    let dirs = std::fs::read_dir(&path).unwrap();

    for dir in dirs {
        let dir = dir.unwrap();
        let path = dir.path();
        if is_dir(&path) {
            result.append(&mut get_project_names(path))
        }
    }

    result
}

pub fn get_cargo_content(path: &PathBuf) -> Option<CargoContent> {
    let content = std::fs::read_to_string(path).ok()?;

    toml::from_str(&content).ok()
}

pub fn is_dir(path: &PathBuf) -> bool {
    std::fs::metadata(&path)
        .map(|meta| meta.is_dir())
        .unwrap_or(false)
}

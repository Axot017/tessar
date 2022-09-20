use std::path::PathBuf;

use crate::model::error::{DynError, Error};

pub fn fetch_flutter(path: &PathBuf) -> Result<(), DynError> {
    std::fs::create_dir_all(path)?;
    let status = std::process::Command::new("git")
        .current_dir(path)
        .args(vec![
            "clone",
            "https://github.com/flutter/flutter.git",
            "-b",
            "stable",
        ])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Box::new(Error::FlutterFetchFailed))
    }
}

pub fn precache_flutter(
    flutter_path: &PathBuf,
    image: &str,
    platform: &str,
) -> Result<(), DynError> {
    let status = std::process::Command::new("docker")
        .current_dir(flutter_path)
        .args(vec![
            "run",
            "--rm",
            "--platform",
            platform,
            "-v",
            format!(
                "{}:/usr/flutter",
                flutter_path.to_str().expect("flutter_path is empty")
            )
            .as_str(),
            "-w",
            "/usr/flutter",
            image,
            "./bin/flutter",
            "precache",
        ])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Box::new(Error::FlutterFetchFailed))
    }
}

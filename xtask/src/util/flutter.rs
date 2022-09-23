use std::path::PathBuf;

use crate::model::error::DynError;

pub fn _fetch_flutter(path: &PathBuf) -> Result<(), DynError> {
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
        todo!()
    }
}

pub fn _precache_flutter(flutter_path: &PathBuf) -> Result<(), DynError> {
    let status = std::process::Command::new("flutter")
        .current_dir(flutter_path)
        .args(vec![
            "precache",
            "--no-android",
            "--no-ios",
            "--no-linux",
            "--no-macos",
            "--no-windows",
            "--no-universal",
            "--no-fuchsia",
            "--web",
        ])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        todo!()
    }
}

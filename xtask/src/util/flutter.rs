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

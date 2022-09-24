use crate::{
    model::error::{DynError, Error},
    util::project::project_root,
};

use super::FetchFlutterArgs;

pub fn fetch_flutter(args: &FetchFlutterArgs) -> Result<(), DynError> {
    let tmp_path = project_root().join("tmp");
    std::fs::create_dir_all(&tmp_path).ok();

    let flutter_path = tmp_path.join("flutter");
    std::fs::remove_dir_all(&flutter_path).ok();

    let status = std::process::Command::new("git")
        .current_dir(&tmp_path)
        .args(vec![
            "clone",
            "https://github.com/flutter/flutter.git",
            "--branch",
            &args.version,
            "--depth",
            "1",
        ])
        .status()?;
    if !status.success() {
        return Err(Box::new(Error::FailedToFetchFlutter));
    }

    std::process::Command::new("ls")
        .current_dir(&flutter_path)
        .status()?;
    let flutter_bin_path = flutter_path.join("bin");
    std::process::Command::new("ls")
        .current_dir(&flutter_bin_path)
        .status()?;
    let status = std::process::Command::new("flutter")
        .current_dir(&flutter_bin_path)
        .args(vec!["precache"])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Box::new(Error::FailedToPrecacheFlutter))
    }
}

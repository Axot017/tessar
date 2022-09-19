use std::path::PathBuf;

use crate::model::error::DynError;

pub fn get_system_user() -> Result<String, DynError> {
    let output = std::process::Command::new("id").arg("-u").output()?.stdout;
    let user = String::from_utf8_lossy(&output);

    Ok(user.trim().to_owned())
}

pub fn get_system_group() -> Result<String, DynError> {
    let output = std::process::Command::new("id").arg("-g").output()?.stdout;
    let group = String::from_utf8_lossy(&output);

    Ok(group.trim().to_owned())
}

pub fn make_dir_owned(dir: &PathBuf) -> Result<(), DynError> {
    let user = get_system_user()?;
    let group = get_system_group()?;
    std::process::Command::new("sudo")
        .current_dir(&dir)
        .args(vec!["chown", "-R", &format!("{}:{}", user, group), "."])
        .status()?;

    Ok(())
}

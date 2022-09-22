use std::io::Cursor;

use crate::{
    model::error::DynError,
    util::{project::project_root, zip::zip_file},
};

use super::CreateDartLayerArgs;

pub fn create_dart_layer(args: &CreateDartLayerArgs) -> Result<(), DynError> {
    let url = format!(
        "https://storage.googleapis.com/dart-archive/channels/stable/release/{}/sdk/dartsdk-linux-arm64-release.zip", 
        args.version,
    );

    let tmp_dir = project_root().join("tmp");
    std::fs::create_dir_all(&tmp_dir).ok();

    let dart_sdk_dir = tmp_dir.join("dart-sdk");
    std::fs::remove_dir_all(&dart_sdk_dir).ok();

    let zip_dir = tmp_dir.join("dart.zip");
    std::fs::remove_file(&zip_dir).ok();

    println!("Test 1");
    {
        let mut file = std::fs::File::create(&zip_dir)?;
        println!("Test 2");
        let response = reqwest::blocking::get(url)?;
        println!("Test 3");
        let mut content = Cursor::new(response.bytes()?);
        println!("Test 4");
        std::io::copy(&mut content, &mut file)?;
    }

    println!("Test 5");
    std::process::Command::new("unzip")
        .current_dir(&tmp_dir)
        .arg(zip_dir)
        .status()?;

    println!("Test 6");
    let dart_bin_dir = dart_sdk_dir.join("bin").join("dart");
    println!("Test 7");
    let layer_dir = project_root().join("target").join("layer").join("dart.zip");
    println!("Test 8");
    std::fs::remove_file(&layer_dir).ok();
    println!("Test 9");

    let path_in_zip = std::path::Path::new("bin").to_path_buf().join("dart");
    println!("Test 10");
    zip_file(&dart_bin_dir, &layer_dir, &path_in_zip)?;
    println!("Test 11");

    Ok(())
}

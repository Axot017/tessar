use super::CreateFlutterLayerArgs;
use crate::{
    model::error::DynError,
    util::{
        flutter::{fetch_flutter, precache_flutter},
        project::project_root,
        zip::zip_dir,
    },
};

pub fn create_flutter_layer(args: &CreateFlutterLayerArgs) -> Result<(), DynError> {
    let tmp_path = project_root().join("tmp");
    let flutter_path = tmp_path.join("flutter");
    let flutter_bin_path = flutter_path.join("bin");
    let layer_path = project_root().join("target").join("layer");
    println!("Test 1");
    std::fs::remove_dir_all(&tmp_path).ok();
    println!("Test 2");
    fetch_flutter(&tmp_path)?;
    println!("Test 3");
    precache_flutter(&flutter_path, &args.platform)?;
    println!("Test 4");

    std::fs::create_dir_all(&layer_path)?;
    println!("Test 5");

    zip_dir(&flutter_bin_path, &layer_path.join("flutter_layer.zip"))
}

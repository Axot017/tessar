use super::CreateFlutterLayerArgs;
use crate::{
    model::error::DynError,
    util::{
        flutter::{fetch_flutter, precache_flutter},
        project::project_root,
        system::make_dir_owned,
        zip::zip_dir,
    },
};

pub fn create_flutter_layer(args: &CreateFlutterLayerArgs) -> Result<(), DynError> {
    let tmp_path = project_root().join("tmp");
    let flutter_path = tmp_path.join("flutter");
    let flutter_bin_path = flutter_path.join("bin");
    let layer_path = project_root().join("target").join("layer");
    std::fs::remove_dir_all(&tmp_path).ok();
    fetch_flutter(&tmp_path)?;
    precache_flutter(&flutter_path, &args.image, &args.platform)?;

    std::fs::create_dir_all(&layer_path)?;

    if args.use_sudo {
        make_dir_owned(&flutter_bin_path)?;
    }
    zip_dir(
        &flutter_bin_path,
        &layer_path.join("flutter_layer.zip"),
        Some(&std::path::Path::new("bin").to_path_buf()),
    )
}

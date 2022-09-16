use crate::{
    model::error::DynError,
    util::{
        flutter::{fetch_flutter, precache_flutter},
        project::project_root,
    },
};

use super::CreateFlutterLayerArgs;

pub fn create_flutter_layer(args: &CreateFlutterLayerArgs) -> Result<(), DynError> {
    let tmp_path = project_root().join("tmp");
    std::fs::remove_dir_all(&tmp_path)?;
    fetch_flutter(&tmp_path)?;
    precache_flutter(&tmp_path.join("flutter"), &args.platform)
}

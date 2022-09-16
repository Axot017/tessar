use crate::{
    model::error::DynError,
    util::{flutter::fetch_flutter, project::project_root},
};

use super::CreateFlutterLayerArgs;

pub fn create_flutter_layer(_args: &CreateFlutterLayerArgs) -> Result<(), DynError> {
    let tmp_path = project_root().join("tmp");
    fetch_flutter(&tmp_path)?;
    Ok(())
}

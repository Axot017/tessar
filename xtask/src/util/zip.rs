use std::{
    io::{Read, Write},
    path::PathBuf,
};

use walkdir::WalkDir;
use zip::write::FileOptions;

use crate::model::error::DynError;

pub fn zip_dir(dir: &PathBuf, output_file: &PathBuf) -> Result<(), DynError> {
    let file = std::fs::File::create(&output_file)?;

    let walkdir = WalkDir::new(dir);
    let it = walkdir.into_iter().filter_map(|entry| entry.ok());
    let mut buffer = Vec::new();
    let mut zip = zip::ZipWriter::new(file);

    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(std::path::Path::new(dir))?;

        if path.is_file() {
            let mut file_in =
                std::fs::File::open(&path).expect(&format!("Failed to open {path:?}"));
            zip.start_file(name.to_str().unwrap(), FileOptions::default())?;

            file_in.read_exact(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        }
    }
    Ok(())
}

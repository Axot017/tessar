use std::{
    io::{Read, Write},
    path::PathBuf,
};

use walkdir::WalkDir;
use zip::write::FileOptions;

use crate::model::error::DynError;

pub fn zip_dir(dir: &PathBuf, output_file: &PathBuf) -> Result<(), DynError> {
    println!("Test 1");
    let file = std::fs::File::create(&output_file)?;
    println!("Test 2");

    let walkdir = WalkDir::new(dir);
    println!("Test 3");
    let it = walkdir.into_iter().filter_map(|entry| entry.ok());
    println!("Test 4");
    let mut buffer = Vec::new();
    println!("Test 5");
    let mut zip = zip::ZipWriter::new(file);
    println!("Test 6");

    for entry in it {
        println!("Test 7");
        let path = entry.path();
        let name = path.strip_prefix(std::path::Path::new(dir))?;

        if path.is_file() {
            let mut file_in = std::fs::File::open(&path)?;
            zip.start_file(name.to_str().unwrap(), FileOptions::default())?;

            file_in.read_exact(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        }
        println!("Test 8");
    }
    Ok(())
}

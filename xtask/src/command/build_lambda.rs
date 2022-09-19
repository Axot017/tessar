use std::io::Write;

use std::io::Read;
use zip::write::FileOptions;

use crate::command::BuildLambdaArgs;
use crate::model::error::DynError;
use crate::util::cargo::run_cargo_build;
use crate::util::project::get_project_names;
use crate::util::project::project_root;

pub fn build_lambda(args: &BuildLambdaArgs) -> Result<(), DynError> {
    println!("Test 1");
    run_cargo_build(&args.target, args.use_cross)?;
    println!("Test 2");
    let lambdas_names = get_project_names(project_root().join("backend").join("lambda"));
    println!("Test 3");
    let target_path = project_root().join("target");
    println!("Test 4");
    let release_path = target_path.join(&args.target).join("release");
    println!("Test 5");
    let lambdas_dir = target_path.join("lambdas");
    println!("Test 6");

    std::fs::remove_dir_all(&lambdas_dir).ok();
    println!("Test 7");

    std::fs::create_dir_all(&lambdas_dir)?;

    for lambda in lambdas_names {
        println!("Test 8");
        let lambda_executable_path = release_path.join(&lambda);
        println!("Test 8");
        let mut lambda_executable = std::fs::File::open(&lambda_executable_path)?;
        println!("Test 9");
        let zip_name = format!("{}.zip", lambda);
        println!("Test 10");
        let zip_path = lambdas_dir.join(zip_name);
        println!("Test 11");
        let file = std::fs::File::create(&zip_path)?;
        println!("Test 12");

        let mut zip = zip::ZipWriter::new(file);
        println!("Test 13");
        zip.start_file(&args.entrypoint, FileOptions::default())?;
        println!("Test 14");
        let metadata = std::fs::metadata(&lambda_executable_path)?;
        println!("Test 15");
        let mut buffer = vec![0; metadata.len() as usize];
        println!("Test 16");
        lambda_executable.read_exact(&mut buffer)?;
        println!("Test 17");
        zip.write_all(&buffer)?;
        println!("Test 18");
        zip.finish()?;
    }

    Ok(())
}

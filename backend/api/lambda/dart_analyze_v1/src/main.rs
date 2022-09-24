use std::path::Path;

use lambda_http::{run, service_fn, Body, Error, Request, Response};
use tracing::log::error;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let opt_path = Path::new("/").join("opt");
    error!("opt: {}", opt_path.is_dir());
    let lib_path = opt_path.join("lib");
    error!("lib: {}", lib_path.is_dir());
    let project_path = lib_path.join("dart_project");
    error!("project: {}", project_path.is_dir());

    let result = std::process::Command::new("dart")
        .arg("analyze")
        .arg(project_path)
        .output()
        .unwrap();

    let result = String::from_utf8(result.stdout).unwrap_or_else(|e| e.to_string());

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(result.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

use lambda_http::{run, service_fn, Body, Error, Request, Response};

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let result = std::process::Command::new("dart")
        .current_dir("/opt/dart_project")
        .arg("analyze")
        .output()
        .unwrap();

    let result2 = String::from_utf8(result.stdout).unwrap_or_else(|e| e.to_string());

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(result2.into())
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


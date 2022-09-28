use actix_web::{get, post, web::Json, HttpResponse, Responder};

use crate::dto::{
    analyze_result_dto::{AnalyzeResultDto, Lint},
    file_dto::FileDto,
    version_dto::VersionDto,
};

#[get("/version")]
async fn version() -> impl Responder {
    let dto = VersionDto {
        message: "x.y.z".to_owned(),
    };

    HttpResponse::Ok().json(dto)
}

#[post("/format")]
async fn format(files: Json<Vec<FileDto>>) -> impl Responder {
    HttpResponse::Ok().json(files)
}

#[post("/analyze")]
async fn analyze(_files: Json<Vec<FileDto>>) -> impl Responder {
    let dto = AnalyzeResultDto {
        success: true,
        lints: vec![Lint {
            level: "info".to_owned(),
            message: "test message".to_owned(),
            file: "some/file.dart".to_owned(),
            column: 1,
            row: 10,
        }],
    };

    HttpResponse::Ok().json(dto)
}

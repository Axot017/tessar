use actix_web::{post, web::Json, HttpResponse, Responder};

use common_api::dto::code::{file_dto::FileDto, raw_message_dto::RawMessageDto};

#[post("/format")]
async fn format(files: Json<Vec<FileDto>>) -> impl Responder {
    HttpResponse::Ok().json(files)
}

#[post("/analyze/raw")]
async fn analyze_raw(_files: Json<Vec<FileDto>>) -> impl Responder {
    let dto = RawMessageDto {
        success: true,
        message: "Raw dart analyze result".to_owned(),
    };

    HttpResponse::Ok().json(dto)
}

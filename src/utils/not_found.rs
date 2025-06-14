use actix_web::{HttpResponse, Responder};
use actix_web::http::StatusCode;

use crate::utils::response::GenericResponse;

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(GenericResponse {
        status: StatusCode::NOT_FOUND.as_u16(),
        data: None::<()>,
        message: "The requested resource was not found".to_string(),
    })
} 
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

use crate::utils::response::GenericResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Validation error")]
    Validation(validator::ValidationErrors),
    #[error("Forbidden: {0}")]
    Forbidden(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(_) => HttpResponse::InternalServerError().json(GenericResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                data: None::<()>,
                message: "Internal server error".to_string(),
            }),
            AppError::InvalidCredentials => HttpResponse::Unauthorized().json(GenericResponse {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                data: None::<()>,
                message: "Invalid credentials".to_string(),
            }),
            AppError::InternalServerError => HttpResponse::InternalServerError().json(GenericResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                data: None::<()>,
                message: "Internal server error".to_string(),
            }),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(GenericResponse {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                data: None::<()>,
                message: msg.clone(),
            }),
            AppError::Validation(errors) => HttpResponse::BadRequest().json(GenericResponse {
                status: StatusCode::BAD_REQUEST.as_u16(),
                data: Some(errors.clone()),
                message: "Validation error".to_string(),
            }),
            AppError::Forbidden(msg) => HttpResponse::Forbidden().json(GenericResponse {
                status: StatusCode::FORBIDDEN.as_u16(),
                data: None::<()>,
                message: msg.clone(),
            }),
        }
    }
}
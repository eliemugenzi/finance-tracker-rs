use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgDatabaseError;
use validator::ValidationErrors;
use std::fmt;
use log::{error, warn, info};
use bcrypt::BcryptError;

use crate::utils::response::GenericResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Validation error: {0}")]
    Validation(ValidationErrors),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Resource already exists: {0}")]
    ResourceExists(String),
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        AppError::Validation(errors)
    }
}

impl From<BcryptError> for AppError {
    fn from(err: BcryptError) -> Self {
        AppError::InternalServerError(format!("Password hashing error: {}", err))
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::InternalServerError(err)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        // Log the error before returning the response
        match self {
            AppError::Database(e) => {
                error!("Database error: {:?}", e);
                if let sqlx::Error::Database(db_err) = e {
                    if let Some(code) = db_err.code() {
                        if code == "23505" {
                            let message = if db_err.message().contains("users_username_key") {
                                "Username is already taken"
                            } else if db_err.message().contains("users_email_key") {
                                "Email is already taken"
                            } else {
                                "Resource already exists"
                            };
                            warn!("Database constraint violation: {}", message);
                            return HttpResponse::Conflict().json(GenericResponse {
                                status: StatusCode::CONFLICT.as_u16(),
                                data: None::<()>,
                                message: message.to_string(),
                            });
                        }
                    }
                }
                HttpResponse::InternalServerError().json(GenericResponse {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    data: None::<()>,
                    message: format!("Database error: {}", e),
                })
            },
            AppError::InvalidCredentials(msg) => {
                warn!("Invalid credentials: {}", msg);
                HttpResponse::Unauthorized().json(GenericResponse {
                    status: StatusCode::UNAUTHORIZED.as_u16(),
                    data: None::<()>,
                    message: msg.clone(),
                })
            },
            AppError::InternalServerError(msg) => {
                error!("Internal server error: {}", msg);
                HttpResponse::InternalServerError().json(GenericResponse {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    data: None::<()>,
                    message: msg.clone(),
                })
            },
            AppError::Unauthorized(msg) => {
                warn!("Unauthorized: {}", msg);
                HttpResponse::Unauthorized().json(GenericResponse {
                    status: StatusCode::UNAUTHORIZED.as_u16(),
                    data: None::<()>,
                    message: msg.clone(),
                })
            },
            AppError::Validation(errors) => {
                warn!("Validation error: {:?}", errors);
                HttpResponse::BadRequest().json(GenericResponse {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    data: Some(errors.clone()),
                    message: format!("Validation error: {}", errors),
                })
            },
            AppError::Forbidden(msg) => {
                warn!("Forbidden: {}", msg);
                HttpResponse::Forbidden().json(GenericResponse {
                    status: StatusCode::FORBIDDEN.as_u16(),
                    data: None::<()>,
                    message: msg.clone(),
                })
            },
            AppError::ResourceExists(msg) => {
                info!("Resource exists: {}", msg);
                HttpResponse::Conflict().json(GenericResponse {
                    status: StatusCode::CONFLICT.as_u16(),
                    data: None::<()>,
                    message: msg.clone(),
                })
            },
        }
    }
}
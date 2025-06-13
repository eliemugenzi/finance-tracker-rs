use actix_web::{http::StatusCode, web, HttpResponse, Responder, HttpRequest, HttpMessage};
use sqlx::PgPool;
use validator::Validate;

use crate::modules::users::dtos::{RegisterRequest, LoginRequest};
use crate::modules::users::services::{register_new_user, login_user};
use crate::utils::jwt::Claims;
use crate::utils::response::GenericResponse;
use crate::utils::errors::AppError;
use crate::middleware::jwt::JwtMiddleware;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .service(
                web::resource("/profile")
                    .wrap(JwtMiddleware)
                    .route(web::get().to(get_profile))
            ),
    );
}

async fn register(
    pool: web::Data<PgPool>,
    user: web::Json<RegisterRequest>,
) -> impl Responder {
    if let Err(errors) = user.validate() {
        return Err(AppError::Validation(errors));
    }
    match register_new_user(&pool, &user.username, &user.email, &user.password, user.role.as_ref()).await {
        Ok(_) => Ok(HttpResponse::Created().json(GenericResponse {
            status: StatusCode::CREATED.as_u16(),
            data: None::<()>,
            message: "User registered successfully".to_string(),
        })),
        Err(e) => Err(AppError::Database(e)),
    }
}

async fn login(
    pool: web::Data<PgPool>,
    credentials: web::Json<LoginRequest>,
) -> impl Responder {
    if let Err(errors) = credentials.validate() {
        return Err(AppError::Validation(errors));
    }
    match login_user(&pool, &credentials.username, &credentials.password).await {
        Ok(token) => Ok(HttpResponse::Ok().json(GenericResponse {
            status: StatusCode::OK.as_u16(),
            data: Some(serde_json::json!({ "token": token })),
            message: "Login successful".to_string(),
        })),
        Err(_) => Err(AppError::InvalidCredentials),
    }
}

async fn get_profile(req: HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions_mut().get::<Claims>() {
        Ok(HttpResponse::Ok().json(GenericResponse {
            status: StatusCode::OK.as_u16(),
            data: Some(serde_json::json!({
                "username": claims.sub,
                "user_id": claims.user_id,
                "role": claims.role
            })),
            message: "Profile retrieved successfully".to_string(),
        }))
    } else {
        Err(AppError::Unauthorized("Unauthorized".to_string()))
    }
}
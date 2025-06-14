use actix_web::{http::StatusCode, web, HttpResponse, Responder, HttpRequest, HttpMessage};
use sqlx::PgPool;
use validator::Validate;

use crate::modules::users::dtos::{RegisterRequest, LoginRequest};
use crate::modules::users::services::{register_new_user, login_user};
use crate::utils::jwt::Claims;
use crate::utils::response::GenericResponse;
use crate::utils::errors::AppError;
use crate::middleware::jwt::JwtMiddleware;
use crate::utils::roles::Role;
use crate::AppState;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(
                web::resource("/register")
                    .route(web::post().to(register))
            )
            .service(
                web::resource("/login")
                    .route(web::post().to(login))
            )
            .service(
                web::resource("/profile")
                    .wrap(JwtMiddleware)
                    .route(web::get().to(get_profile))
            ),
    );
}

async fn register(
    state: web::Data<AppState>,
    user_data: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    user_data.validate()?;

    let user = register_new_user(
        &state.db,
        &user_data.username,
        &user_data.email,
        &user_data.password,
        user_data.role.as_ref(),
    ).await?;

    Ok(HttpResponse::Created().json(GenericResponse {
        status: StatusCode::CREATED.as_u16(),
        data: Some(serde_json::to_value(user).unwrap()),
        message: "User registered successfully".to_string(),
    }))
}

async fn login(
    state: web::Data<AppState>,
    credentials: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    credentials.validate()?;

    let token = login_user(
        &state.db,
        &credentials.username,
        &credentials.password,
    ).await?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: StatusCode::OK.as_u16(),
        data: Some(serde_json::json!({ "token": token })),
        message: "Login successful".to_string(),
    }))
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
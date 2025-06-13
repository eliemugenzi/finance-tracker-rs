use actix_web::{http::StatusCode, web, HttpResponse, Responder, HttpRequest, HttpMessage};
use serde::Serialize;
use sqlx::PgPool;
use validator::Validate;

use crate::modules::transactions::dtos::TransactionRequest;
use crate::modules::transactions::services::{record_user_transaction, list_user_transactions, get_user_financial_summary};
use crate::utils::jwt::Claims;
use crate::utils::response::GenericResponse;
use crate::utils::errors::AppError;
use crate::middleware::jwt::JwtMiddleware;
use crate::middleware::rbac::RbacMiddleware;
use crate::utils::roles::Role;

#[derive(Serialize)]
pub struct SummaryResponse {
    pub(crate) total_income: f64,
    pub(crate) total_expense: f64,
    pub(crate) balance: f64,
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/transactions")
            .service(
                web::resource("")
                    .wrap(JwtMiddleware)
                    .route(web::post().to(add_transaction))
                    .route(web::get().to(list_transactions))
            )
            .service(
                web::resource("/summary")
                    .wrap(RbacMiddleware {
                        allowed_roles: vec![Role::USER, Role::ADMIN]
                    })
                    .wrap(JwtMiddleware)
                    .route(web::get().to(summary))
            ),
    );
}

async fn add_transaction(
    pool: web::Data<PgPool>,
    transaction: web::Json<TransactionRequest>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(errors) = transaction.validate() {
        return Err(AppError::Validation(errors));
    }
    let claims = req.extensions_mut()
        .get::<Claims>()
        .expect("Claims should be present")
        .clone();
    match record_user_transaction(
        &pool,
        claims.user_id,
        transaction.amount,
        &transaction.category,
        &transaction.description,
        &transaction.date,
    )
        .await
    {
        Ok(_) => Ok(HttpResponse::Created().json(GenericResponse {
            status: StatusCode::CREATED.as_u16(),
            data: None::<()>,
            message: "Transaction recorded successfully".to_string(),
        })),
        Err(e) => Err(AppError::Database(e)),
    }
}

async fn list_transactions(pool: web::Data<PgPool>, req: HttpRequest) -> impl Responder {
    let claims = req.extensions_mut().get::<Claims>().expect("Claims should be present").clone();
    match list_user_transactions(&pool, claims.user_id).await {
        Ok(transactions) => Ok(HttpResponse::Ok().json(GenericResponse {
            status: StatusCode::OK.as_u16(),
            data: Some(transactions),
            message: "Transactions retrieved successfully".to_string(),
        })),
        Err(e) => Err(AppError::Database(e)),
    }
}

async fn summary(pool: web::Data<PgPool>, req: HttpRequest) -> impl Responder {
    let claims = req.extensions_mut().get::<Claims>().expect("Claims should be present").clone();
    match get_user_financial_summary(&pool, claims.user_id).await {
        Ok(summary) => Ok(HttpResponse::Ok().json(GenericResponse {
            status: StatusCode::OK.as_u16(),
            data: Some(summary),
            message: "Summary retrieved successfully".to_string(),
        })),
        Err(e) => Err(AppError::Database(e)),
    }
}
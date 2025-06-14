use actix_web::{http::StatusCode, web, HttpResponse, Responder, HttpRequest, HttpMessage};
use serde::Serialize;
use validator::Validate;
use log::{error, warn};

use crate::modules::transactions::dtos::TransactionRequest;
use crate::modules::transactions::services::{record_user_transaction, list_user_transactions, get_user_financial_summary};
use crate::utils::jwt::Claims;
use crate::utils::response::GenericResponse;
use crate::utils::errors::AppError;
use crate::middleware::jwt::JwtMiddleware;
use crate::middleware::rbac::RbacMiddleware;
use crate::utils::roles::Role;
use crate::AppState;

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
    state: web::Data<AppState>,
    transaction: web::Json<TransactionRequest>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(errors) = transaction.validate() {
        warn!("Transaction validation failed: {:?}", errors);
        return Err(AppError::Validation(errors));
    }

    let claims = match req.extensions_mut().get::<Claims>() {
        Some(claims) => claims.clone(),
        None => {
            error!("Claims not found in request extensions");
            return Err(AppError::InternalServerError("Authentication state error".to_string()));
        }
    };

    match record_user_transaction(
        &state.db,
        claims.user_id,
        transaction.amount,
        &transaction.category,
        &transaction.description,
    ).await {
        Ok(transaction) => {
            log::info!("Transaction recorded successfully for user {}: {}", claims.user_id, transaction);
            Ok(HttpResponse::Created().json(GenericResponse {
                status: StatusCode::CREATED.as_u16(),
                data: Some(transaction),
                message: "Transaction recorded successfully".to_string(),
            }))
        },
        Err(e) => {
            error!("Failed to record transaction for user {}: {:?}", claims.user_id, e);
            Err(AppError::Database(e))
        }
    }
}

async fn list_transactions(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let claims = match req.extensions_mut().get::<Claims>() {
        Some(claims) => claims.clone(),
        None => {
            error!("Claims not found in request extensions");
            return Err(AppError::InternalServerError("Authentication state error".to_string()));
        }
    };

    match list_user_transactions(&state.db, claims.user_id).await {
        Ok(transactions) => {
            log::info!("Retrieved {} transactions for user {}", transactions.len(), claims.user_id);
            Ok(HttpResponse::Ok().json(GenericResponse {
                status: StatusCode::OK.as_u16(),
                data: Some(transactions),
                message: "Transactions retrieved successfully".to_string(),
            }))
        },
        Err(e) => {
            error!("Failed to list transactions for user {}: {:?}", claims.user_id, e);
            Err(AppError::Database(e))
        }
    }
}

async fn summary(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let claims = match req.extensions_mut().get::<Claims>() {
        Some(claims) => claims.clone(),
        None => {
            error!("Claims not found in request extensions");
            return Err(AppError::InternalServerError("Authentication state error".to_string()));
        }
    };

    match get_user_financial_summary(&state.db, claims.user_id).await {
        Ok(summary) => {
            log::info!("Retrieved financial summary for user {}: income={}, expense={}, balance={}", 
                claims.user_id, summary.total_income, summary.total_expense, summary.balance);
            Ok(HttpResponse::Ok().json(GenericResponse {
                status: StatusCode::OK.as_u16(),
                data: Some(summary),
                message: "Summary retrieved successfully".to_string(),
            }))
        },
        Err(e) => {
            error!("Failed to get financial summary for user {}: {:?}", claims.user_id, e);
            Err(AppError::Database(e))
        }
    }
}
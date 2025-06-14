use sqlx::PgPool;

use crate::modules::transactions::models::Transaction;
use crate::modules::transactions::repositories::{insert_transaction, find_transactions_by_user_id, calculate_user_transaction_summary};
use crate::modules::transactions::routes::SummaryResponse;

pub async fn record_user_transaction(
    pool: &PgPool,
    user_id: i32,
    amount: f64,
    category: &str,
    description: &str,
) -> Result<Transaction, sqlx::Error> {
    insert_transaction(pool, user_id, amount, category, description).await
}

pub async fn list_user_transactions(pool: &PgPool, user_id: i32) -> Result<Vec<Transaction>, sqlx::Error> {
    find_transactions_by_user_id(pool, user_id).await
}

pub async fn get_user_financial_summary(pool: &PgPool, user_id: i32) -> Result<SummaryResponse, sqlx::Error> {
    let (total_income, total_expense) = calculate_user_transaction_summary(pool, user_id).await?;
    Ok(SummaryResponse {
        total_income,
        total_expense: total_expense.abs(),
        balance: total_income + total_expense,
    })
}
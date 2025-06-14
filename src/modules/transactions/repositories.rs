use sqlx::PgPool;
use bigdecimal::BigDecimal;
use std::str::FromStr;

use crate::modules::transactions::models::Transaction;

pub async fn insert_transaction(
    pool: &PgPool,
    user_id: i32,
    amount: f64,
    category: &str,
    description: &str,
) -> Result<Transaction, sqlx::Error> {
    if amount.is_nan() || amount.is_infinite() {
        return Err(sqlx::Error::Decode("Invalid f64: NaN or infinite".into()));
    }
    let amount_str = format!("{:.2}", amount); // Format to 2 decimal places
    let amount_bd = BigDecimal::from_str(&amount_str).map_err(|e| {
        sqlx::Error::Decode(format!("Failed to convert f64 to BigDecimal: {}", e).into())
    })?;

    let record = sqlx::query!(
        "INSERT INTO transactions (user_id, amount, description, category, updated_at) VALUES ($1, $2, $3, $4, NOW()) RETURNING id, user_id, amount::float8 as amount, description, category, created_at, updated_at",
        user_id,
        amount_bd,
        description,
        category
    )
        .fetch_one(pool)
        .await?;

    Ok(Transaction {
        id: record.id,
        user_id: record.user_id,
        amount: record.amount.unwrap_or_default(),
        description: record.description,
        category: Some(record.category),
        created_at: record.created_at.expect("created_at is NOT NULL"),
        updated_at: record.updated_at.expect("updated_at is NOT NULL"),
    })
}

pub async fn find_transactions_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<Transaction>, sqlx::Error> {
    let records = sqlx::query!(
        "SELECT id, user_id, amount::float8 as amount, description, category, created_at, updated_at FROM transactions WHERE user_id = $1",
        user_id
    )
        .fetch_all(pool)
        .await?;

    Ok(records.into_iter().map(|r| Transaction {
        id: r.id,
        user_id: r.user_id,
        amount: r.amount.unwrap_or_default(),
        description: r.description,
        category: Some(r.category),
        created_at: r.created_at.expect("created_at is NOT NULL"),
        updated_at: r.updated_at.expect("updated_at is NOT NULL"),
    }).collect())
}

pub async fn calculate_user_transaction_summary(pool: &PgPool, user_id: i32) -> Result<(f64, f64), sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT
            COALESCE(SUM(CASE WHEN amount > 0 THEN (amount::float8) ELSE 0 END), 0) as total_income,
            COALESCE(SUM(CASE WHEN amount < 0 THEN (amount::float8) ELSE 0 END), 0) as total_expense
        FROM transactions
        WHERE user_id = $1
        "#,
        user_id
    )
        .fetch_one(pool)
        .await?;
    
    Ok((
        result.total_income.unwrap_or(0.0),
        result.total_expense.unwrap_or(0.0)
    ))
}

pub async fn find_transaction_by_id(pool: &PgPool, id: i32) -> Result<Option<Transaction>, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT id, user_id, amount::float8 as amount, description, category, created_at, updated_at FROM transactions WHERE id = $1",
        id
    )
        .fetch_optional(pool)
        .await?;

    Ok(record.map(|r| Transaction {
        id: r.id,
        user_id: r.user_id,
        amount: r.amount.unwrap_or_default(),
        description: r.description,
        category: Some(r.category),
        created_at: r.created_at.expect("created_at is NOT NULL"),
        updated_at: r.updated_at.expect("updated_at is NOT NULL"),
    }))
}
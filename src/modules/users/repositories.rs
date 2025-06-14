use crate::utils::roles::Role;
use sqlx::PgPool;
use chrono::{DateTime, Utc};

use crate::modules::users::models::User;

pub async fn check_user_exists(
    pool: &PgPool,
    username: &str,
    email: &str,
) -> Result<Option<String>, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT username, email FROM users WHERE username = $1 OR email = $2",
        username,
        email
    )
        .fetch_optional(pool)
        .await?;

    Ok(record.map(|r| {
        if r.username == username {
            "Username is already taken".to_string()
        } else {
            "Email is already taken".to_string()
        }
    }))
}

pub async fn insert_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
    role: &str,
) -> Result<User, sqlx::Error> {
    let record = sqlx::query!(
        "INSERT INTO users (username, email, password_hash, role, updated_at) VALUES ($1, $2, $3, $4, NOW()) RETURNING id, username, email, password_hash, role as \"role: Role\", COALESCE(created_at, NOW()) as created_at, updated_at",
        username,
        email,
        password_hash,
        role
    )
        .fetch_one(pool)
        .await?;

    Ok(User {
        id: record.id,
        username: record.username,
        email: record.email,
        password_hash: record.password_hash,
        role: record.role,
        created_at: record.created_at.expect("created_at is NOT NULL"),
        updated_at: record.updated_at.expect("updated_at is NOT NULL"),
    })
}

pub async fn find_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT id, username, email, password_hash, role as \"role: Role\", COALESCE(created_at, NOW()) as created_at, updated_at FROM users WHERE username = $1",
        username
    )
        .fetch_optional(pool)
        .await?;

    Ok(record.map(|r| User {
        id: r.id,
        username: r.username,
        email: r.email,
        password_hash: r.password_hash,
        role: r.role,
        created_at: r.created_at.expect("created_at is NOT NULL"),
        updated_at: r.updated_at.expect("updated_at is NOT NULL"),
    }))
}
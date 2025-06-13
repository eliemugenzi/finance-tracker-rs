use sqlx::PgPool;

use crate::utils::bcrypt::hash_password;
use crate::utils::jwt::{generate_token, Claims};
use crate::utils::roles::Role;
use crate::modules::users::models::User;
use crate::modules::users::repositories::{insert_user, find_user_by_username};

pub async fn register_new_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password: &str,
    role: Option<&Role>,
) -> Result<User, sqlx::Error> {
    let password_hash = hash_password(password).expect("Failed to hash password");
    let role = role.unwrap_or(&Role::USER).as_str().to_string();
    insert_user(pool, username, email, &password_hash, &role).await
}

pub async fn login_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<String, sqlx::Error> {
    let user = find_user_by_username(pool, username).await?;

    if let Some(user) = user {
        if crate::utils::bcrypt::verify_password(password, &user.password_hash)
            .expect("Failed to verify password")
        {
            let claims = Claims {
                sub: user.username,
                user_id: user.id,
                role: user.role.as_str().to_string(),
                exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
            };
            let token = generate_token(&claims).expect("Failed to generate token");
            Ok(token)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}
use sqlx::PgPool;

use crate::utils::bcrypt::hash_password;
use crate::utils::jwt::{generate_token, Claims};
use crate::utils::roles::Role;
use crate::modules::users::models::User;
use crate::modules::users::repositories::{insert_user, find_user_by_username, check_user_exists};
use crate::utils::errors::AppError;
use log::{error, warn, info};

pub async fn register_new_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password: &str,
    role: Option<&Role>,
) -> Result<User, AppError> {
    // Check if username or email already exists
    if let Some(message) = check_user_exists(pool, username, email).await? {
        return Err(AppError::ResourceExists(message));
    }

    let password_hash = hash_password(password)?;
    let role = role.unwrap_or(&Role::USER).as_str().to_string();
    
    match insert_user(pool, username, email, &password_hash, &role).await {
        Ok(user) => {
            info!("User registered successfully: {}", user);
            Ok(user)
        },
        Err(e) => {
            error!("Failed to register user {}: {:?}", username, e);
            Err(AppError::Database(e))
        }
    }
}

pub async fn login_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<String, AppError> {
    let user = find_user_by_username(pool, username)
        .await
        .map_err(|e| {
            error!("Database error while finding user {}: {:?}", username, e);
            AppError::Database(e)
        })?;
    
    let user = match &user {
        Some(user) => {
            info!("Found user: {}", user);
            user
        },
        None => {
            warn!("No user found with username: {}", username);
            return Err(AppError::InvalidCredentials("User not found".to_string()));
        }
    };
    
    if !crate::utils::bcrypt::verify_password(password, &user.password_hash)? {
        warn!("Invalid password for user: {}", username);
        return Err(AppError::InvalidCredentials("Invalid password".to_string()));
    }

    let claims = Claims {
        sub: user.username.clone(),
        user_id: user.id,
        role: user.role.as_str().to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    
    match generate_token(&claims) {
        Ok(token) => {
            info!("Generated token for user: {}", username);
            Ok(token)
        },
        Err(e) => {
            error!("Failed to generate token for user {}: {:?}", username, e);
            Err(AppError::InternalServerError("Failed to generate authentication token".to_string()))
        }
    }
}
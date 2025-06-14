use serde::Deserialize;
use validator::{Validate, ValidationError};
use crate::utils::roles::Role;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[serde(default)]
    pub role: Option<Role>,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

// fn validate_role(role: &Option<String>) -> Result<(), ValidationError> {
//     if let Some(r) = role {
//         if Role::from_str(&r.to_uppercase()).is_err() {
//             let mut error = ValidationError::new("role");
//             error.message = Some("Role must be 'USER' or 'ADMIN'".into());
//             return Err(error);
//         }
//     }
//     Ok(())
// }
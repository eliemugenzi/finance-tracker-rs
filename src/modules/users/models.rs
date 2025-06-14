use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer};
use std::fmt;
use std::str::FromStr;
use crate::utils::roles::Role;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    #[serde(deserialize_with = "deserialize_role")]
    pub role: Role,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User {{ id: {}, username: {}, email: {}, role: {}, created_at: {}, updated_at: {} }}",
            self.id,
            self.username,
            self.email,
            self.role,
            self.created_at.format("%Y-%m-%d %H:%M:%S"),
            self.updated_at.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

fn deserialize_role<'de, D>(deserializer: D) -> Result<Role, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Role::from_str(&s).map_err(de::Error::custom)
}
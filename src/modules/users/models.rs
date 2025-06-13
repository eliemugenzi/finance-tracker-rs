use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer};
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

fn deserialize_role<'de, D>(deserializer: D) -> Result<Role, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Role::from_str(&s).map_err(de::Error::custom)
}
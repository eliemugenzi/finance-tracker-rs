use serde::{Deserialize, Serialize};
use std::fmt;
use sqlx::{Type, Decode, Postgres, postgres::{PgHasArrayType, PgTypeInfo}};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "VARCHAR", rename_all = "UPPERCASE")]
pub enum Role {
    USER,
    ADMIN,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::USER => "USER",
            Role::ADMIN => "ADMIN",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "USER" => Ok(Role::USER),
            "ADMIN" => Ok(Role::ADMIN),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub amount: f64,
    pub description: Option<String>, // Now optional
    pub category: Option<String>, // Now optional
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction {{ id: {}, user_id: {}, amount: {:.2}, category: {}, description: {}, created_at: {}, updated_at: {} }}",
            self.id,
            self.user_id,
            self.amount,
            self.category.as_deref().unwrap_or("None"),
            self.description.as_deref().unwrap_or("None"),
            self.created_at.format("%Y-%m-%d %H:%M:%S"),
            self.updated_at.format("%Y-%m-%d %H:%M:%S")
        )
    }
}
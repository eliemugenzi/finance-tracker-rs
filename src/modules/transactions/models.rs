use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

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
use serde::Deserialize;
use validator::{Validate};
use chrono::NaiveDate;

#[derive(Deserialize, Validate)]
pub struct TransactionRequest {
    #[validate(range(min = 100.0, max = 1000000.0, message = "Amount must be between 100 and 1,000,000"))]
    pub amount: f64,
    #[validate(length(min = 1, max = 50, message = "Category must be between 1 and 50 characters"))]
    pub category: String,
    #[validate(length(max = 255, message = "Description cannot exceed 255 characters"))]
    pub description: String,
    #[serde(with = "date_format")]
    pub date: NaiveDate,
}

mod date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
    }
}
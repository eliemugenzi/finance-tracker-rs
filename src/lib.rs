use sqlx::PgPool;

pub mod modules;
pub mod utils;
pub mod middleware;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

// Re-export commonly used items
pub use modules::*;
pub use utils::*;
pub use middleware::*; 
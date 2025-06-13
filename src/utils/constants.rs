pub mod api {
    pub const API_PREFIX: &str = "/api/v1";
    pub const API_VERSION: &str = "v1";
}

pub mod auth {
    pub const JWT_EXPIRY_SECONDS: i64 = 24 * 3600; // 24 hours
}

pub mod db {
    pub const MAX_USERNAME_LENGTH: usize = 50;
}

pub mod server {
    pub const PORT: u16 = 8080; // Default port
}
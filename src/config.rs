use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL no está definida");

    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET no está definida");

        let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .ok()
            .and_then(|h| h.parse::<i64>().ok())
            .unwrap_or(24);

        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let server_port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3000);

        Self {
            database_url,
            jwt_secret,
            jwt_expiration_hours,
            server_host,
            server_port,
        }
    }
}

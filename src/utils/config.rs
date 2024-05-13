use std::env;

pub struct Config {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub domain: String,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            google_client_id: env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set"),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
                .expect("GOOGLE_CLIENT_SECRET not set"),
            domain: env::var("DOMAIN").expect("DOMAIN not set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not set"),
        }
    }
}

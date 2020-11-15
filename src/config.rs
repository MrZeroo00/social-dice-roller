use dotenv::dotenv;
use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;
use std::env;

/// Creates rocket config from environment variables
pub fn from_env() -> Config {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let mut database_config = HashMap::new();
    database_config.insert("url", Value::from(database_url));

    let mut databases = HashMap::new();
    databases.insert("sqlite_database", Value::from(database_config));

    Config::build(Environment::Development)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}

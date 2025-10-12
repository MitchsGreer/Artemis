use std::env;
use dotenvy::dotenv;

const DB_ADDRESS: &str = "DB_ADDRESS";
const POSTGRES_DB: &str = "POSTGRES_DB";
const POSTGRES_USER: &str = "POSTGRES_USER";
const POSTGRES_PASSWORD: &str = "POSTGRES_PASSWORD";
const POSTGRES_PORT: &str = "POSTGRES_PORT";

pub fn db_address() -> String {
    dotenv().ok();
    return env::var(DB_ADDRESS).expect("DB_ADDRESS is not set in the .env file.");
}

pub fn db_name() -> String {
    dotenv().ok();
    return env::var(POSTGRES_DB).expect("POSTGRES_DB is not set in the .env file.");
}

pub fn db_user() -> String {
    dotenv().ok();
    return env::var(POSTGRES_USER).expect("POSTGRES_USER is not set in the .env file.");
}

pub fn db_password() -> String {
    dotenv().ok();
    return env::var(POSTGRES_PASSWORD).expect("POSTGRES_PASSWORD is not set in the .env file.");
}

pub fn db_port() -> u16 {
    dotenv().ok();
    return env::var(POSTGRES_PORT)
        .expect("POSTGRES_PORT is not set in the .env file.")
        .parse()
        .expect("POSTGRES_PORT is not a valid integer.");
}
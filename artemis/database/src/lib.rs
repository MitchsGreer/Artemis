use dotenvy::dotenv;
use postgres::Client;
use std::env;

// TODO: remove and pull from repo environment variables
const DEFAULT_DB_USERNAME: &str = "postgres";
const PASSWORD_KEY: &str = "POSTGRES_PASSWORD";

// utilize the "sqlx" crate from crates.io to manage postgres DB container.
// the container should essentially be the same as having run:
// docker run --name artemis-db --env-file .env -d postgres
pub async fn init() {
    // TODO: implement
}

pub fn connect_to_db() -> Client {
    let db_password = get_db_password();

    // TODO: pull host:port details from config
    let mut db_config = postgres::Config::new();
    db_config
        .user(DEFAULT_DB_USERNAME)
        .password(db_password)
        .dbname("postgres")
        .host("localhost")
        .port(5432);

    return db_config
        .connect(postgres::NoTls)
        .expect("ERROR: failed to connect to db");
}

fn get_db_password() -> String {
    // load .env into local environment variables
    dotenv().ok();

    let db_password: String =
        env::var(PASSWORD_KEY).expect("The postgres password is not set in the .env file.");
    return db_password;
}

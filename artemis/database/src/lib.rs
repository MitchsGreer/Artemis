use postgres::Client;

pub mod envs;

pub fn connect_to_db() -> Client {
    let db_address: String = envs::db_address();
    let db_name: String = envs::db_name();
    let db_user: String = envs::db_user();
    let db_password: String = envs::db_password();
    let db_port: u16 = envs::db_port();

    let mut db_config = postgres::Config::new();
    db_config
        .user(db_user.as_str())
        .password(db_password.as_str())
        .dbname(db_name.as_str())
        .host(db_address.as_str())
        .port(db_port);

    return db_config
        .connect(postgres::NoTls)
        .expect("ERROR: failed to connect to db");
}


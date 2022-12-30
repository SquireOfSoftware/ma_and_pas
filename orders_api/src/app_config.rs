use deadpool_postgres::{Config, Pool};
use std::env::var;
use tokio_postgres::NoTls;

const PG_USER: &str = "POSTGRES_USER";
const PG_PASSWORD: &str = "POSTGRES_PASSWORD";
const DB_NAME: &str = "DB_NAME";
const DB_HOST: &str = "DB_HOST";
const DB_PORT: &str = "DB_PORT";
const APP_PORT: &str = "APP_PORT";

pub async fn get_app_port() -> u16 {
    match var(APP_PORT) {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => 8001,
    }
}

pub async fn create_db_pool() -> Pool {
    Config::create_pool(
        &Config {
            user: Some(
                var(PG_USER)
                    .expect(&format!("{} must be set", PG_USER).to_string())
                    .to_string(),
            ),
            password: Some(
                var(PG_PASSWORD)
                    .expect(&format!("{} must be set", PG_PASSWORD).to_string())
                    .to_string(),
            ),
            dbname: Some(
                var(DB_NAME)
                    .expect(&format!("{} must be set", DB_NAME))
                    .to_string()
                    .to_string(),
            ),
            host: Some(
                var(DB_HOST)
                    .expect(&format!("{} must be set", DB_HOST))
                    .to_string()
                    .to_string(),
            ),
            hosts: None,
            port: Some(
                var(DB_PORT)
                    .expect(&format!("{} must be set", DB_PORT).to_string())
                    .parse::<u16>()
                    .unwrap(),
            ),
            ports: None,
            connect_timeout: None,
            keepalives: None,
            keepalives_idle: None,
            application_name: Some("orders_api".to_string()),
            channel_binding: None,
            manager: None,
            options: None,
            ssl_mode: None,
            target_session_attrs: None,
            pool: None,
        },
        None,
        NoTls,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn default_app_port_when_no_env_is_provided() {
        let port = get_app_port().await;
        assert_eq!(8001, port);
    }
}

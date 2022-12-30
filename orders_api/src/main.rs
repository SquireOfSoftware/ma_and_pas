use actix_web::web::Data;
use actix_web::{guard, web, App, HttpServer};
use async_graphql::{EmptySubscription, Schema};

use deadpool_postgres::Config;
use tokio_postgres::NoTls;

use orders_api::graphql::{QueryRoot, MutationRoot};
use orders_api::routes::graphql::{index, index_playground};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = Config::create_pool(
        &Config {
            user: Some("postgres".to_string()),
            password: Some("test".to_string()),
            dbname: Some("test".to_string()),
            host: Some("localhost".to_string()),
            hosts: None,
            port: Some(5432),
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
    .unwrap();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .finish();
    // you must provision the data into the schema, it seems

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}

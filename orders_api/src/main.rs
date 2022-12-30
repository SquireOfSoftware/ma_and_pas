use actix_web::web::Data;
use actix_web::{guard, web, App, HttpServer};
use async_graphql::{EmptySubscription, Schema};

use dotenvy::dotenv;

use orders_api::app_config::{create_db_pool, get_app_port};
use orders_api::graphql::{MutationRoot, QueryRoot};
use orders_api::routes::graphql::{index, index_playground};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();

    let pool = create_db_pool().await;

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
    .bind(("127.0.0.1", get_app_port().await))?
    .run()
    .await
}

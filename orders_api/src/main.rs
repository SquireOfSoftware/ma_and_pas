use actix_web::error::HttpError;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use deadpool_postgres::{Client, Config, Pool};
use tokio_postgres::NoTls;

use orders_api::models::{QueryRoot, CustomError, MutationRoot};

pub type ShopSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn index(schema: Data<ShopSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse, HttpError> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

async fn initialise_db(db_pool: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let client: Client = db_pool.get().await.map_err(CustomError::PoolError)?;

    client
        .execute(
            "CREATE TABLE IF NOT EXISTS burgers (
       id serial PRIMARY KEY,
       burger_type VARCHAR (50) NOT NULL,
       cost serial NOT NULL
    );",
            &[],
        )
        .await?;

    client
        .execute(
            "INSERT INTO burgers (burger_type, cost) VALUES ($1, $2)",
            &[&"cheese".to_string(), &320],
        )
        .await?;

    Ok(HttpResponse::Ok().json("dummy data has been initalised"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = Config::create_pool(
        &deadpool_postgres::Config {
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
            .service(
                web::resource("/menu/initialise")
                    .guard(guard::Get())
                    .to(initialise_db),
            )
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}

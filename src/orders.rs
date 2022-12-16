use std::sync::{Arc, Mutex};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, guard};
use actix_web::error::HttpError;
use actix_web::web::Data;
use async_graphql::{http::{playground_source, GraphQLPlaygroundConfig}, Schema, Object, EmptySubscription, EmptyMutation, FieldResult, Context};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub type ShopSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

// this is the implementation of the query
#[Object(extends)]
impl QueryRoot {
    async fn hello(&self) -> FieldResult<&str> {
        Ok("hello")
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

// this is my schema?
pub struct Menu {
    hello: &'static str
}

impl Menu {
    pub async fn new() -> Self {
        Self {
            hello: "hello"
        }
    }
}

async fn index(schema: Data<ShopSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse, HttpError> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

// #[get("/orders/{order_id}")]
// async fn get_order(path: web::Path<u32>) -> impl Responder {
//     let order_id = path.into_inner();
//     HttpResponse::Ok().body(format!("hello {}", order_id))
// }
//
// #[post("/orders")]
// async fn create_order(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(Menu::new())
        .finish();
    // you must provision the data into the schema, it seems

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            // .service(get_order)
            // .service(create_order)
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
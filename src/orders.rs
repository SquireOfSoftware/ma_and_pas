use actix_web::{get, post, web, web::Data, App, HttpResponse, HttpServer, Responder, guard};
use actix_web::error::HttpError;
use async_graphql::{http::{playground_source, GraphQLPlaygroundConfig}, EmptyMutation, Schema, Object, EmptySubscription, Context};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub type ShopSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

// #[Object]
// struct MenuItem {
//     id: String,
//     name: String
// }

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

async fn index(schema: web::Data<ShopSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[get("/orders/{order_id}")]
async fn get_order(path: web::Path<u32>) -> impl Responder {
    let order_id = path.into_inner();
    HttpResponse::Ok().body(format!("hello {}", order_id))
}

#[post("/orders")]
async fn create_order(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .finish();

    HttpServer::new(move || {
        App::new()
            .service(get_order)
            .service(create_order)
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
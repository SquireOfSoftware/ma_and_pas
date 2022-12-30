use actix_web::error::HttpError;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use actix_web::web::Data;
use actix_web::HttpResponse;
use crate::models::ShopSchema;

pub async fn index(schema: Data<ShopSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_playground() -> Result<HttpResponse, HttpError> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}
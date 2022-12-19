use actix_web::{web, App, HttpResponse, HttpServer, guard};
use actix_web::error::HttpError;
use actix_web::web::Data;
use async_graphql::{http::{playground_source, GraphQLPlaygroundConfig}, Schema, Object, EmptySubscription, FieldResult, SimpleObject, Enum};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub type ShopSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
pub enum Size {
    Small,
    Medium,
    Large
}

#[derive(SimpleObject, Clone, Eq, PartialEq)]
pub struct Burger {
    id: String,
    burger_type: String,
}

#[derive(SimpleObject, Clone)]
pub struct Fries {
    id: String,
    size: Size
}

#[derive(Enum, PartialEq, Eq, Clone, Copy)]
pub enum DrinkType {
    Water,
    Coke,
    Sprite,
    OrangeJuice
}

#[derive(SimpleObject, Clone)]
pub struct Drink {
    id: String,
    size: Size,
    drink_type: DrinkType
}

#[derive(SimpleObject, Clone)]
pub struct Meal {
    id: String,
    name: String,
    burger: Burger,
    fries: Fries,
    drink: Drink
}

// this is my schema?
#[derive(SimpleObject, Clone)]
pub struct Menu {
    hello: String,
    meals: Vec<Meal>
}

impl Menu {
    pub async fn new() -> Self {
        Self {
            hello: "hello".to_string(),
            meals: [
                Meal {
                    id: "123".to_string(),
                    name: "Standard meal".to_string(),
                    burger: Burger {
                        id: "124".to_string(),
                        burger_type: "Cheesy".to_string()
                    },
                    fries: Fries {
                        id: "125".to_string(),
                        size: Size::Large
                    },
                    drink: Drink {
                        id: "126".to_string(),
                        size: Size::Large,
                        drink_type: DrinkType::Water
                    },
                }
            ].to_vec()
        }
    }
}

pub struct QueryRoot;

// this is the implementation of the query
#[Object]
impl QueryRoot {
    async fn hello(&self) -> FieldResult<&str> {
        Ok("hello")
    }
    async fn menu(&self) -> FieldResult<Menu> {
        Ok(Menu::new().await)
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
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
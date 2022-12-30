use actix_web::error::HttpError;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, ResponseError};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptySubscription, Enum, FieldResult, Object, Schema, SimpleObject,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use derive_more::{Display, From};

use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;
use tokio_postgres::{NoTls};

use deadpool_postgres::{Client, Config, Pool, PoolError};

use orders_api::models::Burger;

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
    Large,
}

#[derive(SimpleObject, Clone)]
pub struct Fries {
    id: Option<String>,
    size: Size,
    cost: i32,
}

#[derive(Enum, PartialEq, Eq, Clone, Copy)]
pub enum DrinkType {
    Water,
    Coke,
    Sprite,
    OrangeJuice,
}

#[derive(SimpleObject, Clone)]
pub struct Drink {
    id: Option<String>,
    size: Size,
    drink_type: DrinkType,
    cost: i32,
}

#[derive(SimpleObject, Clone)]
pub struct Meal {
    id: Option<String>,
    name: String,
    burger: Burger,
    fries: Fries,
    drink: Drink,
    cost: i32,
}

// this is my schema?
#[derive(SimpleObject, Clone)]
pub struct Menu {
    hello: String,
    meals: Vec<Meal>,
    burgers: Vec<Burger>,
}

impl Menu {
    pub async fn new() -> Self {
        Self {
            hello: "hello".to_string(),
            meals: [Meal {
                id: Some("123".to_string()),
                name: "Standard meal".to_string(),
                cost: 1200,
                burger: Burger {
                    id: Some("124".to_string()),
                    burger_type: "Cheesy".to_string(),
                    cost: 500,
                },
                fries: Fries {
                    id: Some("125".to_string()),
                    size: Size::Large,
                    cost: 300,
                },
                drink: Drink {
                    id: Some("126".to_string()),
                    size: Size::Large,
                    drink_type: DrinkType::Water,
                    cost: 200,
                },
            }]
            .to_vec(),
            burgers: [Burger {
                id: Some("124".to_string()),
                burger_type: "Cheesy".to_string(),
                cost: 500,
            }]
            .to_vec(),
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
    async fn get_burgers(&self, ctx: &Context<'_>) -> FieldResult<Vec<Burger>> {
        let db = &ctx
            .data_unchecked::<Pool>()
            .get()
            .await
            .map_err(CustomError::PoolError)?;

        let result = db.query("select * from burgers", &[])
            .await
            .unwrap();

        Ok(result.into_iter().map(|row| Burger::from(row)).collect::<Vec<Burger>>().to_vec())
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

#[derive(Display, From, Debug)]
pub enum CustomError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}

impl std::error::Error for CustomError {}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::NotFound => HttpResponse::NotFound().finish(),
            CustomError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
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

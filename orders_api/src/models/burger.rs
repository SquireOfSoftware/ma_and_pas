use async_graphql::{ SimpleObject };
use tokio_postgres::{ Row};
use serde::Serialize;

#[derive(SimpleObject, Clone, Eq, PartialEq, Serialize, Debug)]
pub struct Burger {
    pub id: Option<String>,
    pub burger_type: String,
    pub cost: i32,
}

impl From<Row> for Burger {
    fn from(row: Row) -> Self {
        Self {
            id: Some(row.get::<&str, i32>("id").to_string()),
            burger_type: row.get("burger_type"),
            cost: row.get("cost"),
        }
    }
}
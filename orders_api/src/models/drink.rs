use crate::models::Size;
use async_graphql::Enum;
use async_graphql::SimpleObject;
use serde::Serialize;
use tokio_postgres::Row;

#[derive(Enum, PartialEq, Eq, Clone, Copy)]
pub enum DrinkType {
    Water,
    Coke,
    Sprite,
    OrangeJuice,
}

#[derive(SimpleObject, Clone, Eq, PartialEq, Debug, Serialize)]
pub struct Drink {
    pub id: Option<String>,
    pub name: String,
    pub size: Size,
    pub cost: i32,
    pub active: bool,
}

impl From<Row> for Drink {
    fn from(row: Row) -> Self {
        Self {
            id: Some(row.get::<&str, &str>("code_name").to_string()),
            name: row.get::<&str, &str>("name").to_string(),
            active: row.get::<&str, bool>("active"),
            cost: row.get::<&str, i32>("cost"),
            size: Size::from(row.get::<&str, &str>("size")),
        }
    }
}

use crate::models::{Burger, Drink, Person, Side};
use async_graphql::SimpleObject;
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(SimpleObject, Clone, Eq, PartialEq, Serialize, Debug)]
pub struct Order {
    pub id: Option<String>,
    pub cost: Option<i32>,
    pub burgers: Vec<Burger>,
    pub drinks: Vec<Drink>,
    pub sides: Vec<Side>,
}

impl Order {
    pub fn build(row: &Row,
             burgers: Vec<Burger>,
             drinks: Vec<Drink>,
             sides: Vec<Side>) -> Self {
        Self {
            id: Some(row.get::<&str, Uuid>("id").to_string()),
            cost: Some(row.get::<&str, i32>("cost")),
            burgers,
            drinks,
            sides,
        }
    }
}
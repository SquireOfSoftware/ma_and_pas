use crate::models::{Burger, Drink, Side};
use async_graphql::SimpleObject;
use serde::Serialize;

#[derive(SimpleObject, Clone, Eq, PartialEq, Serialize, Debug)]
pub struct Order {
    pub person: String,
    pub id: Option<String>,
    pub cost: Option<i32>,
    pub burgers: Option<Vec<Burger>>,
    pub drinks: Option<Vec<Drink>>,
    pub sides: Option<Vec<Side>>,
}

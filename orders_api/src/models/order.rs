use async_graphql::{InputObject, SimpleObject};
use serde::Serialize;
use crate::models::{Burger, Drink, Side, Meal};

#[derive(SimpleObject, Clone, Eq, PartialEq, Serialize, Debug)]
pub struct Order {
    pub person: String,
    pub id: Option<String>,
    pub cost: u32,
    pub burgers: Vec<Burger>,
    pub drinks: Vec<Drink>,
    pub fries: Vec<Side>
}

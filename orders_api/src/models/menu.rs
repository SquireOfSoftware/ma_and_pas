use crate::models::Burger;
use async_graphql::{Enum, SimpleObject};
use derive_more::Display;
use serde::Serialize;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Display)]
pub enum MenuItem {
    Burger,
    Fries,
    Drink,
}

#[derive(SimpleObject, Clone)]
pub struct Menu {
    hello: String,
    burgers: Vec<Burger>,
}

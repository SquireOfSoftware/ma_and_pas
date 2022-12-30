use crate::models::Meal;
use crate::models::Burger;
use crate::models::burger::BurgerType;
use crate::models::Drink;
use crate::models::DrinkType;
use crate::models::Fries;
use crate::models::Size;
use async_graphql::{Enum, SimpleObject};
use derive_more::Display;
use serde::Serialize;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Display)]
pub enum MenuItem {
    Burger,
    Fries,
    Drink
}

#[derive(SimpleObject, Clone)]
pub struct Menu {
    hello: String,
    burgers: Vec<Burger>,
}

impl Menu {
    pub async fn new() -> Self {
        Self {
            hello: "hello".to_string(),
            burgers: []
                .to_vec(),
        }
    }
}
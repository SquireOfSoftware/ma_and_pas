use crate::models::Burger;
use crate::models::Drink;
use crate::models::Fries;
use async_graphql::SimpleObject;

#[derive(SimpleObject, Clone)]
pub struct Meal {
    pub id: Option<String>,
    pub name: String,
    pub burger: Burger,
    pub fries: Fries,
    pub drink: Drink,
    pub cost: i32,
}

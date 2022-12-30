use async_graphql::SimpleObject;
use crate::models::Burger;
use crate::models::Fries;
use crate::models::Drink;

#[derive(SimpleObject, Clone)]
pub struct Meal {
    pub id: Option<String>,
    pub name: String,
    pub burger: Burger,
    pub fries: Fries,
    pub drink: Drink,
    pub cost: i32,
}
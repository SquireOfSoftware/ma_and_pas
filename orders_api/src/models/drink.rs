use crate::models::Size;
use async_graphql::Enum;
use async_graphql::SimpleObject;

#[derive(Enum, PartialEq, Eq, Clone, Copy)]
pub enum DrinkType {
    Water,
    Coke,
    Sprite,
    OrangeJuice,
}

#[derive(SimpleObject, Clone)]
pub struct Drink {
    pub id: Option<String>,
    pub size: Size,
    pub drink_type: DrinkType,
    pub cost: i32,
}
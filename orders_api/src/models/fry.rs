use crate::models::size::Size;
use async_graphql::SimpleObject;

#[derive(SimpleObject, Clone)]
pub struct Fries {
    pub id: Option<String>,
    pub size: Size,
    pub cost: i32,
}
use async_graphql::{InputObject, SimpleObject};
use serde::Serialize;

#[derive(InputObject, SimpleObject, Clone, Eq, PartialEq, Serialize, Debug)]
pub struct Order {
    person: String,
    id: Option<String>,

}
use async_graphql::{Enum, SimpleObject};
use tokio_postgres::Row;
use serde::Serialize;
use std::str::FromStr;
use derive_more::Display;
use crate::models::burger::BurgerType::{Beef, Cheese, Chicken, Fish, Ham, Unknown};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Display)]
pub enum BurgerType {
    Cheese,
    Ham,
    Beef,
    Chicken,
    Fish,
    Unknown,
}

impl FromStr for BurgerType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "cheese" => Ok(Cheese),
            "ham" => Ok(Ham),
            "beef" => Ok(Beef),
            "chicken" => Ok(Chicken),
            "fish" => Ok(Fish),
            _ => Ok(Unknown),
        }
    }
}

#[derive(SimpleObject, Clone, Eq, PartialEq, Serialize, Debug)]
pub struct Burger {
    pub id: Option<String>,
    pub name: String,
    pub active: bool,
    pub cost: i32,
}

impl From<Row> for Burger {
    fn from(row: Row) -> Self {
        Self {
            id: Some(row.get::<&str, &str>("code_name").to_string()),
            name: row.get::<&str, &str>("name").to_string(),
            active: row.get::<&str, bool>("active"),
            cost: row.get::<&str, i32>("cost")
        }
    }
}
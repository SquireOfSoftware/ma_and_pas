use crate::models::Size::{Large, Medium, Small, Unknown};
use async_graphql::Enum;

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
pub enum Size {
    Small,
    Medium,
    Large,
    Unknown,
}

impl From<&str> for Size {
    fn from(size: &str) -> Self {
        match size {
            "small" => Small,
            "medium" => Medium,
            "large" => Large,
            _ => Unknown,
        }
    }
}

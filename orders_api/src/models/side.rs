use crate::models::size::Size;
use async_graphql::SimpleObject;
use serde::Serialize;
use tokio_postgres::Row;

#[derive(SimpleObject, Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Side {
    pub id: Option<String>,
    pub name: String,
    pub size: Size,
    pub cost: i32,
    pub side_type: String,
}

impl From<Row> for Side {
    fn from(row: Row) -> Self {
        Self {
            id: Some(row.get::<&str, &str>("code_name").to_string()),
            name: row.get::<&str, &str>("name").to_string(),
            size: Size::from(row.get::<&str, &str>("size")),
            side_type: row.get::<&str, &str>("type").to_string(),
            cost: row.get::<&str, i32>("cost"),
        }
    }
}

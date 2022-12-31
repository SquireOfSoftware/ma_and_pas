use tokio_postgres::Row;
use async_graphql::SimpleObject;
use chrono::{DateTime, Local, Utc};
use serde::Serialize;
use tokio_postgres::types::Timestamp;
use uuid::Uuid;

#[derive(SimpleObject, Clone, Eq, PartialEq, Debug, Serialize)]
pub struct Person {
    pub id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub created_date: String
}

impl From<Row> for Person {
    fn from(row: Row) -> Self {
        Self {
            id: Some(row.get::<&str, Uuid>("id").to_string()),
            first_name: row.get::<&str, &str>("first_name").to_string(),
            last_name: row.get::<&str, &str>("last_name").to_string(),
            created_date: row.get::<&str, DateTime<Local>>("created_date").to_string()
        }
    }
}
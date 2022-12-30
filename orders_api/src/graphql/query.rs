use crate::models::{Burger, Menu, CustomError};
use async_graphql::{Context, FieldResult, Object};
use deadpool_postgres::Pool;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> FieldResult<&str> {
        Ok("hello")
    }
    async fn menu(&self) -> FieldResult<Menu> {
        Ok(Menu::new().await)
    }
    async fn get_burgers(&self, ctx: &Context<'_>) -> FieldResult<Vec<Burger>> {
        let db = &ctx
            .data_unchecked::<Pool>()
            .get()
            .await
            .map_err(CustomError::PoolError)?;

        let result = db.query("select * from burgers", &[])
            .await
            .unwrap();

        Ok(result.into_iter().map(|row| Burger::from(row)).collect::<Vec<Burger>>().to_vec())
    }
}
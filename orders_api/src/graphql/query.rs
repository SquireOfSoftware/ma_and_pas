use crate::models::{Burger, CustomError, Drink};
use async_graphql::{Context, FieldResult, Object};
use deadpool_postgres::Pool;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> FieldResult<&str> {
        Ok("hello")
    }
    async fn get_burgers(&self, ctx: &Context<'_>) -> FieldResult<Vec<Burger>> {
        let db = &ctx
            .data_unchecked::<Pool>()
            .get()
            .await
            .map_err(CustomError::PoolError)?;

        let result = db.query("select * from burgers where active = true", &[])
            .await
            .unwrap();

        Ok(result.into_iter().map(|row| Burger::from(row)).collect::<Vec<Burger>>().to_vec())
    }

    async fn get_drinks(&self, ctx: &Context<'_>) -> FieldResult<Vec<Drink>> {
        let db = &ctx
            .data_unchecked::<Pool>()
            .get()
            .await
            .map_err(CustomError::PoolError)?;

        let result = db.query("select * from sides where active = true and type ='drink'", &[])
            .await
            .unwrap();

        Ok(result.into_iter().map(|row| Drink::from(row)).collect::<Vec<Drink>>().to_vec())
    }
}
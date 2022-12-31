use actix_web::{Error, ResponseError};
use crate::models::{Burger, CustomError, Drink, Person, Side};
use async_graphql::{Context, FieldResult, Object};
use deadpool_postgres::{Object, Pool};
use log::warn;
use tokio_postgres::types::IsNull::No;
use tokio_postgres::types::ToSql;
use crate::models::CustomError::NotFound;
use crate::models::MenuItem::Fries;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> FieldResult<&str> {
        Ok("hello")
    }
    async fn burgers(&self, ctx: &Context<'_>, ids: Option<Vec<String>>) -> FieldResult<Vec<Burger>> {
        let db = &ctx
            .data_unchecked::<Pool>()
            .get()
            .await
            .map_err(CustomError::PoolError)?;

        match ids {
            None => {
                let result = db
                    .query("select * from burgers where active = true", &[])
                    .await
                    .unwrap();

                Ok(result
                    .into_iter()
                    .map(|row| Burger::from(row))
                    .collect::<Vec<Burger>>()
                    .to_vec())
            }
            Some(ids) => {
                Ok(get_burgers_from(ids, db).await?)
            }
        }
    }

    async fn drinks(&self, ctx: &Context<'_>, ids: Option<Vec<String>>) -> FieldResult<Vec<Drink>> {
        let db = &ctx
            .data_unchecked::<Pool>()
            .get()
            .await
            .map_err(CustomError::PoolError)?;

        match ids {
            None => {
                let result = db
                    .query(
                        "select * from sides where active = true and type ='drink'",
                        &[],
                    )
                    .await
                    .unwrap();

                Ok(result
                    .into_iter()
                    .map(|row| Drink::from(row))
                    .collect::<Vec<Drink>>()
                    .to_vec())
            }
            Some(ids) => Ok(get_drinks_from(ids, db).await?)
        }
    }

    async fn sides(&self, ctx: &Context<'_>, ids: Option<Vec<String>>) -> FieldResult<Vec<Side>> {
        let db = &ctx
            .data_unchecked::<Pool>()
            .get()
            .await
            .map_err(CustomError::PoolError)?;

        match ids {
            None => {
                let result = db
                    .query(
                        "select * from sides where active = true and type != 'drink'",
                        &[],
                    )
                    .await
                    .unwrap();

                Ok(result
                    .into_iter()
                    .map(|row| Side::from(row))
                    .collect::<Vec<Side>>()
                    .to_vec()
                )
            }
            Some(ids) => Ok(get_sides_from(ids, db).await?)
        }
    }

    async fn people(&self, ctx: &Context<'_>, ids: Option<Vec<String>>) -> FieldResult<Vec<Person>> {
        let db = &ctx
            .data_unchecked::<Pool>()
            .get()
            .await
            .map_err(CustomError::PoolError)?;

        match ids {
            None => {
                let result = db
                    .query(
                        "select * from people",
                        &[],
                    )
                    .await
                    .unwrap();

                Ok(result
                    .into_iter()
                    .map(|row| Person::from(row))
                    .collect::<Vec<Person>>()
                    .to_vec())
            }
            Some(ids) => Ok(get_people_from(ids, db).await?)
        }
    }
}

pub async fn get_burgers_from(ids: Vec<String>, db: &Object) -> Result<Vec<Burger>, CustomError> {
    let result = db
        .query("select * from burgers where active = TRUE and code_name = any ($1)",
               &[&ids])
        .await
        .unwrap();

    match (ids.len() > 0, result.len() != ids.len()) {
        (true, false) => Ok(result
            .into_iter()
            .map(|row| Burger::from(row))
            .collect::<Vec<Burger>>()
            .to_vec()),
        (true, true) => Err(NotFound),
        _ => Err(NotFound)
    }
}

pub async fn get_drinks_from(ids: Vec<String>, db: &Object) -> Result<Vec<Drink>, CustomError> {
    let result = db
        .query("select * from sides where active = TRUE and type = 'drink' and code_name = any ($1)",
               &[&ids])
        .await
        .unwrap();

    match (ids.len() > 0, result.len() != ids.len()) {
        (true, false) => Ok(result
            .into_iter()
            .map(|row| Drink::from(row))
            .collect::<Vec<Drink>>()
            .to_vec()),
        (true, true) => Err(NotFound),
        _ => Err(NotFound)
    }
}

pub async fn get_sides_from(ids: Vec<String>, db: &Object) -> Result<Vec<Side>, CustomError> {
    let result = db
        .query("select * from sides where active = TRUE and type != 'drink' and code_name = any ($1)",
               &[&ids])
        .await
        .unwrap();

    match (ids.len() > 0, result.len() != ids.len()) {
        (true, false) => Ok(result
            .into_iter()
            .map(|row| Side::from(row))
            .collect::<Vec<Side>>()
            .to_vec()),
        (true, true) => Err(NotFound),
        _ => Err(NotFound)
    }
}

pub async fn get_people_from(ids: Vec<String>, db: &Object) -> Result<Vec<Person>, CustomError> {
    let result = db.query("select * from people where id = any ($1)", &[&ids])
        .await
        .unwrap();

    match (ids.len() > 0, result.len() != ids.len()) {
        (true, false) => Ok(result
            .into_iter()
            .map(|row| Person::from(row))
            .collect::<Vec<Person>>()
            .to_vec()),
        (true, true) => Err(NotFound),
        _ => Err(NotFound)
    }
}
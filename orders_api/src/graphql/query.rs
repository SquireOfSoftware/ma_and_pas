use crate::models::CustomError::NotFound;
use crate::models::{Burger, CustomError, Drink, Person, Side};
use async_graphql::{Context, FieldResult, Object};
use deadpool_postgres::{Object, Pool};
use std::str::FromStr;
use uuid::Uuid;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> FieldResult<&str> {
        Ok("hello")
    }
    async fn burgers(
        &self,
        ctx: &Context<'_>,
        ids: Option<Vec<String>>,
    ) -> FieldResult<Vec<Burger>> {
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
            Some(ids) => Ok(get_burgers_from(ids, db).await?),
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
            Some(ids) => Ok(get_drinks_from(ids, db).await?),
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
                    .to_vec())
            }
            Some(ids) => Ok(get_sides_from(ids, db).await?),
        }
    }

    async fn people(
        &self,
        ctx: &Context<'_>,
        ids: Option<Vec<String>>,
    ) -> FieldResult<Vec<Person>> {
        let db = &ctx
            .data_unchecked::<Pool>()
            .get()
            .await
            .map_err(CustomError::PoolError)?;

        match ids {
            None => {
                let result = db.query("select * from people", &[]).await.unwrap();

                Ok(result
                    .into_iter()
                    .map(|row| Person::from(row))
                    .collect::<Vec<Person>>()
                    .to_vec())
            }
            Some(ids) => Ok(get_people_from(ids, db).await?),
        }
    }
}

pub async fn get_burgers_from(ids: Vec<String>, db: &Object) -> Result<Vec<Burger>, CustomError> {
    let result = db
        .query(
            "select * from burgers where active = TRUE and code_name = any ($1)",
            &[&ids],
        )
        .await
        .unwrap();

    match (ids.len() > 0, result.len() != ids.len()) {
        (true, false) => Ok(result
            .into_iter()
            .map(|row| Burger::from(row))
            .collect::<Vec<Burger>>()
            .to_vec()),
        (true, true) => Err(NotFound),
        (false, false) => Ok(vec![]),
        _ => Err(NotFound),
    }
}

pub async fn get_drinks_from(ids: Vec<String>, db: &Object) -> Result<Vec<Drink>, CustomError> {
    let result = db
        .query(
            "select * from sides where active = TRUE and type = 'drink' and code_name = any ($1)",
            &[&ids],
        )
        .await
        .unwrap();

    dbg!(
        ids.len() > 0,
        result.len() != ids.len(),
        result.len(),
        ids.len()
    );

    match (ids.len() > 0, result.len() != ids.len()) {
        (true, false) => Ok(result
            .into_iter()
            .map(|row| Drink::from(row))
            .collect::<Vec<Drink>>()
            .to_vec()),
        (true, true) => Err(NotFound),
        (false, false) => Ok(vec![]),
        _ => Err(NotFound),
    }
}

pub async fn get_sides_from(ids: Vec<String>, db: &Object) -> Result<Vec<Side>, CustomError> {
    let result = db
        .query(
            "select * from sides where active = TRUE and type != 'drink' and code_name = any ($1)",
            &[&ids],
        )
        .await
        .unwrap();

    match (ids.len() > 0, result.len() != ids.len()) {
        (true, false) => Ok(result
            .into_iter()
            .map(|row| Side::from(row))
            .collect::<Vec<Side>>()
            .to_vec()),
        (true, true) => Err(NotFound),
        (false, false) => Ok(vec![]),
        _ => Err(NotFound),
    }
}

pub async fn get_people_from(ids: Vec<String>, db: &Object) -> Result<Vec<Person>, CustomError> {
    let uuid_list: Vec<Uuid> = ids.iter().map(|id| Uuid::from_str(id).unwrap()).collect();

    let raw_result = db
        .query(
            "select * from people where id = any ($1)",
            &[&&uuid_list[..]],
        )
        .await;

    let result = raw_result.unwrap();

    match (ids.len() > 0, result.len() != ids.len()) {
        (true, false) => Ok(result
            .into_iter()
            .map(|row| Person::from(row))
            .collect::<Vec<Person>>()
            .to_vec()),
        (true, true) => Err(NotFound),
        (false, false) => Ok(vec![]),
        _ => Err(NotFound),
    }
}

pub async fn get_person_from(id: String, db: &Object) -> Result<Person, CustomError> {
    let raw_result = db
        .query_one(
            "select * from people where id = ($1)",
            &[&Uuid::from_str(id.as_str()).unwrap()],
        )
        .await;

    match raw_result {
        Ok(row) => Ok(Person::from(row)),
        Err(_) => Err(NotFound),
    }
}

use async_graphql::{Context, FieldResult, Object};
use deadpool_postgres::Pool;
use crate::models::{Burger, BurgerType, CustomError};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn add_burger(&self, ctx: &Context<'_>, burger_type: BurgerType, cost: i32) -> FieldResult<Burger> {
        let db = &ctx
            .data_unchecked::<Pool>()
            .get()
            .await
            .map_err(CustomError::PoolError)?;

        let result = db
            .execute(
                "INSERT INTO burgers (burger_type, cost) VALUES ($1, $2)",
                &[&burger_type.to_string().to_lowercase(), &cost],
            )
            .await
            .unwrap();

        Ok(Burger {
            id: Some(result.to_string()),
            burger_type,
            cost,
        })
    }
}

// impl InputType for Mutation {
//     type RawValueType = ();
//
//     fn type_name() -> Cow<'static, str> {
//         todo!()
//     }
//
//     fn create_type_info(registry: &mut Registry) -> String {
//         todo!()
//     }
//
//     fn parse(value: Option<async_graphql_value::ConstValue>) -> InputValueResult<Self> {
//         todo!()
//     }
//
//     fn to_value(&self) -> async_graphql_value::ConstValue {
//         todo!()
//     }
//
//     fn as_raw_value(&self) -> Option<&Self::RawValueType> {
//         todo!()
//     }
// }
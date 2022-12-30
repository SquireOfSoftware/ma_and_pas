use async_graphql::{EmptySubscription, Schema};
use crate::graphql::{MutationRoot, QueryRoot};

pub type ShopSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

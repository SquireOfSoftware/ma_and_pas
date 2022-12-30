use crate::graphql::{MutationRoot, QueryRoot};
use async_graphql::{EmptySubscription, Schema};

pub type ShopSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

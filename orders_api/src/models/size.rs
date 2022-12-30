use async_graphql::Enum;

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
pub enum Size {
    Small,
    Medium,
    Large,
}
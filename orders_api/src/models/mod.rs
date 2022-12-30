mod burger;
mod drink;
mod error;
mod fry;
mod meal;
mod menu;
mod size;

pub use burger::Burger;
pub use drink::Drink;
pub use drink::DrinkType;
pub use error::CustomError;
pub use fry::Fries;
pub use meal::Meal;
pub use menu::Menu;
pub use size::Size;

mod query;
mod mutation;
pub use query::QueryRoot;
pub use mutation::MutationRoot;
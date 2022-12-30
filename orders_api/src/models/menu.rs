use crate::models::Meal;
use crate::models::Burger;
use crate::models::burger::BurgerType;
use crate::models::Drink;
use crate::models::DrinkType;
use crate::models::Fries;
use crate::models::Size;
use async_graphql::SimpleObject;

#[derive(SimpleObject, Clone)]
pub struct Menu {
    hello: String,
    meals: Vec<Meal>,
    burgers: Vec<Burger>,
}

impl Menu {
    pub async fn new() -> Self {
        Self {
            hello: "hello".to_string(),
            meals: [Meal {
                id: Some("123".to_string()),
                name: "Standard meal".to_string(),
                cost: 1200,
                burger: Burger {
                    id: Some("124".to_string()),
                    burger_type: BurgerType::Cheese,
                    cost: 500,
                },
                fries: Fries {
                    id: Some("125".to_string()),
                    size: Size::Large,
                    cost: 300,
                },
                drink: Drink {
                    id: Some("126".to_string()),
                    size: Size::Large,
                    drink_type: DrinkType::Water,
                    cost: 200,
                },
            }]
                .to_vec(),
            burgers: [Burger {
                id: Some("124".to_string()),
                burger_type: BurgerType::Cheese,
                cost: 500,
            }]
                .to_vec(),
        }
    }
}
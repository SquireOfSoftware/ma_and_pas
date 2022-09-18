package com.squireofsoftware.orders.meals

import com.squireofsoftware.orders.burgers.Burger
import com.squireofsoftware.orders.drinks.Drink
import com.squireofsoftware.orders.drinks.Drinks
import com.squireofsoftware.orders.fries.Fries
import com.squireofsoftware.orders.menu.Meal
import com.squireofsoftware.orders.sizes.Sizes

class SmallMacMeal: Meal(name = "The Smol Mac Meal") {
    val burger = Burger(name = "The Small Mac")
    val fries = Fries(size = Sizes.Medium)
    val drink = Drink(size = Sizes.Medium, flavour = Drinks.Coke)
}

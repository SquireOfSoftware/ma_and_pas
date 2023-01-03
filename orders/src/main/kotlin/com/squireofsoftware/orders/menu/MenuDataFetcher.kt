package com.squireofsoftware.orders.menu

import com.netflix.graphql.dgs.DgsComponent
import com.netflix.graphql.dgs.DgsQuery
import com.squireofsoftware.orders.burgers.Burger
import com.squireofsoftware.orders.drinks.Drink
import com.squireofsoftware.orders.drinks.Drinks
import com.squireofsoftware.orders.fish.Fish
import com.squireofsoftware.orders.fish.FishType
import com.squireofsoftware.orders.fries.Fries
import com.squireofsoftware.orders.meals.FishAndChips
import com.squireofsoftware.orders.meals.SmallMacMeal
import com.squireofsoftware.orders.sizes.Sizes

@DgsComponent
class MenuDataFetcher {
    private val burger = Burger("The Small Mac")
    private val smallMacMeal = SmallMacMeal()
    private val smallFries = Fries(size = Sizes.Small)
    private val smallCoke = Drink(size = Sizes.Small, flavour = Drinks.Coke)
    private val fish = Fish(FishType.Grilled)
    private val fishAndChips = FishAndChips(FishType.Battered, Sizes.Medium)

    @DgsQuery
    fun menu(): List<MenuItem> {
        return listOf(burger, smallMacMeal, smallFries, smallCoke, fish, fishAndChips)
    }
}
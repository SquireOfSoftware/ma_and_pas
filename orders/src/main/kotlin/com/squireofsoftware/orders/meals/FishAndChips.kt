package com.squireofsoftware.orders.meals

import com.squireofsoftware.orders.fish.Fish
import com.squireofsoftware.orders.fish.FishType
import com.squireofsoftware.orders.fries.Fries
import com.squireofsoftware.orders.menu.Meal
import com.squireofsoftware.orders.sizes.Sizes

class FishAndChips(fishType: FishType, size: Sizes): Meal(name = "Fush and Chups") {
    val fish = Fish(fishType)
    val chips = Fries(size)
}
package com.squireofsoftware.orders.drinks

import com.squireofsoftware.orders.menu.ItemType
import com.squireofsoftware.orders.menu.MenuItem
import com.squireofsoftware.orders.sizes.Sizes
import kotlin.random.Random
import kotlin.random.nextInt

class Drink(val size: Sizes, val flavour: Drinks): MenuItem(type = ItemType.drink, name = flavour.name) {
    val capacity = when (size) {
        Sizes.Small -> Random.nextInt(IntRange(100, 120))
        Sizes.Medium -> Random.nextInt(IntRange(200, 300))
        Sizes.Large -> Random.nextInt(IntRange(400, 450))
    }
}
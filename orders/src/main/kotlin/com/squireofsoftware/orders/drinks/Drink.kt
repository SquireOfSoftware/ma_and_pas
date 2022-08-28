package com.squireofsoftware.orders.drinks

import com.squireofsoftware.orders.menu.MenuItem
import com.squireofsoftware.orders.sizes.Sizes
import java.util.*

import kotlin.random.Random
import kotlin.random.nextInt

class Drink(val size: Sizes, val type: Drinks): MenuItem {
    val id = UUID.randomUUID()
    val name = type.name
    val capacity = when (size) {
        Sizes.Small -> Random.nextInt(IntRange(100, 120))
        Sizes.Medium -> Random.nextInt(IntRange(200, 300))
        Sizes.Large -> Random.nextInt(IntRange(400, 450))
    }
}
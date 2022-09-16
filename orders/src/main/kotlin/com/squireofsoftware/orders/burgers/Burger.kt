package com.squireofsoftware.orders.burgers

import com.squireofsoftware.orders.menu.ItemType
import com.squireofsoftware.orders.menu.MenuItem
import java.util.UUID

data class Burger(val name: String): MenuItem(type = ItemType.burger) {
    val id = UUID.randomUUID()
}

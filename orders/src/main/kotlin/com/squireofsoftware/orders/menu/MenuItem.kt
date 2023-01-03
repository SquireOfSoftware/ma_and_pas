package com.squireofsoftware.orders.menu

import java.util.*

open class MenuItem(val type: ItemType, val name: String) {
    val id = UUID.randomUUID()
}
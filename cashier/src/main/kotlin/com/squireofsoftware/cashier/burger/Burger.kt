package com.squireofsoftware.cashier.burger

import com.squireofsoftware.cashier.item.Item
import java.util.*

data class Burger(val id: UUID, val name: String, val displayName: String, val price: Int) {
    companion object {
        fun toBurger(item: Item): Burger {
            return Burger(
                id = item.id,
                name = item.name,
                displayName = item.displayName,
                price = item.price
            )
        }
    }
}
package com.squireofsoftware.orders.fries

import com.squireofsoftware.orders.menu.ItemType
import com.squireofsoftware.orders.menu.MenuItem
import com.squireofsoftware.orders.sizes.SizeRange
import com.squireofsoftware.orders.sizes.Sizes

class Fries(val size: Sizes) : MenuItem(type = ItemType.fries, name = "Fries") {
    val fryRange = when (size) {
        Sizes.Small -> SizeRange(15, 25)
        Sizes.Medium -> SizeRange(30, 40)
        Sizes.Large -> SizeRange(45, 55)
    }
}

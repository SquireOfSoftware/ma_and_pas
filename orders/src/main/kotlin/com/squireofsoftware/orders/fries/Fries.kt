package com.squireofsoftware.orders.fries

import com.squireofsoftware.orders.menu.MenuItem
import com.squireofsoftware.orders.sizes.SizeRange
import com.squireofsoftware.orders.sizes.Sizes
import java.util.*

class Fries(val size: Sizes) : MenuItem {
    val id = UUID.randomUUID()
    val name = "Fries"
    val fryRange = when (size) {
        Sizes.Small -> SizeRange(15, 25)
        Sizes.Medium -> SizeRange(30, 40)
        Sizes.Large -> SizeRange(45, 55)
    }
}

package com.squireofsoftware.orders.burgers

import com.squireofsoftware.orders.menu.ItemType
import com.squireofsoftware.orders.menu.MenuItem

class Burger(name: String): MenuItem(type = ItemType.burger, name = name)

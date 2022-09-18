package com.squireofsoftware.orders.fish

import com.squireofsoftware.orders.menu.ItemType
import com.squireofsoftware.orders.menu.MenuItem

class Fish(val prepType: FishType): MenuItem(type = ItemType.fish, name = "Fish")
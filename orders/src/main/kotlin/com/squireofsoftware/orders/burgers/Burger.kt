package com.squireofsoftware.orders.burgers

import com.squireofsoftware.orders.menu.MenuItem
import java.util.UUID

data class Burger(val id: UUID, val name: String): MenuItem

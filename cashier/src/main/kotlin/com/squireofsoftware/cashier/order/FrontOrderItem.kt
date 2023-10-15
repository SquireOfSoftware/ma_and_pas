package com.squireofsoftware.cashier.order

import com.squireofsoftware.cashier.item.DishType
import java.util.UUID

data class FrontOrderItem(
    val id: UUID,
    val dishType: DishType,
    val name: String,
    val createdAt: Long,
    val lastUpdated: Long,
    val state: State,
)

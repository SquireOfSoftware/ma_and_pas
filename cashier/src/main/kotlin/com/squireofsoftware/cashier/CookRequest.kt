package com.squireofsoftware.cashier

import com.squireofsoftware.cashier.item.DishType
import kotlinx.serialization.Serializable
import java.util.*

@Serializable
class CookRequest(
    @Serializable(with = UUIDSerializer::class)
    val subOrderId: UUID,
    val dishType: DishType,
    val dishName: String,
    @Serializable(with = UUIDSerializer::class)
    val orderId: UUID
)
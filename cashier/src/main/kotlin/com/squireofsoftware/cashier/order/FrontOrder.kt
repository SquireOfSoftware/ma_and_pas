package com.squireofsoftware.cashier.order

import com.squireofsoftware.cashier.UUIDSerializer
import kotlinx.serialization.Serializable
import java.util.*

@Serializable
data class FrontOrder(
    @Serializable(with = UUIDSerializer::class)
    val id: UUID,
    val createdAt: Long,
    val lastUpdated: Long,
    val state: State,
    val price: Int,
)
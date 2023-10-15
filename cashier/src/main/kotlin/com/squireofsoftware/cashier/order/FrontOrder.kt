package com.squireofsoftware.cashier.order

import java.util.*

data class FrontOrder(
    val id: UUID,
    val createdAt: Long,
    val lastUpdated: Long,
    val state: State,
    val price: Int,
)
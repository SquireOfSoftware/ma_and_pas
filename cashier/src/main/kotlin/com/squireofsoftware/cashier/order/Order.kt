package com.squireofsoftware.cashier.order

import java.util.UUID

data class Order(
    val id: UUID,
    val ids: List<UUID>
)

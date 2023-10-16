package com.squireofsoftware

import java.util.*

class CookRequest(
    val subOrderId: UUID,
    val dishType: String,
    val dishName: String,
    val orderId: UUID
)
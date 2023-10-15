package com.squireofsoftware.cashier.order

import java.util.UUID

data class Request(
    val id: String = UUID.randomUUID().toString(),
    val items: List<String>
)
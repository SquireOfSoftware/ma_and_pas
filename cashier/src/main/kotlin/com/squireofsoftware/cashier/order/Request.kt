package com.squireofsoftware.cashier.order

import java.util.*

data class Request(
    val id: String = UUID.randomUUID().toString(),
    val items: List<String>
)
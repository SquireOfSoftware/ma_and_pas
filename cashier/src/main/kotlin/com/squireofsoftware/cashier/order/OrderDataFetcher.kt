package com.squireofsoftware.cashier.order

import com.netflix.graphql.dgs.DgsComponent
import com.netflix.graphql.dgs.DgsQuery
import java.util.*

@DgsComponent
class OrderDataFetcher {
    @DgsQuery
    fun activeOrders(): List<Order> {
        return listOf(
            Order(UUID.randomUUID(), emptyList())
        )
    }
}
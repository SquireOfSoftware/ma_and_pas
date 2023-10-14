package com.squireofsoftware.cashier.order

import com.netflix.graphql.dgs.DgsComponent
import com.netflix.graphql.dgs.DgsMutation
import graphql.schema.DataFetchingEnvironment
import java.util.UUID

@DgsComponent
class OrderMutation {
    @DgsMutation
    fun createOrder(dataFetchingEnvironment: DataFetchingEnvironment): Order {
        return Order(UUID.randomUUID(), emptyList())
    }
}
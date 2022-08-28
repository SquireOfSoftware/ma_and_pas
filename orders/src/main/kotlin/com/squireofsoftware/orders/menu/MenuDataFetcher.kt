package com.squireofsoftware.orders.menu

import com.netflix.graphql.dgs.DgsComponent
import com.netflix.graphql.dgs.DgsQuery
import com.squireofsoftware.orders.burgers.Burger
import java.util.*

@DgsComponent
class MenuDataFetcher {
    private val burger = Burger(UUID.randomUUID(), "The Small Mac");

    @DgsQuery
    fun menu(): List<MenuItem> {
        return listOf(burger)
    }
}
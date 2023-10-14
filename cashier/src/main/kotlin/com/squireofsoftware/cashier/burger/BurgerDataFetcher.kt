package com.squireofsoftware.cashier.burger

import com.netflix.graphql.dgs.DgsComponent
import com.netflix.graphql.dgs.DgsQuery
import java.util.UUID

@DgsComponent
class BurgerDataFetcher {
    private val burgers = listOf(
        Burger(UUID.randomUUID(), "cheese"),
        Burger(UUID.randomUUID(), "chicken"))

    @DgsQuery
    fun burgers(): List<Burger> {
        return burgers
    }
}
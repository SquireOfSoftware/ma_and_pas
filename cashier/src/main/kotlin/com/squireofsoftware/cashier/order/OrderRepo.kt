package com.squireofsoftware.cashier.order

import org.springframework.data.repository.CrudRepository
import java.util.UUID

interface OrderRepo: CrudRepository<Order, UUID> {
    fun findAllByStateIsNotInOrderByLastUpdatedDesc(states: Set<State>): List<Order>
}
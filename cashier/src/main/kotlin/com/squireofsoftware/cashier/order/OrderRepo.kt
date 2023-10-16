package com.squireofsoftware.cashier.order

import org.springframework.data.repository.CrudRepository
import java.util.*

interface OrderRepo: CrudRepository<Order, UUID> {
    fun findAllByStateIsNotInOrderByLastUpdatedDesc(states: Set<State>): List<Order>
    fun findAllByStateInOrderByLastUpdatedDesc(states: Set<State>): List<Order>
}
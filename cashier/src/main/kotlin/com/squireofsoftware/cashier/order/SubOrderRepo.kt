package com.squireofsoftware.cashier.order

import org.springframework.data.repository.CrudRepository
import java.util.*

interface SubOrderRepo: CrudRepository<SubOrder, UUID> {
    fun findAllByOrderId(orderId: UUID): List<SubOrder>
}
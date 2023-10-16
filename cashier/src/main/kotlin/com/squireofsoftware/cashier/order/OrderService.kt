package com.squireofsoftware.cashier.order

import com.squireofsoftware.cashier.acknowledgements.OrderNotFoundException
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Service
import org.springframework.transaction.annotation.Transactional
import org.springframework.web.bind.annotation.PathVariable
import java.util.*

@Service
class OrderService(
    @Autowired
    val orderRepo: OrderRepo
) {
    @Transactional
    fun completeOrder(@PathVariable orderId: UUID) {
        val order = orderRepo.findById(orderId)
        if (order.isEmpty) {
            throw OrderNotFoundException(orderId)
        }
        val actualOrder = order.get()
        actualOrder.state = State.completed
        orderRepo.save(actualOrder)
    }
}
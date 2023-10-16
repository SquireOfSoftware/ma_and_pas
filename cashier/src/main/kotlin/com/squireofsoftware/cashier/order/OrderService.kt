package com.squireofsoftware.cashier.order

import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Service
import org.springframework.transaction.annotation.Transactional
import org.springframework.web.bind.annotation.PathVariable
import java.util.*

@Service
class OrderService(
    @Autowired
    val orderRepo: OrderRepo,
    @Autowired
    val subOrderRepo: SubOrderRepo
) {
    companion object {
        val finishStates = setOf(State.completed, State.failed)
    }

    private val LOGGER: Logger = LoggerFactory.getLogger(this::class.java)

    @Transactional
    fun completeSubOrder(@PathVariable subOrderId: UUID): SubOrder {
        val subOrder = subOrderRepo.findById(subOrderId)
        if (subOrder.isEmpty) {
            throw SubOrderNotFoundException(subOrderId)
        }
        val actualOrder = subOrder.get()
        actualOrder.state = State.completed
        LOGGER.info("Completing the suborder $subOrderId")
        return subOrderRepo.save(actualOrder)
    }

    @Transactional
    fun checkIfOrderIsComplete(subOrder: SubOrder) {
        val subOrders = subOrderRepo.findAllByOrderId(subOrder.orderId)
        val subOrderStates = subOrders.map { it.state }
        if (subOrderStates.all {
            it == State.completed
        }) {
            val order = getOrder(subOrder.orderId)
            order.state = State.completed
            orderRepo.save(order)
            LOGGER.info("Order ${order.id} is done")
        }
        else if (subOrderStates.any {
            it == State.failed
        }) {
            val order = getOrder(subOrder.orderId)
            order.state = State.failed
            orderRepo.save(order)
            LOGGER.info("Order ${order.id} is marked as failed")
        } else {
            LOGGER.info("Order ${subOrder.orderId} is still going")
        }
    }

    fun getOrder(orderId: UUID): Order {
        val order = orderRepo.findById(orderId)
        if (order.isEmpty) throw OrderNotFoundException(orderId = orderId)
        return order.get()
    }
}
package com.squireofsoftware.cashier.order

import com.squireofsoftware.cashier.CookRequest
import com.squireofsoftware.cashier.event.KafkaService
import com.squireofsoftware.cashier.item.InvalidItemsException
import com.squireofsoftware.cashier.item.ItemRepo
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.json.Json
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
    val subOrderRepo: SubOrderRepo,
    @Autowired
    val itemRepo: ItemRepo,
    @Autowired
    val kafkaService: KafkaService
) {
    companion object {
        val finishStates = setOf(State.completed, State.failed)
    }

    private val logger: Logger = LoggerFactory.getLogger(this::class.java)

    @Transactional
    fun completeSubOrder(@PathVariable subOrderId: UUID): SubOrder {
        val subOrder = subOrderRepo.findById(subOrderId)
        if (subOrder.isEmpty) {
            throw SubOrderNotFoundException(subOrderId)
        }
        val actualOrder = subOrder.get()
        actualOrder.state = State.completed
        logger.info("Completing the suborder $subOrderId")
        return subOrderRepo.save(actualOrder)
    }

    @Transactional
    fun checkIfOrderIsComplete(subOrder: SubOrder) {
        // check if it is already done
        val order = getOrder(subOrder.orderId)
        if (order.state != State.completed && order.state != State.failed) {
            val subOrders = subOrderRepo.findAllByOrderId(subOrder.orderId)
            val subOrderStates = subOrders.map { it.state }
            if (subOrderStates.all {
                    it == State.completed
                }) {
                order.state = State.completed
                orderRepo.save(order)
                logger.info("Order ${order.id} is done")
            } else if (subOrderStates.any {
                    it == State.failed
                }) {
                order.state = State.failed
                orderRepo.save(order)
                logger.info("Order ${order.id} is marked as failed")
            } else {
                logger.info("Order ${subOrder.orderId} is still going")
                // we could spawn a timer here to check for race conditions
            }
        } else {
            logger.info("Order ${order.id} was already marked as ${order.state}")
        }
    }

    fun getOrder(orderId: UUID): Order {
        val order = orderRepo.findById(orderId)
        if (order.isEmpty) throw OrderNotFoundException(orderId = orderId)
        return order.get()
    }

    fun createOrder(orderId: UUID, subOrderIds: List<UUID>): Order {
        val currentTime = System.currentTimeMillis()
        val requestedItems = itemRepo.findAllByIdIn(subOrderIds)

        if (requestedItems.isEmpty()) {
            throw InvalidItemsException(subOrderIds)
        }

        val totalPrice = requestedItems.sumOf {
            it.price
        }

        val order = orderRepo.save(Order(
            id = orderId,
            createdAt = currentTime,
            lastUpdated = currentTime,
            price = totalPrice
        ))

        val itemMap = requestedItems.associateBy { it.id }

        val subOrders =
            requestedItems.map {
                SubOrder(
                    id = UUID.randomUUID(),
                    itemId = it.id,
                    createdAt = currentTime,
                    lastUpdated = currentTime,
                    state = State.requested,
                    orderId = order.id
                )
            }

        subOrderRepo.saveAll(subOrders)

        subOrders.forEach {
            val cookRequest = it.toCookRequest(itemMap[it.itemId]!!)
            val serialisedRequest = Json.encodeToString(CookRequest.serializer(), cookRequest)
            logger.info(serialisedRequest)
            kafkaService.sendEvent(serialisedRequest)
        }

        return order
    }

    fun findSubOrders(orderId: UUID): List<SubOrder> {
        return subOrderRepo.findAllByOrderId(orderId = orderId)
    }

    fun getActiveOrders(): List<Order> {
        return orderRepo.findAllByStateIsNotInOrderByLastUpdatedDesc(finishStates)
    }

    fun getCompletedOrders(): List<Order> {
        return orderRepo.findAllByStateInOrderByLastUpdatedDesc(finishStates)
    }
}
package com.squireofsoftware.cashier.order

import com.netflix.graphql.dgs.*
import com.squireofsoftware.cashier.event.KafkaService
import com.squireofsoftware.cashier.item.InvalidItemException
import com.squireofsoftware.cashier.item.ItemRepo
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.json.Json
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.transaction.annotation.Transactional
import java.util.*

@DgsComponent
class FrontOrderDataFetcher(
    @Autowired
    val orderRepo: OrderRepo,
    @Autowired
    val itemRepo: ItemRepo,
    @Autowired
    val subOrderRepo: SubOrderRepo,
    @Autowired
    val kafkaService: KafkaService
) {
    private val finishStates = setOf(State.completed, State.failed)
    @DgsData(parentType = "Query", field = "activeOrders")
    fun activeOrders(): List<FrontOrder> {
        return orderRepo.findAllByStateIsNotInOrderByLastUpdatedDesc(finishStates)
            .map { it.toFrontOrder() }
    }

    @DgsData(parentType = "Order", field = "subOrders")
    fun activeOrderItems(dataFetchingEnvironment: DgsDataFetchingEnvironment): List<FrontOrderItem> {
        val order = dataFetchingEnvironment.getSource<FrontOrder>()

        val subOrders = subOrderRepo.findAllByOrderId(orderId = order.id)

        val items = itemRepo.findAllByIdIn(subOrders.map { it.itemId }).associateBy { it.id }

        return subOrders.map {
            FrontOrderItem(
                id = it.id,
                dishType = items[it.itemId]!!.dishType,
                name = items[it.itemId]!!.name,
                createdAt = it.createdAt,
                lastUpdated = it.lastUpdated,
                state = it.state
            )
        }
    }

    @Transactional
    @DgsMutation
    fun createOrder(@InputArgument request: Request): FrontOrder {
        val currentTime = System.currentTimeMillis()
        val requestedItems = itemRepo.findAllByIdIn(request.items.map{ UUID.fromString(it) })

        if (requestedItems.isEmpty()) {
            throw InvalidItemException(request.items)
        }

        val totalPrice = requestedItems.sumOf {
            it.price
        }

        val order = orderRepo.save(Order(
            id = UUID.fromString(request.id),
            createdAt = currentTime,
            lastUpdated = currentTime,
            price = totalPrice
        ))

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

        kafkaService.sendEvent(Json.encodeToString(String.Companion.serializer(), order.toFrontOrder().id.toString()))

        return order.toFrontOrder()
    }
}
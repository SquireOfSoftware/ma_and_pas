package com.squireofsoftware.cashier.order

import com.netflix.graphql.dgs.*
import com.squireofsoftware.cashier.item.ItemRepo
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.transaction.annotation.Transactional
import java.util.*

@DgsComponent
class FrontOrderDataFetcher(
    @Autowired
    val itemRepo: ItemRepo,
    @Autowired
    val orderService: OrderService
) {
    @DgsQuery
    fun activeOrders(): List<FrontOrder> {
        return orderService.getActiveOrders()
            .map { it.toFrontOrder() }
    }

    @DgsQuery
    fun completedOrders(): List<FrontOrder> {
        return orderService.getCompletedOrders()
            .map { it.toFrontOrder() }
    }

    @DgsData(parentType = "Order", field = "subOrders")
    fun activeOrderItems(dataFetchingEnvironment: DgsDataFetchingEnvironment): List<FrontOrderItem> {
        val order = dataFetchingEnvironment.getSource<FrontOrder>()

        val subOrders = orderService.findSubOrders(order.id)

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
        return orderService.createOrder(
            UUID.fromString(request.id),
            request.items.map{ UUID.fromString(it) }
        )
            .toFrontOrder()
    }
}
package com.squireofsoftware.cashier.acknowledgements

import com.squireofsoftware.cashier.order.OrderRepo
import com.squireofsoftware.cashier.order.OrderService
import com.squireofsoftware.cashier.order.State
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.http.HttpStatus
import org.springframework.transaction.annotation.Transactional
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.PutMapping
import org.springframework.web.bind.annotation.ResponseBody
import org.springframework.web.bind.annotation.ResponseStatus
import org.springframework.web.bind.annotation.RestController
import java.util.*
import java.util.logging.Logger

@RestController
class AcknowledgementController(
    @Autowired
    val orderService: OrderService
) {
    val logger = Logger.getLogger(AcknowledgementController::class.simpleName)
    @PutMapping("/orders/{orderId}")
    @ResponseStatus(HttpStatus.OK)
    @Transactional
    fun completeOrder(@PathVariable orderId: UUID) {
        logger.info("Receiving order: $orderId")
        orderService.completeOrder(orderId)
    }
}
package com.squireofsoftware.cashier.acknowledgements

import com.squireofsoftware.cashier.order.OrderService
import jakarta.servlet.http.HttpServletRequest
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.http.HttpStatus
import org.springframework.transaction.annotation.Transactional
import org.springframework.web.bind.annotation.*
import java.util.*


@RestController
class AcknowledgementController(
    @Autowired
    val orderService: OrderService
) {
    val logger: Logger = LoggerFactory.getLogger(AcknowledgementController::class.simpleName)
    @PutMapping("/subOrders/{subOrderId}")
    @ResponseStatus(HttpStatus.OK)
    @Transactional
    fun completeOrder(@PathVariable subOrderId: UUID) {
        logger.info("Receiving Sub Order: $subOrderId")
        val completedSubOrder = orderService.completeSubOrder(subOrderId)
        orderService.checkIfOrderIsComplete(completedSubOrder)
    }

    @GetMapping("/subOrder/{test}/**")
    @ResponseStatus(HttpStatus.OK)
    fun testFun(@PathVariable test: String,
                request: HttpServletRequest,
                @RequestParam("hello") hello: String?): String {
        return "$test ${request.requestURI} $hello"
    }
}
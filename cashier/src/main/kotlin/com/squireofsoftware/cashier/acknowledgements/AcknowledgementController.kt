package com.squireofsoftware.cashier.acknowledgements

import com.squireofsoftware.cashier.order.OrderService
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.http.HttpStatus
import org.springframework.http.client.SimpleClientHttpRequestFactory
import org.springframework.transaction.annotation.Transactional
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.PutMapping
import org.springframework.web.bind.annotation.ResponseStatus
import org.springframework.web.bind.annotation.RestController
import org.springframework.web.client.RestTemplate
import org.springframework.web.client.getForEntity
import java.net.InetSocketAddress
import java.net.Proxy
import java.util.*


@RestController
class AcknowledgementController(
    @Autowired
    val orderService: OrderService,
    val test: Boolean
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

    fun testProxy(host: String, port: Int, url: String) {
        val restTemplate = if (test) {
            val proxy = Proxy(Proxy.Type.HTTP, InetSocketAddress(host, port))
            val requestFactory = SimpleClientHttpRequestFactory()
            requestFactory.setProxy(proxy)
            requestFactory.setReadTimeout(10)
            RestTemplate(requestFactory)
        } else {
            RestTemplate()
        }

        restTemplate.getForEntity<Void>(url);
    }
}
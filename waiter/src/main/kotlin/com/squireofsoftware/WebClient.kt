package com.squireofsoftware

import io.micronaut.core.async.annotation.SingleResult
import io.micronaut.http.annotation.Put
import io.micronaut.http.client.annotation.Client
import jakarta.inject.Singleton
import org.reactivestreams.Publisher
import java.util.*

@Client(id = "acknowledgements")
@Singleton
interface WebClient {
    @Put("/subOrders/{subOrderId}")
    @SingleResult
    fun sendAcknowledgement(subOrderId: UUID): Publisher<Void>
}
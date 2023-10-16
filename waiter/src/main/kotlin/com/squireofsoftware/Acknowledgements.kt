package com.squireofsoftware

import io.micronaut.configuration.kafka.annotation.KafkaKey
import io.micronaut.configuration.kafka.annotation.KafkaListener
import io.micronaut.configuration.kafka.annotation.OffsetReset
import io.micronaut.configuration.kafka.annotation.Topic
import io.micronaut.context.annotation.Value
import io.micronaut.core.io.buffer.ByteBuffer
import io.micronaut.http.HttpRequest
import io.micronaut.http.HttpResponse
import io.micronaut.http.client.HttpClient
import io.micronaut.http.client.annotation.Client
import io.micronaut.http.uri.UriBuilder
import jakarta.inject.Inject
import org.reactivestreams.Publisher
import org.slf4j.Logger
import org.slf4j.LoggerFactory.getLogger
import java.util.*


@KafkaListener(offsetReset = OffsetReset.EARLIEST)
class Acknowledgements(
){

    @field:Client(id = "acknowledgements")
    @Inject
    lateinit var httpClient: HttpClient

    @Topic("acknowledgements")
    fun receive(@KafkaKey key: String?, payload: String?): Publisher<HttpResponse<ByteBuffer<*>>>? {
        LOG.info("Got Order - {} by {}", payload, key)
        val orderId = UUID.fromString(payload!!.replace("\"", ""))
        val uri = UriBuilder.of("/orders/").path(orderId.toString()).build()
        LOG.info("Acknowledging $orderId...")
        return httpClient.exchange(HttpRequest.PUT(uri, String::class))
    }

    companion object {
        private val LOG: Logger = getLogger(Acknowledgements::class.java)
    }
}
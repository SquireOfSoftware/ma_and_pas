package com.squireofsoftware

import io.micronaut.configuration.kafka.annotation.KafkaKey
import io.micronaut.configuration.kafka.annotation.KafkaListener
import io.micronaut.configuration.kafka.annotation.OffsetReset
import io.micronaut.configuration.kafka.annotation.Topic
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.int
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.reactivestreams.Publisher
import org.slf4j.Logger
import org.slf4j.LoggerFactory.getLogger
import java.util.*

@KafkaListener(offsetReset = OffsetReset.EARLIEST)
class Acknowledgements(
    private val webClient: WebClient
){
    @Topic("acknowledgements")
    fun receive(@KafkaKey key: String?, payload: String?): Publisher<Void> {
        LOG.info("Got Sub Order - {} by {}", payload, key)
        val jsonObject = Json.parseToJsonElement(payload!!).jsonObject
        val cookRequest = CookRequest(
            subOrderId = UUID.fromString(jsonObject["subOrderId"]!!.jsonPrimitive.content),
            orderId = UUID.fromString(jsonObject["orderId"]!!.jsonPrimitive.content),
            dishName = jsonObject["dishName"]!!.jsonPrimitive.content,
            dishType = jsonObject["dishType"]!!.toString()
        )

        LOG.info("Acknowledging ${cookRequest.subOrderId}...")
        return webClient.sendAcknowledgement(cookRequest.subOrderId)
    }

    companion object {
        private val LOG: Logger = getLogger(Acknowledgements::class.java)
    }
}
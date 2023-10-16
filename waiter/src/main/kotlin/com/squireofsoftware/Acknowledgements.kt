package com.squireofsoftware

import io.micronaut.configuration.kafka.annotation.KafkaKey
import io.micronaut.configuration.kafka.annotation.KafkaListener
import io.micronaut.configuration.kafka.annotation.OffsetReset
import io.micronaut.configuration.kafka.annotation.Topic
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
        LOG.info("Got Order - {} by {}", payload, key)
        val orderId = UUID.fromString(payload!!.replace("\"", ""))
        LOG.info("Acknowledging $orderId...")
        return webClient.sendAcknowledgement(orderId)
    }

    companion object {
        private val LOG: Logger = getLogger(Acknowledgements::class.java)
    }
}
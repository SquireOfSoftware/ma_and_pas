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
        LOG.info("Got Sub Order - {} by {}", payload, key)
        val subOrderId = UUID.fromString(payload!!.replace("\"", ""))
        LOG.info("Acknowledging $subOrderId...")
        return webClient.sendAcknowledgement(subOrderId)
    }

    companion object {
        private val LOG: Logger = getLogger(Acknowledgements::class.java)
    }
}
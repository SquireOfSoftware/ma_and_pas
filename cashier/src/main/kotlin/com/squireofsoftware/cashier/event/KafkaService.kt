package com.squireofsoftware.cashier.event

import org.springframework.beans.factory.annotation.Autowired
import org.springframework.kafka.core.KafkaTemplate
import org.springframework.stereotype.Service

@Service
class KafkaService(
    val kafkaTemplate: KafkaTemplate<String, String>,
    @Autowired
    val topicName: String
) {
    fun sendEvent(message: String) {
        kafkaTemplate.send(topicName, message)
    }
}
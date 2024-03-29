package com.squireofsoftware.cashier

import org.apache.kafka.clients.admin.NewTopic
import org.springframework.beans.factory.annotation.Value
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.kafka.config.TopicBuilder


@Configuration
class KafkaConfig(
    @Value(value = "\${topic.name}")
    val subOrderTopicName: String
) {
    @Bean
    fun topicCreate(): NewTopic {
        return TopicBuilder.name(subOrderTopicName).build()
    }

    @Bean
    fun subOrderTopicName(): String {
        return subOrderTopicName
    }
}
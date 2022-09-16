package com.squireofsoftware.orders

import org.springframework.boot.web.servlet.FilterRegistrationBean
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.web.cors.CorsConfiguration
import org.springframework.web.cors.UrlBasedCorsConfigurationSource
import org.springframework.web.filter.CorsFilter

@Configuration
class OrdersConfiguration {
    @Bean
    fun corsConfigurationSource(): FilterRegistrationBean<CorsFilter> {
        val cors = UrlBasedCorsConfigurationSource()
        val config = CorsConfiguration()

        config.allowedMethods = listOf(CorsConfiguration.ALL)
        config.allowedHeaders = listOf(CorsConfiguration.ALL)
        config.allowedOrigins = listOf(CorsConfiguration.ALL)

        cors.registerCorsConfiguration("/graphql/**", config)
        val bean = FilterRegistrationBean(CorsFilter(cors))
        bean.order = 0

        return bean
    }
}
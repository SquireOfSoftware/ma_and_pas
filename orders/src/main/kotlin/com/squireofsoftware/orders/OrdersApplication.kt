package com.squireofsoftware.orders

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.web.servlet.config.annotation.CorsRegistry
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer

@Configuration
@SpringBootApplication
class OrdersApplication {
//	@Bean
//	fun corsConfigurer(): WebMvcConfigurer {
//		return WebMvcConfigurer {
//			@Override
//			fun addCorsMapping(registry: CorsRegistry) {
//				registry.addMapping("*/*").allowedOrigins("*")
//			}
//		}
//	}
}

fun main(args: Array<String>) {
	runApplication<OrdersApplication>(*args)
}
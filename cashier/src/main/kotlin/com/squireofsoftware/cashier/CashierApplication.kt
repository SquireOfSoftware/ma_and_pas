package com.squireofsoftware.cashier

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class CashierApplication

fun main(args: Array<String>) {
	runApplication<CashierApplication>(*args)
}

package com.squireofsoftware.cashier.order

import java.util.*

class OrderNotFoundException(orderId: UUID) : RuntimeException("Order $orderId was not found")
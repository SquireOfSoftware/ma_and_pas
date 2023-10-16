package com.squireofsoftware.cashier.acknowledgements

import java.util.*

class OrderNotFoundException(orderId: UUID) : RuntimeException("Order $orderId was not found")
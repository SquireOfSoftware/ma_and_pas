package com.squireofsoftware.cashier.order

import java.util.*

class SubOrderNotFoundException(subOrderId: UUID) : RuntimeException("Sub Order $subOrderId was not found")
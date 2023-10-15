package com.squireofsoftware.cashier.item

class InvalidItemException(ids: List<String>): RuntimeException("Could not find the items of: $ids")
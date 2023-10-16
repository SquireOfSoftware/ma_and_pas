package com.squireofsoftware.cashier.item

import java.util.UUID

class InvalidItemsException(ids: List<UUID>): RuntimeException("Could not find the items of: $ids")
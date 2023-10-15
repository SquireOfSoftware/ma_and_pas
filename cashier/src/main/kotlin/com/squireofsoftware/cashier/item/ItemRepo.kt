package com.squireofsoftware.cashier.item

import org.springframework.data.repository.CrudRepository
import java.util.UUID

interface ItemRepo: CrudRepository<Item, UUID> {
    fun findAllByDishType(dishType: DishType): List<Item>
    fun findAllByIdIn(ids: List<UUID>): List<Item>
}
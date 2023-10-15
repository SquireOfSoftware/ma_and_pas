package com.squireofsoftware.cashier.item

import org.springframework.data.repository.CrudRepository
import java.util.*

interface ItemRepo: CrudRepository<Item, UUID> {
    fun findAllByDishType(dishType: DishType): List<Item>
    fun findAllByIdIn(ids: List<UUID>): List<Item>
}
package com.squireofsoftware.cashier.burger

import com.netflix.graphql.dgs.DgsComponent
import com.netflix.graphql.dgs.DgsQuery
import com.squireofsoftware.cashier.item.DishType
import com.squireofsoftware.cashier.item.ItemRepo
import org.springframework.beans.factory.annotation.Autowired

@DgsComponent
class BurgerDataFetcher(
    @Autowired
    val itemRepo: ItemRepo
) {
    @DgsQuery
    fun burgers(): List<Burger> {
        return itemRepo.findAllByDishType(DishType.burger).map {
            Burger.toBurger(it)
        }
    }
}
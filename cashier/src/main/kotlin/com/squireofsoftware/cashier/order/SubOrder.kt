package com.squireofsoftware.cashier.order

import com.squireofsoftware.cashier.CookRequest
import com.squireofsoftware.cashier.item.Item
import jakarta.persistence.*
import java.util.*

@Entity
@Table(name = "suborders")
data class SubOrder(
    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    val id: UUID = UUID.randomUUID(),
    @Column
    val orderId: UUID,
    @Column
    val itemId: UUID,
    @Column
    val createdAt: Long,
    @Column
    val lastUpdated: Long,
    @Enumerated(EnumType.STRING)
    var state: State = State.requested,
) {
    fun toCookRequest(item: Item): CookRequest {
        return CookRequest(
            subOrderId = id,
            orderId = orderId,
            dishType = item.dishType,
            dishName = item.name
        )
    }
}
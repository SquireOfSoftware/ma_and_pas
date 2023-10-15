package com.squireofsoftware.cashier.order

import jakarta.persistence.*
import java.util.UUID

@Entity
@Table(name = "orders")
data class Order(
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    val id: UUID,
    @Column
    val createdAt: Long,
    @Column
    val lastUpdated: Long,
    @Enumerated(EnumType.STRING)
    val state: State = State.requested,
    @Column
    val price: Int,
) {
    fun toFrontOrder(): FrontOrder {
        return FrontOrder(
            id = id,
            createdAt = createdAt,
            lastUpdated = lastUpdated,
            state = state,
            price = price
        )
    }
}

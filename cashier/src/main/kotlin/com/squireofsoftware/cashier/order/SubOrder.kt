package com.squireofsoftware.cashier.order

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
)
package com.squireofsoftware.cashier.item

import jakarta.persistence.*
import java.util.*

@Entity
@Table(name = "items")
data class Item(@Id
                @GeneratedValue(strategy = GenerationType.UUID)
                val id: UUID,
                @Column
                val displayName: String,
                @Column
                val name: String,
                @Column
                @Enumerated(EnumType.STRING)
                val dishType: DishType,
                @Column
                val price: Int)

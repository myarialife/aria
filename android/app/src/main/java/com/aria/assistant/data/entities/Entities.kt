package com.aria.assistant.data.entities

import androidx.room.Entity
import androidx.room.ForeignKey
import androidx.room.Index
import androidx.room.PrimaryKey
import androidx.room.TypeConverter
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken

/**
 * 助手会话实体
 */
@Entity(tableName = "assistant_conversations")
data class AssistantConversation(
    @PrimaryKey(autoGenerate = true) val id: Long = 0,
    val title: String,
    val createdAt: Long,
    val updatedAt: Long
)

/**
 * 助手消息实体
 */
@Entity(
    tableName = "assistant_messages",
    foreignKeys = [
        ForeignKey(
            entity = AssistantConversation::class,
            parentColumns = ["id"],
            childColumns = ["conversationId"],
            onDelete = ForeignKey.CASCADE
        )
    ],
    indices = [Index("conversationId")]
)
data class AssistantMessage(
    @PrimaryKey(autoGenerate = true) val id: Long = 0,
    val conversationId: Long = 0,
    val content: String,
    val isFromUser: Boolean,
    val timestamp: Long
)

/**
 * 收集的数据实体
 */
@Entity(tableName = "collected_data")
data class CollectedData(
    @PrimaryKey(autoGenerate = true) val id: Long = 0,
    val type: String,
    val content: String,
    val timestamp: Long,
    val isSynced: Boolean = false,
    val syncTimestamp: Long? = null,
    val rewardAmount: Double? = null
)

/**
 * 钱包交易记录实体
 */
@Entity(tableName = "wallet_transactions")
data class WalletTransaction(
    @PrimaryKey val id: String,
    val amount: Double,
    val timestamp: Long,
    val type: String,
    val status: String,
    val description: String? = null,
    val fromAddress: String? = null,
    val toAddress: String? = null
)

/**
 * 带消息的会话
 */
data class ConversationWithMessages(
    val conversation: AssistantConversation,
    val messages: List<AssistantMessage>
)

/**
 * 类型转换器
 */
class Converters {
    @TypeConverter
    fun fromString(value: String): Map<String, Any> {
        val mapType = object : TypeToken<Map<String, Any>>() {}.type
        return Gson().fromJson(value, mapType)
    }
    
    @TypeConverter
    fun fromMap(map: Map<String, Any>): String {
        return Gson().toJson(map)
    }
} 
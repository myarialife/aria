package com.aria.assistant.data.dao

import androidx.lifecycle.LiveData
import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.aria.assistant.data.entities.AssistantMessage

/**
 * 助手消息DAO
 */
@Dao
interface AssistantMessageDao {
    
    /**
     * 插入消息
     */
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(message: AssistantMessage): Long
    
    /**
     * 批量插入消息
     */
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertAll(messages: List<AssistantMessage>): List<Long>
    
    /**
     * 获取指定会话的所有消息
     */
    @Query("SELECT * FROM assistant_messages WHERE conversationId = :conversationId ORDER BY timestamp ASC")
    fun getMessagesForConversation(conversationId: Long): LiveData<List<AssistantMessage>>
    
    /**
     * 获取指定ID的消息
     */
    @Query("SELECT * FROM assistant_messages WHERE id = :id")
    suspend fun getMessageById(id: Long): AssistantMessage?
    
    /**
     * 获取指定会话最后一条消息
     */
    @Query("SELECT * FROM assistant_messages WHERE conversationId = :conversationId ORDER BY timestamp DESC LIMIT 1")
    suspend fun getLastMessageForConversation(conversationId: Long): AssistantMessage?
    
    /**
     * 删除指定会话的所有消息
     */
    @Query("DELETE FROM assistant_messages WHERE conversationId = :conversationId")
    suspend fun deleteMessagesForConversation(conversationId: Long)
    
    /**
     * 删除所有消息
     */
    @Query("DELETE FROM assistant_messages")
    suspend fun deleteAllMessages()
    
    /**
     * 搜索指定会话中的消息
     */
    @Query("SELECT * FROM assistant_messages WHERE conversationId = :conversationId AND content LIKE '%' || :query || '%' ORDER BY timestamp ASC")
    fun searchMessagesInConversation(conversationId: Long, query: String): LiveData<List<AssistantMessage>>
    
    /**
     * 搜索所有消息
     */
    @Query("SELECT * FROM assistant_messages WHERE content LIKE '%' || :query || '%' ORDER BY timestamp DESC")
    fun searchAllMessages(query: String): LiveData<List<AssistantMessage>>
    
    /**
     * 获取指定会话的消息数量
     */
    @Query("SELECT COUNT(*) FROM assistant_messages WHERE conversationId = :conversationId")
    suspend fun getMessageCountForConversation(conversationId: Long): Int
} 
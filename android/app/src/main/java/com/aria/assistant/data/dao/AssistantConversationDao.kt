package com.aria.assistant.data.dao

import androidx.lifecycle.LiveData
import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import androidx.room.Transaction
import com.aria.assistant.data.entities.AssistantConversation
import com.aria.assistant.data.entities.ConversationWithMessages

/**
 * 助手会话DAO
 */
@Dao
interface AssistantConversationDao {
    
    /**
     * 插入会话
     */
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(conversation: AssistantConversation): Long
    
    /**
     * 获取所有会话
     */
    @Query("SELECT * FROM assistant_conversations ORDER BY updatedAt DESC")
    fun getAllConversations(): LiveData<List<AssistantConversation>>
    
    /**
     * 获取指定ID的会话
     */
    @Query("SELECT * FROM assistant_conversations WHERE id = :id")
    suspend fun getConversationById(id: Long): AssistantConversation?
    
    /**
     * 获取带有消息的会话
     */
    @Transaction
    @Query("SELECT * FROM assistant_conversations WHERE id = :id")
    fun getConversationWithMessages(id: Long): LiveData<ConversationWithMessages>
    
    /**
     * 更新会话标题
     */
    @Query("UPDATE assistant_conversations SET title = :title, updatedAt = :timestamp WHERE id = :id")
    suspend fun updateConversationTitle(id: Long, title: String, timestamp: Long)
    
    /**
     * 更新会话的更新时间
     */
    @Query("UPDATE assistant_conversations SET updatedAt = :timestamp WHERE id = :id")
    suspend fun updateConversationTimestamp(id: Long, timestamp: Long)
    
    /**
     * 删除会话
     */
    @Query("DELETE FROM assistant_conversations WHERE id = :id")
    suspend fun deleteConversation(id: Long)
    
    /**
     * 删除所有会话
     */
    @Query("DELETE FROM assistant_conversations")
    suspend fun deleteAllConversations()
    
    /**
     * 搜索会话
     */
    @Query("SELECT * FROM assistant_conversations WHERE title LIKE '%' || :query || '%' ORDER BY updatedAt DESC")
    fun searchConversations(query: String): LiveData<List<AssistantConversation>>
    
    /**
     * 获取会话数量
     */
    @Query("SELECT COUNT(*) FROM assistant_conversations")
    suspend fun getConversationCount(): Int
} 
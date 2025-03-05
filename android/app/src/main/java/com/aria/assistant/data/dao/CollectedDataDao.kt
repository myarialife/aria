package com.aria.assistant.data.dao

import androidx.lifecycle.LiveData
import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.aria.assistant.data.entities.CollectedData

/**
 * 数据收集DAO
 */
@Dao
interface CollectedDataDao {
    
    /**
     * 插入收集的数据
     */
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(data: CollectedData): Long
    
    /**
     * 批量插入数据
     */
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertAll(data: List<CollectedData>): List<Long>
    
    /**
     * 获取所有收集的数据
     */
    @Query("SELECT * FROM collected_data ORDER BY timestamp DESC")
    fun getAllCollectedData(): LiveData<List<CollectedData>>
    
    /**
     * 获取未同步的数据
     */
    @Query("SELECT * FROM collected_data WHERE isSynced = 0")
    suspend fun getUnsyncedData(): List<CollectedData>
    
    /**
     * 获取收集的数据数量
     */
    @Query("SELECT COUNT(*) FROM collected_data")
    suspend fun getCollectedDataCount(): Int
    
    /**
     * 获取总奖励金额
     */
    @Query("SELECT SUM(rewardAmount) FROM collected_data")
    suspend fun getTotalRewards(): Double?
    
    /**
     * 更新同步状态
     */
    @Query("UPDATE collected_data SET isSynced = :isSynced, syncTimestamp = :syncTimestamp, rewardAmount = :rewardAmount WHERE id = :id")
    suspend fun updateSyncStatus(id: Long, isSynced: Boolean, syncTimestamp: Long, rewardAmount: Double?)
    
    /**
     * 按类型获取数据
     */
    @Query("SELECT * FROM collected_data WHERE type = :type ORDER BY timestamp DESC")
    fun getCollectedDataByType(type: String): LiveData<List<CollectedData>>
    
    /**
     * 删除超过指定时间的数据
     */
    @Query("DELETE FROM collected_data WHERE timestamp < :timestampThreshold")
    suspend fun deleteOldData(timestampThreshold: Long)
    
    /**
     * 删除所有数据
     */
    @Query("DELETE FROM collected_data")
    suspend fun deleteAllData()
} 
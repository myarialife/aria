package com.aria.assistant.data.dao

import androidx.lifecycle.LiveData
import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.aria.assistant.data.entities.WalletTransaction

/**
 * 钱包交易DAO
 */
@Dao
interface WalletTransactionDao {
    
    /**
     * 插入钱包交易
     */
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(transaction: WalletTransaction)
    
    /**
     * 批量插入钱包交易
     */
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertAll(transactions: List<WalletTransaction>)
    
    /**
     * 获取所有交易
     */
    @Query("SELECT * FROM wallet_transactions ORDER BY timestamp DESC")
    fun getAllTransactions(): LiveData<List<WalletTransaction>>
    
    /**
     * 按交易类型获取交易
     */
    @Query("SELECT * FROM wallet_transactions WHERE type = :type ORDER BY timestamp DESC")
    fun getTransactionsByType(type: String): LiveData<List<WalletTransaction>>
    
    /**
     * 获取指定ID的交易
     */
    @Query("SELECT * FROM wallet_transactions WHERE id = :id")
    suspend fun getTransactionById(id: String): WalletTransaction?
    
    /**
     * 获取交易总额
     */
    @Query("SELECT SUM(amount) FROM wallet_transactions WHERE type = :type")
    suspend fun getTotalAmountByType(type: String): Double?
    
    /**
     * 删除所有交易
     */
    @Query("DELETE FROM wallet_transactions")
    suspend fun deleteAllTransactions()
    
    /**
     * 删除指定交易
     */
    @Query("DELETE FROM wallet_transactions WHERE id = :id")
    suspend fun deleteTransaction(id: String)
    
    /**
     * 获取交易数量
     */
    @Query("SELECT COUNT(*) FROM wallet_transactions")
    suspend fun getTransactionCount(): Int
} 
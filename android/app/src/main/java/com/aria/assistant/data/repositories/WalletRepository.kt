package com.aria.assistant.data.repositories

import android.content.Context
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import com.aria.assistant.blockchain.SolanaWalletManager
import com.aria.assistant.data.AppDatabase
import com.aria.assistant.data.entities.WalletTransaction
import com.aria.assistant.network.AriaApiService
import com.aria.assistant.network.models.TokenRewardRequest
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

/**
 * 钱包存储库 - 负责管理钱包相关操作
 */
class WalletRepository(
    private val context: Context,
    private val apiService: AriaApiService
) {
    private val walletManager = SolanaWalletManager(context)
    private val walletDatabase = AppDatabase.getInstance(context)
    
    // 当前钱包地址
    private val _walletAddress = MutableLiveData<String>()
    val walletAddress: LiveData<String> = _walletAddress
    
    // 当前代币余额
    private val _tokenBalance = MutableLiveData<Double>()
    val tokenBalance: LiveData<Double> = _tokenBalance
    
    init {
        refreshWalletInfo()
    }
    
    /**
     * 刷新钱包信息
     */
    suspend fun refreshWalletInfo() {
        withContext(Dispatchers.IO) {
            if (walletManager.hasWallet()) {
                val address = walletManager.getWalletAddress()
                _walletAddress.postValue(address)
                
                try {
                    val balance = walletManager.getAriaTokenBalance()
                    _tokenBalance.postValue(balance)
                } catch (e: Exception) {
                    // 如果本地钱包获取失败，尝试从API获取
                    try {
                        val response = apiService.getWalletInfo(address)
                        if (response.isSuccessful && response.body()?.success == true) {
                            _tokenBalance.postValue(response.body()?.data?.balance ?: 0.0)
                        }
                    } catch (e: Exception) {
                        e.printStackTrace()
                    }
                }
            } else {
                _walletAddress.postValue("")
                _tokenBalance.postValue(0.0)
            }
        }
    }
    
    /**
     * 创建新钱包
     */
    suspend fun createWallet(): List<String>? {
        return withContext(Dispatchers.IO) {
            try {
                val mnemonic = walletManager.createWallet()
                refreshWalletInfo()
                mnemonic
            } catch (e: Exception) {
                e.printStackTrace()
                null
            }
        }
    }
    
    /**
     * 导入钱包
     */
    suspend fun importWallet(mnemonic: List<String>): Boolean {
        return withContext(Dispatchers.IO) {
            try {
                walletManager.importWalletFromMnemonic(mnemonic)
                refreshWalletInfo()
                true
            } catch (e: Exception) {
                e.printStackTrace()
                false
            }
        }
    }
    
    /**
     * 获取交易历史
     */
    fun getTransactionHistory(): LiveData<List<WalletTransaction>> {
        return walletDatabase.walletTransactionDao().getAllTransactions()
    }
    
    /**
     * 请求代币奖励
     */
    suspend fun requestTokenReward(): Boolean {
        return withContext(Dispatchers.IO) {
            try {
                val address = walletManager.getWalletAddress()
                val request = TokenRewardRequest(address)
                val response = apiService.requestTokenReward(request)
                
                if (response.isSuccessful && response.body()?.success == true) {
                    // 更新本地交易记录
                    response.body()?.data?.transactionInfo?.let { transaction ->
                        val walletTx = WalletTransaction(
                            id = transaction.txId,
                            amount = transaction.amount,
                            timestamp = System.currentTimeMillis(),
                            type = "REWARD",
                            status = "COMPLETED",
                            description = "数据收集奖励",
                            fromAddress = transaction.fromAddress,
                            toAddress = address
                        )
                        walletDatabase.walletTransactionDao().insert(walletTx)
                    }
                    
                    refreshWalletInfo()
                    return@withContext true
                }
                false
            } catch (e: Exception) {
                e.printStackTrace()
                false
            }
        }
    }
    
    /**
     * 检查是否有钱包
     */
    fun hasWallet(): Boolean {
        return walletManager.hasWallet()
    }
} 
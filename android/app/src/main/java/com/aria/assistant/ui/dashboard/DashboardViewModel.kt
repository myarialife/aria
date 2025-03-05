package com.aria.assistant.ui.dashboard

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.viewModelScope
import com.aria.assistant.data.repositories.DataRepository
import com.aria.assistant.data.repositories.WalletRepository
import com.aria.assistant.network.AriaApiService
import com.aria.assistant.network.RetrofitHelper
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import javax.inject.Inject

/**
 * 主页面ViewModel
 */
@HiltViewModel
class DashboardViewModel @Inject constructor(
    application: Application,
    private val walletRepository: WalletRepository,
    private val dataRepository: DataRepository
) : AndroidViewModel(application) {
    
    private val apiService: AriaApiService = RetrofitHelper.apiService
    
    // 钱包状态
    private val _walletStatus = MutableLiveData<Boolean>()
    val walletStatus: LiveData<Boolean> = _walletStatus
    
    // 代币余额
    private val _tokenBalance = MutableLiveData<Double>()
    val tokenBalance: LiveData<Double> = _tokenBalance
    
    // 数据收集状态
    private val _dataCollectionEnabled = MutableLiveData<Boolean>()
    val dataCollectionEnabled: LiveData<Boolean> = _dataCollectionEnabled
    
    // 收集的数据计数
    private val _collectedDataCount = MutableLiveData<Int>()
    val collectedDataCount: LiveData<Int> = _collectedDataCount
    
    // 对话计数
    private val _conversationCount = MutableLiveData<Int>()
    val conversationCount: LiveData<Int> = _conversationCount
    
    // 总奖励
    private val _totalRewards = MutableLiveData<Double>()
    val totalRewards: LiveData<Double> = _totalRewards
    
    // 用户级别
    private val _userLevel = MutableLiveData<Int>()
    val userLevel: LiveData<Int> = _userLevel
    
    init {
        loadInitialData()
    }
    
    /**
     * 加载初始数据
     */
    private fun loadInitialData() {
        viewModelScope.launch(Dispatchers.IO) {
            // 加载钱包信息
            _walletStatus.postValue(walletRepository.hasWallet())
            
            // 监听钱包余额
            walletRepository.tokenBalance.observeForever { balance ->
                _tokenBalance.postValue(balance)
            }
            
            // 监听数据收集状态
            dataRepository.dataCollectionEnabled.observeForever { isEnabled ->
                _dataCollectionEnabled.postValue(isEnabled)
            }
            
            // 监听收集的数据计数
            dataRepository.collectedDataCount.observeForever { count ->
                _collectedDataCount.postValue(count)
            }
            
            // 监听总奖励
            dataRepository.totalRewards.observeForever { rewards ->
                _totalRewards.postValue(rewards)
            }
            
            // 获取用户统计信息
            fetchUserStats()
        }
    }
    
    /**
     * 获取用户统计信息
     */
    private fun fetchUserStats() {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                val response = apiService.getUserStats()
                if (response.isSuccessful && response.body()?.success == true) {
                    val stats = response.body()?.data
                    stats?.let {
                        _conversationCount.postValue(it.conversationCount)
                        _totalRewards.postValue(it.totalRewards)
                        _userLevel.postValue(it.userLevel)
                    }
                }
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }
    
    /**
     * 刷新所有数据
     */
    fun refresh() {
        viewModelScope.launch {
            walletRepository.refreshWalletInfo()
            dataRepository.refreshDataCount()
            dataRepository.loadTotalRewards()
            fetchUserStats()
        }
    }
} 
package com.aria.assistant.data.repositories

import android.content.Context
import android.content.SharedPreferences
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import com.aria.assistant.data.AppDatabase
import com.aria.assistant.data.entities.CollectedData
import com.aria.assistant.network.AriaApiService
import com.aria.assistant.network.models.DataPermissionRequest
import com.aria.assistant.network.models.SubmitDataRequest
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject

/**
 * 数据收集仓库类
 */
class DataRepository(
    private val context: Context,
    private val apiService: AriaApiService
) {
    private val prefs: SharedPreferences = context.getSharedPreferences("aria_prefs", Context.MODE_PRIVATE)
    private val database = AppDatabase.getInstance(context)
    
    companion object {
        const val KEY_DATA_COLLECTION_ENABLED = "data_collection_enabled"
        const val KEY_PERMISSION_PREFIX = "permission_"
        
        // 权限类型
        const val PERMISSION_LOCATION = "location"
        const val PERMISSION_CONTACTS = "contacts"
        const val PERMISSION_CALENDAR = "calendar"
        const val PERMISSION_SMS = "sms"
    }
    
    // 数据收集状态
    private val _dataCollectionEnabled = MutableLiveData<Boolean>()
    val dataCollectionEnabled: LiveData<Boolean> = _dataCollectionEnabled
    
    // 总奖励金额
    private val _totalRewards = MutableLiveData<Double>()
    val totalRewards: LiveData<Double> = _totalRewards
    
    // 已收集的数据计数
    private val _collectedDataCount = MutableLiveData<Int>()
    val collectedDataCount: LiveData<Int> = _collectedDataCount
    
    init {
        loadDataCollectionStatus()
        refreshDataCount()
    }
    
    /**
     * 加载数据收集状态
     */
    private fun loadDataCollectionStatus() {
        val isEnabled = prefs.getBoolean(KEY_DATA_COLLECTION_ENABLED, false)
        _dataCollectionEnabled.postValue(isEnabled)
    }
    
    /**
     * 设置数据收集状态
     */
    fun setDataCollectionEnabled(enabled: Boolean) {
        prefs.edit().putBoolean(KEY_DATA_COLLECTION_ENABLED, enabled).apply()
        _dataCollectionEnabled.postValue(enabled)
    }
    
    /**
     * 刷新数据计数
     */
    suspend fun refreshDataCount() {
        withContext(Dispatchers.IO) {
            val count = database.collectedDataDao().getCollectedDataCount()
            _collectedDataCount.postValue(count)
        }
    }
    
    /**
     * 获取未同步的数据
     */
    suspend fun getUnsyncedData(): List<CollectedData> {
        return withContext(Dispatchers.IO) {
            database.collectedDataDao().getUnsyncedData()
        }
    }
    
    /**
     * 保存收集的数据
     */
    suspend fun saveCollectedData(type: String, content: String) {
        withContext(Dispatchers.IO) {
            val data = CollectedData(
                type = type,
                content = content,
                timestamp = System.currentTimeMillis(),
                isSynced = false
            )
            database.collectedDataDao().insert(data)
            refreshDataCount()
        }
    }
    
    /**
     * 同步数据到服务器
     */
    suspend fun syncDataToServer(): Boolean {
        return withContext(Dispatchers.IO) {
            try {
                val unsyncedData = getUnsyncedData()
                if (unsyncedData.isEmpty()) {
                    return@withContext true
                }
                
                val dataItems = unsyncedData.map { data ->
                    val contentJson = try {
                        JSONObject(data.content)
                    } catch (e: Exception) {
                        JSONObject().put("raw", data.content)
                    }
                    
                    SubmitDataRequest.DataItem(
                        id = data.id,
                        type = data.type,
                        content = contentJson.toString(),
                        timestamp = data.timestamp
                    )
                }
                
                val request = SubmitDataRequest(dataItems)
                val response = apiService.submitCollectedData(request)
                
                if (response.isSuccessful && response.body()?.success == true) {
                    // 更新已同步的数据
                    response.body()?.data?.syncedData?.forEach { syncedItem ->
                        val localData = unsyncedData.find { it.id == syncedItem.id }
                        localData?.let {
                            it.apply {
                                database.collectedDataDao().updateSyncStatus(
                                    id = syncedItem.id,
                                    isSynced = true,
                                    syncTimestamp = System.currentTimeMillis(),
                                    rewardAmount = syncedItem.reward
                                )
                            }
                        }
                    }
                    
                    // 更新总奖励
                    loadTotalRewards()
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
     * 加载总奖励
     */
    suspend fun loadTotalRewards() {
        withContext(Dispatchers.IO) {
            try {
                // 尝试从API获取最新奖励信息
                val statsResponse = apiService.getUserStats()
                if (statsResponse.isSuccessful && statsResponse.body()?.success == true) {
                    _totalRewards.postValue(statsResponse.body()?.data?.totalRewards ?: 0.0)
                } else {
                    // 如果API调用失败，回退到本地计算
                    val localTotal = database.collectedDataDao().getTotalRewards() ?: 0.0
                    _totalRewards.postValue(localTotal)
                }
            } catch (e: Exception) {
                e.printStackTrace()
                // 如果API调用异常，回退到本地计算
                val localTotal = database.collectedDataDao().getTotalRewards() ?: 0.0
                _totalRewards.postValue(localTotal)
            }
        }
    }
    
    /**
     * 设置数据权限
     */
    suspend fun setDataPermission(permissionType: String, enabled: Boolean): Boolean {
        // 保存本地权限设置
        val prefKey = KEY_PERMISSION_PREFIX + permissionType
        prefs.edit().putBoolean(prefKey, enabled).apply()
        
        // 同步到服务器
        return withContext(Dispatchers.IO) {
            try {
                val request = DataPermissionRequest(permissionType, enabled)
                val response = apiService.updateDataPermissions(request)
                response.isSuccessful && response.body()?.success == true
            } catch (e: Exception) {
                e.printStackTrace()
                false
            }
        }
    }
    
    /**
     * 获取数据权限
     */
    fun getDataPermission(permissionType: String): Boolean {
        val prefKey = KEY_PERMISSION_PREFIX + permissionType
        return prefs.getBoolean(prefKey, false)
    }
} 
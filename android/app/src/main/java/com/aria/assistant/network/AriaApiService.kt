package com.aria.assistant.network

import com.aria.assistant.network.models.ApiResponse
import com.aria.assistant.network.models.AssistantResponse
import com.aria.assistant.network.models.CollectedDataResponse
import com.aria.assistant.network.models.UserStatsResponse
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Response
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import retrofit2.http.Body
import retrofit2.http.GET
import retrofit2.http.POST
import retrofit2.http.Path
import retrofit2.http.Query
import java.util.concurrent.TimeUnit

/**
 * Aria API服务接口
 */
interface AriaApiService {
    
    /**
     * 用户登录
     */
    @POST("auth/login")
    suspend fun login(
        @Body credentials: Map<String, String>
    ): Response<ApiResponse<String>> // 返回JWT token
    
    /**
     * 用户注册
     */
    @POST("auth/register")
    suspend fun register(
        @Body userData: Map<String, String>
    ): Response<ApiResponse<String>> // 返回JWT token
    
    /**
     * 获取用户统计信息
     */
    @GET("user/stats")
    suspend fun getUserStats(): Response<UserStatsResponse>
    
    /**
     * 更新数据权限
     */
    @POST("data/permissions")
    suspend fun updateDataPermissions(
        @Body permissions: Map<String, Boolean>
    ): Response<ApiResponse<Boolean>>
    
    /**
     * 提交收集的数据
     */
    @POST("data/submit")
    suspend fun submitCollectedData(
        @Body data: Map<String, Any>
    ): Response<CollectedDataResponse>
    
    /**
     * 获取AI助手回复
     */
    @POST("assistant/query")
    suspend fun getAssistantResponse(
        @Body query: Map<String, String>
    ): Response<AssistantResponse>
    
    /**
     * 获取钱包信息
     */
    @GET("wallet/{address}")
    suspend fun getWalletInfo(
        @Path("address") address: String
    ): Response<ApiResponse<Map<String, Any>>>
    
    /**
     * 请求代币奖励
     */
    @POST("wallet/reward")
    suspend fun requestTokenReward(
        @Body request: Map<String, Any>
    ): Response<ApiResponse<Map<String, Any>>>
    
    companion object {
        private const val BASE_URL = "https://api.ariaassistant.com/v1/"
        
        /**
         * 创建API服务实例
         */
        fun create(): AriaApiService {
            val logger = HttpLoggingInterceptor().apply { 
                level = HttpLoggingInterceptor.Level.BODY 
            }
            
            val client = OkHttpClient.Builder()
                .addInterceptor(logger)
                .connectTimeout(15, TimeUnit.SECONDS)
                .readTimeout(15, TimeUnit.SECONDS)
                .writeTimeout(15, TimeUnit.SECONDS)
                .build()
            
            return Retrofit.Builder()
                .baseUrl(BASE_URL)
                .client(client)
                .addConverterFactory(GsonConverterFactory.create())
                .build()
                .create(AriaApiService::class.java)
        }
    }
} 
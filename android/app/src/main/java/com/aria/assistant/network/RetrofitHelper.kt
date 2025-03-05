package com.aria.assistant.network

import com.aria.assistant.BuildConfig
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import java.util.concurrent.TimeUnit

/**
 * Retrofit帮助类 - 用于创建API服务实例
 */
object RetrofitHelper {
    
    private const val BASE_URL = "https://api.ariaassistant.io/v1/"
    private const val TIMEOUT = 30L
    
    // 创建OkHttpClient
    private val okHttpClient by lazy {
        val builder = OkHttpClient.Builder()
            .connectTimeout(TIMEOUT, TimeUnit.SECONDS)
            .readTimeout(TIMEOUT, TimeUnit.SECONDS)
            .writeTimeout(TIMEOUT, TimeUnit.SECONDS)
            
        // 添加日志拦截器（仅在DEBUG模式下）
        if (BuildConfig.DEBUG) {
            val logging = HttpLoggingInterceptor()
            logging.level = HttpLoggingInterceptor.Level.BODY
            builder.addInterceptor(logging)
        }
        
        // 添加认证拦截器
        builder.addInterceptor(AuthInterceptor())
        
        builder.build()
    }
    
    // 创建Retrofit实例
    private val retrofit by lazy {
        Retrofit.Builder()
            .baseUrl(BASE_URL)
            .client(okHttpClient)
            .addConverterFactory(GsonConverterFactory.create())
            .build()
    }
    
    // 创建API服务
    val apiService: AriaApiService by lazy {
        retrofit.create(AriaApiService::class.java)
    }
} 
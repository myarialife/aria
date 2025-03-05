package com.aria.assistant.network

import android.content.Context
import android.content.SharedPreferences
import com.aria.assistant.AriaApplication
import okhttp3.Interceptor
import okhttp3.Response

/**
 * 认证拦截器 - 为所有请求添加认证令牌
 */
class AuthInterceptor : Interceptor {
    
    companion object {
        private const val AUTH_HEADER = "Authorization"
        private const val TOKEN_KEY = "auth_token"
        private const val PREFS_NAME = "aria_prefs"
        
        /**
         * 保存令牌
         */
        fun saveToken(token: String) {
            val context = AriaApplication.getInstance()
            val prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
            prefs.edit().putString(TOKEN_KEY, token).apply()
        }
        
        /**
         * 清除令牌
         */
        fun clearToken() {
            val context = AriaApplication.getInstance()
            val prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
            prefs.edit().remove(TOKEN_KEY).apply()
        }
        
        /**
         * 获取令牌
         */
        fun getToken(): String? {
            val context = AriaApplication.getInstance()
            val prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
            return prefs.getString(TOKEN_KEY, null)
        }
        
        /**
         * 是否已认证
         */
        fun isAuthenticated(): Boolean {
            return getToken() != null
        }
    }
    
    override fun intercept(chain: Interceptor.Chain): Response {
        val request = chain.request()
        
        // 检查请求URL是否包含登录或注册路径（这些不需要令牌）
        val url = request.url.toString()
        if (url.contains("/login") || url.contains("/register")) {
            return chain.proceed(request)
        }
        
        // 获取认证令牌
        val token = getToken()
        
        // 如果有令牌，则添加到请求头
        return if (token != null) {
            val newRequest = request.newBuilder()
                .header(AUTH_HEADER, "Bearer $token")
                .build()
            chain.proceed(newRequest)
        } else {
            // 如果没有令牌，则继续原始请求
            chain.proceed(request)
        }
    }
} 
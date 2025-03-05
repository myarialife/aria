package com.aria.assistant.network.models

import com.google.gson.annotations.SerializedName

/**
 * 通用API响应类
 */
data class ApiResponse<T>(
    @SerializedName("success") val success: Boolean,
    @SerializedName("message") val message: String?,
    @SerializedName("data") val data: T?
)

/**
 * 用户统计信息响应
 */
data class UserStatsResponse(
    @SerializedName("totalRewards") val totalRewards: Double,
    @SerializedName("dataCollected") val dataCollected: Int,
    @SerializedName("dataProcessed") val dataProcessed: Int,
    @SerializedName("tokenBalance") val tokenBalance: Double
)

/**
 * 助手回复响应
 */
data class AssistantResponse(
    @SerializedName("message") val message: String,
    @SerializedName("timestamp") val timestamp: Long,
    @SerializedName("conversationId") val conversationId: String?
)

/**
 * 数据收集响应
 */
data class CollectedDataResponse(
    @SerializedName("dataId") val dataId: String,
    @SerializedName("reward") val reward: Double,
    @SerializedName("timestamp") val timestamp: Long
)

/**
 * 钱包信息响应
 */
data class WalletInfoResponse(
    @SerializedName("address") val address: String,
    @SerializedName("balance") val balance: Double,
    @SerializedName("transactions") val transactions: List<TransactionInfo>
)

/**
 * 交易信息
 */
data class TransactionInfo(
    @SerializedName("id") val id: String,
    @SerializedName("amount") val amount: Double,
    @SerializedName("timestamp") val timestamp: Long,
    @SerializedName("type") val type: String,
    @SerializedName("status") val status: String
)

/**
 * 用户信息响应
 */
data class UserInfoResponse(
    @SerializedName("id") val id: String,
    @SerializedName("username") val username: String,
    @SerializedName("email") val email: String,
    @SerializedName("walletAddress") val walletAddress: String?,
    @SerializedName("dataCollectionEnabled") val dataCollectionEnabled: Boolean,
    @SerializedName("dataPermissions") val dataPermissions: Map<String, Boolean>,
    @SerializedName("createdAt") val createdAt: Long,
    @SerializedName("lastLogin") val lastLogin: Long
) 
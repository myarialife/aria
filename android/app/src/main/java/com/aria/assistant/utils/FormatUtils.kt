package com.aria.assistant.utils

import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

/**
 * 格式化ARIA代币数量
 */
fun formatAriaTokenAmount(amount: Double): String {
    return String.format(Locale.getDefault(), "%.4f ARIA", amount)
}

/**
 * 格式化SOL数量
 */
fun formatSolAmount(amount: Double): String {
    return String.format(Locale.getDefault(), "%.6f SOL", amount)
}

/**
 * 格式化时间戳为日期
 */
fun formatTimestamp(timestamp: Long): String {
    val date = Date(timestamp)
    val format = SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.getDefault())
    return format.format(date)
}

/**
 * 格式化时间戳为相对时间
 */
fun formatRelativeTime(timestamp: Long): String {
    val now = System.currentTimeMillis()
    val diff = now - timestamp
    
    return when {
        diff < 60 * 1000 -> "刚刚"
        diff < 60 * 60 * 1000 -> "${diff / (60 * 1000)}分钟前"
        diff < 24 * 60 * 60 * 1000 -> "${diff / (60 * 60 * 1000)}小时前"
        diff < 30 * 24 * 60 * 60 * 1000L -> "${diff / (24 * 60 * 60 * 1000)}天前"
        else -> formatTimestamp(timestamp)
    }
}

/**
 * 格式化文件大小
 */
fun formatFileSize(size: Long): String {
    if (size <= 0) return "0 B"
    
    val units = arrayOf("B", "KB", "MB", "GB", "TB")
    val digitGroups = (Math.log10(size.toDouble()) / Math.log10(1024.0)).toInt()
    
    return String.format(
        Locale.getDefault(),
        "%.1f %s",
        size / Math.pow(1024.0, digitGroups.toDouble()),
        units[digitGroups]
    )
}

/**
 * 格式化钱包地址（缩短显示）
 */
fun formatWalletAddress(address: String): String {
    if (address.length <= 12) return address
    return "${address.substring(0, 6)}...${address.substring(address.length - 6)}"
}

/**
 * 格式化交易ID（缩短显示）
 */
fun formatTransactionId(txId: String): String {
    if (txId.length <= 12) return txId
    return "${txId.substring(0, 6)}...${txId.substring(txId.length - 6)}"
}

/**
 * 格式化数据类型为可读文本
 */
fun formatDataType(type: String): String {
    return when (type.lowercase()) {
        "location" -> "位置数据"
        "contacts" -> "通讯录数据"
        "calendar" -> "日历数据"
        "sms" -> "短信数据"
        "other" -> "其他数据"
        else -> type
    }
}

/**
 * 格式化隐私级别为可读文本
 */
fun formatPrivacyLevel(level: String): String {
    return when (level.lowercase()) {
        "public" -> "公开"
        "protected" -> "受保护"
        "private" -> "私有"
        "anonymized" -> "已匿名化"
        else -> level
    }
}
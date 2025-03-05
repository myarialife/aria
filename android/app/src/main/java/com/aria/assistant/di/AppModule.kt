package com.aria.assistant.di

import android.content.Context
import com.aria.assistant.data.AppDatabase
import com.aria.assistant.data.repositories.DataRepository
import com.aria.assistant.data.repositories.WalletRepository
import com.aria.assistant.network.AriaApiService
import com.aria.assistant.network.RetrofitHelper
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

/**
 * 应用依赖注入模块 - 提供各种单例
 */
@Module
@InstallIn(SingletonComponent::class)
object AppModule {
    
    /**
     * 提供API服务
     */
    @Provides
    @Singleton
    fun provideApiService(): AriaApiService {
        return RetrofitHelper.apiService
    }
    
    /**
     * 提供数据库
     */
    @Provides
    @Singleton
    fun provideAppDatabase(@ApplicationContext context: Context): AppDatabase {
        return AppDatabase.getInstance(context)
    }
    
    /**
     * 提供钱包仓库
     */
    @Provides
    @Singleton
    fun provideWalletRepository(
        @ApplicationContext context: Context,
        apiService: AriaApiService
    ): WalletRepository {
        return WalletRepository(context, apiService)
    }
    
    /**
     * 提供数据仓库
     */
    @Provides
    @Singleton
    fun provideDataRepository(
        @ApplicationContext context: Context,
        apiService: AriaApiService
    ): DataRepository {
        return DataRepository(context, apiService)
    }
} 
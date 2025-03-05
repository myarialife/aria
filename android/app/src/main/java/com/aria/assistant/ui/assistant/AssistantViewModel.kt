package com.aria.assistant.ui.assistant

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.viewModelScope
import com.aria.assistant.data.entities.AssistantConversation
import com.aria.assistant.data.entities.AssistantMessage
import com.aria.assistant.data.repository.AssistantRepository
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import java.util.UUID

class AssistantViewModel(application: Application) : AndroidViewModel(application) {
    
    private val repository: AssistantRepository
    
    // 当前会话ID
    var conversationId: Long = 0
    // 是否是新会话
    var isNewConversation: Boolean = true
    
    // 消息列表
    private val _messages = MutableLiveData<List<AssistantMessage>>(emptyList())
    val messages: LiveData<List<AssistantMessage>> = _messages
    
    // 加载状态
    private val _isLoading = MutableLiveData<Boolean>(false)
    val isLoading: LiveData<Boolean> = _isLoading
    
    // 错误消息
    private val _errorMessage = MutableLiveData<String>("")
    val errorMessage: LiveData<String> = _errorMessage
    
    init {
        val database = (application as com.aria.assistant.AriaApplication).database
        repository = AssistantRepository(database.assistantConversationDao())
        
        // 创建新的会话
        createNewConversation()
    }
    
    private fun createNewConversation() {
        viewModelScope.launch(Dispatchers.IO) {
            val conversation = AssistantConversation(
                title = "新会话 - ${System.currentTimeMillis()}",
                createdAt = System.currentTimeMillis(),
                updatedAt = System.currentTimeMillis()
            )
            conversationId = repository.insertConversation(conversation)
            isNewConversation = true
        }
    }
    
    fun sendMessage(content: String) {
        if (_isLoading.value == true) return
        
        // 添加用户消息
        val userMessage = AssistantMessage(
            conversationId = conversationId,
            content = content,
            isFromUser = true,
            timestamp = System.currentTimeMillis()
        )
        addMessage(userMessage)
        
        // 调用AI助手API
        _isLoading.value = true
        viewModelScope.launch {
            try {
                // 在MVP中，我们可以使用一个简单的延迟模拟AI响应
                val dummyResponses = listOf(
                    "我理解你想了解更多关于这个问题，让我为你解释一下...",
                    "这是一个很好的问题。根据我的分析...",
                    "我已经处理了你的请求，以下是我的答案...",
                    "基于你提供的信息，我认为...",
                    "我已经分析了你的问题，这是我的建议..."
                )
                
                // 模拟网络延迟
                kotlinx.coroutines.delay(1500)
                
                // 创建AI回复消息
                val aiMessage = AssistantMessage(
                    conversationId = conversationId,
                    content = dummyResponses.random() + "\n\n你的问题是: $content",
                    isFromUser = false,
                    timestamp = System.currentTimeMillis()
                )
                
                addMessage(aiMessage)
                
                // 实际项目中，这里会调用真实的AI API
                // val response = assistantApiService.sendMessage(content)
                // addMessage(AssistantMessage(conversationId = conversationId, content = response.message, isFromUser = false))
                
            } catch (e: Exception) {
                _errorMessage.value = "获取AI回复失败: ${e.message}"
            } finally {
                _isLoading.value = false
            }
        }
    }
    
    fun addMessage(message: AssistantMessage) {
        // 将消息添加到内存中的列表
        val currentList = _messages.value.orEmpty().toMutableList()
        currentList.add(message)
        _messages.value = currentList
        
        // 保存消息到数据库
        viewModelScope.launch(Dispatchers.IO) {
            repository.insertMessage(message.copy(conversationId = conversationId))
        }
    }
    
    fun loadConversation(id: Long) {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                val conversation = repository.getConversationWithMessages(id)
                if (conversation != null) {
                    conversationId = conversation.conversation.id
                    _messages.postValue(conversation.messages)
                    isNewConversation = false
                } else {
                    createNewConversation()
                }
            } catch (e: Exception) {
                _errorMessage.postValue("加载会话失败: ${e.message}")
                createNewConversation()
            }
        }
    }
} 
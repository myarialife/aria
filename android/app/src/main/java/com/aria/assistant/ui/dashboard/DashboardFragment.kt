package com.aria.assistant.ui.dashboard

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import androidx.navigation.fragment.findNavController
import com.aria.assistant.R
import com.aria.assistant.databinding.FragmentDashboardBinding
import com.aria.assistant.utils.formatAriaTokenAmount
import dagger.hilt.android.AndroidEntryPoint

/**
 * 主页面Fragment
 */
@AndroidEntryPoint
class DashboardFragment : Fragment() {
    
    private var _binding: FragmentDashboardBinding? = null
    private val binding get() = _binding!!
    
    private lateinit var viewModel: DashboardViewModel
    
    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        _binding = FragmentDashboardBinding.inflate(inflater, container, false)
        return binding.root
    }
    
    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)
        
        viewModel = ViewModelProvider(this)[DashboardViewModel::class.java]
        
        setupUI()
        setupObservers()
    }
    
    /**
     * 设置UI
     */
    private fun setupUI() {
        // 设置钱包卡片点击事件
        binding.cardWallet.setOnClickListener {
            findNavController().navigate(R.id.action_dashboardFragment_to_walletFragment)
        }
        
        // 设置数据卡片点击事件
        binding.cardData.setOnClickListener {
            findNavController().navigate(R.id.action_dashboardFragment_to_dataFragment)
        }
        
        // 设置助手卡片点击事件
        binding.cardAssistant.setOnClickListener {
            findNavController().navigate(R.id.action_dashboardFragment_to_assistantFragment)
        }
        
        // 设置社区卡片点击事件
        binding.cardCommunity.setOnClickListener {
            findNavController().navigate(R.id.action_dashboardFragment_to_communityFragment)
        }
        
        // 刷新按钮
        binding.btnRefresh.setOnClickListener {
            viewModel.refresh()
        }
    }
    
    /**
     * 设置观察者
     */
    private fun setupObservers() {
        // 观察钱包状态
        viewModel.walletStatus.observe(viewLifecycleOwner) { hasWallet ->
            binding.tvWalletStatus.text = if (hasWallet) {
                getString(R.string.wallet_connected)
            } else {
                getString(R.string.wallet_not_connected)
            }
        }
        
        // 观察代币余额
        viewModel.tokenBalance.observe(viewLifecycleOwner) { balance ->
            binding.tvTokenBalance.text = formatAriaTokenAmount(balance)
        }
        
        // 观察数据收集状态
        viewModel.dataCollectionEnabled.observe(viewLifecycleOwner) { isEnabled ->
            binding.tvDataStatus.text = if (isEnabled) {
                getString(R.string.data_collection_enabled)
            } else {
                getString(R.string.data_collection_disabled)
            }
        }
        
        // 观察收集的数据计数
        viewModel.collectedDataCount.observe(viewLifecycleOwner) { count ->
            binding.tvDataCount.text = getString(R.string.data_count_format, count)
        }
        
        // 观察助手对话计数
        viewModel.conversationCount.observe(viewLifecycleOwner) { count ->
            binding.tvAssistantCount.text = getString(R.string.conversation_count_format, count)
        }
        
        // 观察总奖励
        viewModel.totalRewards.observe(viewLifecycleOwner) { rewards ->
            binding.tvTotalRewards.text = formatAriaTokenAmount(rewards)
        }
        
        // 观察用户级别
        viewModel.userLevel.observe(viewLifecycleOwner) { level ->
            binding.tvUserLevel.text = getString(R.string.user_level_format, level)
        }
    }
    
    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
} 
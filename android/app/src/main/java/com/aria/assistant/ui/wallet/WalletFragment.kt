package com.aria.assistant.ui.wallet

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.lifecycleScope
import com.aria.assistant.R
import com.aria.assistant.databinding.FragmentWalletBinding
import kotlinx.coroutines.launch

class WalletFragment : Fragment() {

    private var _binding: FragmentWalletBinding? = null
    private val binding get() = _binding!!
    
    private lateinit var viewModel: WalletViewModel
    
    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        _binding = FragmentWalletBinding.inflate(inflater, container, false)
        return binding.root
    }
    
    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)
        
        // 初始化ViewModel
        viewModel = ViewModelProvider(this).get(WalletViewModel::class.java)
        
        setupUI()
        setupObservers()
        
        // 加载钱包状态
        viewModel.checkWalletStatus()
    }
    
    private fun setupUI() {
        // 设置创建钱包按钮点击事件
        binding.createWalletButton.setOnClickListener {
            if (!viewModel.hasWallet()) {
                createWallet()
            } else {
                Toast.makeText(context, "钱包已存在", Toast.LENGTH_SHORT).show()
            }
        }
        
        // 设置导入钱包按钮点击事件
        binding.importWalletButton.setOnClickListener {
            // 在实际应用中，这里应该打开一个对话框让用户输入助记词
            // 简化版只是显示一个提示
            Toast.makeText(context, "请在设置中完成钱包导入", Toast.LENGTH_SHORT).show()
        }
        
        // 设置发送代币按钮点击事件
        binding.sendTokensButton.setOnClickListener {
            // 在实际应用中，这里应该打开一个对话框让用户输入接收地址和金额
            Toast.makeText(context, "发送代币功能即将上线", Toast.LENGTH_SHORT).show()
        }
        
        // 设置接收代币按钮点击事件
        binding.receiveTokensButton.setOnClickListener {
            // 在实际应用中，这里应该显示当前钱包地址的二维码
            val address = viewModel.getWalletAddress()
            if (address != null) {
                Toast.makeText(context, "您的钱包地址: $address", Toast.LENGTH_LONG).show()
            } else {
                Toast.makeText(context, "请先创建钱包", Toast.LENGTH_SHORT).show()
            }
        }
    }
    
    private fun setupObservers() {
        // 观察钱包状态
        viewModel.walletStatus.observe(viewLifecycleOwner) { hasWallet ->
            updateWalletUI(hasWallet)
        }
        
        // 观察钱包余额
        viewModel.walletBalance.observe(viewLifecycleOwner) { balance ->
            binding.balanceTextView.text = "$balance ARI"
        }
        
        // 观察错误消息
        viewModel.errorMessage.observe(viewLifecycleOwner) { errorMessage ->
            if (errorMessage.isNotEmpty()) {
                Toast.makeText(context, errorMessage, Toast.LENGTH_SHORT).show()
            }
        }
    }
    
    private fun updateWalletUI(hasWallet: Boolean) {
        if (hasWallet) {
            // 显示钱包信息区域
            binding.walletInfoLayout.visibility = View.VISIBLE
            binding.walletSetupLayout.visibility = View.GONE
            
            // 刷新钱包余额
            lifecycleScope.launch {
                viewModel.refreshBalance()
            }
            
            // 显示钱包地址
            val address = viewModel.getWalletAddress()
            binding.addressTextView.text = address?.let { 
                val formatted = if (it.length > 20) {
                    it.substring(0, 10) + "..." + it.substring(it.length - 10)
                } else {
                    it
                }
                "地址: $formatted"
            } ?: "地址: 未知"
        } else {
            // 显示钱包设置区域
            binding.walletInfoLayout.visibility = View.GONE
            binding.walletSetupLayout.visibility = View.VISIBLE
        }
    }
    
    private fun createWallet() {
        lifecycleScope.launch {
            try {
                val mnemonic = viewModel.createWallet()
                
                // 在实际应用中，这里应该显示助记词并提醒用户备份
                // 简化版只是显示一个提示
                Toast.makeText(context, "钱包创建成功！请妥善保管您的助记词", Toast.LENGTH_LONG).show()
                
            } catch (e: Exception) {
                Toast.makeText(context, "创建钱包失败: ${e.message}", Toast.LENGTH_SHORT).show()
            }
        }
    }
    
    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
} 
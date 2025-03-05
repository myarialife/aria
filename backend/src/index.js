const express = require('express');
const cors = require('cors');
const morgan = require('morgan');
const helmet = require('helmet');
const compression = require('compression');
const mongoose = require('mongoose');
const dotenv = require('dotenv');
const path = require('path');

// 加载环境变量
dotenv.config();

// 导入路由
const authRoutes = require('./api/routes/auth');
const userRoutes = require('./api/routes/user');
const dataRoutes = require('./api/routes/data');
const assistantRoutes = require('./api/routes/assistant');
const walletRoutes = require('./api/routes/wallet');

// 初始化应用
const app = express();
const PORT = process.env.PORT || 3000;

// 中间件
app.use(helmet()); // 安全头
app.use(compression()); // 压缩响应
app.use(morgan('combined')); // 日志
app.use(cors()); // 跨域支持
app.use(express.json({ limit: '10mb' })); // JSON解析
app.use(express.urlencoded({ extended: true, limit: '10mb' }));

// API路由
app.use('/api/auth', authRoutes);
app.use('/api/user', userRoutes);
app.use('/api/data', dataRoutes);
app.use('/api/assistant', assistantRoutes);
app.use('/api/wallet', walletRoutes);

// 健康检查端点
app.get('/health', (req, res) => {
  res.status(200).json({ status: 'ok', timestamp: new Date().toISOString() });
});

// 错误处理中间件
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(err.statusCode || 500).json({
    error: {
      message: err.message || 'Internal Server Error',
      status: err.statusCode || 500
    }
  });
});

// 404处理
app.use((req, res) => {
  res.status(404).json({
    error: {
      message: 'Resource not found',
      status: 404
    }
  });
});

// 连接到MongoDB
mongoose
  .connect(process.env.MONGODB_URI || 'mongodb://localhost:27017/aria', {
    useNewUrlParser: true,
    useUnifiedTopology: true
  })
  .then(() => {
    console.log('Connected to MongoDB');
    
    // 启动服务器
    app.listen(PORT, () => {
      console.log(`Server running on port ${PORT}`);
    });
  })
  .catch(err => {
    console.error('Could not connect to MongoDB', err);
    process.exit(1);
  });

// 处理进程终止信号
process.on('SIGINT', () => {
  mongoose.connection.close(() => {
    console.log('MongoDB connection closed');
    process.exit(0);
  });
});
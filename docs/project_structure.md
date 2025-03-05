# ARIA项目目录结构

本文档描述了ARIA项目的目录结构和各个组件的用途。

## 顶层目录结构

```
aria/
├── android/              # Android客户端应用
├── aria-token/           # Solana代币智能合约
├── assets/               # 项目资源文件（图片、徽标等）
├── backend/              # 后端服务API
├── docs/                 # 项目文档
├── keys/                 # 密钥目录（不纳入版本控制）
├── scripts/              # 项目脚本（部署、构建等）
└── README.md             # 项目概述
```

## Android 客户端

```
android/
└── app/
    └── src/
        └── main/
            ├── java/com/aria/assistant/   # Java/Kotlin源代码
            │   ├── AriaApplication.kt     # 应用入口类
            │   ├── blockchain/            # 区块链交互组件
            │   ├── data/                  # 数据层
            │   │   ├── dao/               # 数据访问对象
            │   │   ├── entities/          # 数据实体类
            │   │   └── repositories/      # 数据仓库
            │   ├── di/                    # 依赖注入
            │   ├── models/                # 模型类
            │   ├── network/               # 网络请求
            │   │   └── models/            # API请求/响应模型
            │   ├── services/              # 后台服务
            │   ├── ui/                    # 用户界面
            │   │   ├── assistant/         # AI助手界面
            │   │   ├── dashboard/         # 主控面板界面
            │   │   ├── data/              # 数据管理界面
            │   │   └── wallet/            # 钱包界面
            │   └── utils/                 # 工具类
            └── res/                       # Android资源文件
                ├── drawable/              # 图像资源
                ├── layout/                # 布局文件
                ├── menu/                  # 菜单文件
                ├── navigation/            # 导航图
                └── values/                # 字符串、颜色等资源
```

## Solana 代币智能合约

```
aria-token/
├── keys/                 # 密钥目录（不纳入版本控制）
├── scripts/              # 部署脚本
└── src/                  # 合约源代码
    ├── error.rs          # 错误处理
    ├── events.rs         # 事件定义
    ├── instruction.rs    # 指令定义
    ├── lib.rs            # 主合约逻辑
    ├── metadata.rs       # 代币元数据
    ├── pump_fun.rs       # pump.fun集成
    ├── security.rs       # 安全管理
    ├── token_economy.rs  # 代币经济机制
    └── upgradable.rs     # 合约升级机制
```

## 后端服务

```
backend/
└── src/
    ├── api/                # API端点
    │   ├── controllers/    # 控制器
    │   └── routes/         # 路由定义
    ├── config/             # 配置文件
    ├── models/             # 数据模型
    ├── services/           # 业务服务
    ├── utils/              # 工具函数
    └── index.js            # 后端入口文件
```

## 文档目录

```
docs/
├── api/                  # API文档
├── architecture/         # 架构设计文档
├── images/               # 文档中使用的图片
└── project_structure.md  # 项目结构文档（本文件）
```

## 脚本目录

```
scripts/
├── android/              # Android客户端构建脚本
├── backend/              # 后端服务脚本
└── deployment/           # 部署脚本
```

## 资源目录

```
assets/
├── aria-logo.svg         # ARIA标志
└── other-resources...    # 其他项目资源
``` 
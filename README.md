# Zero2Prod
> 一个使用 Rust 构建的 Web 应用项目，采用模块化架构设计。

## 项目结构
```tree
zero2prod/
├── api/                           # API 接口层
│   └── src/
│       ├── lib.rs
│       └── sub_api.rs             # 订阅相关 API
├── app/                           # 应用入口
│   └── src/
│       └── main.rs
├── common/                        # 公共模块
│   └── src/
│       ├── error.rs               # 错误处理
│       ├── macros.rs              # 宏定义
│       └── model/                 # 数据模型
│           ├── entity/            # 数据库实体
│           └── subscription_dto/  # DTO 对象
├── database/                      # 数据库访问层
│   └── src/
│       ├── crud/                  # CRUD 操作
│── migration/                     # 数据库迁移
│   └── src/
│       └── m20250116_*.rs         # 迁移文件
├── service/                       # 业务逻辑层
├── setting/                       # 配置管理
└── utils/                         # 工具库
```

## 主要功能模块

### API 层 (api)
- 处理 HTTP 请求响应
= 实现 RESTful API 接口
- 包含订阅相关的 API 实现

### 数据库层 (database)
- 实现数据库 CRUD 操作
- 处理数据持久化

### 公共模块 (common)
- 定义错误处理
- 包含通用宏
- 定义 DTO 和实体对象
- 提供共享功能

### 数据库迁移 (migration)
提供数据库迁移工具,支持以下操作:
- 生成迁移文件
- 应用/回滚迁移
- 查看迁移状态
- 重置数据库

#### 命令
```sh
# Generate a new migration file
cargo run --bin migration -- generate MIGRATION_NAME

# Apply all pending migrations
cargo run --bin migration
cargo run --bin migration -- up

# Apply first 10 pending migrations
cargo run --bin migration -- up -n 10

# Rollback last applied migrations
cargo run --bin migration -- down

# Rollback last 10 applied migrations
cargo run --bin migration -- down -n 10

# Drop all tables from the database, then reapply all migrations
cargo run --bin migration -- fresh

# Rollback all applied migrations, then reapply all migrations
cargo run --bin migration -- refresh

# Rollback all applied migrations
cargo run --bin migration -- reset

# Check the status of all migrations
cargo run --bin migration -- status
```

## 技术栈
- [axum]()
- [sea-orm]()


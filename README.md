# Zero to Production in Rust with Axum

## 项目结构
- [`api`]("/api") 路由
- [`app`]("/app") 主程序
- [`common`]("/common") 
  - [`dto`](/common/src/dto) DTO
  - [`error.rs`]("/common/src/error.rs") 自定义错误
  - [`response`](/common/src/response.rs) 自定义响应
- [`middleware`]("/middleware") 中间件
- [`migrations`]("/middleware") 数据库迁移
- [`service`]("/service") handler
- [`utils`]("/utils") 辅助函数
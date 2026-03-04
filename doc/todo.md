# 当前状况

- 项目初始化,启动web服务(Hello, world!)
- 设置共享数据状态,添加和查询数据接口
- 分离main.rs,拆分多个结构
- 集成tracing,输入日志到terminal和文件
- 集成config,从配置文件加载配置项
- 当前代码量 188, ^_^

# TODO

## 一、基础设施

1. 连接pg数据库（sqlx + 连接池 + 迁移 migrate）
2. 连接redis缓存（deadpool-redis / fred，封装 get/set/del/expire）
3. 容器化部署（Dockerfile 多阶段构建 + docker-compose 编排 pg/redis/app）
4. 统一配置管理（config crate，支持 dev/prod 多环境 .toml 切换）
5. 统一错误处理（自定义 AppError，实现 IntoResponse，返回标准 JSON 错误体）
6. 统一响应体封装（ApiResponse<T>，包含 code / message / data 字段）

## 二、安全与认证

7. JWT 登录鉴权（jsonwebtoken，access_token + refresh_token，中间件拦截）
8. 密码加密（argon2 / bcrypt 哈希存储，禁止明文）
9. RBAC 权限管理（用户 → 角色 → 权限，接口级鉴权中间件）
10. 接口限流（tower-governor，防暴力破解与 DDoS）
11. 幂等（请求唯一 key 存 redis，防重复提交）
12. 请求参数校验（validator crate，DTO 字段注解式校验）

## 三、核心业务功能

13. 简单管理系统雏形（用户/角色/菜单 CRUD，前端用 AI 生成）
14. 文件管理（本地上传 + MinIO/S3 对象存储，分片上传，文件元信息入库）
15. 审计日志（中间件记录操作者/接口/入参/耗时/结果，异步落库）
16. 分页查询封装（通用 PageRequest / PageResponse，配合 sqlx 生成 LIMIT/OFFSET）
17. 定时任务（tokio-cron-scheduler，如定时清理日志、刷新 token 黑名单）

## 四、高并发与异步

18. 异步任务处理（tokio + async-channel 内存队列，或接入 RabbitMQ/Redis Stream）
19. 高并发压测与优化（wrk/k6 压测，分析锁竞争、连接池瓶颈，改用 DashMap 替换 Mutex）
20. WebSocket 聊天室（axum WebSocket，broadcast channel 广播，在线用户管理）

## 五、可观测性

21. 结构化日志（tracing + JSON 格式输出，日志级别动态调整）
22. 健康检查接口（`GET /health` 返回服务/数据库/缓存状态，供 k8s 探针使用）
23. 链路追踪（opentelemetry + Jaeger，分布式请求链路可视化）
24. 指标监控（prometheus metrics 接口，结合 Grafana 展示 QPS/延迟/错误率）

## 六、进阶方向

25. API 文档自动生成（utoipa + Swagger UI，路由注解生成 OpenAPI 3.0）
26. 多租户管理（租户 ID 隔离，数据行级过滤或 schema 级隔离）
27. AI 集成（调用 OpenAI / Ollama API，封装对话接口，流式 SSE 响应）
28. API 网关 / 微服务架构（服务注册发现、负载均衡、跨服务认证）
29. 邮件 / 短信通知（lettre 发邮件，接入短信服务商 SDK）
30. 国际化（i18n 多语言错误消息，Accept-Language 头解析）

---

## 总结思路

- **第一阶段**：打好基础设施（数据库、缓存、错误处理、配置、容器化）。
- **第二阶段**：核心业务（认证鉴权、RBAC、CRUD、文件、分页、幂等）。
- **第三阶段**：稳定性与性能（异步任务、高并发、可观测性）。
- **第四阶段**：进阶拓展（AI 集成、微服务、全栈、多租户）。
 
> 后端学习
> 总结思路：
> 先打好 基础设施能力（数据库、缓存、异步、高并发、容器化）。
> 再做 核心业务功能（管理系统、RBAC、文件管理）。
> 最后可以尝试 AI 集成/微服务/全栈前端，形成完整的 Rust 项目经验。
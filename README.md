# 博物馆藏品管理系统

基于 Rust/Axum/Tera/SQLite 构建的博物馆藏品管理系统，提供文物入库建档、养护记录、临展调配、文物修复台账及参观预约等功能。

## 技术栈

- **后端框架**: Axum 0.7
- **模板引擎**: Tera
- **数据库**: SQLite (via rusqlite)
- **会话管理**: Cookie + 内存会话存储
- **密码加密**: bcrypt
- **静态文件**: tower-http ServeDir

## 功能模块

### 文物入库建档
- 文物信息增删改查
- 支持分类（陶瓷、青铜器、书画、玉器、丝织品等）
- 状态管理（在库、在展、养护中、修复中）
- 入库日期记录

### 养护记录
- 关联文物实体
- 记录养护方法、执行人、时间
- 支持备注说明

### 临展调配
- 展览创建与管理
- 多文物关联参展
- 展览状态跟踪（筹备中、进行中、已结束）
- 策展人信息

### 文物修复台账
- 修复人员和方法记录
- 费用统计
- 修复状态跟踪（进行中、已完成、暂停）
- 关联文物详情

### 参观预约
- 访客在线预约
- 选择参观展览
- 管理员确认/取消预约
- 预约状态管理

### 用户认证
- 用户注册与登录
- 角色权限（admin/staff）
- Cookie 会话保持

### 管理控制台
- 文物总数统计
- 在展/修复中文物数量
- 展览、养护、修复、预约统计
- 最近入库文物和预约列表

## 项目结构

```
repo/
├── Cargo.toml
├── src/
│   ├── main.rs              # 程序入口
│   ├── config/              # 配置模块
│   │   ├── app.rs           # 应用常量
│   │   ├── database.rs      # 数据库初始化
│   │   └── seed.rs          # 种子数据
│   ├── handlers/            # 请求处理器
│   │   ├── home.rs          # 首页与控制台
│   │   ├── auth.rs          # 认证处理
│   │   ├── artifact.rs      # 文物管理
│   │   ├── conservation.rs  # 养护记录
│   │   ├── exhibition.rs    # 展览管理
│   │   ├── restoration.rs   # 修复台账
│   │   └── reservation.rs   # 参观预约
│   ├── middleware/           # 中间件
│   │   └── auth.rs          # 认证中间件
│   ├── models/              # 数据模型
│   └── services/            # 业务逻辑
├── templates/               # Tera 模板
│   ├── base.html
│   ├── index.html
│   ├── partials/            # 公共组件
│   ├── auth/                # 认证页面
│   ├── artifacts/           # 文物页面
│   ├── conservations/       # 养护页面
│   ├── exhibitions/         # 展览页面
│   ├── restorations/        # 修复页面
│   ├── reservations/        # 预约页面
│   └── dashboard/           # 控制台
└── static/                  # 静态资源
    ├── css/style.css
    └── js/main.js
```

## 快速开始

### 环境要求

- Rust 1.77+
- SQLite3

### 编译运行

```bash
cd repo
cargo build --release
./target/release/museum-collection
```

服务器将在 `http://0.0.0.0:3000` 启动。

### Docker 部署

```bash
cd museum-collection
docker build -t museum-collection .
docker run -p 3000:3000 -p 2222:22 museum-collection
```

## 默认账号

| 用户名 | 密码 | 角色 | 显示名称 |
|--------|------|------|----------|
| admin | admin123 | 管理员 | 系统管理员 |
| zhangsan | staff123 | 工作人员 | 张三 |
| lisi | staff123 | 工作人员 | 李四 |

## 种子数据

系统首次启动时会自动插入种子数据，包括：

- 3 个用户账号
- 10 件文物（涵盖陶瓷、青铜器、书画、玉器、丝织品等分类）
- 3 条养护记录
- 3 个展览（含展品关联）
- 2 条修复记录
- 5 条参观预约

## 路由说明

| 路由 | 方法 | 说明 |
|------|------|------|
| `/` | GET | 首页 |
| `/dashboard` | GET | 管理控制台 |
| `/auth/login` | GET/POST | 登录 |
| `/auth/register` | GET/POST | 注册 |
| `/auth/logout` | GET | 退出 |
| `/artifacts` | GET | 文物列表 |
| `/artifacts/create` | GET/POST | 新增文物 |
| `/artifacts/:id` | GET | 文物详情 |
| `/artifacts/:id/edit` | GET/POST | 编辑文物 |
| `/artifacts/:id/delete` | POST | 删除文物 |
| `/conservations` | GET | 养护记录列表 |
| `/conservations/create` | GET/POST | 新增养护 |
| `/conservations/:id` | GET | 养护详情 |
| `/exhibitions` | GET | 展览列表 |
| `/exhibitions/create` | GET/POST | 新增展览 |
| `/exhibitions/:id` | GET | 展览详情 |
| `/restorations` | GET | 修复台账列表 |
| `/restorations/create` | GET/POST | 新增修复 |
| `/restorations/:id` | GET | 修复详情 |
| `/reservations` | GET | 预约列表 |
| `/reservations/create` | GET/POST | 新增预约 |
| `/reservations/:id/confirm` | GET | 确认预约 |
| `/reservations/:id/cancel` | GET | 取消预约 |

## 数据库表结构

### users - 用户表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| username | TEXT | 用户名（唯一） |
| password_hash | TEXT | 密码哈希 |
| display_name | TEXT | 显示名称 |
| role | TEXT | 角色（admin/staff） |
| created_at | TEXT | 创建时间 |

### artifacts - 文物表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| name | TEXT | 文物名称 |
| category | TEXT | 分类 |
| era | TEXT | 年代 |
| material | TEXT | 材质 |
| dimensions | TEXT | 尺寸 |
| origin | TEXT | 来源 |
| description | TEXT | 描述 |
| status | TEXT | 状态 |
| entry_date | TEXT | 入库日期 |
| created_at | TEXT | 创建时间 |

### conservations - 养护记录表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| artifact_id | INTEGER | 关联文物ID |
| method | TEXT | 养护方法 |
| performer | TEXT | 执行人 |
| start_date | TEXT | 开始日期 |
| end_date | TEXT | 结束日期 |
| notes | TEXT | 备注 |
| created_at | TEXT | 创建时间 |

### exhibitions - 展览表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| name | TEXT | 展览名称 |
| venue | TEXT | 展馆 |
| start_date | TEXT | 开始日期 |
| end_date | TEXT | 结束日期 |
| curator | TEXT | 策展人 |
| description | TEXT | 描述 |
| status | TEXT | 状态 |
| created_at | TEXT | 创建时间 |

### exhibition_artifacts - 展览文物关联表
| 字段 | 类型 | 说明 |
|------|------|------|
| exhibition_id | INTEGER | 展览ID |
| artifact_id | INTEGER | 文物ID |

### restorations - 修复记录表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| artifact_id | INTEGER | 关联文物ID |
| restorer | TEXT | 修复人 |
| method | TEXT | 修复方法 |
| start_date | TEXT | 开始日期 |
| end_date | TEXT | 结束日期 |
| cost | REAL | 费用 |
| description | TEXT | 描述 |
| status | TEXT | 状态 |
| created_at | TEXT | 创建时间 |

### reservations - 预约表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| visitor_name | TEXT | 访客姓名 |
| phone | TEXT | 联系电话 |
| visit_date | TEXT | 参观日期 |
| visitor_count | INTEGER | 参观人数 |
| exhibition_id | INTEGER | 关联展览ID（可空） |
| status | TEXT | 状态 |
| created_at | TEXT | 创建时间 |

## 许可证

MIT License

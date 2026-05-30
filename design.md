# Windows桌面宠物项目设计文档

## 1. 概述
参考QQ宠物设计一个Windows桌面宠物应用，基于Tauri v2 + React + TypeScript + Rust技术栈。项目目标是创建一个可扩展的架构，支持后续添加学习提醒、系统监控、AI行为、对话系统等功能。

## 2. 技术栈
- **前端**：React + TypeScript + Vite
- **后端**：Rust + Tauri v2
- **状态管理**：Zustand（前端） + 自定义状态机（后端）
- **构建工具**：Vite + Cargo

## 3. 项目目录结构
Code_window_pet/
├── src-tauri/                     # Tauri后端（Rust）
│   ├── src/
│   │   ├── main.rs               # 入口文件
│   │   ├── lib.rs                # 库根
│   │   ├── commands/             # Tauri命令处理
│   │   │   ├── mod.rs
│   │   │   ├── pet_commands.rs   # 宠物相关命令
│   │   │   └── system_commands.rs # 系统相关命令
│   │   ├── modules/              # 核心业务模块
│   │   │   ├── mod.rs
│   │   │   ├── pet/              # 宠物核心模块
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entity.rs     # 宠物实体定义
│   │   │   │   └── attributes.rs # 属性管理
│   │   │   ├── state_machine/    # 状态机模块
│   │   │   │   ├── mod.rs
│   │   │   │   ├── states.rs     # 状态定义
│   │   │   │   ├── events.rs     # 事件定义
│   │   │   │   └── transitions.rs # 转换规则
│   │   │   ├── ai/               # AI行为模块
│   │   │   │   ├── mod.rs
│   │   │   │   ├── behavior.rs   # 行为逻辑
│   │   │   │   └── decision.rs   # 决策引擎
│   │   │   ├── monitor/          # 系统监控模块
│   │   │   │   ├── mod.rs
│   │   │   │   ├── system.rs     # 系统信息采集
│   │   │   │   └── analysis.rs   # 数据分析
│   │   │   ├── reminder/         # 学习提醒模块
│   │   │   │   ├── mod.rs
│   │   │   │   ├── scheduler.rs  # 调度器
│   │   │   │   └── notifier.rs   # 通知器
│   │   │   └── conversation/     # 对话系统模块
│   │   │       ├── mod.rs
│   │   │       ├── parser.rs     # 对话解析
│   │   │       └── generator.rs  # 回复生成
│   │   ├── models/               # 数据模型
│   │   │   ├── mod.rs
│   │   │   ├── pet_model.rs      # 宠物模型
│   │   │   └── config_model.rs   # 配置模型
│   │   ├── services/             # 服务层
│   │   │   ├── mod.rs
│   │   │   ├── pet_service.rs    # 宠物服务
│   │   │   └── event_service.rs  # 事件服务
│   │   ├── event_bus/            # 事件总线
│   │   │   ├── mod.rs
│   │   │   └── bus.rs            # 事件总线实现
│   │   └── utils/                # 工具函数
│   │       ├── mod.rs
│   │       └── helpers.rs
│   ├── Cargo.toml                # Rust依赖配置
│   └── tauri.conf.json           # Tauri配置文件
├── src/                           # 前端（React + TypeScript）
│   ├── main.tsx                  # 应用入口
│   ├── App.tsx                   # 根组件
│   ├── components/               # UI组件
│   │   ├── Pet/                  # 宠物组件
│   │   │   ├── PetContainer.tsx  # 容器组件
│   │   │   ├── PetSprite.tsx     # 精灵渲染
│   │   │   └── PetAnimation.tsx  # 动画控制
│   │   ├── UI/                   # 通用UI组件
│   │   │   ├── Button.tsx
│   │   │   ├── Panel.tsx
│   │   │   └── Tooltip.tsx
│   │   └── Panels/               # 功能面板
│   │       ├── StatusPanel.tsx   # 状态面板
│   │       ├── DialogPanel.tsx   # 对话面板
│   │       ├── ReminderPanel.tsx # 提醒面板
│   │       └── SettingsPanel.tsx # 设置面板
│   ├── hooks/                    # 自定义Hooks
│   │   ├── usePet.ts            # 宠物状态Hook
│   │   ├── useTauriEvent.ts     # Tauri事件Hook
│   │   └── useSystemInfo.ts     # 系统信息Hook
│   ├── stores/                   # 状态管理
│   │   ├── petStore.ts          # 宠物状态Store
│   │   ├── uiStore.ts           # UI状态Store
│   │   └── configStore.ts       # 配置Store
│   ├── services/                 # 前端服务
│   │   ├── tauriBridge.ts       # Tauri桥接服务
│   │   └── api.ts               # API封装
│   ├── types/                    # TypeScript类型定义
│   │   ├── pet.ts               # 宠物相关类型
│   │   ├── events.ts            # 事件类型
│   │   └── tauri.d.ts           # Tauri类型声明
│   ├── utils/                    # 工具函数
│   │   ├── animation.ts         # 动画工具
│   │   └── constants.ts         # 常量定义
│   └── styles/                   # 样式文件
│       ├── global.css           # 全局样式
│       └── components/          # 组件样式
├── package.json                  # 前端依赖配置
├── tsconfig.json                 # TypeScript配置
├── vite.config.ts                # Vite配置
└── DESIGN.md                     # 本文档

## 4. 模块划分

### 4.1 后端模块（Rust）
1. **宠物核心模块**：管理宠物实体、属性、外观和动画状态
2. **状态机模块**：定义宠物状态、事件和转换规则，驱动状态变化
3. **AI行为模块**：控制宠物的自主行为、随机移动和反应
4. **系统监控模块**：监控系统资源（CPU、内存、网络）并映射到宠物状态
5. **提醒模块**：管理学习提醒、定时任务和通知
6. **对话系统模块**：处理宠物与用户的对话，支持AI生成回复
7. **事件总线**：模块间通信的事件系统，实现松耦合
8. **Tauri命令层**：暴露给前端的API接口
9. **服务层**：封装业务逻辑，提供统一接口

### 4.2 前端模块（React）
1. **宠物渲染组件**：负责宠物精灵的渲染和动画
2. **UI组件库**：通用按钮、面板、提示框等
3. **功能面板**：状态显示、对话、提醒、设置等
4. **状态管理**：使用Zustand管理前端状态，与后端同步
5. **Tauri桥接服务**：封装Tauri命令调用，提供类型安全API
6. **自定义Hooks**：封装状态逻辑和副作用

## 5. 状态机设计

### 5.1 状态枚举
```rust
enum PetState {
    Idle,       // 空闲状态
    Happy,      // 开心状态
    Hungry,     // 饥饿状态
    Sleepy,     // 困倦状态
    Playing,    // 玩耍状态
    Studying,   // 学习状态
    Monitoring, // 监控状态
    Talking,    // 对话状态
}
5.2 事件枚举
enum PetEvent {
    TimeTick,           // 时间流逝事件
    UserInteract(UserAction), // 用户交互事件
    SystemEvent(SystemInfo), // 系统事件
    ReminderEvent(Reminder), // 提醒事件
    AiDecision(AiAction),    // AI决策事件
}

enum UserAction {
    Feed,       // 喂食
    Play,       // 玩耍
    Pet,        // 抚摸
    Talk,       // 对话
    Command(String), // 命令
}
5.3 状态转换规则
- Idle + TimeTick → 可能转换为Hungry或Sleepy（基于内部计时器）
- Hungry + UserInteract(Feed) → Happy
- Happy + TimeTick → Idle
- Idle + UserInteract(Play) → Playing
- Playing + TimeTick → Idle
- Idle + ReminderEvent → Studying
- Studying + TimeTick → Idle
- Idle + SystemEvent → Monitoring
- Monitoring + TimeTick → Idle
- Idle + UserInteract(Talk) → Talking
- Talking + TimeTick → Idle
5.4 状态属性
每个状态可以关联属性值（0-100）：
- 饥饿度（Hunger）
- 心情值（Mood）
- 能量值（Energy）
- 清洁度（Cleanliness）
6. 数据流设计
6.1 整体数据流
前端UI ↔ Tauri Bridge ↔ Tauri Commands ↔ 业务模块 ↔ 状态机
   ↑                    ↓
   └─────── 事件系统 ←───┘
6.2 详细流程
1. 初始化流程：
- 前端加载 → 调用get_pet_status命令 → 后端返回宠物状态 → 前端渲染
2. 用户交互流程：
- 用户操作 → 前端调用feed_pet等命令 → 后端处理 → 更新状态机 → 发出pet_status_update事件 → 前端更新UI
3. 自动更新流程：
- 后端定期运行tick函数 → 更新状态机（如饥饿度增加） → 发出状态更新事件 → 前端接收更新
4. 系统监控流程：
- 后端监听系统事件（CPU、内存） → 触发状态转换 → 发出事件 → 前端显示
6.3 通信协议
- 命令调用：前端通过Tauri命令调用后端，同步请求-响应
- 事件订阅：前端订阅后端事件，异步推送更新
- 状态同步：使用事件驱动减少轮询，提高实时性
7. 未来扩展方案
7.1 插件系统设计
trait Plugin {
    fn name(&self) -> &str;
    fn initialize(&mut self, app: &mut App) -> Result<(), Error>;
    fn on_event(&mut self, event: &Event) -> Result<(), Error>;
    fn commands(&self) -> Vec<Command>;
}
7.2 扩展点
1. 命令扩展：通过插件注册新的Tauri命令
2. 事件扩展：监听和发出自定义事件
3. 状态扩展：添加新的宠物状态和转换规则
4. UI扩展：动态加载新的前端组件
5. AI扩展：支持多种AI后端（本地模型、远程API）
7.3 配置驱动
- 使用配置文件（JSON/TOML）启用/禁用功能模块
- 支持运行时配置更新
- 环境变量覆盖
7.4 模块化架构
- 每个功能模块独立开发、测试
- 通过事件总线解耦
- 支持热插拔模块
8. 总结
本设计文档定义了一个可扩展的Windows桌面宠物架构，基于Tauri v2 + React + TypeScript + Rust技术栈。通过清晰的模块划分、状态机设计和数据流规划，为后续添加学习提醒、系统监控、AI行为、对话系统等功能奠定了坚实基础。插件化架构确保了项目的长期可维护性和扩展性。
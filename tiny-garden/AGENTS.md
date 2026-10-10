# 仓库协作指南

## 沟通与工作方式

- 默认使用简体中文沟通，说明、开发记录与新增用户界面文案也使用中文；代码标识符和工具命令保留原有形式。
- 修改前阅读相关代码和文档，沿用现有命名与结构。保持改动聚焦，不覆盖用户已有修改。
- 明确区分已实现、自动验证通过、真实窗口验收通过和规划中的功能；不要将原型或离屏截图描述为完整交付。
- 完成后简要说明改动、验证结果和未验证事项。无法运行的检查如实记录，不宣称通过。

## 项目定位与阅读入口

Tiny Garden 是原创石木风格的自由建造微缩庭院桌面编辑原型，当前先开发 Windows 鼠标版。移动端与触摸后续移植；室内、宠物、手柄、联网和商店集成不在当前首版范围。

- `README.md`：当前功能、限制、运行方式及代码阅读入口。
- `docs/桌面版开发计划.md`：开发顺序与验收基线；旧 Android 路线不作为当前首个交付关口。
- `docs/模块架构.md`：模块责任、领域不变量、编辑事务与异步正确性。
- `docs/上下文自动重构系统架构.md`、`docs/墙路自动重构实现与验收.md`：场景关系设计和当前墙路实现边界。
- `docs/增量建筑生成架构.md`、`docs/优化实现与验收.md`：增量生成、展示与性能预算。
- `docs/美术资产制作计划.md`、`art-source/README.md`：美术源文件、导出与验收。
- `docs/验证记录.md`：工程验证记录。涉及功能或验收状态变化时同步相关文档。

文档可能包含历史状态；判断已实现行为时核对当前代码及最新验收记录。

## 工程结构与依赖边界

Rust workspace 使用 edition 2024、Rust ≥ 1.95，Bevy 固定为 0.19.1。依赖以 `Cargo.toml` 和 `Cargo.lock` 为准，常规命令使用 `--locked`。

| 路径 | crate | 责任 |
|---|---|---|
| `crates/geometry` | `garden-geometry` | 无引擎矩形运算与区域切分 |
| `crates/domain` | `garden-domain` | 建筑聚合、玩家意图与合法性约束 |
| `crates/application` | `garden-application` | 编辑命令、预览、撤销重做、版本与任务票据 |
| `crates/generation` | `garden-generation` | 纯布局、开孔、屋顶、UV、网格缓冲与增量复用 |
| `crates/bevy-adapter` | `garden-bevy` | 插件、后台任务、投影通知、反馈与帧预算 |
| `crates/presentation` | `garden-presentation` | 资产合同、检查器及可选桌面渲染、输入与 UI |

- 四个纯核心 crate 不得直接或间接依赖 Bevy。`domain` 依赖 `geometry`；`application` 和 `generation` 依赖领域及几何；适配与表现位于外层。
- 领域不保存 Entity、Handle、文件路径或输入设备状态；生成不修改领域、不访问 World、不创建实体。
- 新增依赖先更新相应架构决策及 `scripts/check-boundaries.ps1` 的规则，不通过放宽检查掩盖边界问题。保持无 gilrs/手柄依赖。
- 不创建无实现的功能 crate，不引入通用 Repository 框架、DI 容器或完整 Event Sourcing。存在真实替换需求时再引入 trait。
- 游戏入口为 `crates/presentation/src/main.rs` → `app::run()`，窗口、插件和场景组装在 `src/app.rs`；`art-check` 是独立检查程序。

## 编辑、生成与展示约定

- `BuildingDraft` 是不可信输入；通过 `Building::try_new` 校验，通过公开编辑用例修改聚合，保持支撑、体块与层数不变量。
- 玩家已提交意图是权威数据；布局、Mesh、材质、Entity 和动画姿态是派生展示数据，不写回领域或当作存档。
- 拖动更新独立预览，松开只提交一次历史；取消与失败不产生历史。非法操作须向用户提供可见反馈。
- UI 必须消费 `EditFeedback::drain()`。`QueueFull` 应提示繁忙或保留事务重试，不误报成功。
- 后台任务捕获不可变输入；只应用与当前最新 `JobTicket` 完全匹配的结果。保留 Commit → Collect → Dispatch 的系统顺序及延迟命令应用边界。
- 生成使用 `GardenPlugin` 自有 `TaskPool`，避免与渲染器的 `AsyncComputeTaskPool` 争用。
- 保持稳定部件身份、精确缓存 key、局部失效和上传预算；避免为局部编辑重建整个场景或为每块砖、每个窗创建实体。
- UI、镜头与编辑共享明确的手势所有权；拾取匹配当前可见网格，失焦、离开窗口及取消不得恢复旧拖动。

## 性能与功能复用

- 实现功能时将高性能作为设计要求：明确数据规模、算法复杂度和每帧工作量，优先采用局部更新、增量计算、资源复用及有界队列，保持交互响应及时。
- 高频路径避免重复计算、不必要的分配与深拷贝、全场景扫描和主线程阻塞；缓存必须有明确的身份、依赖和失效规则，不能以过期数据换取性能。
- 性能优化通过代表性场景的测量验证，记录构建模式、场景规模、耗时及资源开销，必要时比较优化前后结果；不能仅凭使用缓存、线程池或某种架构宣称高性能。
- 新增功能前搜索已有实现。同一业务规则、算法或编辑行为应有明确的唯一实现位置，其他调用方通过公开接口复用；禁止在 UI、预览、正式生成和适配层分别复制一套相同逻辑。
- 发现重复实现时，在本次改动范围内提取到职责所属模块并统一调用，保持依赖方向；不要为消除几行表面相似代码引入无实际用途的通用框架。
- 预览与正式展示因性能预算采用不同细节时，共享核心规则和参数语义，并说明差异的设计意图及一致性约束，避免两套行为独立演化。

## 中文注释体系与设计意图

- 新增或修改的代码注释使用中文，包括 Rust 文档注释、行内注释及配置/脚本中的说明；标识符、公式、协议名称和必要的原文引用保留原样。修改既有英文注释时同步改为中文。
- 模块级文档注释（`//!`）说明模块职责、设计意图、依赖边界和关键数据流；公共类型与接口的文档注释（`///`）说明语义、使用约束、错误或失败条件，必要时给出示例。
- 关键算法、复杂状态转换、异步调度、缓存失效和性能取舍必须在相邻代码处解释“为什么这样设计”，包括维护的不变量、顺序要求及适用边界，不能只复述代码做了什么。
- 非直观的常量、阈值和预算说明单位、来源或选择依据；涉及 `unsafe` 时用中文说明安全前提及其保证方式。
- 注释按模块、接口、实现三个层次组织，各层只说明自身需要的信息，避免重复和逐行翻译。简单且自明的代码不强行添加注释。
- 跨模块设计与重要取舍记录在相关 `docs/` 文档，代码注释链接到对应说明；文档解释整体方案，邻近注释保留维护该代码必须知道的设计意图。
- 修改行为时同步更新注释和设计文档，删除失效说明；待办注释明确未完成事项及原因，不用含糊注释掩盖实现缺口。

## 常用命令与验证

在仓库根目录使用 PowerShell：

```powershell
cargo fmt --all -- --check
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
./scripts/check-boundaries.ps1
cargo run -p garden-bevy --example headless --locked
cargo run -p garden-presentation --bin art-check --locked
cargo run -p garden-presentation --features desktop --bin tiny-garden --locked
```

- 按改动范围选择验证；纯文档修改不需要重新构建。代码改动先验证相关 crate，涉及跨模块边界时运行 workspace 检查和边界脚本。
- 默认测试不要求 GPU 或 Android SDK；涉及 desktop 专属代码时补充 `cargo check -p garden-presentation --features desktop --all-targets --locked`，以及相关桌面测试或视觉验证。
- CPU 布局/网格性能基准：`cargo run -p garden-generation --example layout_benchmark --release --locked`；它不代表 GPU 帧率或手机性能。
- 离屏截图参数见 README，输出使用未存在的新路径。截图验证呈现，不替代真实鼠标、DPI、失焦和窗口事件验收。
- 当前存在 planned/blockout 资产，`art-check --strict` 可能按设计拒绝；不要为通过检查虚报资产完成度。

## 美术与生成文件

- `art-source/` 保存美术源文件与参考，`assets/` 保存运行时资产；制作与导出遵循各自说明和资产合同。
- 主体按参数生成，不将整栋 GLB 缩放作为建筑编辑实现；保留门窗适配、米制 UV、共享材质和增量资源更新。
- 不手工修改 `target/` 构建产物，不提交本机 `.idea/`、`.codex/` 配置；共享运行配置在 `.run/`。保留 `.gitignore` 指定的本地烘焙迭代文件。

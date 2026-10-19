# 缺失新模块与核心链路的关系

审查日期：2026-10-19。源码基线：`bd8f74a5cfb3c9cc279efb1abd27695bb1bfb8bf`。

结论：缺失文件中包含多处核心链路的连接模块，涉及启动、场景生产、输入/建造调度与主渲染图后半段；也有模式分支、工具和测试文件。不能把全部缺失文件等同于核心业务尚未实现。用户已确认新文件待补交，本报告描述的是当前 checkout 的源码与验收缺口。

本次以当前 Rust 声明、调用与调度注册为依据，没有使用历史完成率或函数库存推断进度。原版等价性仍须在取得实现后另行核验。

## 两个必须分开的判断

1. **编译依赖**：未受当前 cfg 排除的 `mod` 声明找不到文件，即使没有运行时调用也会阻断编译。
2. **运行时链路**：文件对应的函数是否在当前启动、每帧或某个模式/事件分支上执行，须沿调用者判断。

例如 `utils::sampling` 是目前实际编译遇到的第一个 E0583，但在本次检查的生产源码中尚未找到 `sampling::` 消费者，因此不能仅凭它阻断编译就称其为渲染核心算法。相反，`menu_background_host` 在当前 App 安装和主渲染尾部都有直接调用，不能作为无关 UI 文件跳过。

## 已确认的关键调用

| 位置 | 缺失模块与当前调用依据 | 影响及优先级 |
| --- | --- | --- |
| App 装配与运行时注册 | [run_game.rs:211](../../crates/tiny-glade/src/run_game.rs#L211) 调用 `startup_event_runtime::install`；[355](../../crates/tiny-glade/src/run_game.rs#L355) 调用 `startup_construction_runtime::install`；[883](../../crates/tiny-glade/src/run_game.rs#L883) 调用 `history_runtime::install` | 当前构建 App 的直接依赖，优先补交。仅凭调用名不能确认内部注册已完整。 |
| 会话场景与可绘制实体 | [gameplay_startup.rs:117](../../crates/tiny-glade/src/gameplay_startup.rs#L117) 起注册 walls / sky / roof；[195](../../crates/tiny-glade/src/gameplay_startup.rs#L195) 调用 `clearing::init_clearing_complete`；[213](../../crates/tiny-glade/src/gameplay_startup.rs#L213) 调用 `startup_meadow::init_meadow` | 位于真实 OnEnter(Game) 场景生产链，向后续绘制提供实体、资产与放置数据。不能以空场景代替。 |
| 动物与交互启动 | [gameplay_startup.rs:134](../../crates/tiny-glade/src/gameplay_startup.rs#L134) 调用 critters startup；[157](../../crates/tiny-glade/src/gameplay_startup.rs#L157)、[163](../../crates/tiny-glade/src/gameplay_startup.rs#L163) 加载羊资产并生成实体；[234](../../crates/tiny-glade/src/gameplay_startup.rs#L234) 创建 brush previews | 属于完整游戏场景/交互链。动物不是 Vulkan present 的必要算法，但当前启动装配会到达这些入口，不能以空成功替代。 |
| 场景外观与时间 | [run_game.rs:597](../../crates/tiny-glade/src/run_game.rs#L597) 注册 `load_glade_grading`；[606](../../crates/tiny-glade/src/run_game.rs#L606) 注册 `startup_time_of_day` | 当前会话/Startup 的直接依赖，影响真实场景参数。 |
| 鼠标深度与相机交互 | [run_game.rs:450](../../crates/tiny-glade/src/run_game.rs#L450) 初始化 `MouseDepthValue`；[483](../../crates/tiny-glade/src/run_game.rs#L483) 注册 GPU 深度读取系统 | 位于深度读取到输入/相机消费者的连接处，是正常交互链的一部分。不能固定返回零深度或假命中。 |
| 主渲染尾部 | [gameplay_startup.rs:36](../../crates/tiny-glade/src/gameplay_startup.rs#L36) 安装 menu host；[world_postprocess.rs:1141](../../crates/tiny-glade/src/world_postprocess.rs#L1141) 直接调用其 render；[1176](../../crates/tiny-glade/src/world_postprocess.rs#L1176) 在 UI-visible 分支调用 session previews | menu host 外层调用在普通主尾路径；其内部条件待实现补交后验证。session previews 是常见 UI 分支，不应笼统归为无关工具。 |
| 截图与共享纹理 | [gameplay_startup.rs:38](../../crates/tiny-glade/src/gameplay_startup.rs#L38) 安装 capture；[world_postprocess.rs:1137](../../crates/tiny-glade/src/world_postprocess.rs#L1137) 等调用 opportunity；[screenshot.rs:52](../../crates/tiny-glade/src/screenshot.rs#L52) 调用 enqueue | 有实际启动和每帧连接；是否执行捕获取决于真实请求。缺文件阻断宿主编译，不能假装已经完成 GPU 读回与释放。 |
| 世界目标与诊断分支 | [production_renderer.rs:213](../../crates/tiny-glade/src/production_renderer.rs#L213)、[220](../../crates/tiny-glade/src/production_renderer.rs#L220) 使用 `world_frame` 的颜色/深度目标描述；[430](../../crates/tiny-glade/src/production_renderer.rs#L430) 调用 minimal helper | 模块混合两种用途：目标描述被生产路径消费，minimal helper 只用于显式诊断分支。不能把 minimal clear/export 的成功当完整世界渲染成功。 |
| 存档兼容与载入 UI | [load_game.rs:497](../../crates/country-core/src/systems/save/load_game.rs#L497)、[532](../../crates/country-core/src/systems/save/load_game.rs#L532)、[1169](../../crates/country-core/src/systems/save/load_game.rs#L1169) 调用 migration；[run_game.rs:547](../../crates/tiny-glade/src/run_game.rs#L547) 注册 load UI；[614](../../crates/tiny-glade/src/run_game.rs#L614) 注册 preplaced trees | 是读档/兼容与场景恢复闭环的一部分；不能据此声称每个新建场景都执行所有迁移分支。 |

这些缺失点落在如下连接上，并不表示整段已有代码都不存在：

```text
OS 事件与现有主循环
  → 构建 App、注册事件/建造/历史系统 [缺失连接文件]
  → 会话载入、Startup / OnEnter(Game) 场景生产 [多处缺失]
  → 现有 ECS 更新与绘制列表
  → 现有世界渲染图
  → menu / previews / capture 等主尾连接 [缺失]
  → 现有 rhapsody 编译、执行与 present
```

## 不是每帧必经，但也不能删除的部分

- `camera_rig/walk`、`camera_rig/butterfly` 属于相机模式相关实现。当前 `main_camera.rs` 已直接调用 walk 的 teleport、translate 和地形约束函数；判断“非当前模式”不能解除编译依赖，也不意味着它是无关代码。
- `load_game_migration` 属于存档版本与兼容分支；`spawn_preplaced_trees` 已注册在 Game 更新中，实际工作受事件和状态影响。
- `lut_gallery`、`timelapse` 属于照片模式功能；credits / report_bug 为菜单分支。`load_ui`、`menu_button` 则与常规 UI/读档直接相关，不应把整个 toolbar 统一归为低优先级。
- `native_audio` 有 feature 条件，`audio_host` 有 Windows 条件。条件模块不等于可无条件删除：当前 gameplay startup 还存在直接引用 NativeStudio 的消费者，需要核验 feature 与平台装配是否一致。
- `human_scale_ref` 在当前 Update 注册中出现，但功能属于调试/比例参考。`spawn_human_ref`、`preplaced_autoclutter`、`glade_themes`、`prefab/runtime` 等仍需取得文件后核实内部与完整消费者，不能只看名字宣称必经。
- 三个 `cfg(test)` 文件不参与普通游戏运行，但会影响相应测试构建和回归覆盖。

## 当前缺失范围

此次扫描覆盖六个 `src` 目录中的顶层外部模块声明：utils、country-core、tiny-glade、system-critters、toolbar、rhapsody。发现 **53 处声明对应文件缺失**：普通声明 48、测试条件 3、平台/feature 条件 2。不是全仓库或完整递归模块树的业务缺口总数；某个缺失目录模块内部还可能有其他子模块。

| crate | 缺失声明数 | 解释 |
| --- | ---: | --- |
| country-core | 31 | 主要集中在 startup，也包括资源、交互、存档兼容和测试。 |
| tiny-glade | 8 | 多数为宿主装配、运行时注册、渲染尾部与捕获连接。 |
| system-critters | 6 | 动物启动/控制/鸟行为以及一项测试。 |
| toolbar | 7 | 读档 UI、控件及照片/菜单分支混合。 |
| utils | 1 | sampling，目前是实际编译最先报错的文件。 |
| rhapsody | 0 | 本次扫描未发现缺失外部模块声明；不代表引擎语义或游戏集成已全部正确。 |

完整逐项清单见 [MISSING_SOURCE_COUNTS.json](independent-validation/MISSING_SOURCE_COUNTS.json)。该清单用于定位新文件，不用于计算恢复完成率。

## 对当前推进的实际含义

首先补交 App 运行时注册、场景启动和主尾宿主文件，才能对真实游戏链进行构建与运行验收。随后按消费者核验鼠标深度、读档/历史、动物与模式分支，避免“画面出现但交互/恢复失效”。

用户已授权临时桩。这些桩只用于揭示后续编译接口和推进已有实现，调用时明确返回错误或 panic `ORIGINAL_IMPLEMENTATION_PENDING_SYNC`；不产生假资源、不返回空成功、不计入已恢复业务。必经入口仍为桩时，完整启动、世界帧或存档闭环不能报通过。用户补交真实文件后，按桩 manifest 的哈希匹配替换，再执行真实验收。

本轮已有独立 rhapsody CPU/GPU 基线验证，只覆盖其选定测试。它们不能证明缺失宿主文件已经实现，也不能代替完整游戏运行。

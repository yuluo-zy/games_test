# Tiny Glade 可并行逆向模块调查与实施交接

调查日期：2026-10-11。
用途：交给后续 agent 划分专题、继续取证和恢复 Rust 业务源码。
状态：静态模块边界调查完成；本文件没有启动 agent、分配实际负责人或实施新增模块，也不改变现有建筑主线/地形专题的优先级。

## 1. 结论与证据强度

存在适合并行推进的业务子系统。建议采用“业务专题 + 明确共享接口 + 集中整合”，不是把每个 Cargo crate 当成完全独立系统。

- 最适合先新增：相机专题；小生物或树木专题。
- 可选新增：音乐/环境音专题、昼夜/天气 CPU 控制。
- 独立性高但核心收益较小：本地化、主题配置。
- 可并行调查、需契约后集成：水域、道路、颜色/半木/灯光、存档迁移。
- 不应割裂实现：墙、屋顶、开口、门窗附着、楼梯、建筑历史。

本调查实际查看了恢复记录、B Cargo 私有依赖边、三版 PDB 源路径库存、B/C 精选函数索引和现有工作包。尚未逐一恢复候选模块的完整调用图、调度图、类型布局或行为，因此“独立性/优先级”是工程判断，不是已经证实的运行时隔离或工时预测。

证据限制：

1. `tracking/module-coverage.json` 是 PDB 源路径库存，不是已恢复功能清单，也不是进度分母。
2. `evidence/multi-version/{A,B,C}-functions.json` 是精选函数清单，非完整符号库存。当前 B 清单主要覆盖 country-core、decorator、wall-constructor、roof、tiny-glade；未命中 critters/tree/water/lang 等模块不能说明其不存在。
3. Cargo 没有直接依赖边，只能说明构建层面没有该直接依赖；通过 country-core、ECS 资源/消息仍可形成运行时耦合。
4. PDB 源路径与校验和证明构建来源线索，不代表获得作者 Rust 源文本，也不能单独证明跨版行为等价。
5. 候选模块的完整函数归属、真实 caller、输入输出与依赖边，仍由实施 agent 核查。

## 2. 实施前必读与目标边界

先读取、并检查是否已有其他会话更新：

- 两级 `AGENTS.md`。
- `PROJECT_MEMORY.md`、`TASK_BACKLOG.md`、`tracking/project.json`。
- `DEPENDENCY_FILTERING.md`、`tracking/dependency-exclusions.json`。
- `THREE_VERSION_RUST_PLAN.md`、`tracking/baselines.json`。
- 建筑相关专题：`BUILDING_SYSTEM_RECOVERY_PLAN.md`、`tracking/building-system-plan.json`。
- 地形相关专题：`TERRAIN_SYSTEM_RECOVERY_PLAN.md`、`tracking/terrain-tasks.json`。

限定研究原作发行文件/PDB/游戏业务，不转去分析 `tiny-garden` demo。当前源码、类型、状态和 CPU 算法优先，视觉/GPU最终验收后置；若 GPU 计算影响权威状态，相关契约必须提前调查。

B 是当前实现/验证基线；C v1.16.0 是计划最终目标；A 辅助追溯。不得把 B 实现或测试通过直接标记为 C 完成。实际路径、二进制身份、依赖版本以当前 tracking/证据及本机目录为准，不照抄文档中的旧 D: 路径。

依赖直接复用：std/core/alloc、Bevy、glam、fastrand、half、serde/rkyv/ron、公开 noise/math、容器、平台与音频库等。恢复游戏调用策略、参数、组合、状态与事件，不重写库内部。local/path/private 包也必须先查归属，不能自动当作待逆向业务。相机中出现 dolly 等依赖时同样按原版本 API 复用。

## 3. 推荐并行布局

建议初始保持 4 条主线，必要时增加第 5 条：

| 工作线 | 任务 | 集成边界 |
|---|---|---|
| 建筑主线 | 延续现有墙/屋顶/开口/楼梯/历史闭环 | 统一建筑 ID、owner、dirty、删除、复合事务 |
| 地形专题 | 延续真实编辑→求值/读回→dirty→消费者 | 保持现有专题写入限制，不重复实现共享 sampler |
| P01 相机 | 新增；最先推荐 | 输入、地形/碰撞查询为外部契约 |
| P02 小生物 | 新增；先行为核心 | 地形/水域/建筑内外判断为外部契约 |
| P03 音频（可选） | 先音乐与环境音 | 业务事件/参数输入，FMOD 为依赖 |

若优先补编辑内容，可用 P05 树木/森林替换 P02；P04 昼夜/天气与 P06 本地化/主题可作为下一批。此布局是建议，不表示这些工作已被领取或当前无人处理；实施前先确认实际 owner，避免重复。

## 4. 可领取工作包

### P01：相机控制（高适合度，第一推荐）

已定位私有源路径：

- `crates/country-core/src/systems/camera_rig.rs`
- `crates/country-core/src/systems/camera_rig/{normal,free,walk,butterfly,camera_body}.rs`
- `crates/country-core/src/systems/main_camera.rs`

B 静态 VA 定位锚点（仅定位，不是行为验收）：

| 函数 | B VA |
|---|---|
| `MainCamera::new` | `0x14090ec70` |
| `MainCamera::load_parameters` | `0x140911520` |
| `MainCamera::store_parameters` | `0x140911710` |
| `MainCamera::match_free_camera_to_normal` | `0x14090e6f0` |
| `MainCamera::init_walk_camera` | `0x14090e510` |
| `MainCamera::immediately_finish_interpolating` | `0x140911b50` |

恢复范围：真实模式/参数 schema、位置/朝向、focus、插值、zoom/FOV、模式切换、参数保存恢复及必要输入映射。先查普通/自由模式，再按真实依赖推进行走/蝴蝶模式，不以容易部分结束整个领取范围。

边界：不要包入全局 picking/gizmo/建筑交互；行走地形与碰撞查询需调查并显式保留接口。不得猜 delta-time、clamp、角度/坐标空间或参数默认值。

退出标准：模式切换与真实参数输入可驱动状态；store/load 与立即完成插值有原码验证；边界输入/重复切换按原行为处理；walk 查询联接未完成则明确保留未验收状态。

建议专题证据目录：`evidence/parallel-camera/`。

### P02：小生物行为（高适合度，按物种/行为拆）

私有源路径：`crates/systems/critters/src/`。

已见：`boids.rs`、`butterfly.rs`、`duck.rs`、`fish.rs`、`firefly.rs`、`wren.rs`、`wren_boids.rs`，以及 sheep 的 behavior、walk/pet animation、terrain、unstuck、input record/replay 等文件。

已见耦合证据：`ducks_react_to_being_inside_buildings.rs`、`sheep/sheep_react_to_being_inside_buildings.rs`、`sheep/sheep_terrain.rs`。不要将整个动物系统描述为无依赖模块。

B 构建私有依赖：`system-critters → country-core、utils`。

恢复范围：真实状态机、目标选择、移动/避让、群集更新、种子与随机调用顺序、行为/动画参数；完整物种范围由实际函数库存确定。优先选择一个物种形成连续链，再扩展；不把通用寻路/碰撞/数学依赖内部当作业务重写。

共享输入：地形高度、水域/建筑查询、时间步、资源与随机状态。上述是待核查接口类别，不是已确认完整 ABI。

退出标准：真实状态/输入驱动行为更新，状态转移与随机调用得到原码证明；与地形/建筑/水域消费者联接验证；历史样本中的 Sheep 状态不能单独证明整个行为系统。

建议专题证据目录：`evidence/parallel-critters/`。

### P03：音乐与音频游戏控制（高适合度）

私有路径：`country-core/src/systems/audio/`、`resources/audio_resources.rs`、`resources/audio_settings.rs`、`asset_libraries/audio_trigger_library.rs`、音频 startup/load 路径。

精选函数中已有 music player、audio update、各业务音频系统入口；这不是完整清单。

恢复范围：先 music player/system 的生命周期、暂停/恢复、切换、参数与淡入淡出；再环境音和事件→音效映射、材质/状态参数与随机选择。

边界：FMOD/libfmod、音频 DSP 直接复用。墙/屋顶/装饰交互音只消费它们的真实事件，不另建建筑状态。音频触发与 UI/replay 边界须查原作，不自行去重或补事件。

退出标准：真实控制输入驱动同一播放/参数命令序列；暂停/恢复/切换、重复事件与生命周期清理得到证明；设备回放若未验证，区分控制层与实际音频验收。

建议专题证据目录：`evidence/parallel-audio/`。

### P04：昼夜与天气 CPU 控制（较高适合度）

私有路径：`resources/{time_of_day,weather}.rs`、`systems/{time_of_day,weather_system}.rs`、`startup/{load_glade_tod_and_weather,startup_time_of_day}.rs`。

恢复范围：实际时间/天气 schema、初始化与加载、变化控制、过渡、消费者参数与事件。先查真实功能，不从任务标题猜测季节、太阳位置或所有天气能力。

边界：主题、音频与渲染输入显式联接；suncalc 等已有依赖复用。GPU光照/天气效果不混算 CPU 状态完成度。

退出标准：已证命令/时间输入产生原状态与参数；加载恢复和跨边界过渡按原码验收；实际消费者联接与视觉验收分列。

建议专题证据目录：`evidence/parallel-environment/`。

### P05：树木工具与森林管理（中高适合度，建议同一专题）

私有路径：`crates/systems/tree-tool/src/` 与 `crates/systems/forest/src/`。

入口线索：`process_tree_cmds.rs`、`replay_tree.rs`、`ui_place_tree.rs`、`ui_move_tree.rs`、`ui_delete_tree.rs`、`recalc_tree_position_on_terrain_height_change.rs`、`manage_tree_prefabs.rs`、`tree_animation.rs`。

B 私有依赖：`forest → country-core、utils`；`system-tree → country-core、rhapsody、utils`。没有直接依赖 roof/wall-constructor 不证明运行时隔离。

恢复范围：创建/移动/删除、隐藏与预放置转换、命令消费者与历史、prefab ID/参数、地形变化后的更新、必要动画规则。

边界：复用地形查询/dirty 通知、资源 schema 与共享历史事务；rhapsody 归属与渲染能力先查，不将其整体纳入重写。

退出标准：实际放置→移动→地形变化→删除/undo/redo 连续链；验证旧实例、资源和索引清理；CPU状态与渲染输出分层登记。

建议专题证据目录：`evidence/parallel-trees/`。

### P06：本地化与主题配置（独立性高，核心收益较小）

私有路径：`crates/lang-lib/src/{lib,language_library}.rs`、`crates/theme-defs/src/lib.rs`、`crates/theme-fetcher/src/{lib,checked_zip}.rs`、`crates/theme-server-proto/src/lib.rs`。

B 私有依赖：`lang-lib → paths`；`theme-defs` 无私有依赖；`theme-fetcher → theme-defs、theme-server-proto`；`theme-server-proto` 无私有依赖。公共依赖仍需按锁文件核查。

恢复范围：游戏语言资源 key、选择/回退策略、主题 schema、协议/包校验、现有本地样本加载。必须确认实际职责，不能只按包名猜功能。

边界：Serde/RON、ZIP、HTTP 等依赖复用。无需主动访问服务端；外部请求、发布、上传需另行授权。读取/解压不可信资源时使用新空目录，与脚本目录隔离；Python 使用 `-I`。

退出标准：实际语料与调用证据驱动解析/选择/错误策略；缺 key、缺包与损坏输入按原策略处理；本地恢复不等于服务端已实现。

建议专题证据目录：`evidence/parallel-localization-theme/`。

## 5. 可以并行调查但必须等待共享契约的模块

| 模块 | 可先做 | 不可自行替代的共享部分 |
|---|---|---|
| 水域 | `draw_water`、`record_water_history`、`water_audio_grid` 的 schema/命令/状态 | 地形栅格与 dirty、历史 owner、动物/环境音接口；水 shader 后置 |
| 道路 | `systems/paths/` 笔刷、曲线、宽度、端点、历史与 PathRasters 调用 | 已有墙构造对 PathRasters 的使用、台阶/开口、栅格 owner |
| 颜色/半木/灯光 | 枚举、字段写入、事件/历史 mutation | G13 的墙/屋顶/装饰宿主与重建策略；CPU字段和GPU上传分列 |
| 存档/快照/迁移 | schema、现有样本解码、版本差异与缺引用处理 | 稳定对象 ID/关系、G14复合事务、派生重建；不得伪造缺失快照 |
| 建筑资源 | 格式/ID/碰撞 metadata 与消费者证据 | 已有 G09 工作包；先确认 owner，避免重复实现 |

`system-glade-border` 有独立源路径且仅依赖 country-core/utils，可作为小型取证任务；其显示与越界反馈涉及渲染/输入消费者，不因体量小就默认是高收益主线。

## 6. 建筑主线不能割裂的原因

现有建筑证据已明确：

- 权威墙输入影响屋顶、缓存、mask、砖和删除传播。
- 门窗/楼梯对开口的贡献必须由实际墙构造消费。
- 拱与过梁会写 third 高度数组，初始化零不等于最终输入闭合。
- mixed 创建 replay 始终存在 Wall/Roof 两个 bucket，正向 Wall→Roof，undo Roof→Wall，不能 flatten 成旧 Roof-only 流。

截至调查时，最新第三阶段记录仍将 ordinary arches、真实 stairs CPU 后段、Decorator blockers、G14 typed source/bucket barrier 列为后续重点；后续 agent 应读最新记录，不能将此时间点的描述当作永久现状。

因此允许并行取证/受控子模块实现，但建筑 ID、owner、dirty、生命周期、调度与事务由整合方统一。源码变化后必须重新执行相应验证，不能继续继承旧冻结报告的当前性。

## 7. 文件与工具并行规则

以下为建议的新增专题边界，不是已分配的写入授权。领取后由整合方确认实际路径与 owner。

1. 每个专题使用独立 `evidence/parallel-<topic>/`、专题脚本目录和业务源码文件，先检查目标是否存在；已有专题文件不可直接覆盖。
2. 领取记录至少包含专题 ID、负责人、范围、写入文件、基线、共享接口和状态；本文件中所有 P01–P06 当前均为 proposed/unassigned，实际执行时重新确认。
3. 新 crate/共享基础类型/`Cargo.toml`/`Cargo.lock`/共享模块导出/顶层进度，由整合方协调修改。worker 先提交接口需求与局部实现，不各自猜公共字段。
4. 地形继续遵守其现有专用计划的更严格边界；本文件不扩大其写入范围，也不授权修改共用脚本或其他会话证据。
5. 复用持久数据库，按过程摘要/引用→所选反编译→必要指令范围查询；不反复全程序分析。共享数据库类型/注释/导入/分析写操作单独协调，别把只读查询假设推广到所有工具调用。
6. B Ghidra 数据库绑定 patched EXE；真实代码核查/oracle 使用原 `tiny-glade.rne`，记录身份。A/C 当前未绑定数据库；实施时检查最新状态。
7. 使用 procedure_info/search/caller 定位，再 batch_decompile 目标；REA/ReVa/RE-MCP 连接与真实结果如实保存。工具未连接不能宣称取得反编译。
8. 不修改原二进制，不变更 MCP/权限配置，不绕过拒绝。执行前检查工具默认输出、锁及副作用。
9. 同一 Cargo target、原码 harness 输出或批量 oracle 目录可能产生锁/覆盖；运行前分配独立输出，必要时集中串行验证。不得为并行而复制整个原作重新分析。
10. 不可信文件独立目录存放；不要从其目录运行解释器/构建工具，Python读取使用 `-I`。

## 8. 每条线的交付与验收

每条线至少交付：

1. `findings.md`：函数归属、版本、真实输入/输出、字段与控制流、调用/调度边、依赖排除。
2. 契约表：权威输入、状态 owner、读写资源、输出/事件、dirty、删除/取消策略、版本差异、未知字段。接口类别不得冒充已确认 ABI。
3. 游戏自有 Rust 模块；依赖调用与业务规则分离，不以脚本/反编译数量代替实现。
4. 原码验证报告：二进制身份、地址范围、harness/ABI、样本、比较粒度、失败与边界、源码当前性。工具成功不等于行为一致。
5. 集成说明：联接点、共享修改需求、实际消费者验收、尚未完成范围。

阶段分开登记：`located` → `analyzed` → `implemented` → `verified_local` → `system_verified`。局部算法验证不等于整个子系统，更不等于整个游戏完成。

按实际业务测试空/重复/同值、边界/极值、缺对象/缺资源、删除/取消、快速连续更新、历史与保存恢复；NaN/Inf 等仅在真实输入域及原作策略下处理，不擅自统一 sanitize。随机调用、跨架构浮点差异与失败报告必须保留。

允许用明确 fixture 验证内部规则，但完整验收必须使用实际 producer→consumer 链；不得用手填状态或 SetTerrainGrid 等注入替代真实生命周期/编辑来源。

## 9. 可直接转交给实施 agent 的任务说明

> 领取本文一个 Pxx 专题，先读项目规则与最新证据，确认当前 owner 和写入范围。以 B 为当前恢复基线、C 为最终适配目标；核查每个函数的游戏归属并复用原依赖。先建立真实接口契约与函数/调用证据，再恢复完整 Rust 业务链并原码验证。只修改已协调的专题文件，共享基础类型和 workspace 变更交给整合方；不修改其他专题或原二进制。交付源码、findings、接口表、原码报告和联接验收；未知/失败/未完成范围如实登记，不把局部 probe 通过当作系统完成。本文只是调查与推荐，不是完整调用图或已验证实现。

## 10. 调查依据

- `ORIGINAL_MODULE_SCOPE.md`：B 构建条目与候选业务边界。
- `tracking/original-packages.json`：B Cargo 依赖图。
- `tracking/module-coverage.json`：三版源路径模块库存。
- `evidence/multi-version/{A,B,C}-sources.json`：PDB 私有源路径与校验和。
- `evidence/multi-version/{B,C}-functions.json`：精选函数定位锚点，非完整库存。
- `DEPENDENCY_FILTERING.md`：函数归属、依赖复用与业务交付要求。
- `TASK_BACKLOG.md` S12–S26：地形/道路/水域/植被/动物/环境/音频/交互/存档等范围。
- `BUILDING_SYSTEM_RECOVERY_PLAN.md` G09/G13–G17：已有资源、颜色、历史、存档、调度与 C 适配工作包。
- `TERRAIN_SYSTEM_RECOVERY_PLAN.md`：数据库与文件隔离、消费者边界。
- `PROJECT_MEMORY.md` 第三阶段冻结验收：当前建筑联接与未闭合边界。

# country-core 优先模块推进协调简报（2026-10-16 第三轮，含实际状态更新）

## 第八轮（2026-10-16）：剩余模块全深推——4 组 × 3 agent

用户指示：剩余模块全部深推，4 组并行，**每组 3 agent（①逆向/②实现/③review），不削减**；
父作为**中央 agent 动态分发任务**。避开用户三重构区（rhapsody/roof/wall）。

### 4 组文件权（互不重叠）
- **G1 环境域**：`systems/camera_rig/*`、`systems/weather_system.rs`、`systems/time_of_day.rs`、`resources/time_of_day.rs`、`resources/water*.rs`、`resources/water_mask.rs`、`resources/terrain_rock_mask.rs`、`resources/rocky_terrain.rs`、`systems/terrain/*`（state3/is_shape_inside/光标变换）
- **G2 建筑域**：`resources/stairs/*`、`systems/collision.rs`、`systems/mode_manager.rs`、`resources/toolbar.rs`、`resources/photo_mode.rs`、`resources/stairs_walls.rs`
- **G3 GUI+输入域**：`resources/gui.rs`、`resources/ui_settings.rs`、`resources/onboarding.rs`、`resources/photo_mode.rs`(GUI 面)、`systems/backend/input/*`、`systems/paths.rs`、`systems/prefab.rs`、`systems/brush_preview.rs`、`resources/prefab.rs`
- **G4 数据+工具域**：`systems/save/*`（内存二进制）、`resources/critters/sheep/*`、`systems/ui_clutter/*`、`systems/ivy/*`、`components/*`、`geometry/*`、`asset_libraries/*`(CPU 面)、`systems/history.rs` 深层

每组 3 agent：① 逆向/取证（契约）→ ② 实现（DELIVERY）→ ③ review（REVIEW+深挖方向→①，修复→②）。
父（中央）动态分发：各契约/DELIVERY 落盘后分发下一任务、定序、裁决组外申请、逐项复核。
**禁区**：rhapsody/roof/wall（含 snapping/signifier）任何 agent 不可改。country-core 保持零错误。



## 第六轮批次 2 重定向（2026-10-16）：避开用户三重构区

用户指示：rhapsody / roof / wall（含 wall/snapping/signifier）三区为**用户重构区**，第六轮批次 2 一律避开。批次 2 重定向到完全独立的 country-core 字段级取证→实现闭合：
- **Decorators serde**（0x28B/0x16B JSON↔记录架构，快照 apply 关键）
- **save/session 字段名**（GameSaveData 240B / AsyncSaveCmd 变体 / session create 96B+32B+24B 载荷）
- **sheep spawn·movement 状态机 + X4 羊 Query**（0x140a3f470）
- **Terrain strokes 格式、ui_clutter 0x58B、ivy count_range、mod_key 表2 键码**
- **camera_rig / weather / time_of_day 深层补证**（长期未深挖）
- **28-系统逐名**（契约供宿主，不碰 rhapsody 依赖的 G1/G3/G4 asset 组实体）
**禁区**：`crates/rhapsody/*`、`resources/roofs/*`、`systems/roof/*`、`systems/wall/*`（含 snapping/signifier）——任何 agent 不可改；Walls 曲线区跳过（属 wall 区，由用户收）。



## 第六轮（2026-10-16）：按当前架构推进到完成——字段级取证→实现闭合

用户指示"继续按照当前架构推进到完成的模块实现"。续跑三人团队（① RE a5844e030a22117a1 / ② 实现 ab30764d525b73aea / ③ review a92b9b4990d23a6b2）+ 中央台账。整 crate 编译零错误基线。

### 第六轮范围（按可闭合优先）
- **F1 字段级取证→实现闭合**（①取证②实现③复审逐项）：
  - Walls 0x78B 其余 14 qword→WallState、Decorators JSON↔0x28B/0x16B serde 架构、Terrain strokes 序列化格式（C 块 apply 从"结构性真应用"到"逐字段真应用"）
  - GameSaveData 240B / AsyncSaveCmd 各变体 / session create 96B+32B+24B 载荷字段名
  - sheep 完整字段布局（+0x00..0x2f/+0x3c..0x57/+0x5c..0x67）+ spawn/movement 状态机 + X4 羊 Query
  - ui_clutter AutoClutterRecord 0x58B / parent 0x60B 完整布局
  - ivy count_range RNG 界、send_to_gpu seed/wall_normal、mod_key 键码枚举、GameKeyBindings 40×12B
  - Roofs/WallColors/GateColors/Trees/Raster 5 域写入序、on_event 应用对象
- **F2 调度/装配（可登记部分）**：28-系统逐名、G1/G3/G4 asset 组、Schedule.run 宿主驱动模型——产出契约供宿主/D组；**wall/signifier 接入**属用户重构域（收敛后接，不越权）。
- **F3 跨域接线登记**：RmbAutoClutterHit/UiHistoryButtonPressed 生产者、SpawnAutoClutter 消费方、宿主资源注入——登记契约，宿主面。

### 里程碑（第六轮验收门）
- 每个字段级项：①契约→②实现→③复审→父复核；整 crate 保持零错误。
- C 块 apply 达"逐字段真应用"；save/session 字段名定案；sheep 布局/spawn·movement 闭合。
- wall/signifier 在你重构收敛后接入（第六轮先登记契约）。



## 第五轮（2026-10-16）：关闭三个集中缺口——续跑第四轮三人团队

用户指示：开第五轮关掉三个集中块。续跑第四轮同一团队（① RE a5844e030a22117a1 / ② 实现 ab30764d525b73aea / ③ review a92b9b4990d23a6b2），保连续上下文。整 crate 已编译通过。

### 第五轮范围（三个集中块）
- **A 块 ECS 调度器整合（F2 + 库内执行桥 + 28 序）**：把孤儿系统并入生产 Schedule——ivy（add_ivy_systems 现 7 系统，原 22 系统 0x140944070 序）、garden、sheep（systems/debug/sheep_controls）、input（含 update_mod_key_state/update_cursor_position 等）经 register_core_startup/add_game_startup_systems 注册；Schedule 库内执行桥（bevy Schedule.run 驱动模型，① 定原版 run_game_thread 如何驱动）；28-系统精确序。
- **B 块 sheep save 写盘链（F1b）**：`sheep_section_serialized`/`assemble_game_save_sections` 目前零调用方 → 接真实写盘路径（save_game 同步保存组装 Sheep 节，SaveFileSection::Sheep=5 @0x140a51d40；X4 Query 捕获活羊 → SheepControlSnapshot → 序列化入节）。
- **C 块 快照应用 + 会话消费端**：`init_game_state_from_snapshot` 应用（apply_loaded_snapshot 现空函数 → 颜色/时间/羊/地形逐域应用，0x1417a9c50）；会话切换消费者 6 系统接线（crate 内：reset_input_state（F2 同批）、time_of_day/input_record_replay 资源面；库外系统只留宿主注册点不实现）。

### 第五轮文件权（② 写）
`startup/run_game.rs`、`startup/mod.rs`、`src/lib.rs`（执行桥/注册）、`systems/{ivy,garden,backend/input,debug/sheep_controls,audio/mod.rs}` 注册面、`systems/save/{save_game,snapshot}.rs`、`resources/critters/sheep/*`、`systems/{terrain,time_of_day,input_record_replay}` 资源面、`meta_commander.rs`。**绝不可碰 `resources/roofs/geometry/*`、`systems/roof/*`（用户 roof 重构区）**。共享文件改动照旧中央仲裁。

### 里程碑
- ④轮已：整 crate `cargo check` 零错误；T1/T2/T3 全部复核通过无水分。
- ⑤轮验收门：A 块完成后孤儿系统清零、Schedule 可执行；B 块后 Sheep 节真实写盘；C 块后 apply_loaded_snapshot 逐域应用。每块交付照旧 ③ 复审 + 父复核。



## 第四轮（2026-10-16）：逆向关闭剩余缺口——三人团队

用户指示：三个 agent 身份转变——**① Rust 专职逆向 / ② 实现 / ③ review 代码（提出缺失、指导深挖方向、指出功能缺失）**。目标：逆向关闭第三轮登记的全部剩余缺口。

### 第四轮范围（track）
- **T1 garden_audio 130.0 门**：契约已解码（music-DELIVERY.md：bilinear 光栅采样门 ≥0.01 分流 DrawWater/EraseWater、音量 [vol_res+0x1c]/60.0 当 mode!=0xd、130.0=0x43020000、native 6 参签名）→ 逆向补**音量/光栅资源身份**（哪个 SSBO/偏移）→ 正确 owner 实施。
- **T2 ECS 调度器 + startup 装配**：`run_game_thread 0x1405a3440`（39KB 装配与 28 系统全局顺序）、`load_game_session 0x14166e7d0`（78KB 完整系统）、会话切换系统 + `NewSessionStartedCmd`/`SessionLoadedCmd` 生产者、Schedule 库内执行桥。
- **T3 待取证小项定点取证**：ivy count_range.end / flag_8c leaf 清理谓词 / send_to_gpu seed·wall_normal / VERTICAL_MIN 疑点 / 0.68 vs 0.63；ui_draw_terrain jumptable 分支 0/1/3；EditType::Terrain 原 tag（SPEC 9..12 vs 现 15）；ui_clutter parent-map 0x60B 槽位；capture/load 域采集器 FUN_141305170/141308b40/141309840；PostGameLoadCommand 应用 FUN_1412e0b90；sheep register_required_components/picking 分支；input update_mod_key_state 7-handler 跳表 / GameProfile 直读。

### 第四轮角色与文件权
| 角色 | 职责 | 文件权 |
|---|---|---|
| ① RE | 只读逆向：按原机器码产出每个缺口的 `round4/contracts/<gap>-CONTRACT.md`（行为契约+原地址+资源身份） | 只写证据文件，不改源码 |
| ② 实现 | 按契约+SPEC 写真实 Rust；从已解码契约先落地（garden_audio、易取证项），随契约更新迭代 | 写 country-core 源码（**绝不碰 `resources/roofs/geometry/*`、`systems/roof/*`——用户正在做 roof 重构**） |
| ③ Review | 读代码+对照原机器码：指出缺失/水分、输出 `round4/REVIEW.md`（缺项清单 + 深挖方向，喂给 ①）+ 逐实现复审 | 只写报告，不改源码 |

协作协议：①→②（契约喂实现）、③→①（深挖方向）、③→②（修复项）。三人各自独立文件权，只有②写源码→无并发写冲突。整 crate 编译门仍被用户 roof 重构阻塞；②只保证自己文件无错，不运行依赖 roof 的整包 check。

证据目录：`evidence/core-recovery-20261015-adv/round4/{contracts,deliveries,REVIEW.md}`。



## 实际状态更新（2026-10-16 用户批改后，以此为准）

- 用户已提交大批判改（tiny-glade commit `f510c7e7`「库内完整性核验收口与缺口闭合」+ merge `c164567a`），**9 个优先模块的大部分 SPEC 项已实现**：ivy grower 10 相主生长闭包（0x14094ac51 守卫体）＋send_to_gpu 48B shader 布局＋pruner 空间哈希插值；input translate_input 动作↔槽映射 O 全解（滚轮 analog[4] 纠偏）＋InputReplayInitialState/seek 子字段类型化；save_thread/load_game/save_game/snapshot/load_completion_checker；terrain ui_draw_terrain/ui_edit_terrain/terrain_hover_idx/deferred_terrain_history/ui_terrain_focus_mode；ui_clutter rmb_autoclutter/decoclutter_show_pins/ui_autoclutter/ui_focus_decoclutter/debug_spawn_clutter；session.rs（20KB）；audio 参数/事件名定案。
- `contracts.json` 的 `remaining` 是**批改前占位，已过期**。验收前必须对当前源码做实际 diff。
- **历史分析 agent 本轮职责变为「核对」**：对照 SPEC/原机器码逐项验证用户批改的真实实现，输出 `*-DELTA.md`（已真实落地 / 有水分 / 仍缺），而非重新研究。
- 工作树干净，`cargo check -p country-core` 通过（9 warning）。
- **新增第四组 D（rhapsody 接入）**：把 country-core 渲染层真正接入 `tiny-glade/crates/rhapsody`。

## 原始版本（2026-10-15 第二轮，SPEC 依据）

目标：把 9 个 `implemented_bounded` 优先模块往"完整业务闭包"推进，逐模块复核去水分。

## 总则

- 目标 B 版 tiny-glade 的 core 业务一致性；全部新增逻辑落在 `D:/game/tiny-glade/crates/country-core/`。
- **文件所有权隔离**：每组只写本组拥有的文件。对组外文件的任何修改，必须经中央 agent 仲裁后再动。
- **禁止并发放同一个文件**。跨组需要同一文件时，由中央 agent 定序。
- 已经确认的 derive/依赖胶水/Bevy 注册直接复用，不计业务；自定义默认值/字段策略/serde 属性/事件仍是业务。
- 验收纪律：编译通过 ≠ 模块完成。必须：真实业务代码 + 真实消费者 + 原机器码证据 + 诚实的 remaining 标记。禁止伪造加载/成功/空壳。

本简报定义三个隔离组的模块/文件所有权、Windows 逆向工具链入口、证据位置和验收纪律。
目标：把 9 个 `implemented_bounded` 优先模块往"完整业务闭包"推进，逐模块复核去水分。

## 总则

- 目标 B 版 tiny-glade 的 core 业务一致性；全部新增逻辑落在 `D:/game/tiny-glade/crates/country-core/`。
- **文件所有权隔离**：每组只写本组拥有的文件。对组外文件（如共享资源 `resources/session.rs`、`resources/glade_settings.rs`、`resources/history/*`、`systems/history.rs`）的任何修改，必须经中央 agent 仲裁后再动。
- **禁止并发放同一个文件**。跨组需要同一文件时，由中央 agent 定序。
- 已经确认的 derive/依赖胶水/Bevy 注册直接复用，不计业务；自定义默认值/字段策略/serde 属性/事件仍是业务。
- 验收纪律：编译通过 ≠ 模块完成。必须：真实业务代码 + 真实消费者 + 原机器码证据 + 诚实的 remaining 标记。禁止伪造加载/成功/空壳。

## 工具链（Windows 本机，实测入口）

- RE-MCP 反编译：
  `node D:/game/reverse-engineering/tiny-glade/scripts/mcp-job.mjs re_mcp_ghidra <job.json> <out.json>`
  job = JSON 步骤数组。需 open_database{file_path:"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe", run_auto_analysis:false, database_id:<唯一id>} + wait_for_analysis + close_database{save:false}。预分析 116,270 函数。catalog 落盘 `evidence/mcp/re_mcp_ghidra-catalog.json`。
- 原字节/字符串直读（快）：
  `python D:/game/reverse-engineering/tiny-glade/scripts/rne-read.py <va>|--fat <va>|--hex <va> <n>`
  RNE 二进制 SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 权威函数名→地址（v2）：`evidence/country-core-deep/sysres-procedures-v2.json`（1,599 条 country_core systems/resources 过程）。旧 v1 有系统性错位，只作线索。
- 每文件 PDB 元数据：`skeleton/B/crates/country-core/src/**/*.rs` 头注释（函数符号、va、依赖注解）。
- Rust 逆向关键事实：`evidence/country-core-deep/PLAYBOOK.md`（游戏 Vec 是 cap-first {cap@0,ptr@8,len@0x10}、i64::MIN 哨兵、enum niche 判别、Arc 模式等）。
- 内存约束：32GB RAM，空闲波动大；MCP 批量用 `--lock` 全局串行。MCP 反编译不开新项目锁冲突：先查既有作业是否在跑。

## 分组与文件所有权

| 组 | 模块 | 拥有的源文件（相对 crates/country-core/src/） | 历史证据入口 |
|---|---|---|---|
| A 植被 | ivy | `resources/ivy/*`、`systems/ivy.rs`、`systems/ivy/*` | evidence/core-recovery-20261015/ivy/（grower.asm.txt 66KB 等） |
| A 植被 | garden | `systems/garden/*`、`resources/glade_border.rs` | evidence/core-recovery-20261015/garden/ |
| B 生物 | sheep | `resources/critters/*` | evidence/core-recovery-20261015/critters/ |
| B 音频 | music | `systems/audio/music/*`、`systems/audio/mod.rs` | evidence/core-recovery-20261015/music/ |
| B 输入 | input | `systems/backend/input/*` | evidence/core-recovery-20261015/input/ |
| C 世界 | terrain | `systems/terrain/*`、`resources/terrain_editing.rs` | evidence/core-recovery-20261015/terrain/ |
| C 世界 | save | `systems/save/*` | evidence/core-recovery-20261015/save/ |
| C 世界 | session | `resources/session.rs` | evidence/core-recovery-20261015/session/ |
| C 世界 | ui_clutter | `systems/ui_clutter/*` | evidence/core-recovery-20261015/ui-clutter/ |
| D rhapsody | rhapsody 接入 | `crates/country-core/src/rhapsody_contract.rs`、`src/rhapsody_types.rs`、`src/render/*`（含 pass_builder/nani）、`src/systems/render_loop/*`、渲染侧 `src/asset_libraries/*`、`crates/country-core/Cargo.toml`（加 rhapsody 依赖） | evidence/render-recovery/、country-core-deep/rhapsody/、crates/rhapsody/README.md |

B 组 SPEC 目录名沿用 `B-creatures-audio/`，input 的 SPEC 也放该目录（input-SPEC.md）。

## D 组（rhapsody 接入）契约

- **目标**：country-core 渲染层从本地占位/panic 桩切换为真正调用 `crates/rhapsody` 的公开 API。当前状态：`Cargo.toml` 无 rhapsody 依赖；`rhapsody_contract.rs` 5 处 `panic!("GPU backend is not connected")` 桩；`rhapsody_types.rs` 是本地占位类型（注释标了原地址/O 证据）；render/* 通过 rhapsody_types 再导出引用；全 crate 无 `ExecutionChain/ExecutionBackend/execute_graph` 引用。
- **真实逆向接口契约**（不要虚构函数）：rhapsody 公开面 `crates/rhapsody/src/{leek,log_ring,raw,rg}`；`rg::executor::{ExecutionChain,ExecutionBackend,execute_graph,execute_graph_with_context,begin,record,submit_recorded,complete,PassContext,GraphExecution}`；`rg::graph::ResourceBackend`；`raw::device`、`raw::managed`、`raw::texture`/`shader`/`descriptor_set` 等。country-core 想要的接口面已在 `rhapsody_contract.rs`/`rhapsody_types.rs`/`render/*` 注释与旧重建证据里以原地址记录（如 TextureDesc 0x140bd5270、storage 0x140bd5390 等）。原始调用关系证据：`evidence/render-recovery/`（RHAPSODY_PROGRESS.md、BATCHED_RHAPSODY_RECOVERY_PLAN.md、WORKFLOW_RESET.md）、`country-core-deep/rhapsody/RHAPSODY-DEEP-SUMMARY.md`、`skeleton/workflow-design/tiny-glade-recovery/references/rhapsody-work-packages.md`。
- **规则**：
  1. 只调用 rhapsody **已存在**的公开 API；`crates/rhapsody` 本体缺接口时**登记缺口**，不往 rhapsody 里补造函数（`rhapsody` crate 文件归 rhapsody-rebuild 链，不在任何组所有权内）。
  2. 字段/参数以 `rhapsody_types.rs`/`rhapsody_contract.rs` 的 O 地址证据与 `crates/rhapsody` 实际签名为准；找不到原入口的标"待取证"。
  3. 移除/替换 panic 桩与本地占位类型时要保持 render/* 上层（nani、pass_builder、render_loop）签名不破坏，先建编译基线的等价桥。
  4. GPU 真机执行不验收（后置）；本组验收=真实调用接线 + cargo check + 原侧入口核对。
- **交付**：`evidence/core-recovery-20261015-adv/D-rhapsody/rhapsody-WIRING.md`（历史分析/核对）＋`rhapsody-DELIVERY.md`（实现）。

共享但需仲裁的文件：`resources/history/*`、`systems/history.rs`（history 账本；ui_clutter 与 terrain/garden/save 都可能消费）、`resources/glade_settings.rs`（ivy 读 +1c 旗标）、`resources/session.rs`（music 读 ActiveSession）、`resources/audio*`/FMOD、`rhapsody` crate 本体（任何组都不可改，缺接口登记缺口）。修改前发消息给中央 agent。

## 每模块剩余缺口（合同 contracts.json，2026-10-15）

- **ivy**：完整 grower/root spawner、wall-change point-validity pruner、root/branch 删除策略、UI/ActionSet 门、wall jump/hole 约束、GPU 上传/渲染（GPU 后置）。`ivy_leaf_spawner` 需接 GladeSettings +1c 真实旗标。
- **garden**：完整 UI 仲裁/其他 WidgetParams 变体、signifier 所有者、原全局调度、56 字节 ShowPinnedDecoClutter union 与 renderer、下游 garden scatter/audio 消费者。新 GardenPinnedClutterCircle 只是观测到的圆/高度投影，不算完整 pin 系统。
- **sheep**：完整 Sheep owner 与生命周期、真实 Sheep query 所有者、save/load 接线、spawn/movement。
- **music**：完整 GladeAudio track producer、AudioCore/device/bank 启动、live FMOD 集成（FMOD 依赖 libfmod 2.222.6，原锁 checksum 75defbe5…）。music_player::load_tracks 已实现但完整 GladeAudio 加载消费者未恢复。
- **input**：键盘/Steam 映射、reset/replay 集成、update_cursor_position、原全局调度、OS 事件生产者。
- **terrain**：terrain UI/new-stroke 输入生产者、原 plugin 资源初始化与全局调度、terrain history/replay 生命周期、audio 播放、GPU 执行（后置）、下游 wall/collision/scatter 订阅链。
- **save**：完整 AsyncSaveCmd 变体、deferred 命令、主后台保存循环/unsaved-file 重试、每私有域的游戏线程捕获/序列化、快照二进制捕获/load、自动/手动备份生产、session 切换自动跑 GC、完整 save-session 生命周期。
- **session**：完整 NextSession(160B) 所有者、GameSaveData(240B)/MetaCommand/SessionOnLoadCmd 派发、SessionLoadAttempt、完整 world load/commit 事务。空 Vec 是 cap 0 + dangling ptr 8；kind 五条 selector 路径；`create` 0x140b46960 复制 metadata + 24B 可选串/路径 + 字节旗标；`append_on_load` 0x1403b5150 三个 56B MetaCommand → VecDeque → 32B SessionOnLoadCmd。
- **ui_clutter**：原 RMB picking、parent/roof transform 解析、auto→decorator/stair 转换、UI 交互、消费删除 ID 的 generation/mesh owner（渲染消费；owner 后置可先留真接口）。

## 中央 agent 职责

- 台账：`evidence/core-recovery-20261015-adv/PROGRESS.md`（每模块状态/证据/剩余）。
- 接收各组进度同步消息，仲裁组外文件修改请求。
- 不代做实现；发现水分向父报告。

## 交付物格式（每组每模块）

- 历史分析 agent 产出 `evidence/core-recovery-20261015-adv/<组>/<模块>-SPEC.md`：
  要实现的剩余业务契约、原地址/机器码引用、真实消费者、待分类项、明确排除项。
- 实时复合 agent 按 SPEC 实现；实现后写 `evidence/core-recovery-20261015-adv/<组>/<模块>-DELIVERY.md`：
  实际接线 / candidate 链测试 / 原侧验证 / 仍缺项，逐项分离。不得用编译通过冒充原侧验证。

# country-core 逐文件复核报告（2026-10-19）

> 范围：`D:/game/tiny-glade/crates/country-core/src` 全部 641 个 `.rs`、187,215 行。
> 方法：9 个只读复核组逐文件通读 + B 骨架（539 文件）函数名/原地址对照 + 本机编译/测试实测。
> 逐组明细：`RENDER_LOOP_A/B、RENDER、ASSETS、RESOURCES、SYS_DATA、SYS_UI、WALL、WALLS_ROOFS、MISC`-AUDIT.md。
> 口径：百分比是“静态保真度 + 链闭合度”的估计，不是语义等价验收；置信度见各组报告。

---

## 0. 一句话结论

**源码语义还原约 73%，端到端可运行接线约 55%；约 27% 的代码量对应的原作行为尚未复现或未闭合。**
函数名/地址层面覆盖较高（82%），但真正的缺口集中在**整文件缺失、系统未注册、回放/快照/渲染的消费端断开**，而不是 `todo!()` 空壳（全 crate 硬空壳极少）。

---

## 1. 总体数字

| 指标 | 数值 |
|---|---:|
| 源文件 | 641 |
| 代码行 | 187,215 |
| 非测试函数（括号配平统计） | 4,948 |
| 含测试函数 | 8,073 |
| B 骨架文件 | 539 |
| 无同路径源码的骨架文件 | 73（其中约 45 个经核实为真缺失，其余为搬移/改名/合并） |
| 骨架具名函数（过滤闭包/泛型） | 1,526 |
| 名字+原地址可对上 | 1,255（82.2%） |
| 仅名字/相似名 | 144（9.4%） |
| 仅原地址 | 5（0.3%） |
| 完全无名字无地址 | 122（8.0%） |
| 空/平凡函数体（非测试） | 44 |
| 只有日志的函数 | 1 |
| 函数体带“待取证/待①/占位/未接线/未实现”标记 | 57 |

> 注：122 条 `no_trace` 中仍有一部分可能是内联/改名；144 条“仅名字”大多是注释提及。真实业务缺口估计 ≈ 200–270 个函数，另有大量“函数在、但没注册/没生产者/没消费者”的接线缺口。

---

## 2. 模块结构进度

条形为综合还原度（`█` 实心 = 已还原，`░` = 未还原），括号内为端到端接线度。

| # | 模块 | 文件 | 行数 | 源码语义 | 端到端 | 可视化 |
|---:|---|---:|---:|---:|---:|---|
| 1 | **渲染主链** `systems/render_loop` | 103 | 40,483 | 66% | 25% | `█████████████░░░░░░░` |
| 2 | **渲染基础** `render/**`（pass_builder/nani） | 47 | 21,402 | 90% | 80% | `██████████████████░░` |
| 3 | **资源库** `asset_libraries` | 15 | 5,004 | 90% | 80% | `██████████████████░░` |
| 4 | **资源与子系统** `resources/**`（含 stairs/history/ivy/critters/rt） | 102 | 28,292 | 84% | 75% | `█████████████████░░░` |
| 5 | **墙系统** `systems/wall` | 81 | 18,720 | 70% | 50% | `██████████████░░░░░░` |
| 6 | **墙/屋顶资源** `resources/walls` + `roofs` | 72 | 16,099 | 66% | 45% | `█████████████░░░░░░░` |
| 7 | **存档** `systems/save` | 12 | 6,743 | 76% | 65% | `███████████████░░░░░` |
| 8 | **历史/回放** `systems/history` | 12 | 4,046 | 68% | 50% | `██████████████░░░░░░` |
| 9 | **音频** `systems/audio` | 41 | 6,670 | 72% | 60% | `██████████████░░░░░░` |
| 10 | **常春藤** `systems/ivy` | 9 | 2,822 | 80% | 65% | `████████████████░░░░` |
| 11 | **花园** `systems/garden` | 4 | 467 | 85% | 80% | `█████████████████░░░` |
| 12 | **地形** `systems/terrain` | 11 | 3,035 | 85% | 75% | `█████████████████░░░` |
| 13 | **UI/相机/碰撞/输入等** `systems/**` 其余 | 65 | 20,118 | 55% | 45% | `███████████░░░░░░░░░` |
| 14 | **启动装配** `startup` | 22 | 4,325 | 55% | 40% | `███████████░░░░░░░░░` |
| 15 | **输入/组件/几何/工具/根文件** | 45 | 8,989 | 88% | 85% | `██████████████████░░` |
| | **合计/加权** | **641** | **187,215** | **73%** | **55%** | `██████████████░░░░░░` |

分链细项（来自各组）：

| 链 | 源码语义 | 端到端 | 主要断点 |
|---|---:|---:|---|
| 输入 | 85% | 80% | 少量动作槽无消费者 |
| 模式管理 | 90% | 90% | — |
| 时间/天气/时钟 | 90% | 90% | — |
| 地形编辑 | 85% | 75% | `OnTerrainChange` 无消费者 |
| 花园 | 85% | 80% | filter 精确性待人工 |
| 存档写盘 | 76% | 65% | 快照读回被 bail、4 域应用缺失 |
| 历史/回放 | 68% | 50% | Pop/颜色/装饰/楼梯回放消费者缺 |
| 音频 | 72% | 60% | 音乐主系统未接、7 类事件无 producer |
| 常春藤 | 80% | 65% | GPU 上传未注册、音频空壳 |
| 墙体 CPU | 75% | 55% | merge/split/墙高/普通删除历史缺 |
| 墙体 signifier/UI | 20% | 15% | 实体、mesh、widget 链缺 |
| 屋顶 | 75% | 35% | RoofBuffers/RoofTile/DormerTiles/触发体为空壳 |
| 相机 | 45% | 20% | 主更新/Walk/Butterfly/reset/snapshot 缺 |
| UI widget/交互 | 30% | 15% | 10 个工具文件缺失，无通用 hover/click 生产者 |
| 碰撞 | 70% | 45% | 生命周期/插件/下游砖铺放缺 |
| Prefab | 75% | 30% | 系统全未注册 |
| Steam | 60% | 15% | 插件/输入系统未接 |
| 渲染图执行 | 66% | 25% | 无生产 FramePasses/PassHost/ComputeExec |

---

## 3. 仍未复现的内容（按模块）

### 3.1 整文件缺失（已确认 workspace 内无实现）

- **startup（17）**：`clearing`、`load_audio`、`load_glade_grading`、`spawn_brush_previews`、`spawn_human_ref`、`spawn_preplaced_trees`、`startup_critters`、`startup_meadow`、`startup_roof`、`startup_sheep`、`startup_sky_and_sun`、`startup_time_of_day`、`startup_walls`、`glade_themes/{flowery,olden}`、`debug`、`panic_reporter_startup`。
- **UI（8）**：`ui_update`、`hover_intersection`、`debug_visualize_volumes`、`ui_2d_follow_mouse`、`ui_camera_mode_gizmos`、`ui_edit_freehand_walls`、`ui_focus_mode_shape_height`、`ui_move_freehand_walls`、`ui_move_shapes`、`ui_raise_shape`。
- **单文件系统（6）**：`despawn`、`look_at_camera`、`mesh_follow_terrain`、`utils_intersect`（`line_x_shape_overlap`）、`backend/grow_grass`、`backend/icc`（`update_window_icc_profile`）。
- **历史（7）**：`history_process` 的调度段、`replay_garden`、`replay_path`、`replay_water`、`replay_pop_bucket` 的注册段、`signifier_replay_active`、`history_recording` 的合并/删除路径。
- **墙（6）**：`merge_walls`、`wall_splitter`、`set_wall_style`、`upload_instanced_wall_ssbo`、`adjust_wall_height` 的 UI/历史/signifier 三件套、`spatial_hash_update` 的更新入口。
- **资源（8）**：`walls/material_stats`（18 函数中 14 个空/无实现）、`walls/wall_attachment`、`walls/decorator_outline`、`resources/daily`、`feedback_report_resources`、`loading_screen`、`preplaced_autoclutter`、`shape_intersections`、`window_lights`。
- **键位表**：`game_key_bindings.rs` 整文件，7 个绑定函数全无实现。

### 3.2 存档 / 读档（最严重的业务链断点）

- `systems/save/load_game.rs:267`：打开有效 `.snapshot` 直接 `bail!` —— **带快照的存档无法读回**。
- `init_game_state_from_snapshot` 无生产调用者；只有测试调用。
- 快照 14 节中，apply 只处理 4 节：Walls / Decorators / Terrain / Raster；`Roofs/WallColors/GateColors/Trees` 落入 `log::trace!("apply domain …")`。
- Walls 只写回 `id` 与 `height`；0x70B 曲线区、freehand/arch/removed_supports 枚举未打包。
- Decorators 0x28B 逐槽与 0x16B 子记录未解；ID XOR 掩码按 0 处理。
- `GameSaveData` 240B、`AsyncSaveCmd` 余下变体、备份身份 `+0x299` 结构未恢复。
- 快照 GC 入口无生产调用者。

### 3.3 渲染 / rhapsody

- `systems/render_loop` 只有 pass 声明与测试 host；**无生产 `FramePasses` / `PassHost` / `PassFrame` / `ComputeExec`**。
- `tiny-glade/src/production_renderer.rs:322` 的 `compile_world_graph` **固定返回 `Err(world_consumer_missing())`** —— world 帧无法构图。
- `render/execution_host.rs` 是无人调用的重复接缝；`ExecutionChain` 未在游戏侧直接构造；真实执行经 `tiny-glade/render_bridge.rs` 的 `VulkanGraphSession`。
- `rhapsody_contract.rs` 仍有 5 处 `panic!` 桥、若干 `RenderGraph`/`PassBuilder` 占位。
- 已确认缺失：`render_loop::clear_buffer`、`lut_gallery::startup`、`compute_water::water_level_at_xz`、`blur_exclusion_mask`、`logo_overlay::startup`。

### 3.4 历史 / 回放

- 历史 Pop 只注册 `pop_empty_replay_bucket`；`replay_pop_bucket_system`、`history_clear_previous_state` 未注册。
- 颜色/装饰/墙颜色/楼梯音频回放只发事件，**crate 内无 `EventReader` 消费者**。
- 楼梯回放音频载荷全 0；decorator 载荷被丢弃。
- `signifier_replay_active` 缺失。
- 墙变体 5..16 的载荷未恢复，回放对未知 op 直接返回错误。

### 3.5 音频

- `configure_music_consumers`、`load_glade_audio` 无调用者；音乐主系统未接。
- 7 类音频事件无 producer：Building/Clutter/SheepReact/MenuUi/Tree/Timberframe/ElevationSupport。
- `AudioSettings` 有两份定义且未接线；`Weather` 重复；`EditingSpeed` 无生产者。
- 楼梯音频全零；ivy 音频只打日志；建筑音频以 `editing_speed` 顶替面积参数。

### 3.6 墙 / 屋顶 / UI / 相机 / 碰撞

- 墙：merge/split 拓扑、普通删除历史、墙高 UI+历史+signifier 动画、signifier 实体/mesh、形状放置与编辑系统注册、矩形投影 `0x140c99520`、圆形屏幕↔世界角度。
- 屋顶：`RoofBuffers`/`RoofTile`/`DormerTiles`/`EmptyRoofTriggerVolume` 数据面为空壳；两级 UV 索引回退为全槽射线；`publication_fields` 与 `color_selection` 的公共墙记录偏移冲突（0x2a0/0x258/0x299 vs 0x2a8/0x260/0x2a1）。
- UI：`ui_update`/`hover_intersection` 缺失；`UiState` 的 `current_intersection/active_widget_state/is_dragging`、`WidgetParams` 的四个回调、Signifier 的 `minimum_lifespan/animation_easing/parent_to_widget` 缺失；`add_ui` 未接。
- 相机：`main_camera_update` 无实现（仅 moved）、`paused_main_camera_update`/`reset_camera`/`camera_snapshot`/`build` 缺、Walk/Butterfly 相机整文件缺、无 Schedule 注册。
- 碰撞：`CollisionPlugin::build` 无宿主调用、`MeshIntersectionGraph` 未插入、`upsert_collider` 无生产者、mesh 相交下游（砖铺放/鸟巢）未接。
- Steam/Paths/Prefab/Arrow/Photomode/Timelapse：实现多在，但系统/插件全未注册或只有测试调用。

### 3.7 资源层字段级缺口

- `MaterialStats` 为 `Gap` + 4 个空 setter；`WallHistoryEdit` 变体 5..16 载荷未解。
- `stairs` 的结构图 +0x120/+0x180 两张附加表、24B endpoint 载荷、render 写面。
- `RippleSourceSsbo` CPU 生产者字段、`terrain_height` 真实构造点、`tree_storage` 余字段。
- `WallState 0x70B` 无法由记录重建曲线点列。

---

## 4. 五条主业务链闭合状态

| 链 | 状态 | 断点 |
|---|---|---|
| 启动 → 装配 → 主循环 | **部分** | startup 17 文件缺；主相机更新缺；多处 `add_*` 未调用；宿主 Schedule 驱动存在 |
| 会话切换 → 读档 → 应用 | **断** | 有效快照 `bail!`；apply 无生产调用者；4/9 域仅 trace |
| 编辑 → 历史 → 回放 | **部分** | 主干真实；Pop/颜色/装饰/墙变体/楼梯回放消费者缺 |
| 帧循环 → 构图 → GPU 执行 | **断** | world graph 固定 Err；无生产 FramePasses；ExecutionChain 未直接使用 |
| 存档 → 写盘 → 备份 | **基本通** | 写盘/线程/节拍器真实；GameSaveData 字节布局与 Windows 布局两处未闭合 |

---

## 5. 本机编译与测试实测（关键）

环境：Windows 10，Rust 1.97.1（1.87 同样复现）。

1. **`cargo check -p country-core` 当前失败**：
   `systems/save/save_thread.rs:63` 的
   `assert!(size_of::<UnsavedFileEntry>() == 0x80)` 不成立。
   原因：Windows 上 `PathBuf` 为 32B（Linux 为 24B），结构实际 0x88。原作 Windows 版若为 0x80，则 +0 字段很可能不是 `std::path::PathBuf`，而是 24B 的自定义路径表示。
2. 临时放宽该断言后：编译通过（2m26s），执行 `--lib` 测试：
   **2447 通过 / 9 失败 / 0 忽略**。
3. 9 个失败：
   - 3 个 `input::recording`：Windows 上 `NamedTempFile::persist` 覆盖已存在文件返回“拒绝访问 (os error 5)”——**录制文件替换在 Windows 不可用**。
   - 5 个资源/启动测试：测试路径 `../../../../ljxsj_92385/...` 从 `crates/country-core` 上溯 4 级落到 `D:\ljxsj_92385`，实际资源在 `D:\game\ljxsj_92385`；是路径深度错误，不是资源缺失。
   - 1 个 `unsaved_file_list_uses_the_native_0x80_byte_entry`：136 vs 128，同上 PathBuf 布局问题。
4. 结论：**“Linux 上 2452 通过”不能代表 Windows 目标可用**；游戏是 Windows 构建，上述两项都是实际平台缺陷。

---

## 6. 与旧台账的差异

- 旧报告“整体 ≈83%”偏乐观：它主要按函数名/地址与已写代码量计分，未把“系统未注册、无 producer/consumer、生产路径固定 Err/bail”计入。
- 本次按“源码语义 + 端到端接线”双口径：**73% / 55%**。
- 旧扫描中“完全没迹象 304 个”混入了大量闭包/泛型单态化符号；过滤后全 crate 真实 `no_trace` 为 122 条，其中约一半仍是可内联/改名的候选。
- 但旧扫描低估了**整文件缺失**与**接线断裂**：这两类才是当前最大缺口。

---

## 7. 建议的下一步（按收益排序）

1. **修复 Windows 构建与两处平台语义**：`UnsavedFileEntry` 路径表示（还原 24B 布局或改断言+真实布局）、录制文件原子替换（Windows 用 `ReplaceFile`/先删后改名）。
2. **打通读档**：去掉 `load_game.rs:267` 的 `bail!`，把 `init_game_state_from_snapshot` 接入 load 完成系统；补 Roofs/WallColors/GateColors/Trees 四域 apply。
3. **打通渲染构图**：实现生产 `FramePasses`/`PassHost`，让 `compile_world_graph` 不再固定 `Err`；明确 `execution_host` 与 `render_bridge` 的唯一路径。
4. **补 startup 17 个文件**（尤其 `startup_roof`、`startup_sky_and_sun`、`startup_critters`、`load_audio`、`glade_startup_walls`）。
5. **补 UI 工具/相机主更新/碰撞生命周期**（`ui_update`、`hover_intersection`、`main_camera_update`、Walk/Butterfly、`CollisionPlugin`）。
6. **补历史回放消费者**（Pop aux、颜色/装饰/楼梯/墙颜色、signifier replay）。
7. **补墙 merge/split/墙高链与屋顶数据面**（`RoofBuffers`/`RoofTile`/`DormerTiles`/触发体、UV 索引）。
8. 最后做字段级收尾：`MaterialStats`、`WallHistoryEdit 5..16`、`GameSaveData 240B`、`stairs` 两张附加表。

---

## 8. 证据索引

- 本目录各组报告：`RENDER_LOOP_A/B、RENDER、ASSETS、RESOURCES、SYS_DATA、SYS_UI、WALL、WALLS_ROOFS、MISC`-AUDIT.md
- 机械函数对照：`fn_audit.json`（过滤前）、`fn_audit_clean.json`（过滤闭包/泛型后）
- 基线：`tiny-glade/crates/country-core/recovery/STUBS-2026-10-19.md`
- 上层 crate 现状：`reverse-engineering/tiny-glade/upper-modules-plan/STATUS-2026-10-19.md`

---

## 9. 第 9 轮更新（2026-10-19 晚，覆盖上文对应数字）

### 9.1 两组结果

| 组 | 范围 | 结论 | 水分 |
|---|---|---|---|
| G1 存档/读档 | `systems/save/{load_game,snapshot}.rs` + `save_thread.rs` 布局 | **通过** | 2（非阻断） |
| G2 渲染构图 | `systems/render_loop/**` + `tiny-glade/{production_renderer,game_loop,world_graph}` | **通过（G2 范围）** | 0 |

### 9.2 已闭合

- **读档主链**：删除“遇到有效 `.snapshot` 即 bail”；`load_game_system → load_closest_valid_snapshot → load_snapshot_file → init_game_state_from_snapshot(world)` 真实接线。World 级写回 Walls / Roofs / Decorators / WallColors / GateColors / HalftimberColors / Terrain strokes+高度 / Trees / Raster / Stairs / StairsRemovedSupports。
- **契约纠偏落地**：Walls 为语义 `WallState`（点列堆外，不做内联打包）；Decorators 内层 0x58、seed 写 +0x2c；`0x1429cd080` 为 aHash 固定种子而非 ID 掩码（旧 rotate^mask 已删）。
- **真实样本回归**：36 个发行快照、5 个 `"Stairs"` 桶通过；计数命中 162 walls / 73 roofs / 134 decorators / 431 / 0 / 20 / 590 等。
- **渲染构图**：`compile_world_graph` 接 `build_frame → with_main_prelude → world_graph::build_world`（W1–W54）、export 0x22、compile；缺 producer 默认硬错误，仅 `TG_ALLOW_MINIMAL_WORLD_FRAME=1` 或 demo 允许真实 clear+export 最小帧。
- **细节修复**：`main color` 补 `mip_levels(6)`；`main depth` mip1；OOM 降分辨率消费 `world_resolution_scale`；warm-up 接 `warm_up_gameplay_with_world_builder` 并进入 `CompletedGameplayWarmup` 发布门。

### 9.3 编译与测试（Windows 真树）

- `cargo check -p country-core` → **通过**（165 warnings）
- `cargo check -p tiny-glade` → **通过**
- `cargo test -p country-core --lib` → **2467 passed / 0 failed / 0 ignored**
- 平台修复：`UnsavedFileEntry.path` 改 24B `String`（匹配原生 0x80，Windows `PathBuf` 32B）；录制文件替换在 Windows 改为“备份→persist→失败恢复”；资源测试路径深度修正；`critters/duck.rs` 的 trait 调用改 UFCS。

### 9.4 剩余阻断/缺口

1. **compile_main 尾部**：warm-up builder 只覆盖 W1–W54 + TAA 导出，未含 grading/blur/final_post/UI；补齐前不得宣称 warm-up 等价完整 compile_main。
2. **读档残留**：RoofAssociations 表未重建；空 prefab 树回退属 tree-tool；Clutter `to_info` 扩展簇；地形脏区发布；`GameSaveData` 240B 字段。
3. **G1 非阻断残留**：M4/M5/M6/M8；R1 终态失败未发宿主事件（跨组接缝）。
4. **GPU 真机**：仍未验证（项目后置）。

### 9.5 数字修订

- `systems/save`：76% / 65% → **86% / 80%**。
- `systems/render_loop`：66% / 25% → **66% / 55%**（源码语义不变，接线显著提高）。
- country-core 总体（按行加权）：73% / 55% → **约 74% / 62%**。


---

## 10. 第 10 轮更新（2026-10-19 晚）

> 第 9 轮后按五条链调用顺序推进的第 10 轮。用户要求本轮跑完结束。

### 10.1 结果

| 组 | 范围 | 结论 | 水分 |
|---|---|---|---|
| G1 启动→装配→主循环 | startup 13 模块、主相机、装配面 | 修复后**有条件通过**（未二次送审） | 首轮 11，修复后未复评 |
| G2 读档残留 + 编辑历史回放 | 四类 consumer、Pop、RoofAssociations/树/地形 | 实现完成，**复核未闭合** | 2，已定点修复 6 项 |
| G3 compile_main 尾部 | grading/blur/final_post/UI/export | **通过（范围限定）** | 0 |

### 10.2 已闭合

- 13 个 startup 模块创建并接入宿主 `OnEnter(Game)` batch2 / PreStartup；主相机三系统进 Update 并写 `MainCamera` 四矩阵；Walk/Butterfly dolly 落地。
- 颜色回放改为一次性派发、无插值；装饰回放只做 cull-allow；Pop 先校验后弹桶；RoofAssociations 后置重建；空 prefab 走 `InsertTreeCmd` sentinel；FlatRoof 发 `SetFlatRoofStyleCmd`。
- world 帧尾部：flattened → blur → final_post(4×MSAA) → menu/painterly/vermeer/logo → wireframe/ui/mei/cursor/loading；导出改为 `"msaa color out"`，present 走 MSAA resolve。

### 10.3 最终验证（Windows 真树）

- `cargo check -p country-core` → Finished
- `cargo check -p tiny-glade` → Finished
- `cargo test -p country-core --lib` → **2510 passed / 0 failed**
- `cargo test -p tiny-glade --lib` → **37 passed / 0 failed**

### 10.4 仍缺

1. G1 仍显式 U：meadow 1×0x88 spawn、grading 生产调用者、鼠标深度/自由相机输入、firefly 通道、half_cube 几何、FMOD owner；未二次复核。
2. G2 跨 crate 接缝：FlatRoof consumer 在 system_roof、树 sentinel 解析在 tree-tool、stairs 音频只读 kind/position。
3. G3 未恢复：T11 session、T12 LUT 画廊、T13 反馈、T16 ui_2d、T21 egui；`load_glade_grading` 缺失时 tail 返回命名硬错误。
4. GPU 真机仍未验证（项目后置）。

### 10.5 数字修订

- `startup`：55% / 40% → **70% / 62%**。
- `systems/history`：68% / 50% → **78% / 70%**。
- `systems/render_loop`：66% / 55% → **68% / 62%**。
- country-core 总体（按行加权）：74% / 62% → **约 75% / 64%**。

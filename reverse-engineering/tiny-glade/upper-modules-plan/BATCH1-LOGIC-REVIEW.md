# 批次 1 业务逻辑复核（BATCH1-LOGIC-REVIEW）

复核对象：批次 1 六个模块（water / glade-border / timberframe / forest / tree-tool / roof）的模块内部分析报告。
复核人：业务逻辑复核 agent。日期：2026-10-14。
被复核报告：`upper-modules-plan/systems/<mod>/ANALYSIS.md`（6 份）。

## 口径与证据

- 只复核**模块内部实现**：不展开 caller/callee 链，不评价与其他 crate 的接线。
- 覆盖核对基线 = `skeleton/B/crates/systems/<crate>/src/*.rs` 中该 crate 命名空间的**全部 PDB 发射符号**（含 `inline_instance` 内联体）。已剔除依赖单态化（water/lib.rs 里的 `zeno::`、`core::str::` 等）与派生胶水。
- 复核标注沿用报告口径：**O**=原指令/PDB 直接支持；**I**=结构/上下文推断；**U**=未知。
- 抽查：本复核用只读 MCP（`ghidra --worker shared`）fresh 反编译了 4 个函数核对关键结论：`replay_tree`(0x141bc34f0)、`update_water_audio_grid`(0x141baefe0)、`place_tree`(0x142269970)、`ui_delete_tree`(0x141bd2510)、`despawn_trees`(0x1422686d0)。

---

## 1. 复核范围与结论摘要

| 模块 | B 本 ns 符号数（含内联/闭包） | 报告覆盖 | 遗漏 | 多余 | 逻辑清楚度 | 算法标注 | unknown 合理性 |
|---|---:|---:|---|---|---|---|---|
| water | 8 | 8 | 0 | 0 | 好（1 处标签属推断） | 已标注 | 合理 |
| glade-border | 5 | 5 | 0 | 0 | 好 | 已标注 | 合理 |
| timberframe | 10 | 10 | 0 | 0 | 好 | 已标注 | 合理 |
| forest | 9 | 8 | **1**（`is_too_close_to_shape`） | 0 | 好 | 已标注 | 合理 |
| tree-tool | 24 | 24 | 0 | 0 | 好 | 已标注 | 合理 |
| roof | ≈45（37 本 ns 函数 + 闭包） | ≈38 | **6 项**（见 §2.6） | 0 | 好 | 已标注（复用标注到位） | 合理 |
| **合计** | **≈101** | **≈93** | **7 项遗漏** | **0** | — | — | — |

结论摘要：

- 6 份报告整体质量**高**：函数级输入→处理→输出讲得清楚，算法普遍点名并标注了现成实现（roof 对 `reconstruction/roof/*`、core `recovered_building/*` 的「可复用」标注尤其到位），unknown 清单诚实。
- 发现 **7 处符号遗漏**（forest 1 + roof 6），其中多数是内联 helper 或无独立地址文件，部分超出对应 `TASKS.md` 的任务范围，但按「crate 命名空间全部发射函数」口径仍属遗漏。
- 发现 **4 处需裁定的冲突/不一致**（§4），其中 water 计数器语义冲突与 tree-tool/forest 的 `TreeAnimation` 步长冲突经 fresh 反编译核对，可给出裁定方向。
- 发现 **3 处轻微过度断言**（§4 末），均为把「推断的语义命名」写成「O 常量」的措辞问题，不改变数值结论。
- 关键更正（tree-tool `TreeEdit` 判别 0=Insert/1=Move/2=Delete）经双侧 fresh 反编译**确认成立**。

---

## 2. 每模块覆盖核对

> 表内「B 函数」= 骨架 `skeleton/B/crates/systems/<crate>/src/*.rs` 中的符号；「覆盖」= ANALYSIS.md 是否逐函数讲清；「算法」= 是否点名算法/现成实现；「unknown」= 报告 unknown 是否合理。

### 2.1 water（crate `system_water`）

| B 函数（符号） | 形态 | 报告位置 | 覆盖 | 逻辑清楚 | 算法标注 | unknown |
|---|---|---|---|---|---|---|
| `impl$0::build`（WaterPlugin::build） | compiled | §1 | ✔ | ✔ 清楚 | 无自研（注册胶水） | ✔ |
| `draw_water::draw_water` | compiled | §2 | ✔ | ✔（AppMode 分支标签属 I，见 §4） | ✔ lerp/射线求交/圆刷/边界包含/状态机 | ✔ |
| `draw_water::closure$0` | compiled | §2 | ✔ | ✔（signifier 实体装配） | ✔ | ✔ |
| `draw_water::closure$1` | inline | §2 | ✔ | ✔（写 WaterDrawEvent） | ✔ | ✔ |
| `draw_water::closure$2` | inline | §2 | ✔ | ✔（写 WaterEraseEvent） | ✔ | ✔ |
| `draw_water::closure$3` | inline | §2 | ✔ | ✔（write_default，类型 U） | ✔ | ✔ |
| `update_water_audio_grid` | compiled | §3 | ✔ | ✔ | ✔ 滚动窗位掩码/抖动/三重剔除 | ✔（含冲突，见 §4） |
| `record_water_history` | inline | §4 | ✔ | ✔（框架清楚） | 无自研（EventReader 消费） | ✔ |

遗漏 0；多余 0。注意：`lib.rs` 中 `zeno::*`、`core::str::*` 符号是依赖单态化，报告未纳入，**正确**。

### 2.2 glade-border（crate `system_glade_border`）

| B 函数 | 形态 | 报告位置 | 覆盖 | 逻辑清楚 | 算法标注 | unknown |
|---|---|---|---|---|---|---|
| `display_glade_border::display_glade_border` | compiled | §1 | ✔ | ✔ | ✔ 地形抬升/DebugLine/signifier | ✔ |
| `display_glade_border::closure$0` | inline | §2 | ✔ | ✔（`xvy` swizzle 已证） | ✔ | ✔（无） |
| `display_glade_border::closure$1` | compiled | §3 | ✔ | ✔（虚线实体） | ✔ | ✔ |
| `feedback_input_out_of_border` | inline | §4 | ✔ | ✔（去抖 + PlayUiSound#4） | ✔ | ✔ |
| `impl$0::build` | compiled | §5 | ✔ | ✔ | 无自研 | ✔ |

遗漏 0；多余 0。§6 把两处共用的 bevy `EventReader`/`Events` 内联记账单列，归属判断正确（依赖复用）。

### 2.3 timberframe（crate `timberframe`）

| B 函数 | 形态 | 报告位置 | 覆盖 | 逻辑清楚 | 算法标注 | unknown |
|---|---|---|---|---|---|---|
| `impl$0::add_timberframe` | compiled | §2.1 | ✔ | ✔ | 无自研（注册） | ✔ |
| `timberframe_on_glade_unload` | inline | §2.2 | ✔ | 体 U（诚实登记） | — | ✔ |
| `timberframe_beam_count_change_audio::impl$1::default` | inline | §5.1 | ✔ | 体 U | — | ✔ |
| `timberframe_beam_count_change_audio` | compiled | §5.2 | ✔ | ✔（计数差分触发） | ✔ | ✔ |
| `timberframe_circle::construct_timberframe_circle` | compiled | §4.1 | ✔ | ✔ | ✔ 弧长采样/下限 6 | ✔ |
| `timberframe_circle::closure$0` | inline | §4.2 | ✔ | 体 U | — | ✔ |
| `timberframe_circle::closure$1` | inline | §4.2 | ✔ | 体 U | — | ✔ |
| `timberframe_circle::closure$2` | inline | §4.2 | ✔ | 体 U | — | ✔ |
| `timberframe_rectangle::construct_timberframe_rectangle` | compiled | §3.1 | ✔ | ✔ | ✔ 等距布梁 floor(len/1.51)+lerp | ✔ |
| `timberframe_rectangle::closure$0` | inline | §3.2 | ✔ | 体 U | — | ✔ |

遗漏 0；多余 0。常量（1.51/1.5/0.2/0.18/下限 6）经 read_memory+disassemble 双证，可信。

### 2.4 forest（crate `forest`）

| B 函数 | 形态 | 报告位置 | 覆盖 | 逻辑清楚 | 算法标注 | unknown |
|---|---|---|---|---|---|---|
| `impl$0::build` | compiled | §1.1 | ✔ | ✔ | 无自研 | ✔ |
| `manage_tree_prefabs::impl$0::default`（MemCache::default） | inline | 名单 | ✔（仅登记） | — | — | ✔ |
| `manage_tree_prefabs::manage_tree_prefabs` | compiled | §2.1 | ✔ | ✔ | ✔ 栅格采样/空间哈希/RNG/pack_2x16f | ✔ |
| `manage_tree_prefabs::closure$0` | inline | 名单（作 closure_env_0 墙查询回调） | ✔ | 体 U | — | ✔ |
| **`manage_tree_prefabs::is_too_close_to_shape`（源行 211）** | inline | **未出现** | **✘ 遗漏** | — | — | — |
| `tree_animation::handplaced_tree_animation` | compiled | §3.1 | ✔ | ✔ | ✔ smoothstep/射线-盒 | ✔ |
| `tree_animation::handplaced_tree_animation::closure$0` | inline | §3.2 | ✔ | 体 U | — | ✔ |
| `tree_animation::should_remove_tree_canopy_from_view` | inline | §3.3 | ✔ | 体 U | — | ✔ |
| `tree_animation::forest_tree_animation` | compiled | §3.4 | ✔ | ✔ | ✔ 一阶推进/缓动/延迟计时 | ✔ |

**遗漏 1**：`forest::manage_tree_prefabs::is_too_close_to_shape`（源行 211，`inline_instance, instances=2`）。骨架依赖注解显示它使用 `glam::Vec2::distance/dot/sub` + `std::f32::sqrt`，是一个「到某形状的距离」谓词（很可能即 §2.1 所述「墙体邻近判定」的命名函数体，或另一处放置有效性谓词）。报告只描述了「墙体邻近」逻辑却未登记该命名符号，应补。
其余：报告 §2.1 称「墙查询回调 closure_env_0」，但骨架中墙查询回调用的是 `closure$0`（源行 118），命名与 `is_too_close_to_shape` 的对应关系需澄清。

### 2.5 tree-tool（crate `system_tree`）

| B 函数 | 形态 | 报告位置 | 覆盖 | 逻辑清楚 | 算法标注 | unknown |
|---|---|---|---|---|---|---|
| `impl$0::build`（TreePlugin::build） | compiled | lib.rs | ✔ | ✔ | 无自研 | ✔ |
| `process_tree_cmds::process_tree_cmds` | compiled | ✔ | ✔ | ✔ Move→Insert→Delete | ✔ | ✔ |
| `process_tree_cmds::despawn_trees` | compiled | ✔ | ✔ | ✔（阈值 <0.01） | ✔ | ✔ |
| `convert_preplaced_trees::convert_preplaced_trees` | compiled | ✔ | ✔ | ✔（延迟帧计数门控） | ✔ | ✔ |
| `convert_preplaced_trees::closure$0/$1` | inline | ✔ | ✔ | 体 U | — | ✔ |
| `replay_tree::replay_tree` | compiled | ✔ | ✔ | ✔ | ✔ 回放桶/时序 | ✔ |
| `recalc_tree_position_on_terrain_height_change` | compiled | ✔ | ✔ | ✔ | ✔ | ✔ |
| `tree_move_bend::tree_move_bend` | compiled | ✔ | ✔ | ✔（rate 值 U） | ✔ 指数缓动 | ✔ |
| `hide_trees::hide_trees` | compiled | ✔ | ✔ | ✔ | ✔ 位图/点包含/空间哈希 | ✔ |
| `hide_trees::closure$0` | inline | ✔ | ✔ | 体 U | — | ✔ |
| `ui_place_tree::random_tree_prefab` | compiled | ✔ | ✔ | ✔ | ✔ fastrand | ✔ |
| `ui_place_tree::ui_place_tree` | compiled | ✔ | ✔ | ✔ | ✔ | ✔ |
| `ui_place_tree::closure$0/$1` | inline/compiled | ✔ | ✔ | 体 U | — | ✔ |
| `ui_place_tree::seed_cursor_icon` | inline | ✔ | ✔ | 体 U | — | ✔ |
| `ui_place_tree::place_tree` | compiled | ✔ | ✔ | ✔ | ✔ RNG/门控 | ✔ |
| `ui_move_tree::ui_move_tree` | compiled | ✔ | ✔ | ✔ | ✔ 拖拽状态机 | ✔ |
| `ui_move_tree::impl$1::default`（TreeMoveCache::default） | inline | ✔ | ✔ | 体 U | — | ✔ |
| `ui_move_tree::closure$0` | compiled | ✔ | ✔ | 体 U | — | ✔ |
| `ui_move_tree::closure$1` | inline | ✔ | ✔ | 体 U | — | ✔ |
| `ui_move_tree::remap_pos_y_if_in_water` | inline | ✔ | ✔ | 体 U | — | ✔ |
| `ui_delete_tree::ui_delete_tree` | compiled | ✔ | ✔ | ✔ | ✔ 擦除半径点包含 | ✔ |
| `startup_tree_tool::reset_tree_tool` | inline | ✔ | ✔ | ✔（default 覆写） | 无自研 | ✔ |

遗漏 0；多余 0。卡点 A/B（TreeEdit 判别、PlaceTreeCmd 载荷）经 fresh 反编译**确认**（见 §4）。

### 2.6 roof（crate `system_roof`）

报告覆盖（✔）：`create_roof`(+closure$0)、`edit_roof`(+closure$0)、`delete_roof` 三函数、`update_roof_spatial`(+closure$0/get_shape_params)、`roof_record_history`、`replay_roof`(+impl$0::process)、`calculate_roof_trigger_volumes`(+closure$0/1)、`simulate_roof_tile_for_audio`(+simulate_roof_tile)、`wall_height_dt`、`assemble_roof_tiles`(+closure$0/1)、`generate_roof_stone_floor_and_roof_bottom`、`place_roof_spires`(+closure$0)、`switch_roof_type`(+stone_floor_bundle)、`roof_animation`、`ui_edit_roof`/`do_ui_edit_roof`、`ui_edit_roof_ridge_dir` 族、`ui_edit_roof_ridge_length` 族、`ui_edit_roof_ridge_and_height_focused`、`add_roof_systems`、`add_roof_events_and_resources`、`utils::components::register_required_components`（impl$0/1）。

**遗漏 6 项**（报告 0 次提及）：

| 遗漏符号 | 形态 | 说明 |
|---|---|---|
| `highlight_roof::highlight_roof` | inline（源行 7） | 屋顶高亮；`add_roof_events_and_resources` 里提到 `RoofHighlight` 资源但未分析本函数 |
| `lib.rs::gpu_flush_buffers` | inline（源行 149） | 插件 GPU flush；未登记 |
| `ui_edit_roof_tip::ui_edit_roof_tip::closure$0` | inline（源行 81） | **整个文件 `ui_edit_roof_tip.rs` 未覆盖** |
| `visual::place_support_beams::place_roof_support_beams`（closure$1） | inline（源行 95） | **整个文件未覆盖**；仅 `register_required_components` 提到 `RoofSupportBeams` marker |
| `visual::roof_snow::gen_roof_snow_mesh`（closure$1/2/6） | inline | **整个文件未覆盖**；报告把「雪」并入 `roof_animation` 描述，但 `roof_snow.rs` 是独立文件 |
| `ui_edit_roof::update_wall_height_if_needed`、`ui_edit_roof_ridge_dir::get_gizmo_info` | inline helper | 报告未点名（属内联 helper） |

部分覆盖：`utils::assemble_rectangular_roof` / `assemble_circular_roof` 仅作「被调用 helper」点名（各 1 次），未逐段分析——鉴于 CPU 铺瓦已在 `reconstruction/roof/*` 完成、`TASKS.md` 明确「不重复恢复」，此**处理正确**。

说明：上述多数遗漏项在 `roof/TASKS.md` 中也未列任务（TASKS 只列 20 项），故遗漏属「报告覆盖范围 = TASKS 范围」与「crate 全符号」之间的口径差；但按本复核的「全部 B 发射函数」口径仍应登记。

---

## 3. 逻辑总清单（本批次全部函数：逻辑一句话 + 实现方式 + 对应算法）

### water
| 函数 | 逻辑一句话 | 实现方式 | 算法 |
|---|---|---|---|
| `WaterPlugin::build` | 注册 record_water_history / update_water_audio_grid 两系统、插 WaterRaster、注册 3 事件 | Bevy 插件胶水 | 无 |
| `draw_water` | 按 AppMode 分派橡皮/水笔，把光标笔触翻译为水面编辑事件并上传 staging | 状态机+事件发射+core `draw_raster` | 笔刷半径 lerp、光标射线→XZ 求交、圆刷几何、边界包含 |
| `draw_water::closure$0` | 构造笔刷圆符 signifier 实体（"brush_circle_scribble"） | signifier builder 链 | 无 |
| `draw_water::closure$1/2/3` | 分别写 WaterDrawEvent / WaterEraseEvent / Default 事件 | EventWriter 内联 | 无 |
| `update_water_audio_grid` | 逐帧用滚动窗位掩码标记活跃音频格，做距离/水位/遮挡三重剔除并计数 | 位掩码+抖动+剔除 | 64 帧滚动窗、帧位哈希 11c+11、8 级锯齿抖动、双线性采样、射线遮挡 |
| `record_water_history` | 消费水面编辑事件、取最新一条转交 core `record_raster_history` | EventReader 消费 | 无 |

### glade-border
| 函数 | 逻辑一句话 | 实现方式 | 算法 |
|---|---|---|---|
| `display_glade_border` | 非 AppMode(19) 时把边界 2D 轮廓抬升成 3D 折线并画 DebugLine，可选发周长 signifier | 地形采样+debug 线段+signifier | 2D→3D 抬升、bevy Events 记账、DebugLine |
| `closure$0` | 对每个轮廓点采样地形高度合成 Vec3 | 内联 map 闭包 | swizzle `xvy` |
| `closure$1` | 构造虚线 signifier 实体（"dashed_line"，DashedLineLength=f×0.7） | signifier builder + insert 命令 | 虚线参数 |
| `feedback_input_out_of_border` | 收到「输入越界」事件时播放 PlayUiSound#4，Local 去抖 | EventReader+Local 去抖 | 事件去抖 |
| `GladeBorderPlugin::build` | 向 Update 注册两系统链并注册两事件 | Bevy 插件胶水 | 无 |

### timberframe
| 函数 | 逻辑一句话 | 实现方式 | 算法 |
|---|---|---|---|
| `add_timberframe` | 插两资源、注册事件、Update 挂 rect/circle/audio 链、OnExit 挂清理 | Bevy 插件胶水 | 无 |
| `timberframe_on_glade_unload` | 场景卸载清理 TimberframeInstanceBuffer | 内联（体 U） | 无 |
| `TimberframeBeamCount::default` | 木梁计数资源默认初始化 | 内联（体 U） | 无 |
| `timberframe_beam_count_change_audio` | 木梁数 +1 时投递 PlayTimberframeSound（含尺寸/位置） | 计数差分 | 计数差分触发 |
| `construct_timberframe_rectangle` | 沿矩形墙生成垂直/等距/裙板/屋脊木梁实例写 WoodenPlankSsbo | 事件驱动实例生成 | 弧长采样、等距布梁 floor(len/1.51)+lerp、法线→四元数、高度分层、屋脊几何 |
| `construct_timberframe_circle` | 沿圆周生成垂直/裙板/顶环木梁实例 | 同上 | 圆周均匀采样、floor(perimeter/1.5) 下限 6 |
| rect/circle 各 closure | 单根木梁朝向/实例组装、点收集 | 内联 | — |

### forest
| 函数 | 逻辑一句话 | 实现方式 | 算法 |
|---|---|---|---|
| `ForestPlugin::build` | 插 InteractableForestPlacements、三系统链挂 Update | Bevy 插件胶水 | 无 |
| `manage_tree_prefabs` | 每帧判放置点有效性（栅格/岩掩码/墙体邻近）并 spawn/标记删除树实体 | 事件驱动实体生命周期 | 双线性采样、rock mask 求和 clamp、空间哈希圆查询、fastrand、pack_2x16f |
| `is_too_close_to_shape`（遗漏） | 判某点是否过近某形状（Vec2 距离谓词） | 内联 | Vec2::distance/dot/sqrt |
| `handplaced_tree_animation` | 按 AppMode 推进淡出/生长进度→smoothstep，相机射线×触发盒求交，打包输出 | 逐树进度+剔除 | smoothstep、射线-盒求交、pack_2x16f |
| `should_remove_tree_canopy_from_view` | 判树冠是否从视图剔除 | 内联谓词（体 U） | — |
| `forest_tree_animation` | 逐树一阶生长/收缩（延迟计时+2×delta），缓动乘幅度输出，衰减到 0 发 DespawnPrefabCmd | 逐树推进 | 一阶推进+clamp+缓动 |

### tree-tool
| 函数 | 逻辑一句话 | 实现方式 | 算法 |
|---|---|---|---|
| `TreePlugin::build` | 注册资源+7 事件+系统集，OnExit 挂 reset | Bevy 插件胶水 | 无 |
| `process_tree_cmds` | 按 Move→Insert→Delete 顺序消费命令，维护 TreeStorage/位置、spawn/标记实体、发音效 | 事件双缓冲迭代+哈希表 | 双线性采样、SIMD 字符串比较 |
| `despawn_trees` | TreeAnimation 进度 <0.01 时发 DespawnPrefabCmd（payload=entity） | 查询+阈值比较 | 阈值比较 |
| `convert_preplaced_trees` | 延迟数帧后把预置树转运行时树，水/岩掩码门控，记历史起始编辑 | 延迟帧计数门控 | 双线性采样、空间哈希、from_iter 映射 |
| `replay_tree` | 从 ReplayBuffer 按 GameTime 推进，按 TreeEdit 判别重发命令事件并回收桶 | 回放桶+时序 | tag=8 判别、延时阈值、逆序 remove |
| `recalc_tree_position_on_terrain_height_change` | 地形变化时重采样高度写回树 y（±65 边界裁剪） | 事件门控+采样 | 地形高度采样 |
| `tree_move_bend` | 拖动时把 TreeMoveBend 向目标指数缓动，距离≥0.1 时归一化位移写回 | 逐实体缓动 | 指数时间连续 lerp、距离钳制 |
| `hide_trees` | 按区域/近墙命令把树打隐藏位并（条件）发 DeleteTreeCmd+历史 | 位图迭代+点包含 | FixedBitSet 迭代、圆/矩形点包含、空间哈希 |
| `random_tree_prefab` | 门控后随机抽树 prefab 名 | fastrand | wyrand 门控 |
| `ui_place_tree` | 种树工具态内/外判定，内发 PlaceTreeCmd（位置 Vec3），外发反馈事件 | UI 状态机+raycast | 边界判定 |
| `place_tree` | 消费 PlaceTreeCmd，随机 scale/rotation，组 InsertTreeCmd + TreeEdit(Insert) 历史 | RNG+事件组装 | fastrand、门控 rand<0.6 |
| `ui_move_tree` | 拖拽状态机发 MoveTreeCmd，松手记 TreeEdit(Move) 历史 | 状态机+raycast | 地形采样、边界判定 |
| `ui_delete_tree` | 擦除工具态把半径内树发 DeleteTreeCmd + TreeEdit(Delete) 历史 | 位图+点包含 | 擦除半径点包含 |
| `reset_tree_tool` | 装载退出时把 TreeStorage 置 default | default 覆写 | 无 |

### roof（主干）
| 函数 | 逻辑一句话 | 实现方式 | 算法（现成实现） |
|---|---|---|---|
| `create_roof` | 权威墙态→生成 Roof 组件 bundle | 权威状态→组件生成 | Roof::new 参数块（`surface.rs`） |
| `edit_roof` | 按 64B 事件体应用 5 类标量编辑+反馈事件 | ECS 后段 | `edit_rules.rs::apply_edit`（可复用） |
| `process_invalid_roof_notifications` | RoofIsInvalid→构造 Delete 历史条目 | 事件→历史 | — |
| `mark_roof_for_deletion` | 给 roof 插 RoofMarkedForDeletion | Commands insert | — |
| `delete_roofs` | RoofAnimation 删除进度到 0 时 despawn | 查询+阈值 | core `deletion_finished`（可复用） |
| `update_roof_spatial` | 更新 roof 空间/尺寸缓存、gable 尖顶 Y 偏移 -0.69 | 缓存更新 | — |
| `roof_record_history` | 延迟打包 VisualEdit 历史条目 | 延迟历史提交 | `replay.rs::visual_edit_from_observed`（可复用） |
| `replay_roof` / `impl$0::process` | 帧回放调度 + RoofReplayCmd 状态机（progress 混合） | 回放状态机 | `replay.rs::advance*`（可复用） |
| `calculate_roof_trigger_volumes` | 屋顶曲面采样→三角化触发体（矩形 4 剖面段/圆 15 环段） | 曲面采样+三角化 | `surface.rs` 采样（部分复用） |
| `simulate_roof_tile_for_audio` | 瓦片高度指数衰减/风驱动模拟→音频 tag4/5 | 逐瓦模拟 | core `RoofTileSimulation`（可复用） |
| `wall_height_dt` | 墙高变化增量传播到屋顶 | 增量缓存 | 无 |
| `assemble_roof_tiles` | 脏 roof→分发 CPU 铺瓦写 RoofTileSsbo | 脏跟踪+分发 | `pipeline.rs::assemble_roof_tiles_observed`（可复用） |
| `generate_roof_stone_floor_and_roof_bottom` | 圆形 roof 石地板/底面实例（log2(radius*3) 细分） | 实例生成 | 无 |
| `place_roof_spires` | 收集尖顶数据→GPU 上传 | 收集+上传 | `building-roof-spires` 证据 |
| `switch_roof_type` | 屋顶类型切换换 mesh/bundle | 视觉重建 | 无 |
| `roof_animation` | 每 roof 两 f32 向边界衰减（季节/雪） | 逐 roof 推进 | core `RoofAnimation::advance`（可复用） |
| `ui_edit_roof`/`do_ui_edit_roof` | 构建屋顶编辑 slider3d 控件+三角拾取+拖拽反解 | UI 编辑 | `edit_rules.rs`（可复用） |
| `ui_edit_roof_ridge_dir` 族 | 脊向控件/拖拽/切换/记历史 | UI 编辑 | `ridge.rs`（可复用） |
| `ui_edit_roof_ridge_length` 族 | 脊长控件/反解/历史 | UI 编辑 | `ridge.rs::inverse_ridge_01_param`（可复用） |
| `ui_edit_roof_ridge_and_height_focused` | 聚焦态脊/高度编辑 | UI 编辑 | `ridge.rs`/`edit_rules.rs`（可复用） |
| `add_roof_systems` / `add_roof_events_and_resources` | 注册系统/事件/资源 | Bevy 插件胶水 | 无 |
| `register_required_components`（impl$0/1） | RoofSpire/RoofSupportBeams 组件合同注册 | Bevy derive | Bevy 0.16 derive（可复用） |

---

## 4. 冲突与需裁定项（含建议方向）

| # | 冲突 | 证据（复核核对） | 建议裁定方向 |
|---|---|---|---|
| C1 | **water `update_water_audio_grid` 计数器步长**：报告指出活跃格计数对 `+0x10` 做 `±d`（派生增量对）；core 既有叶子 `recovered_systems/water.rs::apply_audio_bit` 建模为 `±1` | fresh 反编译确认：活跃时 `*(u16*)(unit+0x10) += (sVar14,sVar16)`（`+d`）、非活跃时 `-=`；`d` 由 `frame_bit` 派生，∈{-7,-5,…,7}² | **以原指令为准：计数为 `±d`（非 ±1）**。core 叶子 `apply_audio_bit` 的 `±1` 建模应视为**需修正**；但改动属 core 侧，按批次约束**不在本批次做**，登记为 core 任务（音频网格消费者语义）。裁定前不得用 ±1 实现驱动水声 |
| C2 | **tree-tool `TreeEdit` 跨系统判别值**：旧草稿 `tree-tool-rest-analysis.md` 记「1=Move、2=Insert、其余=Delete」；报告更正为 **0=Insert / 1=Move / 2=Delete** | fresh 反编译双侧确认：读侧 `replay_tree` 对 `TreeEdit+0x10`：`==2` 写 Delete 事件（in_R9）、`==1` 写 Move（in_R8）、else 写 Insert（in_RDX）；写侧 `place_tree` disc=0、`ui_delete_tree` disc=2 | **采纳报告更正：0=Insert/1=Move/2=Delete**（读侧 + 写侧一致）。旧草稿作废。接线时以此为准 |
| C3 | **`TreeAnimation` 组件步长不一致（跨报告）**：tree-tool 记 `TreeAnimation` size **0x10**、f32 进度 @+4；forest `handplaced_tree_animation` 记 `TreeAnimation` 元素 stride **0x18**（[+4] 进度、[+8] 第二值、[+0xc] 标志） | fresh 反编译 `despawn_trees` 确认元素 stride **0x10**、`f32@+4` 为进度阈值字段（与 tree-tool 一致）。forest 报告的 0x18 无独立核对证据，且 forest 内部 §3.1（[+4] 进度）与 §3.4（[+0xc] 进度）字段偏移自相矛盾 | **倾向 tree-tool 的 0x10 正确**；forest 需重新核对 `handplaced_tree_animation` 查询组件的步长/字段偏移（可能把 `PrefabInstanceData` 或输出数组步长误当 `TreeAnimation`）。登记为 forest 侧待复核项 |
| C4 | **tree-tool `tree_move_bend` 的 `time_continuous_lerp` rate 值**：报告记 U，旧草稿 radare2 记 5.0/3.22，本轮反编译未显式暴露 XMM rate 实参 | 报告诚实标 U；未在本轮复核展开 | 保持 U；建议下一轮用 radare2/定点反汇编显式取 `time_continuous_lerp` 调用点的 XMM rate 实参裁定（5.0 vs 3.22） |
| C5 | **forest 报告 vs TASKS 前提「风/季节曲线」**：`TASKS.md` 称两动画函数是「风/季节摆动」；报告指出原指令未见风力场/季节相位，进度由 `GameTime.delta` 线性推进 | 与报告描述一致（smoothstep/一阶推进，无正弦场） | **采纳报告**：本 crate 无显式风/季节算法；TASKS 描述应更正为「生长/淡出+相机剔除动画」。风场若存在应在 shader/其它系统（不在本 crate） |

### 过度断言（unknown 写成确定结论，均为措辞级）

| # | 位置 | 问题 | 建议 |
|---|---|---|---|
| O1 | water §2 `draw_water` | 把 `AppMode=9（橡皮）`/`AppMode=15（水笔）` 以加粗常量列出，读作 O；数值 9/15 是 O，但「橡皮/水笔」语义命名是 I | 标注 9/15 为 O、语义标签为 I |
| O2 | water §1 | 「`draw_water` …其注册在别的插件/集成层」属模块外断言 | 改为「不在本插件注册（build 内无第三次 add_systems）」即可，删去对模块外落点的断言 |
| O3 | roof §0 | `RoofEdit` 判别「(4=FlatRimStyle)」带括号、读作已证 | 明确 4 为 I（推断），或补原指令证据 |

其余报告的 U 项登记均诚实（如 timberframe 的 `PlayTimberframeSound` variant 6、forest 的 `flag`「界外=保留」语义、roof 的 `-0.69` 含义等），未见把明显 unknown 写成结论的情形。

---

## 5. 遗留 unknown 汇总（按模块）

**water**：音频网格规模/维度、单元 `+0x14` 字段、单元基址写入方（core 侧）；`RecordWaterEditToHistoryCmd` 字段/载荷；`record_water_history` 取最后一条 vs 逐条 read；`draw_water` DrawState 四相位语义、closure$3 事件类型、水笔分支 core 字段归属；`WaterPlugin::build` 的 `PathRaster/DirtyTilePublisher` 插入方式、add_systems 标签；`intersects_any` 的 `0x32a3` 语义。

**glade-border**：AppMode 判别 0x13 变体名；`GladeBorder`/`ShapeParameters` 内部布局（core）；`GameConfig` 偏移 0x121 语义；栈参 [RBP+0x130] 资源身份及 +0x2c 字段；`DebugLine::color`=1 枚举含义、`DrawTopology` 写入值语义；`ShowDemoBorderOverlay` 载荷；`PlayUiSound` variant#4、`InputOfOutBorderCache` 字段；closure$1 捕获 f32 来源、`DashedLineLength` 布局；插件 SystemSet 名。

**timberframe**：实例字段 `0x0000000200000000` 语义；`TimberframeConstructionParamsCache` +8/+0xc 业务名；事件 `+0x1c` 门控语义；两资源完整布局与 default 初值；`PlayTimberframeSound` 字段/variant 6、音频系统参数寄存器映射；矩形/圆是否有源级共享 helper；各内联闭包逐指令体；1.51 vs 1.5 间距不一致是否有意；SystemSet 标签/chain 顺序。

**forest**：`build` 注册系统顺序与 SystemSet 名；`MemCache` 布局；`flag`「界外=保留」语义；`PlacementDataSsbo::position()` 与各字段名；`AL` 布尔来源；6 个栅格与 water/garden/path 逐一对应；`insert::<InteractableForestTree>` 字段；`TreeAnimation` 组件完整布局（见 C3）；AppMode 15 路分支语义；相机线段几何常量；`should_remove_tree_canopy_from_view` 判定；`TreeCanopyMaxCullDistance` 用法；缓动变体（静态首字节 0x06）；每树幅度/计时器初值来源；输出数组（stride 0x30）所属组件；`-0.3/-0.4` 时间尺度语义。

**tree-tool**：`TreeEdit` 各变体完整字段布局；`PlaceTreeCmd` 精确 x/y/z 顺序（I）；glade 外空载荷事件身份；`HideTrees*Cmd` 载荷字段语义、隐藏位图归属、@+0x24 门控；`convert_preplaced_trees` 完整签名/arg2 事件类型/arg3、arg7 归属；`tree_move_bend` rate（见 C4）；`ActiveSession[+0x58]` 类型；`TreeAnimation` f32@+4 字段语义；`TreeMoveCache` 布局；`EraserSizeMultiplier` 语义；SpawnPrefabCmd/InsertTreeCmd 部分字段；`TreeStorage::insert` 内部 hash 插入；`Update` add_systems 元组精确顺序；`recalc` ±65 边界语义。

**roof**：create_roof 外层查询组件/事件队列资源名/uVar18 枚举/local_250；edit_roof ECS filter/事件队列资源名；delete 族 RoofIsInvalid 产生条件/标记来源；update_roof_spatial 墙 +0x25c..0x26c 字段、-0.69 含义、get_shape_params 内联体；roof_record_history 源 0x98 字段名/16B 前缀；replay_roof +0x1d 置位分支、sync_point_count；trigger_volumes 的 RoofTriggerVolume 容器类型、apply_roof_noise、RoofBoundsDims::as_rectangle、gable_sides；simulate 的 lVar8+0x28 来源、tag4/5 音频事件；wall_height_dt 字段语义；assemble_roof_tiles 脏集合 key；generate_stone_floor 的 NaniInstanceDesc kind9、矩形分支去向；place_spires 尖顶几何/SSBO 布局；switch_roof_type 的 RoofType A/B 名；roof_animation 两 f32 语义名/0.35/0.3；UI 编辑族 action 枚举名、若干 core/utils 依赖（project_cursor_onto_ray、xz_projective_rotation_bad 等）、SignifierFactory 输出；register_required_components 的 `#[require(...)]` 列表；add_roof_systems 系统顺序。

---

## 6. 对分析质量的整体评价与后续建议（仅模块内部实现）

**整体评价**：批次 1 六份报告达到「模块内部逻辑可复核」的可用标准。突出优点：

1. **输入→处理→输出** 三段式清楚，关键常量普遍经 `read_memory`+反汇编双证（timberframe 常量表、water 常量地址表尤佳）。
2. **算法点名 + 复用标注** 到位：roof 明确把 `edit_rules.rs`/`replay.rs`/`pipeline.rs`/`ridge.rs` 标注为「可复用」而非「重新推导」，符合项目「差分仅作校验、复用已恢复实现」的约束；forest/tree-tool 的 `smoothstep`/`time_continuous_lerp`/`fastrand` 亦点名。
3. **unknown 诚实**：大量内联闭包/无独立地址体明确标 U，未伪造实现。
4. 关键更正（tree-tool `TreeEdit` 判别）经复核**成立**，且报告主动标注了与旧草稿的冲突。

**需改进**：

1. **符号遗漏**：forest 漏 `is_too_close_to_shape`；roof 漏 `ui_edit_roof_tip`、`place_support_beams`、`roof_snow`、`highlight_roof`、`gpu_flush_buffers` 及两个内联 helper。建议报告增设「本 crate 命名空间全符号清单」小节，逐符号标注「已分析 / 内联待析 / 依赖复用 / 超 TASKS 范围」，避免覆盖口径差被误读为已全覆盖。
2. **措辞**：把推断语义（如 AppMode 工具名、枚举变体名）与 O 数值分开标注，避免过度断言。
3. **跨报告一致性**：`TreeAnimation` 步长 0x10 vs 0x18 需统一（见 C3）；建议后续对共享 core 组件（TreeAnimation、RoofAnimation、WaterAudioGrid 单元）建立单一「组件布局事实表」，各模块引用同源。
4. **后续复核建议**：C1（音频计数 ±d）、C3（TreeAnimation 步长）、C4（lerp rate）三项建议各自用定点反编译补一次原指令证据后落定；C1 的 core 侧修正按批次约束登记为 core 任务，不在本批次改。

（本复核不主张任何模块已实现/验收完成；仅就模块内部逻辑、覆盖、算法与 unknown 给出核对结论。）

# terrain-DELIVERY — 实现交付（2026-10-16，C 组实时复合 agent）

> 对照 terrain-DELTA 的合成 agent 应关闭缺口逐项落地。文件：`crates/country-core/src/systems/terrain/*`、
> `resources/terrain_editing.rs`（组内）。共享文件改动已获中央仲裁批准：`startup/run_game.rs`
> 生产接线（批准）、`resources/history/replay_cmd_buffer.rs` Terrain 变体（批准）、
> `EditType::Terrain` tag=15 登记"待取证"（不改共享）。
> 逐项分离：**真实代码接线 / candidate 链测试 / 原侧验证 / 仍缺项**。

## 1. terrain_hover_idx 门控修正（DELTA #8）

- **真实代码接线**：`systems/terrain/terrain_hover_idx.rs` 重写为 decomp 0x140aff720 的
  两分支门控（行 75-90）：
  - 高度 ≤ 0（`water.level <= 0.0`）：**仅当 id == -1 才 return**（高度≤0 且 id 有效要写）；
  - 高度 > 0：悬停位门（新增 `MouseTerrainHoverReadback::hovered`）+ id 有效 +
    `distance <= 0.5`（`HOVER_DISTANCE_GATE`）任一不满足 → return；
  - 命中 → `HoveredTerrainStrokeId { tag=1, id }`。
- **原侧验证**：decomp-0x140aff720.txt 行 75-90 逐分支对照（`fVar12<=0.0` 分支仅 `iVar11==-1`
  return；else 分支悬停位/id/dist 三门；`_UNK_142925870`=0.5 已 RNE 核实）。
- **candidate 链测试**：无独立单测；门控逻辑为纯函数分支，可经 World 资源注入验证
  （宿主注入 `MouseTerrainHoverReadback`/`TerrainWaterSample`）。
- **仍缺项**：`FUN_140969720` 真实地形资源查找（串名 0x6c/vtable 高度读）仍以适配资源
  `MouseTerrainHoverReadback`/`TerrainWaterSample` 承载（DELTA §2 行 38-57，宿主注入）。

## 2. ui_draw_terrain 状态机（DELTA #9）

- **真实代码接线**：`systems/terrain/ui_draw_terrain.rs`：
  - **tag 0xc 界外反馈事件无条件发射**（decomp 行 97：`FUN_14092df50(..,&UNK_142adc2f0,0xc,3,2,..)`），
    与画布注册无关；`TerrainOutOfBorder { kind:3, sub:2 }`；
  - 画布注册三道门（高度≤0/未悬停/距离>0.5 → 不注册）→ `TerrainCanvasRegister{layer:0,enabled:1}`；
  - **结尾状态机**（decomp 行 134-165）：事件 subtype `cvar9`（点按=3，默认 2）+ 上帧状态
    `prev` + `inside` 折叠为状态字节 `uVar11`（0..3）写回 `TerrainDrawState::state`；
  - **new_stroke 移到点按分支**（`cvar9 == 3 && inside`）才启动笔画 + `EvTerrainStrokeChanged`
    （修正"每帧 new_stroke"水分）；
  - DrawState 8B 元素记录仅在非界内时 push（修正"无条件 push(0)"）。
- **原侧验证**：decomp-0x140a9fdc0.txt 行 73-165 对照；状态机跳表 `&UNK_142adc498` 目标
  函数 Ghidra UNRECOVERED_JUMPTABLE（分支 0/1/3 未恢复）。
- **candidate 链测试**：无独立单测；`TerrainDrawState::state` 状态机为可测纯函数分支。
- **仍缺项**：跳表分支 0/1/3 的目标函数行为（点按分支的 new_stroke 已落地，其余待取证）；
  `FUN_140ad2970`（is_shape_inside）真实边界判定依赖宿主 UI 画布 widget（登记待接）；
  `func_0x000140c97540` 光标→世界变换未复现（沿用 cursor + height 采样）。

## 3. replay_terrain + TerrainReplayCmd（DELTA #10）

- **真实代码接线**：`systems/terrain/replay_terrain.rs`：
  - `TerrainReplayCmd::forward/backward`（0x140994f30/0x140995040）：duration =
    `clamp(Σ相邻点距*0.01, 0.4, 3.0)`，无点默认 0.3（RNE 常量 0x3e99999a/0x3ecccccd/
    0x3c23d70a/0x40400000 全核实）；断言 `stroke_length.is_finite()`（@0x142a94c34）；
  - `replay_terrain` 系统（0x140995150 语义）**消费共享 `ReplayBuffer` 前桶的
    `ReplayCommand::Terrain`**（中央仲裁批准接入 `replay_cmd_buffer.rs`）：
    instant 路径（duration<0.4 → eased=1 立即插入点集 + next_index = max(id+1, 原值)）；
    动画路径 `eased=1-(1-t)^3` 渐进 reapply；undo → 移除笔画；`elapsed += dt*speed`；
    完成逆序移除；
  - `make_instant` 接入 `ReplayBuffer::make_animations_instant` 分支。
- **原侧验证**：decomp-0x140995150.txt 行 71-199（instant/动画路径）、decomp-0x140994f30.txt
  （duration 计算）；0.3/0.4/0.01/3.0/1.0 常量 RNE 直读核实。
- **candidate 链测试**：`adjacent_distance_sum`/`eased`/`TerrainReplayCmd::forward` 为纯函数，
  可加单测（未及加，登记）。
- **仍缺项**：`ReplayBucket::expand` 对 `EditType::Terrain` 返回
  `UnrestoredReplayOwner`（共享 TerrainEdit 无点集载荷；真实点集 TerrainReplayCmd 由地形
  回放 producer 直接入桶）；共享 `TerrainEdit::revert/reapply`（0x140858d30/0x1408592d0，
  CurveU 进度）属共享 terrain_edit owner，本模块以 `TerrainStroke` 点集 + 完整 revert/
  渐进 reapply 的 CPU 侧模型落地（待 owner 恢复后对齐）。

## 4. ui_edit_terrain 16 位脏掩码 + 台阶/decorator Bucket（DELTA #4）

- **真实代码接线**：`systems/terrain/ui_edit_terrain.rs`：
  - 脏掩码扩展为 **16 位掩码模型**（`DIRTY_MASK_KNOWN = HEIGHT|FALLOFF|POINTS`），位 0/1/2
    语义已映射，其余 13 位参数位 **U/待取证**（decomp-0x140aef1f0 行 270-285）；
  - `StairsPlacementEntry` + `StairsPlacementBucket` 资源（0x28B 元素：tag@0、float@+4、
    对齐字节@+8、双 DecoratorId@+0xc、+0x1c 颜色/角、+0x20 计数）= **terrain→stairs/
    decorator 消费链** CPU 侧接口；`ui_edit_terrain` 消费并登记收集面，stairs 域消费后清空。
- **原侧验证**：decomp-0x140aef1f0.txt case 2（indexmap Bucket push，0x28B 元素布局）。
- **仍缺项**：16 位掩码其余 13 位的参数位语义（U 待取证）；台阶放置数据的 UI 输入
  生产面（宿主 UI 接线）；`get_last_mut_stroke`/`insert_or_morph_last_point`
  （0x140924900/0x140a97880）消费待接。

## 5. terrain 系统生产注册（DELTA #11）

- **真实代码接线**：`startup/run_game.rs::register_core_startup` 已接入（**中央仲裁批准**）：
  `register_terrain_events(world)` + `add_terrain_cpu_systems(schedule)` +
  `add_terrain_tool_systems::<()>(schedule)`（含 replay_terrain、terrain_hover_idx、
  ui_draw_terrain、ui_edit_terrain、ui_terrain_focus_mode、history_record_terrain）。
  运行期缺资源（AppMode/History/TerrainHeightsData/SSBO）由宿主提供，不伪造。
- **仍缺项**：原版 add_terrain_systems 0x1409455f0 的精确 28-系统顺序未复现（本地顺序）；
  原 plugin 资源初始化与全局调度待取证。

## 汇总

- **真实接线**：terrain_hover_idx 两分支门控、ui_draw_terrain 状态机 + tag 0xc 事件 +
  点按启动笔画、replay_terrain 系统 + TerrainReplayCmd。
- **仍缺项**：jumptable 分支 0/1/3 待取证；is_shape_inside/光标→世界变换宿主面；
  `ReplayCommand::Terrain` 与 EditType::Terrain tag（共享文件仲裁中）；生产注册（仲裁中）。
- **原侧验证**：全部基于 decomp-0x140aff720/a9fdc0/995150/994f30 + RNE 常量直读。
- **编译**：组内文件无编译错误；整 crate `cargo check` 被其它组在制改动阻塞（中央已协调）。

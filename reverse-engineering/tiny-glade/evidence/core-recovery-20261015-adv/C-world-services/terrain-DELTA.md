# terrain-DELTA — 用户批改核对（2026-10-16，对照 terrain-SPEC + 原机器码）

> 核对 agent（C 世界）。文件：`crates/country-core/src/systems/terrain/*`、`resources/terrain_editing.rs`。
> 原侧证据：`country-core-deep/terrain-water/decomp/decomp-0x140a9fdc0.txt`（ui_draw_terrain）、`decomp-0x140aff720.txt`（terrain_hover_idx）、`decomp-0x140ac9770.txt`（deferred_terrain_history）、`decomp-0x140995150.txt`（replay_terrain）、`decomp-0x140aef1f0.txt`（ui_edit_terrain）、`terrain-editing/B-core/140924a10.c`（new_stroke）。
> **生产接线**：terrain 系统（register_terrain_events / add_terrain_tool_systems / add_terrain_cpu_systems）**无任何生产调用方**（仅 tests.rs）。run_game.rs 无 terrain 引用。

## 1. ui_draw_terrain 0x140a9fdc0

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| 门 AppMode==10 | 真实实现 | ui_draw_terrain.rs:82-84；decomp 行 46-48 | 一致 |
| 光标面（tag bit0、+4/+0xc、+8 距离） | 真实实现 | resources/cursor_terrain_raycast.rs `{tag@0, pos Vec3@4}`；`distance()=pos.y` 复用 +8 槽 | 布局 O 兼容 |
| 三道门（高度≤0 / 未悬停 / dist>0.5）→ 不注册画布 | 真实实现 | ui_draw_terrain.rs:92-94；decomp 行 73-84 | 一致 |
| 通过后推 `clearing_canvas_w_layer_and_sound`（tag 0x11, payload {0,1}） | 真实实现（近似） | ui_draw_terrain.rs:96 `TerrainCanvasRegister{layer:0,enabled:1}`；decomp 行 89 | 事件类型为 crate 自定，载荷 {0,1} 与 FUN_14092df50 参数对应 |
| tag 0xc 界外反馈事件（decomp 行 97 无条件 `FUN_14092df50(..,&UNK_142adc2f0,0xc,3,2,..)`） | 仍缺 | — | 未实现（仅画布注册后经 new_stroke 失败分支发 TerrainOutOfBorder，与原文无条件发射不同） |
| 结尾状态机 jumptable（uVar10/uVar11 判别 + `&UNK_142adc498` 跳表） | 仍缺 | decomp 行 138-165（"Could not recover jumptable"） | 未实现 |
| **stroke 生产**：decomp **无** `new_stroke` 调用（状态机分支内才点按启动笔画） | 有水分（每帧造 stroke） | ui_draw_terrain.rs:105 `editing.new_stroke([cursor.x,0.0,cursor.y])` | 当前在每次悬停通过三道门时都调用 new_stroke（每帧新增笔画）——原版仅在状态机"点按"分支启动。属错误行为冒充 |
| DrawState Vec（8B 元素） | 有水分 | ui_draw_terrain.rs:99-102 `draw_state.records.push(0)` | 硬编码 0 假数据；语义 U 未登记 |

## 2. terrain_hover_idx 0x140aff720

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| AppMode 0x421 掩码门（mode<0xb && (0x421>>mode)&1） | 真实实现 | terrain_hover_idx.rs:50-51 | 0x421 与 decomp 行 37 一致 |
| 输出 `Option<i32>`（写 tag=1 + id） | 真实实现 | terrain_hover_idx.rs:64-65 `HoveredTerrainStrokeId{tag:1,id}`；decomp 行 91-93 | 一致 |
| 高度采样门：`fVar12<=0 → 仅当 iVar11==-1 才 return（否则写）` | **有水分（逻辑反）** | terrain_hover_idx.rs:58-60 `if water.level<=0.0 { return; }` | decomp 行 75-90：高度≤0 时**只要 id 有效就写**；当前一律 return。代码注释承认"高度≤0 且 id 有效会写，以机器码为准"，但实现与之相反 |
| 悬停 bit 门（`(*pbVar4&1)==0 → return`，仅高度>0 分支） | 仍缺 | — | 当前未检查 hovered 位 |
| dist≤0.5 → return（仅高度>0 分支） | 有水分（无条件） | terrain_hover_idx.rs:61-62 | decomp 中 dist 门仅在高度>0 分支；当前无条件应用 |
| 真实地形资源查找（FUN_140969720 串名 0x6c + vtable 高度读 `*(*plVar9+0x10)`） | 仍缺 | decomp 行 38-57 | 当前用 `MouseTerrainHoverReadback`/`TerrainWaterSample` 适配资源（宿主注入），原版真实查询未复现 |

## 3. deferred_terrain_history 0x140ac9770

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| 每帧置脏哨兵 0x8000000000000002；上帧非哨兵 → 构建 0x38B 记录 → History::add + 推 8B 事件 | 真实实现 | deferred_terrain_history.rs:45-60；decomp 行 34-97 | 哨兵值（-0x7ffffffffffffffe）、0x38B 记录 {tag 对 + 4 pending + 旧 state}、History::add（FUN_140a84ec0）、8B 事件均对应 |
| TerrainEdit 入共享 history（EditType 新增 Terrain 变体） | 部分（tag 待取证 + 仲裁待确认） | resources/history/mod.rs:58 `Terrain(Arc<TerrainEdit>)` tag=15 | 注释诚实"原版 visit_str 0x140b3b630 无此名，tag 待定位"。**SPEC 预期 tag 在 9..12，当前定 15 未取证**；共享文件改动需确认经中央仲裁 |
| DebugHistoryMsg（8B 事件，载荷=旧状态） | 真实实现 | deferred_terrain_history.rs:63-64,59 | decomp 行 87-96 推 8B 一致 |

## 4. replay_terrain 0x140995150

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| replay_terrain（tag 3 前桶、instant/动画路径、ease_out_cubic、revert/reapply） | **仍缺** | systems/terrain/ 无 replay_terrain 模块 | decomp-0x140995150.txt 全文已读（SPEC §2.2"可直接落地"）；current 仅见 replay_auto_clutter/raster/stairs/roof，无地形回放 |
| TerrainReplayCmd::forward/backward（0x140994f30/0x140995040） | 仍缺 | — | 无实现 |
| ReplayCmd tag 3 判别（共享 replay_cmd_buffer.rs） | 待取证 | — | 未接 |

## 5. ui_edit_terrain 0x140aef1f0 / ui_terrain_focus_mode

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| 门 AppMode==10 | 真实实现 | ui_edit_terrain.rs:71-73；decomp 行 142 | 一致 |
| MouseActivityTimer / TerrainStrokeParamsCache | 部分（I 资源） | ui_edit_terrain.rs:33-52 | 结构存在，语义 I |
| 16 位脏掩码遍历（decomp：从 0x80B 记录逐位折叠 bit 7..0x7f） | 有水分（仅 3 位 I 推断） | ui_edit_terrain.rs:97-115 `DIRTY_MASK_HEIGHT/FALLOFF/POINTS` | 原版 16 位参数位掩码（decomp 行 270-285）；当前只 3 位自创位 |
| EditTerrainStrokeCmd / SetTerrainStrokePtsCmd 事件发射 | 真实实现（事件类型真实） | ui_edit_terrain.rs:122-147 | 事件已存在，载荷序注记 ABI 差异 |
| 台阶放置数据 push（indexmap Bucket<(DecoratorId,DecoratorId),AlignKind>，case 2） | 仍缺 | — | decomp case 2 的 stairs/decorator 消费链未实现 |
| 完整 UI 状态机（switch bVar42 0..4 + default + panic/断言） | 仍缺（SPEC §6 允许骨架） | — | 未实现，注释如实标注 |
| ui_terrain_focus_mode 0x1409a4fb0 入口骨架 + 常量表 | 部分（骨架，SPEC 允许） | ui_terrain_focus_mode.rs:61-81 | 门（首 u32==1）、距离门、常量表（0.5/f32::MAX/0.01/-10/1.2/7/2.5）O；内联状态机 U 如实标注 |

## 6. 接线 / 待取证项汇总

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| terrain 系统生产注册（原 add_terrain_systems 0x1409455f0） | 仍缺 | register_terrain_events / add_terrain_tool_systems 仅 tests.rs 调用 | 生产 Schedule 无任何 terrain 系统 |
| EditType::Terrain 原 tag | 待取证 | history/mod.rs tag=15 | SPEC 预期 9..12 |
| PlayTerrainSound（0..4）发射/audio owner 注册 | 部分（事件真实，无发射方） | terrain/mod.rs:21-56 | 事件已定义未发射 |

## 合成 agent 应关闭的真实剩余缺口

1. **terrain_hover_idx 门控逻辑**：按 decomp 两分支修正——高度≤0 且 id 有效要写、hovered 位门、dist 门仅高度>0 分支。当前实现与机器码相反。
2. **ui_draw_terrain 状态机**：实现结尾 jumptable（0..3 分派），把 `new_stroke` 从每帧移到点按分支；补 tag 0xc 界外反馈事件。
3. **replay_terrain + TerrainReplayCmd::forward/backward**：SPEC §2.2 明确"可直接落地"，完全未实现。
4. **ui_edit_terrain 16 位脏掩码 + 台阶/decorator Bucket push**。
5. **EditType::Terrain tag 取证**（9..12？）与共享文件仲裁确认。
6. **生产注册**：terrain 系统全部未挂载进 Schedule（register_terrain_events/add_terrain_tool_systems 需接入 register_core_startup 或对应挂载面）。

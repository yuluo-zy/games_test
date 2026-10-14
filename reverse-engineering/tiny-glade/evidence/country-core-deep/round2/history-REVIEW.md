# history 域逐文件审查（round2，2026-10-14，agent tg_a1）

对照基准：`evidence/country-core-deep/history/DIGEST.md`（O 条目）+ v2 索引
（sysres-procedures-v2.json，已对齐 S_PUB32）+ 本轮 closure 结果
（round2/history-CLOSURE.md）。状态：ok = 与证据一致；需修 = 有具体错误；
未知 = 无证据可裁决（占位/边界不扣分，但登记）。

## resources/history/

| file | 状态 | 具体错误（代码写 X，证据为 Y） | 缺失项 |
|---|---|---|---|
| mod.rs | ok | 布局表/方法行为与机器码一致。文档内残留两处旧名：布局表证据列写 "new_from_params 0x140a844f0"（真名 insert_starting_edit，正文已用真名，属文档不一致）；`add_decorator_diff` 保留 2 参签名（真 4 参：diff + stairs + remap），`remove_snapshot_record` 注释"委托 FUN_140d79f90 + prev 回调"无证据（CLOSURE：该名无独立符号，疑似内联） | `closest_history_snapshot` free fn（重建自造名；真逻辑在 save::snapshot 域，见 CLOSURE）；History 信封 serde（内联，见 CLOSURE） |
| history_chain.rs | ok | — | — |
| replay_cmd_buffer.rs | 需修 | 文档头 "ReplayBuffer 环形 {head,buf,cap,len,modification_counter}" 无证据；证据（replay_pop_bucket 0x140b38250）：头部 = 桶 Vec（cap-first，元素 0x40）@+0x00..+0x18 + 活跃计数 @+0x18。`pop_front` 实现里 expect 信息 "on empty buffer" 自造（U）。`current_replay_cmd`/`replay_active` 等辅助语义 U | ReplayBuffer 精确字段布局；桶 0x40B 的 @+0x28..+0x40 字段 |
| decorator_edit.rs | 需修 | ① `is_default_lantern_brightness(_record: &DecoratorRecord)->bool` 签名错——证据（0x140b3f910 机器码）：`(f32)->bool`，语义 `x == 1.0f`，且与 `is_default_stair_width` 同址同码（ICF）。② `convert_before_after_states_to_changes` 地址写 0x140b31220——真址 0x140b3fab0；真签名 `(out:*mut Change, before:*const State, after:*const State)`（按 style 字节分变体比较浮点、输出 changed 布尔 @+0x40/+0x52），非 `(&self)->Vec<Gap>` 方法。③ `is_default_chimney_smoke` 0x1408c5680：地址有效（ICF 多符号同址，含 is_default_allow_connection 等 ~12 个谓词），但本域无法单独裁决其语义（U 保留） | DecoratorHistoryState 的 0x50B 记录字段名 |
| wall_history.rs | 需修 | `ev_height_changed` 建模为 `(&mut self, WallId)->Option<EditType>`——证据：它是 History 的 trait 方法，真流程 = `EditType::new_height_changed(buf, wall, h0, h1)`（0x140a83fb0，写 24B 记录 {1,1,niche+2,wall,h0,h1} + 0x108B blob + FUN_141696f50）→ `History::add`（track-caller 文件参数）；无 Option 返回。`EditType::new_height_changed()` 无参重建也错（真 4 参构造器，写 niche 判别 0x8000000000000002） | ev_new_wall 的 0x108B blob 内字段布局（wall_index/roof_shape_params/bend_vector/pad0/custom0/custom1 已证，完整布局 U） |
| wall_compound_edit.rs | 需修 | ① `panic_deprecated_operation` 地址写 0x140a83120——真址 0x140a82e20。② `build` 注释地址（sysres 0x1414022c0）错——真址 0x140a82d50；0x1414022c0 = WallHistoryEdit serde Deserialize 族。③ 缺 `update_compatibility`（CLOSURE：候选区间 0x140a82940-0x140a82b20 全为 drop glue，疑似内联，无独立符号） | CompoundEditBuilder 内部状态布局；update_compatibility 语义 |
| terrain_edit.rs | 需修 | ① `duration` 证据列 "(0x140858d30 区)"——0x140858d30 = revert（0.99f 阈值 + panic "Requested terrain stroke {id}"）；duration 公式 O 在 `TerrainReplayCmd::forward` 0x140994f30 内联（clamp(Σ点距*0.01, 0.4, 3.0)、空 0.3、"stroke_length.is_finite()" 断言）；无独立 duration 符号（CLOSURE）。② `revert`/`reapply` 空壳 + "领域 apply 边界" 注释——证据已有可写行为（hashbrown 48B 元素表查 stroke id、清 active 标志），可升级为 O 骨架 | update_terrain_curve 0x140859740 的完整行为（本轮已解出：hash 查表 + 点集拷贝 + fold + 全局时间 lerp 进度，见 CLOSURE） |
| auto_clutter_edit.rs | ok（U 为主） | serialize 0x140a18320 / deserialize 0x140a18500 与 v2 一致 ✓ | 负载字段 |
| color_edit.rs | ok（U 为主） | 字段名（Decorator/Terrain/chimney_h_offset/allow_connection）与 0x1417bf8a0 证据一致 | 目标枚举闭合性 |
| path_edit.rs | 需修 | serde 地址注释写 "0x16e8340 / 0x16e8510"（丢高位 1）——真址 0x1416e8340（serialize）/ 0x1416e8510（deserialize）。`RasterEditSerialized` 的 From（0x140a976b0）未建模 | `impl$1::from` 0x140a976b0 语义 |
| stairs_history_edit.rs | 需修 | `StairsHistoryDiff` 建模为 {before/after Vec<StairGraphEdge>, hash_before/after}（JSON schema），但运行时 `StairsDiffTable` 96B = {tag0,tag1,4×u64, 两个 Vec<u32> 边对}（cap-first，cap@+0x30/+0x48），两型混用未区分。`is_empty` 注释 "容器 +0x30 非空" 对（原版读 cap），但实现读 before/after len（等价、字段不同） | 运行时表与 JSON 型的映射 |
| tree_edit.rs | 需修 | serialize 0x17c2600 / deserialize 0x17c2f60 丢高位——真址 0x1417c2600 / 0x1417c2f60。`missing_prefab(_prefab:&str)->Self` 签名 U（v2 有 0x1409e6210 符号，参数面未证） | missing_prefab 参数面 |
| （roof_edit.rs） | 缺失 | 文件不存在（wp13 合同列 12 模块，重建缺 roof_edit）。证据已有：serde Serialize 0x140259e30 / Deserialize 0x140b38b80、visit_str 0x140b392d0（操作 tag FlatRoofRimStyleChange/SwitchRoofRidgeDir）、RoofEdit::serialize 0x140259e30 | 整个 roof_edit 模块 |

## systems/history/

| file | 状态 | 具体错误（代码写 X，证据为 Y） | 缺失项 |
|---|---|---|---|
| mod.rs | ok | AppMode::Replay=0xd ✓；系统地址表与 v2 一致 | — |
| history_process.rs | 需修 | ① `HistoryCmd::undo()=0x00000000 / redo()=0x00000001` 自造命令字——证据仅 interrupt()=0x02000200（O 机器码）；undo/redo 的完整命令字（byte[0] 族）U。② 重建 `process_history_commands` 缺：undo 时命令 Vec 反转、Arc clone 两表、桶 push、第二事件流（16B 记录 byte[+9] 跳表 0x142ae5420）、ReplayCmd tag drop 分派（0x142ae545c）、"Can't process History while recording…" log | 第二事件流消费；generate_replay_cmds_from_edit 真共享核心（4 单态均 316B 跳板尾调 0x141443xxx） |
| history_recording.rs | 需修 | ① 文件名/注释：0x1409dd790 = `history_finish_recording`（v2 + S_PUB32 一致），DIGEST 首轮曾误标 history_start_recording，重建文件两函数都挂同一地址。② `history_start_recording` 的真地址无独立符号（U）。③ 系统参数面（多事件游标 + AppMode 检查，见反编译）未建模 | history_start_recording 真址 |
| replay_wall.rs | ok（骨架） | WallReplayEditCmd::process 0x140abfb30 自递归 8 参（cmd, dt*速度, &mut Mut, …）已证；重建注释一致 | process 内部 vtable 0x142ab4910 分派细节 |
| replay_terrain.rs | 需修 | forward/backward 输出布局注释 "+0x8=2 方向位" 与机器码不符——证据（0x140994f30）：`{ptr@+0x00, u32@+0x08=0, duration f32@+0x0c, u32@+0x10=2}`（+0x10=2，+0x08=0） | ease_out 系数歧义（U） |
| replay_stairs.rs | ok（骨架） | 判别 10、undo 分支、PlayStairsAudioCmd 与证据一致 | 边集 insert/remove 的精确表结构 |
| replay_auto_clutter.rs | ok（骨架） | 判别 9、undo 分支与证据一致 | — |
| replay_pop_bucket.rs | ok（骨架） | 前桶空判定（cmds len@+0x10==0）、pop_front、装饰/楼梯双 Arc 应用链与机器码一致（FUN_14155ba30 / 1408ff210→140900ba0→14157cec0→140b50cc0→140b50d60→1408fefe0 链已 O 登记） | 收尾 FUN_140b2cb30/140b743e0/1408cd5c0 的语义 |
| replay_raster.rs | ok（骨架） | panic "!cmd.edit.stroke.points.is_empty()"（replay_raster.rs）✓；6 单态地址与 v2 一致。`RasterReplayCmd::new` 的 duration=0.2 默认值 U（证据 forward 的默认 0.3 是无点分支） | undo 字节（payload+0x3？）与 +0x19/+0x1a 标志 |
| replay_path.rs | 需修 | `generate_path_replay_cmds` 注释地址 "0x14016286d0" 丢高位 1——真址 0x1416286d0（跳板：niche 判别 dec/cmp 4 过滤后尾跳 0x1414433xx 共享核心）。Cobblestone3 用 discrim==2 与 Cobblestone2 共用（U：真实判别分界未证） | 各 PathFilter 的判别字节映射 |
| replay_garden.rs | 需修 | GardenFilter 用 `ReplayCmd::Path(e)=>Some(e)` 占位（注释自认 "U 判别歧义"）；真实现 = replay_raster<Garden…> 单态（0x1409f6ff0）的闭包 env，判别映射 U | 花园/水面的判别字节 |
| replay_water.rs | 需修 | 同上（0x1409f4650 单态；生成宿主 0x141444c70） | 同上 |
| replay_mode.rs | ok | AppMode!=0xd 早退、"Replay mode started/finished" log、+0x28=1.0f 写与证据一致 | HistoryCmd 命令字变体（0x03000100 等，注释自认 U） |
| finish_replay.rs | ok（骨架） | 与反编译一致（pending 恰 1 条 redo → unfreeze、双空 → restore AppMode、MetaCommander 哨兵） | toolbar switch_mode 条件细节（+0x80/+0x88） |
| signifier_replay_active.rs | ok（骨架） | ease_out_elastic_jiggle 公式、enum2$ 判别、register_required_components 0x140b1d680 与证据一致 | — |

## 汇总

- ok：9 文件（mod.rs、history_chain、auto_clutter_edit、color_edit、systems/mod、
  replay_wall、replay_stairs、replay_auto_clutter、replay_pop_bucket、
  replay_raster、replay_mode、finish_replay、signifier_replay_active）
- 需修：11 文件（replay_cmd_buffer、decorator_edit、wall_history、
  wall_compound_edit、terrain_edit、path_edit、stairs_history_edit、tree_edit、
  history_process、history_recording、replay_terrain、replay_path、replay_garden、
  replay_water）——错误类型集中为：① 地址丢高位 1（0x16e8510→0x1416e8510 类）4 处；
  ② sysres-v1 错名/错址沿用的注释 5 处；③ 签名与机器码不符 3 处
  （is_default_lantern_brightness、convert_before_after_states_to_changes、
  ev_height_changed/new_height_changed）；④ 文档头布局与证据冲突 1 处
  （ReplayBuffer"环形"）。
- 缺失：roof_edit.rs 整文件；update_compatibility 未建模（证据侧也无独立符号）。

# crates/systems/tree-tool/src

状态：部分草稿。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

六个文件的命令/删除/回放/地形跟随/弯曲/预置树恢复草稿。

## 接线与验证

接线：not_established。

reva/RNE反汇编交叉核对与语义草稿；未登记完整编译/原码差分/系统验收。

## 已恢复的函数和类型范围

- process_tree_cmds Move→Insert→Delete
- despawn_trees/replay_tree
- recalc_tree_position_on_terrain_height_change/tree_move_bend/convert_preplaced_trees

## 明确缺口

- PlaceTreeCmd载荷和TreeEdit跨系统判别值
- 完整UI/startup/hide_trees及真实World consumer

下一闭包：PlaceTreeCmd载荷和TreeEdit跨系统判别值

## 证据

- [evidence/multi-baseline/tree-process-cmds-analysis.md](../../evidence/multi-baseline/tree-process-cmds-analysis.md)
- [evidence/multi-baseline/tree-tool-rest-analysis.md](../../evidence/multi-baseline/tree-tool-rest-analysis.md)
- [evidence/multi-baseline/tree-tool-extra-analysis.md](../../evidence/multi-baseline/tree-tool-extra-analysis.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/tree-tool/src/convert_preplaced_trees.rs` | A,B,C | [reconstruction/tree-tool/src/convert_preplaced_trees.rs](../../reconstruction/tree-tool/src/convert_preplaced_trees.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/tree-tool/src/hide_trees.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/tree-tool/src/lib.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/tree-tool/src/process_tree_cmds.rs` | A,B,C | [reconstruction/tree-tool/src/process_tree_cmds.rs](../../reconstruction/tree-tool/src/process_tree_cmds.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/tree-tool/src/recalc_tree_position_on_terrain_height_change.rs` | A,B,C | [reconstruction/tree-tool/src/recalc_tree_position_on_terrain_height_change.rs](../../reconstruction/tree-tool/src/recalc_tree_position_on_terrain_height_change.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/tree-tool/src/replay_tree.rs` | A,B,C | [reconstruction/tree-tool/src/replay_tree.rs](../../reconstruction/tree-tool/src/replay_tree.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/tree-tool/src/startup_tree_tool.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/tree-tool/src/tree_move_bend.rs` | A,B,C | [reconstruction/tree-tool/src/tree_move_bend.rs](../../reconstruction/tree-tool/src/tree_move_bend.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/tree-tool/src/ui_delete_tree.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/tree-tool/src/ui_move_tree.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/tree-tool/src/ui_place_tree.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/tree-tool/src/convert_preplaced_trees.rs

[reconstruction/tree-tool/src/convert_preplaced_trees.rs](../../reconstruction/tree-tool/src/convert_preplaced_trees.rs)

SHA256：`cc035422e0fca5a216d95151efb5d3c1f3c2a54f1fdce4a231ce961d16247442`；缺口注释标记数：14。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `convert_preplaced_trees` | 32 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L13：//!   门控 → EventWriter 发转换命令（疑似 DespawnPrefabCmd，unknown）+
- L19：//! 等为 country_core 资源。字段布局部分证据，其余 unknown。
- L31：/// 字段/参数类型部分为 unknown。
- L36：//   事件类型 unknown（疑似 DespawnPrefabCmd——移除被转换的预放置预制体）。
- L46：// arg7 [E+0x38]：[unknown] 资源（+0x10 存指针被写、+0x1c 取值）。
- L67：/// [evidence] SpawnPreplacedTrees 事件（plugin build 注册）；stride 与载荷 unknown。
- L71：/// 类型名 unknown（疑似 DespawnPrefabCmd）。
- L78：/// 资源归属 unknown。
- L86：/// [evidence] 水面/浮点栅格源（arg4；as_float_grid 采样上下文）。布局 unknown。
- L89：/// [evidence] TweakerView（arg5；as_inner_mut 0x14067dd80）。布局 unknown。
- L93：/// 实体表 +0x110/+0x118。完整布局 unknown（依赖墙系统）。
- L96：/// [unknown] arg7 资源。+0x10 存指针被写、+0x1c 取值。
- L105：/// 其余字段（@+0x10..0x48：名字/位置/长度等）布局按 analysis 文档，部分 unknown。
- L106：pub unknown: [u8; 0x38],
### reconstruction/tree-tool/src/despawn_trees.rs

[reconstruction/tree-tool/src/despawn_trees.rs](../../reconstruction/tree-tool/src/despawn_trees.rs)

SHA256：`36e963997f6a219a78b11440b4eb724fa2c142f61ea3fc450d546212a1ca205c`；缺口注释标记数：6。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `despawn_trees` | 49 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L16：//! country_core::components 组件，内部字段 unknown 处不伪造。
- L24：/// 其余字段 unknown。
- L26：/// [evidence] f32 @+4：动画进度/淡出值，< 0.01 触发销毁。字段名 unknown。
- L28：/// @+0 及 @+8..+0x10 未确认（unknown）。
- L29：pub unknown: [u8; 12],
- L33：/// Delete 分支经 Commands 推入；疑似零尺寸标记组件（unknown）。
### reconstruction/tree-tool/src/process_tree_cmds.rs

[reconstruction/tree-tool/src/process_tree_cmds.rs](../../reconstruction/tree-tool/src/process_tree_cmds.rs)

SHA256：`68b62d8df469902c146ca9fd016ce220d35dafadfbbc40c0c64f3dc4f0c60a85`；缺口注释标记数：16。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `next_index` | 46 | 未由扫描证明；以证据限定范围为准 |
| `insert` | 52 | 未由扫描证明；以证据限定范围为准 |
| `get` | 57 | 未由扫描证明；以证据限定范围为准 |
| `remove` | 60 | 未由扫描证明；以证据限定范围为准 |
| `process_tree_cmds` | 191 | 未由扫描证明；以证据限定范围为准 |
| `water_sample` | 274 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：//!   `unknown`。未确认字段保留 `unknown`，不伪造。
- L92：/// f32 @+0x20（unknown 语义）。
- L96：/// byte @+0x28（spawn 门控输入；unknown 语义）。
- L100：/// @+0x38..0x40 未读字段（unknown）。
- L107：/// 证据：position Vec3 @+0（12B）；f32 @+0xc（inverse_lerp 读取，unknown 语义）。
- L111：/// f32 @+0xc（unknown，疑似水面/高度缓存）。
- L118：/// 其余 unknown。
- L122：/// u64 @+0x10（unknown）。
- L126：/// f32 @+0x24（unknown）。
- L128：/// f32 @+0x28（unknown）。
- L132：/// f32 @+0x30（=1.0，unknown）。
- L134：/// 其余 @+0x34..0x48（unknown）。
- L141：/// 值（unknown：疑似树索引/实体；Insert 分支由 TreeStorage::insert 返回值写入）。
- L167：/// is_corrupted 方法门控种树。对象实际 Rust 类型 unknown。
- L169：/// [evidence] +0x58 处会话状态对象（内部布局 unknown）。
- L173：/// ActiveSession[+0x58] 处对象（仅确认三个查询方法行为，内部 unknown）。
### reconstruction/tree-tool/src/recalc_tree_position_on_terrain_height_change.rs

[reconstruction/tree-tool/src/recalc_tree_position_on_terrain_height_change.rs](../../reconstruction/tree-tool/src/recalc_tree_position_on_terrain_height_change.rs)

SHA256：`0210161d138a1f9ed5782cf30db3e6aba5a876d413d7206eedf97ea38ca8dea8`；缺口注释标记数：5。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `recalc_tree_position_on_terrain_height_change` | 62 | 未由扫描证明；以证据限定范围为准 |
| `sample_at_world_xz` | 93 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L29：/// tree_move_bend 写 step（见该文件）。其余字段 unknown。
- L33：/// [evidence] f32 @+0xc（unknown 语义）。
- L35：/// @+0x10..+0x24 未确认（unknown）。
- L36：pub unknown: [u8; 0x14],
- L41：/// @+0x2c..+0x30 未确认（unknown）。
### reconstruction/tree-tool/src/replay_tree.rs

[reconstruction/tree-tool/src/replay_tree.rs](../../reconstruction/tree-tool/src/replay_tree.rs)

SHA256：`401ee26e52c7ca887ed7432120affe8bacfc9397b61791e91fa78d037dae7fd1`；缺口注释标记数：14。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `front_mut_bucket` | 40 | 未由扫描证明；以证据限定范围为准 |
| `iter_mut` | 51 | 未由扫描证明；以证据限定范围为准 |
| `remove` | 54 | 未由扫描证明；以证据限定范围为准 |
| `replay_tree` | 120 | 未由扫描证明；以证据限定范围为准 |
| `read_u64` | 193 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L19：//! unknown）。GameTime delta 证据：f32 @+0x28（两处独立调用点一致）。
- L30：/// [evidence] f32 @+0x28 = 延时除数（回放倍速，语义 unknown）。
- L32：/// 内部双缓冲桶（front/back；细节 unknown）。
- L34：/// [evidence] f32 @+0x28：ReplayCmd.delay 的除数（回放倍速，语义 unknown）。
- L73：/// byte @+0x1c：flag（选两种负载布局；语义 unknown）。
- L80：/// 1 = Move、2 = Insert、其余 = Delete。字段布局部分证据，其余 unknown。
- L88：pub unknown: [u8; 0x60],
- L113：pub unknown: [u8; 0x38],
- L148：let index = read_u64(&edit.unknown, 0x30);
- L151：position: Vec3::ZERO, // [evidence] position 字段偏移见 TreeEdit（unknown）
- L158：name: String::new(), // [evidence] 复制 TreeEdit 名字；字段偏移 unknown
- L160：unknown: [0; 0x38],
- L165：let index = read_u64(&edit.unknown, 0x18);
- L194：0 // 依赖布局 unknown，草稿占位
### reconstruction/tree-tool/src/tree_move_bend.rs

[reconstruction/tree-tool/src/tree_move_bend.rs](../../reconstruction/tree-tool/src/tree_move_bend.rs)

SHA256：`6710a7f44212e7b342b704a568ed9b3e3e4725addd54f5a3a0ba6875eff2e5b6`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `tree_move_bend` | 48 | 未由扫描证明；以证据限定范围为准 |
| `time_continuous_lerp` | 93 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L24：/// 本轮证据：Vec2（stride 8）@+0/+4，每帧 lerp/step 写回。语义 unknown
- L36：/// 其余字段（@+0xc..+0x24、+0x2c..）unknown。
- L37：pub unknown: [u8; 0x18],
- L80：//     变则 [RDI+0x24] = step_x\|step_y<<32）。字段名 unknown。

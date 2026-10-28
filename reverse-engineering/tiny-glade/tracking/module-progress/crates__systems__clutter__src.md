# crates/systems/clutter/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

autoclutter区域/筛选/放置/实体管理、merlons/stitches、auto-clutter历史草稿接可编译crate。

## 接线与验证

接线：not_established。

clutter+audio check/build/test5/5，使用glue占位；未通过原作完整行为验证。

## 已恢复的函数和类型范围

- populate_autoclutter_regions/process_autoclutter_candidates
- add_* gates/windows/birdnests/lantern/tree
- manage_autoclutter_entities
- detect/construct_inter_roof_merlons、stitch_bricks
- AutoClutterEdit/replay与删除记录

## 明确缺口

- BrickTransformSsbo/StairsFloatingBrickParams字段与真实资源producer
- 22 TODO和3空位；动画/mesh加载/GPU bush读回与完整调度

下一闭包：BrickTransformSsbo/StairsFloatingBrickParams字段与真实资源producer

## 证据

- [evidence/multi-baseline/compile-clutter-audio.json](../../evidence/multi-baseline/compile-clutter-audio.json)
- [evidence/multi-baseline/clutter-placement-analysis.md](../../evidence/multi-baseline/clutter-placement-analysis.md)
- [evidence/multi-baseline/clutter-extra-analysis.md](../../evidence/multi-baseline/clutter-extra-analysis.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/clutter/src/autoclutter/add_autoclutter_around_gates.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/add_autoclutter_around_windows.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/add_autoclutter_on_windows.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/add_birdnests.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/add_lantern_water_setdresssing.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/add_preplaced_autoclutter.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/add_tree_vegetation_setdressing.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/autoclutter_animation.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/load_clutter_meshes.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/manage_autoclutter_entities.rs` | B,C | [reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/manage_autoclutter_entities.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/manage_autoclutter_entities.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/mod.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/populate_autoclutter_regions.rs` | B,C | [reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/populate_autoclutter_regions.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/populate_autoclutter_regions.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/process_autoclutter_candidates.rs` | B,C | [reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/process_autoclutter_candidates.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/process_autoclutter_candidates.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/clutter/src/autoclutter/read_bush_positions_from_gpu.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/add_birdnests.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/add_clutter_around_gates.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/add_clutter_around_windows.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/add_clutter_on_windows.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/add_lantern_water_setdresssing.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/add_preplaced_clutter.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/add_tree_vegetation_setdressing.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/clutter_animation.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/load_clutter_meshes.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/manage_clutter_entities.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/mod.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/populate_clutter_regions.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/process_clutter_candidates.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/clutter/read_bush_positions_from_gpu.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/inter_roof_merlons/construct_inter_roof_merlons.rs` | A,B,C | [reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/merlons.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/merlons.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/clutter/src/inter_roof_merlons/detect_inter_roof_merlons.rs` | A,B,C | [reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/merlons.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/merlons.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/clutter/src/inter_roof_merlons/inter_roof_merlons_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/inter_shape_stitches/detect_intra_shape_corners.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/clutter/src/inter_shape_stitches/stitch_bricks.rs` | A,B,C | [reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/stitches.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/stitches.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/clutter/src/lib.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs)

SHA256：`f0de6bb3e439d91cec459fa201eea0be133dd9f5cb36c378de2507cc435cb00d`；缺口注释标记数：8。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `delete` | 52 | 未由扫描证明；以证据限定范围为准 |
| `history_record_auto_clutter` | 62 | 未由扫描证明；以证据限定范围为准 |
| `start_input_recording` | 75 | 未由扫描证明；以证据限定范围为准 |
| `add` | 76 | 未由扫描证明；以证据限定范围为准 |
| `handle_rmb_autoclutter` | 89 | 未由扫描证明；以证据限定范围为准 |
| `from` | 116 | 未由扫描证明；以证据限定范围为准 |
| `ui_convert_autoclutter_to_decoclutter` | 125 | 未由扫描证明；以证据限定范围为准 |
| `replay_auto_clutter` | 142 | 未由扫描证明；以证据限定范围为准 |
| `erase_decorator_pair` | 201 | 未由扫描证明；以证据限定范围为准 |
| `unknown` | 207 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L22：//! 保留 unknown。
- L101：unknown(); // 编辑记录（stride 0x48）写入 indexmap
- L103：unknown(); // Rapier raycast 拾取链
- L126：unknown();
- L190：/// +0x20/+0x28 unknown。
- L203：unknown();
- L207：fn unknown() -> ! {
- L208：unimplemented!("反汇编级证据：精确逻辑见 evidence/multi-baseline/clutter-extra-analysis.md")
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/add_rules.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/add_rules.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/add_rules.rs)

SHA256：`db1ee6708a06ff9bf434466327e9211143c770cfe23a4c7c5d204786205e6c42`；缺口注释标记数：8。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `add_autoclutter_around_gates` | 31 | 未由扫描证明；以证据限定范围为准 |
| `gate_index` | 51 | 未由扫描证明；以证据限定范围为准 |
| `hash_gate_placement` | 56 | 未由扫描证明；以证据限定范围为准 |
| `add_autoclutter_around_windows` | 69 | 未由扫描证明；以证据限定范围为准 |
| `add_autoclutter_on_windows` | 89 | 未由扫描证明；以证据限定范围为准 |
| `add_birdnests` | 102 | 未由扫描证明；以证据限定范围为准 |
| `add_lantern_water_setdresssing` | 128 | 未由扫描证明；以证据限定范围为准 |
| `hash_setdressing` | 150 | 未由扫描证明；以证据限定范围为准 |
| `add_tree_setdressing` | 164 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L13：/// 本草稿以 push 到 spawn_candidates 表达；字段语义见 AutoClutterSpawn（unknown 标注）。
- L16：// TODO(evidence): 精确字段布局（+0x18=2、+0x30 byte、+0x60 缩放等）待 0x68 记录重排。
- L43：/// 门组件（country_core；字段布局 unknown）。
- L47：/// unknown。
- L52：// TODO(evidence): 门在查询中的序号。
- L58：ClutterRegionId(0) // TODO(evidence): hash 值来源。
- L90：// TODO(evidence): 跳表目标与各分支语义待精读。
- L152：0 // TODO(evidence): 键组成。
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/manage_autoclutter_entities.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/manage_autoclutter_entities.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/manage_autoclutter_entities.rs)

SHA256：`5e183f05e50d7216b396d9e0fac451d7d100c2c22b001d3fd2fc22e7dae0cbbb`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `manage_autoclutter_entities` | 15 | 未由扫描证明；以证据限定范围为准 |
| `orient_look` | 48 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L31：// TODO(evidence): 组件集合（ColorId、位置、朝向）与 insert 命令 closure 精确内容。
- L49：// TODO(evidence): 目标向量来源（墙面/区域法向）。
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/mod.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/mod.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/mod.rs)

SHA256：`510d1a5a2077717e9641f6dbd929c7f5fd76468c64c3ca3dcc340ff07919cfcf`；缺口注释标记数：23。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `add_region` | 148 | 未由扫描证明；以证据限定范围为准 |
| `flush_candidates_pending_removal` | 167 | 未由扫描证明；以证据限定范围为准 |
| `mark_dirty` | 173 | 未由扫描证明；以证据限定范围为准 |
| `remove_stale_regions` | 179 | 未由扫描证明；以证据限定范围为准 |
| `is_equivalent` | 186 | 未由扫描证明；以证据限定范围为准 |
| `reset_autoclutter_for_new_frame` | 192 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L5：//! 字段布局为证据；未确认字段保留 `unknown`，不伪造。
- L37：/// 其余判别值 unknown。
- L38：Unknown(u8),
- L57：/// 中心/基向量（unknown 精确语义；add_region 用 Rectangle2d::aabb 推 AABB）。
- L66：/// 区域 id（hashbrown 键；hash_one<&GlobalId>，值内容 unknown）。
- L70：/// 自动杂物对象（实体组件；字段布局 unknown，manage 用其键建集合）。
- L73：/// unknown 字段；仅确认键类型 u64。
- L98：// 布局：hashbrown 域 @+0x60 前还有 +0x30 cap / +0x40 len 等，部分 unknown。
- L107：/// - +0x34 u32、+0x4c u32、+0x5c u32 等（unknown 语义）
- L112：/// unknown 字段。
- L113：pub unknown: [u64; 14],
- L121：/// - +0x2c：u32（unknown）
- L123：/// - +0x31..0x34：unknown
- L125：/// - +0x40：u64（unknown；is_started 门控输出）
- L126：/// - +0x48/+0x50/+0x54/+0x5c：unknown
- L138：/// +0x38 位置（unknown 精确字段；证据仅确认打包）。
- L140：/// 其余字段 unknown（+0x18、+0x2c、+0x40、+0x48..0x64）。
- L141：pub unknown: [u64; 8],
- L166：/// 冲刷待移除候选（B 0x140ee4bf0；语义 unknown，仅确认被 add_region/populate 调用）。
- L169：// TODO(evidence): 反编译 0x140ee4bf0 确认。
- L175：// TODO(evidence): 反编译 0x140ee48c0 确认。
- L185：/// 逐字段等价比较（add_region 内联：逐 float/byte 比较，unknown 精确字段集）。
- L193：// evidence：mod.rs 顶层函数；语义 unknown。
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/populate_autoclutter_regions.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/populate_autoclutter_regions.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/populate_autoclutter_regions.rs)

SHA256：`96069629521b5fa7e01de5b1e8bfa2d59a686a12c5af5ee54011f3f772eb4b1c`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `populate_autoclutter_regions` | 37 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：//! 恢复为 Res/ResMut/事件读取，精确映射 unknown（入口只保存 RCX/RDX/R8/R9+5 栈参）。
- L22：/// 杂物网格单元（read_bush_positions_from_gpu 共用；stride unknown）。
- L47：// TODO(evidence): 44KB 主体按 PDB 分段（L114–950）逐段反汇编复现；
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/process_autoclutter_candidates.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/process_autoclutter_candidates.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/autoclutter/process_autoclutter_candidates.rs)

SHA256：`f5ca065b1c69b5dd1b945481595050328c7817ee3773478b17475d22f14708eb`；缺口注释标记数：10。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `process_autoclutter_candidates` | 30 | 未由扫描证明；以证据限定范围为准 |
| `filter_candidate` | 56 | 未由扫描证明；以证据限定范围为准 |
| `cmp_candidate` | 62 | 未由扫描证明；以证据限定范围为准 |
| `forced` | 69 | 未由扫描证明；以证据限定范围为准 |
| `spawn_from_candidate` | 77 | 未由扫描证明；以证据限定范围为准 |
| `rectangles_overlap` | 90 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L31：// 证据：从 state.region_candidates 构建有序候选（u16 下标），过滤闭包 L40 条件 unknown。
- L55：/// 过滤闭包（PDB closure$0 L40；具体条件 unknown）。
- L57：// TODO(evidence): 反编译 from_iter filter 闭包确认。
- L61：/// 排序比较器（PDB closure$1 L56；比较字段 unknown——距离/深度）。
- L63：// TODO(evidence): driftsort_main 比较闭包字段 unknown。
- L71：// 本草稿 AutoClutterCandidate 未保留 +0x30 字段，标记 unknown。
- L72：false // TODO(evidence): 补 +0x30 字节字段。
- L78：// TODO(evidence): 16 字段复制精确映射（+0x0..+0x64）。
- L85：unknown: [0; 8],
- L91：// TODO(evidence): 0x20 stride 输入布局确认。
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/merlons.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/merlons.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/merlons.rs)

SHA256：`c505d8de7efe0cf7d5b64c8f32f3dd4af98ec427fc71763ef1620af16b2f3abc`；缺口注释标记数：14。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `add` | 32 | 未由扫描证明；以证据限定范围为准 |
| `push` | 53 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 57 | 未由扫描证明；以证据限定范围为准 |
| `is_flat` | 65 | 未由扫描证明；以证据限定范围为准 |
| `is_flat_w_no_rim` | 70 | 未由扫描证明；以证据限定范围为准 |
| `roof_shape_at_intersection_y` | 74 | 未由扫描证明；以证据限定范围为准 |
| `is_halftimbered` | 84 | 未由扫描证明；以证据限定范围为准 |
| `roof_entity` | 119 | 未由扫描证明；以证据限定范围为准 |
| `expand` | 129 | 未由扫描证明；以证据限定范围为准 |
| `contains_shape` | 131 | 未由扫描证明；以证据限定范围为准 |
| `perimeter` | 135 | 未由扫描证明；以证据限定范围为准 |
| `dirty_walls` | 161 | 未由扫描证明；以证据限定范围为准 |
| `detect_inter_roof_merlons` | 182 | 未由扫描证明；以证据限定范围为准 |
| `construct_inter_roof_merlons` | 213 | 未由扫描证明；以证据限定范围为准 |
| `circular_slice` | 251 | 未由扫描证明；以证据限定范围为准 |
| `len` | 257 | 未由扫描证明；以证据限定范围为准 |
| `segments` | 260 | 未由扫描证明；以证据限定范围为准 |
| `bilinear_pos_dir` | 266 | 未由扫描证明；以证据限定范围为准 |
| `u` | 269 | 未由扫描证明；以证据限定范围为准 |
| `circle_as_curve3` | 275 | 未由扫描证明；以证据限定范围为准 |
| `rectangle_as_curve3` | 278 | 未由扫描证明；以证据限定范围为准 |
| `random_splits_jitter` | 281 | 未由扫描证明；以证据限定范围为准 |
| `style_of` | 284 | 未由扫描证明；以证据限定范围为准 |
| `unknown` | 288 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L14：//! 字段为证据；未确认保留 unknown。
- L31：/// 证据：洞记录 stride 0x28；字段 +0x10/+0x18 语义 unknown。
- L33：// TODO(evidence): country-core 洞写入。
- L39：/// +0x10（unknown 精确语义）。
- L41：/// +0x18（unknown）。
- L54：// TODO(evidence): SparseInstanceBuffer 布局。
- L66：// TODO(evidence): 平顶判定。
- L107：/// 城垛断口描述（detect 的 retain 类型；布局 unknown）。
- L187：// TODO(evidence): 脏屋顶遍历 + 交点断口 → 洞。
- L189：unknown();
- L237：// position/size/种类4/(-100,-100)/scale：country-core 布局 unknown。
- L285：// TODO(evidence): 形状→墙样式查询。
- L288：fn unknown() -> ! {
- L289：unimplemented!("反汇编级证据：精确逻辑见 evidence/multi-baseline/clutter-extra-analysis.md")
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/mod.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/mod.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/mod.rs)

SHA256：`7b79bd6b0beac491004d46bd97a4765df623155debc608a20377c45e458581c1`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/stitches.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/stitches.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/stitches.rs)

SHA256：`7de828d4603265576b166fd3172aa285e5c355a9ded9ef44a20a09c758acf80c`；缺口注释标记数：9。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `max_y` | 34 | 未由扫描证明；以证据限定范围为准 |
| `wall_space` | 38 | 未由扫描证明；以证据限定范围为准 |
| `subtract` | 46 | 未由扫描证明；以证据限定范围为准 |
| `project_clamp` | 49 | 未由扫描证明；以证据限定范围为准 |
| `from_world_space` | 57 | 未由扫描证明；以证据限定范围为准 |
| `roof_ty` | 69 | 未由扫描证明；以证据限定范围为准 |
| `add_hole_at_shape_intersection` | 78 | 未由扫描证明；以证据限定范围为准 |
| `intersect_shapes` | 90 | 未由扫描证明；以证据限定范围为准 |
| `detect_intra_shape_corners` | 119 | 未由扫描证明；以证据限定范围为准 |
| `spawn_stitches` | 167 | 未由扫描证明；以证据限定范围为准 |
| `spawn_stitch_bricks` | 189 | 未由扫描证明；以证据限定范围为准 |
| `unknown` | 203 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L17：//! 字段为证据；未确认保留 unknown。
- L62：/// 角点（detect 的 retain 类型；布局 unknown）。
- L83：unknown();
- L94：unknown();
- L124：unknown();
- L179：unknown(); // random_splits + 逐点写 0x40 记录
- L196：unknown();
- L203：fn unknown() -> ! {
- L204：unimplemented!("反汇编级证据：精确逻辑见 evidence/multi-baseline/clutter-extra-analysis.md")

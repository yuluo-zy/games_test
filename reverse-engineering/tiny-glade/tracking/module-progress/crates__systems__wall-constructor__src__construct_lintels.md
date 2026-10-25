# crates/systems/wall-constructor/src/construct_lintels

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

construct_lintels/place_support_beams限定CPU链

## 接线与验证

接线：部分World链或候选，按所列范围。

仅所列局部/连续链报告；不继承其他family验收。

## 已恢复的函数和类型范围

- construct_lintels/place_support_beams限定CPU链

## 明确缺口

- 完整梁/悬挑/资源生命周期
- 未建立整个子目录逐函数等价验收

下一闭包：完整梁/悬挑/资源生命周期

## 证据

- [evidence/building-structures/findings.md](../../evidence/building-structures/findings.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/wall-constructor/src/construct_lintels/mod.rs` | A,B,C | [reconstruction/building/wall_structures.rs](../../reconstruction/building/wall_structures.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_lintels/place_support_beams.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_lintels/profile.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/wall_structures.rs

[reconstruction/building/wall_structures.rs](../../reconstruction/building/wall_structures.rs)

SHA256：`989e5bfdccfeecf072172ac6ea5995e5ddbc03aacc3fceeb20f5894e15f7290e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from` | 14 | 未由扫描证明；以证据限定范围为准 |
| `min_source` | 39 | 未由扫描证明；以证据限定范围为准 |
| `periodic_subtract` | 46 | 未由扫描证明；以证据限定范围为准 |
| `project` | 57 | 未由扫描证明；以证据限定范围为准 |
| `materialize_wall_segments` | 70 | 未由扫描证明；以证据限定范围为准 |
| `prepare_lintel_support` | 167 | 未由扫描证明；以证据限定范围为准 |
| `rdp_support_indices` | 228 | 未由扫描证明；以证据限定范围为准 |
| `observed_bytes` | 293 | 未由扫描证明；以证据限定范围为准 |
| `length_yxz` | 321 | 未由扫描证明；以证据限定范围为准 |
| `propose` | 324 | 未由扫描证明；以证据限定范围为准 |
| `consume` | 350 | 未由扫描证明；以证据限定范围为准 |
| `affine_instance` | 357 | 未由扫描证明；以证据限定范围为准 |
| `emit_plank` | 373 | 未由扫描证明；以证据限定范围为准 |
| `walk_lintel` | 409 | 未由扫描证明；以证据限定范围为准 |
| `extend_xz` | 462 | 未由扫描证明；以证据限定范围为准 |
| `add_diagonal` | 500 | 未由扫描证明；以证据限定范围为准 |
| `emit_range` | 553 | 未由扫描证明；以证据限定范围为准 |
| `lintel_support_instances` | 579 | 未由扫描证明；以证据限定范围为准 |
| `accumulate_lintel_third_heights` | 675 | 未由扫描证明；以证据限定范围为准 |
| `construct_lintel_supports` | 700 | 未由扫描证明；以证据限定范围为准 |

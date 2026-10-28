# crates/systems/wall-constructor/src/utils

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

resolve_hole_overlap、trim_rows、split_into_random_rows限定策略

## 接线与验证

接线：部分World链或候选，按所列范围。

仅所列局部/连续链报告；不继承其他family验收。

## 已恢复的函数和类型范围

- resolve_hole_overlap、trim_rows、split_into_random_rows限定策略

## 明确缺口

- wall_corners/wall_skirts、half_timber与全部helper来源归属
- 未建立整个子目录逐函数等价验收

下一闭包：wall_corners/wall_skirts、half_timber与全部helper来源归属

## 证据

- [evidence/building-constructor/B-wall-constructor-verification.json](../../evidence/building-constructor/B-wall-constructor-verification.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/wall-constructor/src/utils/half_timber.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/utils/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/utils/resolve_hole_overlap.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/utils/sine_wave.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/utils/split_into_random_rows.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/utils/stairs_signifiers.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/utils/trim_rows.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/utils/wall_corners.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/utils/wall_skirts.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/wall_constructor.rs

[reconstruction/building/wall_constructor.rs](../../reconstruction/building/wall_constructor.rs)

SHA256：`22a44d9360049aecebf14b984811495257fdf8674dd9c1a55a164bacb7d8b320`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `observed_bytes` | 13 | 未由扫描证明；以证据限定范围为准 |
| `dims` | 24 | 未由扫描证明；以证据限定范围为准 |
| `center` | 27 | 未由扫描证明；以证据限定范围为准 |
| `from_center` | 30 | 未由扫描证明；以证据限定范围为准 |
| `contains` | 34 | 未由扫描证明；以证据限定范围为准 |
| `clamp01` | 37 | 未由扫描证明；以证据限定范围为准 |
| `resolve_hole_overlap` | 53 | 未由扫描证明；以证据限定范围为准 |
| `cut_wall_bricks` | 198 | 未由扫描证明；以证据限定范围为准 |
| `random_splits_into` | 210 | 未由扫描证明；以证据限定范围为准 |
| `split_into_random_rows` | 225 | 未由扫描证明；以证据限定范围为准 |
| `observed_bytes` | 255 | 未由扫描证明；以证据限定范围为准 |
| `trim_rows` | 268 | 未由扫描证明；以证据限定范围为准 |
| `observed_bytes` | 373 | 未由扫描证明；以证据限定范围为准 |
| `emit_resolved_wall_brick` | 411 | 未由扫描证明；以证据限定范围为准 |
| `normalize_wall_holes` | 456 | 未由扫描证明；以证据限定范围为准 |
| `intersects` | 502 | 未由扫描证明；以证据限定范围为准 |
| `consume_wall_proposal` | 509 | 未由扫描证明；以证据限定范围为准 |
| `emit_plain_wall_brick` | 577 | 未由扫描证明；以证据限定范围为准 |
| `construct_resolved_plain_wall_proposal` | 593 | 未由扫描证明；以证据限定范围为准 |
| `select_native_row_holes` | 626 | 未由扫描证明；以证据限定范围为准 |
| `split_plain_row_horizontal` | 669 | 未由扫描证明；以证据限定范围为准 |

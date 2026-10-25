# crates/systems/wall-constructor/src/segmentation

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

calculate_wall_path_segmentation；sample_wall_path_intersections；split_wall_path_segment_into_platform_stairs

## 接线与验证

接线：部分World链或候选，按所列范围。

仅所列局部/连续链报告；不继承其他family验收。

## 已恢复的函数和类型范围

- calculate_wall_path_segmentation
- sample_wall_path_intersections
- split_wall_path_segment_into_platform_stairs

## 明确缺口

- 完整路径mask/material-stat已验限定范围；更多形状/消费者
- 未建立整个子目录逐函数等价验收

下一闭包：完整路径mask/material-stat已验限定范围；更多形状/消费者

## 证据

- [evidence/building-segmentation/findings.md](../../evidence/building-segmentation/findings.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs` | A,B,C | [reconstruction/building/wall_segmentation.rs](../../reconstruction/building/wall_segmentation.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs` | A,B,C | [reconstruction/building/wall_path_preparation.rs](../../reconstruction/building/wall_path_preparation.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs` | A,B,C | [reconstruction/building/wall_platform_segmentation.rs](../../reconstruction/building/wall_platform_segmentation.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/wall_path_preparation.rs

[reconstruction/building/wall_path_preparation.rs](../../reconstruction/building/wall_path_preparation.rs)

SHA256：`85793179f54a2a415f6a7b8007026059a685b844a43133f6ae0b00157288e417`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_encoded` | 23 | 未由扫描证明；以证据限定范围为准 |
| `encoded` | 52 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 55 | 未由扫描证明；以证据限定范围为准 |
| `sample_sum` | 58 | 未由扫描证明；以证据限定范围为准 |
| `prepare_wall_path` | 84 | 未由扫描证明；以证据限定范围为准 |
| `clamp_unit` | 148 | 未由扫描证明；以证据限定范围为准 |
| `distance` | 152 | 未由扫描证明；以证据限定范围为准 |
| `interpolate` | 155 | 未由扫描证明；以证据限定范围为准 |
| `midpoint` | 158 | 未由扫描证明；以证据限定范围为准 |
| `append` | 161 | 未由扫描证明；以证据限定范围为准 |
| `prepare_nonempty` | 167 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_platform_segmentation.rs

[reconstruction/building/wall_platform_segmentation.rs](../../reconstruction/building/wall_platform_segmentation.rs)

SHA256：`2717dfdd1dac12792dbbd1fd37b9660c86595b680f97151002e06bad77f4c07b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `split_platform_wall_masks` | 29 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_segmentation.rs

[reconstruction/building/wall_segmentation.rs](../../reconstruction/building/wall_segmentation.rs)

SHA256：`b8d92ab62004878b6bd6663abfcd2bb2ad081314a142b2686463f88818199c97`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `for_closed` | 22 | 未由扫描证明；以证据限定范围为准 |
| `for_open` | 49 | 未由扫描证明；以证据限定范围为准 |
| `len` | 76 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 79 | 未由扫描证明；以证据限定范围为准 |
| `index_at` | 82 | 未由扫描证明；以证据限定范围为准 |
| `observed_bytes` | 94 | 未由扫描证明；以证据限定范围为准 |
| `for_point_count` | 112 | 未由扫描证明；以证据限定范围为准 |
| `observed_bytes` | 134 | 未由扫描证明；以证据限定范围为准 |
| `wall_path_segmentation` | 164 | 未由扫描证明；以证据限定范围为准 |
| `norm` | 239 | 未由扫描证明；以证据限定范围为准 |
| `sample` | 242 | 未由扫描证明；以证据限定范围为准 |
| `score` | 252 | 未由扫描证明；以证据限定范围为准 |
| `process_segment` | 314 | 未由扫描证明；以证据限定范围为准 |
| `add_splits` | 384 | 未由扫描证明；以证据限定范围为准 |
| `add_stairs_length` | 426 | 未由扫描证明；以证据限定范围为准 |
| `material_stats_for_update` | 431 | 未由扫描证明；以证据限定范围为准 |

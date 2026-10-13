# crates/systems/wall-constructor/src/construct_platform_stairs

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

80B装饰源→cells/碰撞/96B记录/holes/stats

## 接线与验证

接线：部分World链或候选，按所列范围。

仅所列局部/连续链报告；不继承其他family验收。

## 已恢复的函数和类型范围

- 80B装饰源→cells/碰撞/96B记录/holes/stats

## 明确缺口

- 扩大consumer输入边界；新Rapier接线整批未验收
- 未建立整个子目录逐函数等价验收

下一闭包：扩大consumer输入边界；新Rapier接线整批未验收

## 证据

- [evidence/building-platform-stairs/B-rapier-backend-verification.json](../../evidence/building-platform-stairs/B-rapier-backend-verification.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/wall-constructor/src/construct_platform_stairs/bottom_riser.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs` | A,B,C | [reconstruction/building/wall_platform_stairs.rs](../../reconstruction/building/wall_platform_stairs.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_platform_stairs/stone.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_platform_stairs/wooden_corners.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/wall_platform_collision.rs

[reconstruction/building/wall_platform_collision.rs](../../reconstruction/building/wall_platform_collision.rs)

SHA256：`e546dd6386e8a904d45bc3e89e3ab4d494ca56fe85d7c57b078221d29bd179bd`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `source_discriminant` | 16 | 未由扫描证明；以证据限定范围为准 |
| `indexed_mesh_hash` | 24 | 未由扫描证明；以证据限定范围为准 |
| `ordered_eq` | 76 | 未由扫描证明；以证据限定范围为准 |
| `from_geometry` | 95 | 未由扫描证明；以证据限定范围为准 |
| `source_discriminant` | 125 | 未由扫描证明；以证据限定范围为准 |
| `subworld` | 128 | 未由扫描证明；以证据限定范围为准 |
| `is_sensor` | 132 | 未由扫描证明；以证据限定范围为准 |
| `platform_collider_action` | 144 | 未由扫描证明；以证据限定范围为准 |
| `pose` | 169 | 未由扫描证明；以证据限定范围为准 |
| `insert` | 170 | 未由扫描证明；以证据限定范围为准 |
| `remove` | 175 | 未由扫描证明；以证据限定范围为准 |
| `new` | 199 | 未由扫描证明；以证据限定范围为准 |
| `remove` | 205 | 未由扫描证明；以证据限定范围为准 |
| `update` | 214 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_platform_rapier_backend.rs

[reconstruction/building/wall_platform_rapier_backend.rs](../../reconstruction/building/wall_platform_rapier_backend.rs)

SHA256：`23d31535776ada55d7cd3ff141f5844e5cf8d8999e8d101339df4fe55f4c88de`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `original_discriminant` | 22 | 未由扫描证明；以证据限定范围为准 |
| `new` | 58 | 未由扫描证明；以证据限定范围为准 |
| `flush_queries` | 70 | 未由扫描证明；以证据限定范围为准 |
| `cast_ray` | 78 | 未由扫描证明；以证据限定范围为准 |
| `dirty_aabb` | 115 | 未由扫描证明；以证据限定范围为准 |
| `pose` | 123 | 未由扫描证明；以证据限定范围为准 |
| `insert` | 139 | 未由扫描证明；以证据限定范围为准 |
| `remove` | 175 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_platform_stair_rows.rs

[reconstruction/building/wall_platform_stair_rows.rs](../../reconstruction/building/wall_platform_stair_rows.rs)

SHA256：`3d1587a9f52b0980ce0028a3517b4a8d552948bba8105a77629c364d6d6e1e25`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `project` | 28 | 未由扫描证明；以证据限定范围为准 |
| `boundaries` | 32 | 未由扫描证明；以证据限定范围为准 |
| `rectangle_closest` | 44 | 未由扫描证明；以证据限定范围为准 |
| `circle_closest` | 65 | 未由扫描证明；以证据限定范围为准 |
| `append_slice` | 75 | 未由扫描证明；以证据限定范围为准 |
| `circular_slice` | 92 | 未由扫描证明；以证据限定范围为准 |
| `circle` | 114 | 未由扫描证明；以证据限定范围为准 |
| `rectangle` | 122 | 未由扫描证明；以证据限定范围为准 |
| `platform_stair_footprint` | 130 | 未由扫描证明；以证据限定范围为准 |
| `original_cos` | 199 | 未由扫描证明；以证据限定范围为准 |
| `original_sin` | 203 | 未由扫描证明；以证据限定范围为准 |
| `normal` | 206 | 未由扫描证明；以证据限定范围为准 |
| `platform_stair_gpu_instance` | 242 | 未由扫描证明；以证据限定范围为准 |
| `platform_stair_rows` | 252 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_platform_stairs.rs

[reconstruction/building/wall_platform_stairs.rs](../../reconstruction/building/wall_platform_stairs.rs)

SHA256：`8299cbcdd1a23b4baab47ede95bc43ddd14cbb27ee95b2cc71a8a999e41f8621`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `collect_platform_stair_blockers` | 45 | 未由扫描证明；以证据限定范围为准 |
| `wall_space_distance` | 76 | 未由扫描证明；以证据限定范围为准 |
| `prepare_platform_stair_segments` | 88 | 未由扫描证明；以证据限定范围为准 |
| `platform_stair_cell` | 200 | 未由扫描证明；以证据限定范围为准 |
| `platform_stair_geometry` | 299 | 未由扫描证明；以证据限定范围为准 |
| `platform_stair_holes` | 391 | 未由扫描证明；以证据限定范围为准 |

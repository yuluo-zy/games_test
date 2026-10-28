# crates/country-core/src/systems/camera_rig

状态：部分草稿。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

normal/free/walk/butterfly/camera_body候选源码；不是完整MainCamera。

## 接线与验证

接线：not_established。

构造器native返回已观察；Rust完整差分/系统未验收。

## 已恢复的函数和类型范围

- camera_body/normal/free/walk/butterfly的候选更新函数

## 明确缺口

- 完整input/mode transition和MainCamera编排
- 真实terrain/collision producer与跳表复核、C适配

下一闭包：完整input/mode transition和MainCamera编排

## 证据

- [tracking/parallel-modules-stopped-snapshot.json](../parallel-modules-stopped-snapshot.json)
- [evidence/parallel-camera/B-constructor-native.json](../../evidence/parallel-camera/B-constructor-native.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/systems/camera_rig/butterfly.rs` | A,B,C | [reconstruction/camera/butterfly.rs](../../reconstruction/camera/butterfly.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/camera_rig/camera_body.rs` | A,B,C | [reconstruction/camera/camera_body.rs](../../reconstruction/camera/camera_body.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/camera_rig/free.rs` | A,B,C | [reconstruction/camera/free.rs](../../reconstruction/camera/free.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/camera_rig/normal.rs` | A,B,C | [reconstruction/camera/normal.rs](../../reconstruction/camera/normal.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/camera_rig/walk.rs` | A,B,C | [reconstruction/camera/walk.rs](../../reconstruction/camera/walk.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/camera/butterfly.rs

[reconstruction/camera/butterfly.rs](../../reconstruction/camera/butterfly.rs)

SHA256：`31b072cfa5641c8a13ae019d00877e44afd94b8f5feb2af9a617a3bef61b5ff8`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 7 | 未由扫描证明；以证据限定范围为准 |
| `follow` | 10 | 未由扫描证明；以证据限定范围为准 |
| `advance` | 11 | 未由扫描证明；以证据限定范围为准 |
| `rotate_yaw_pitch` | 15 | 未由扫描证明；以证据限定范围为准 |
| `update` | 16 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/camera/camera_body.rs

[reconstruction/camera/camera_body.rs](../../reconstruction/camera/camera_body.rs)

SHA256：`70fa49cb55f1df9881c9f937d753c5c78d8fd7a070f05af4345b9898910d540a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `sample_height` | 5 | 未由扫描证明；以证据限定范围为准 |
| `walk_height_blend` | 8 | 未由扫描证明；以证据限定范围为准 |
| `rotate_yaw_pitch` | 18 | 未由扫描证明；以证据限定范围为准 |
| `update` | 19 | 未由扫描证明；以证据限定范围为准 |
| `lerp` | 21 | 未由扫描证明；以证据限定范围为准 |
| `smooth` | 22 | 未由扫描证明；以证据限定范围为准 |
| `bounds_xz` | 24 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/camera/free.rs

[reconstruction/camera/free.rs](../../reconstruction/camera/free.rs)

SHA256：`7256b96f1ea3dfe67ced67ba99cb177f54e1fa4c6d5b003708544c7e7cf8444b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 7 | 未由扫描证明；以证据限定范围为准 |
| `from_position_angles` | 8 | 未由扫描证明；以证据限定范围为准 |
| `set` | 9 | 未由扫描证明；以证据限定范围为准 |
| `get_position` | 10 | 未由扫描证明；以证据限定范围为准 |
| `get_yaw_pitch` | 11 | 未由扫描证明；以证据限定范围为准 |
| `transform` | 12 | 未由扫描证明；以证据限定范围为准 |
| `translate_screen_space` | 13 | 未由扫描证明；以证据限定范围为准 |
| `rotate_yaw_pitch` | 21 | 未由扫描证明；以证据限定范围为准 |
| `update` | 22 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/camera/mod.rs

[reconstruction/camera/mod.rs](../../reconstruction/camera/mod.rs)

SHA256：`ac37349e64300a858fbacab2d3c9974df7d2bc530083fdd2730f83ef37b02f99`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/camera/normal.rs

[reconstruction/camera/normal.rs](../../reconstruction/camera/normal.rs)

SHA256：`aea038682adf44b0007726ad2336021fc7250cd0b7b44906e644f62975abca91`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `original_b` | 9 | 未由扫描证明；以证据限定范围为准 |
| `transform` | 13 | 未由扫描证明；以证据限定范围为准 |
| `constrain_to_be_above_terrain` | 14 | 未由扫描证明；以证据限定范围为准 |
| `translate_screen_space` | 15 | 未由扫描证明；以证据限定范围为准 |
| `begin_rotation` | 16 | 未由扫描证明；以证据限定范围为准 |
| `finish_rotation` | 17 | 未由扫描证明；以证据限定范围为准 |
| `load` | 18 | 未由扫描证明；以证据限定范围为准 |
| `zoom` | 19 | 未由扫描证明；以证据限定范围为准 |
| `rotation` | 21 | 未由扫描证明；以证据限定范围为准 |
| `rotate_yaw_pitch` | 23 | 未由扫描证明；以证据限定范围为准 |
| `update` | 24 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/camera/schema.rs

[reconstruction/camera/schema.rs](../../reconstruction/camera/schema.rs)

SHA256：`40b8e35d4d11987361cb44e9b512c4babd48b8d592625b8cd487cc1fdfcc4d11`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `default_vertical_fov_degrees` | 14 | 未由扫描证明；以证据限定范围为准 |
| `original_b` | 23 | 未由扫描证明；以证据限定范围为准 |
| `immediately_set_vertical_fov_degrees` | 31 | 未由扫描证明；以证据限定范围为准 |
| `update` | 32 | 未由扫描证明；以证据限定范围为准 |
| `new` | 37 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/camera/walk.rs

[reconstruction/camera/walk.rs](../../reconstruction/camera/walk.rs)

SHA256：`f7880ca23a3b995c5c5b78c51845e7aea9534f8cde165d0e772b843f4ccdb2ba`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 7 | 未由扫描证明；以证据限定范围为准 |
| `position` | 8 | 未由扫描证明；以证据限定范围为准 |
| `rotation` | 9 | 未由扫描证明；以证据限定范围为准 |
| `teleport` | 10 | 未由扫描证明；以证据限定范围为准 |
| `translate_screen_space` | 11 | 未由扫描证明；以证据限定范围为准 |
| `constrain_to_be_above_terrain` | 20 | 未由扫描证明；以证据限定范围为准 |
| `rotate_yaw_pitch` | 31 | 未由扫描证明；以证据限定范围为准 |
| `update` | 32 | 未由扫描证明；以证据限定范围为准 |

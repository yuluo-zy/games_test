# crates/systems/wall-constructor/src/construct_arches

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

construct_arches/profile；regular/freeform arch输出、final arch/third和120B ConstructGateCmd

## 接线与验证

接线：部分World链或候选，按所列范围。

仅所列局部/连续链报告；不继承其他family验收。

## 已恢复的函数和类型范围

- construct_arches/profile
- regular/freeform arch输出、final arch/third和120B ConstructGateCmd

## 明确缺口

- 更多支撑族、门命令最终consumer
- 未建立整个子目录逐函数等价验收

下一闭包：更多支撑族、门命令最终consumer

## 证据

- [evidence/building-arches-freehand/findings.md](../../evidence/building-arches-freehand/findings.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/wall-constructor/src/construct_arches/mod.rs` | A,B,C | [reconstruction/building/wall_arches.rs](../../reconstruction/building/wall_arches.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_arches/profile.rs` | A,B,C | [reconstruction/building/wall_arches.rs](../../reconstruction/building/wall_arches.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_arches/wooden_arch_frame.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/wall_arches.rs

[reconstruction/building/wall_arches.rs](../../reconstruction/building/wall_arches.rs)

SHA256：`b85196d19d9878883e72ea1da44f2d5cf99b305884adb617a50d663fdf60e24a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from` | 20 | 未由扫描证明；以证据限定范围为准 |
| `length_xyz` | 51 | 未由扫描证明；以证据限定范围为准 |
| `maxnum` | 54 | 未由扫描证明；以证据限定范围为准 |
| `minnum` | 61 | 未由扫描证明；以证据限定范围为准 |
| `clamp_ratio` | 68 | 未由扫描证明；以证据限定范围为准 |
| `highest` | 76 | 未由扫描证明；以证据限定范围为准 |
| `regular_profile` | 88 | 未由扫描证明；以证据限定范围为准 |
| `periodic_delta` | 120 | 未由扫描证明；以证据限定范围为准 |
| `project_period` | 129 | 未由扫描证明；以证据限定范围为准 |
| `emit_arch_brick` | 137 | 未由扫描证明；以证据限定范围为准 |
| `walk_profile` | 168 | 未由扫描证明；以证据限定范围为准 |
| `construct_regular_wall_arches` | 240 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_arches_freehand.rs

[reconstruction/building/wall_arches_freehand.rs](../../reconstruction/building/wall_arches_freehand.rs)

SHA256：`816eb303c84790b8573d4aa67a5bb19abadcadf14e0b32ae119073fd5cdce668`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `maxnum` | 29 | 未由扫描证明；以证据限定范围为准 |
| `clamp` | 36 | 未由扫描证明；以证据限定范围为准 |
| `length` | 44 | 未由扫描证明；以证据限定范围为准 |
| `curve` | 47 | 未由扫描证明；以证据限定范围为准 |
| `slice` | 51 | 未由扫描证明；以证据限定范围为准 |
| `highest` | 70 | 未由扫描证明；以证据限定范围为准 |
| `profile` | 82 | 未由扫描证明；以证据限定范围为准 |
| `walk` | 142 | 未由扫描证明；以证据限定范围为准 |
| `construct_freehand_wall_arches` | 209 | 未由扫描证明；以证据限定范围为准 |

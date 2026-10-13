# crates/systems/wall-constructor/src/construct_gates

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

Gate120→CPU木门/支撑/post fallback、候选mesh/entity/Nani生命周期

## 接线与验证

接线：部分World链或候选，按所列范围。

仅所列局部/连续链报告；不继承其他family验收。

## 已恢复的函数和类型范围

- Gate120→CPU木门/支撑/post fallback、候选mesh/entity/Nani生命周期

## 明确缺口

- 新增整批未统一验收；原MeshLib/ShaderLib/SSBO所有权
- 未建立整个子目录逐函数等价验收

下一闭包：新增整批未统一验收；原MeshLib/ShaderLib/SSBO所有权

## 证据

- [evidence/building-gates/findings.md](../../evidence/building-gates/findings.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/wall-constructor/src/construct_gates/mod.rs` | A,B,C | [reconstruction/building/gates.rs](../../reconstruction/building/gates.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_gates/systems.rs` | A,B,C | [reconstruction/building/gates.rs](../../reconstruction/building/gates.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/gates.rs

[reconstruction/building/gates.rs](../../reconstruction/building/gates.rs)

SHA256：`c220e13f914fc5322ed7277d7b02cc697947cf72668928c9d22286db191cb2e7`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `add_quad` | 32 | 未由扫描证明；以证据限定范围为准 |
| `new` | 74 | 未由扫描证明；以证据限定范围为准 |
| `sample` | 102 | 未由扫描证明；以证据限定范围为准 |
| `sample_position` | 110 | 未由扫描证明；以证据限定范围为准 |
| `coordinate` | 115 | 未由扫描证明；以证据限定范围为准 |
| `length3` | 122 | 未由扫描证明；以证据限定范围为准 |
| `normal` | 128 | 未由扫描证明；以证据限定范围为准 |
| `offset_height` | 139 | 未由扫描证明；以证据限定范围为准 |
| `panel` | 142 | 未由扫描证明；以证据限定范围为准 |
| `recover_gate_shape` | 159 | 未由扫描证明；以证据限定范围为准 |
| `recover_gate_support_bricks` | 337 | 未由扫描证明；以证据限定范围为准 |
| `slice_gate_curve` | 438 | 未由扫描证明；以证据限定范围为准 |
| `canonical_bytes` | 475 | 未由扫描证明；以证据限定范围为准 |
| `facing_rotation` | 497 | 未由扫描证明；以证据限定范围为准 |
| `nani` | 511 | 未由扫描证明；以证据限定范围为准 |
| `construct_wooden_gate_doors` | 528 | 未由扫描证明；以证据限定范围为准 |
| `construct_gate_cpu` | 619 | 未由扫描证明；以证据限定范围为准 |

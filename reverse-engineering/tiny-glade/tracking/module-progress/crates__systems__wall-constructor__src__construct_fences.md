# crates/systems/wall-constructor/src/construct_fences

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

construct_fences/process_segment部分候选

## 接线与验证

接线：部分World链或候选，按所列范围。

仅所列局部/连续链报告；不继承其他family验收。

## 已恢复的函数和类型范围

- construct_fences/process_segment部分候选

## 明确缺口

- 完整floor/fence消费链及新代码验收
- 未建立整个子目录逐函数等价验收

下一闭包：完整floor/fence消费链及新代码验收

## 证据

- [evidence/building-fences/findings.md](../../evidence/building-fences/findings.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/wall-constructor/src/construct_fences/mod.rs` | A,B,C | [reconstruction/building/fences.rs](../../reconstruction/building/fences.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_fences/process_segment.rs` | A,B,C | [reconstruction/building/fences.rs](../../reconstruction/building/fences.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/fences.rs

[reconstruction/building/fences.rs](../../reconstruction/building/fences.rs)

SHA256：`48834249c43626f78d6af8ac841788816e6ca4ce199e8a022f742d4a4287201a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `split_fence_angles` | 30 | 未由扫描证明；以证据限定范围为准 |
| `post` | 42 | 未由扫描证明；以证据限定范围为准 |
| `packed` | 48 | 未由扫描证明；以证据限定范围为准 |
| `instance` | 54 | 未由扫描证明；以证据限定范围为准 |
| `process_fence_segment` | 61 | 未由扫描证明；以证据限定范围为准 |
| `distance3` | 125 | 未由扫描证明；以证据限定范围为准 |
| `fence_curve` | 128 | 未由扫描证明；以证据限定范围为准 |
| `resample_fence_curve` | 138 | 未由扫描证明；以证据限定范围为准 |
| `construct_fences_cpu` | 179 | 未由扫描证明；以证据限定范围为准 |

# crates/utils/src/components

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：11/2/9。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source_with_pending_trait_schema_classification。

B原接口台账：11项已观察，2项已有部分业务分析，9项未触及；具体状态见逐接口证据。

## 接线与验证

接线：candidate_crate_exported_original_game_consumer_not_accepted。

原指令对照覆盖指定实例与输入；静态候选、归属待定项及原消费者链分别记录，不能据此验收整个目录。

## 已恢复的函数和类型范围

- utils::components::transform::Transform::abs_diff_eq
- utils::components::transform::Transform::look_at

## 明确缺口

- 尚未分析的接口逐项列于 INTERFACE_PROGRESS.md
- 泛型策略及边界输入尚未全部对照
- 原游戏消费者链尚未验收

下一闭包：按原接口缺口补充业务分析、复用原依赖实现并对照修正。

## 证据

- [evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)
- [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json)
- [evidence/shared-foundation-2026-10-13/geometry-curves/interface-batch-02/coverage-delta.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-batch-02/coverage-delta.json)
- [FOUNDATION_BUSINESS_COMPARISON.md](../../FOUNDATION_BUSINESS_COMPARISON.md)
- [tracking/shared-foundation-business-comparison.json](../shared-foundation-business-comparison.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/utils/src/components/transform.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/components/transform.rs](../../reconstruction/shared-foundation/crates/utils/src/components/transform.rs)：B_PDB_definition_source_and_reviewed_interface_contract | 全集/逐函数映射待核对 | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/utils/src/components/mod.rs

[reconstruction/shared-foundation/crates/utils/src/components/mod.rs](../../reconstruction/shared-foundation/crates/utils/src/components/mod.rs)

SHA256：`e05ce78b275d81473d160c16b611657b805dc793e81ff228ac6486e550ba33a0`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/shared-foundation/crates/utils/src/components/transform.rs

[reconstruction/shared-foundation/crates/utils/src/components/transform.rs](../../reconstruction/shared-foundation/crates/utils/src/components/transform.rs)

SHA256：`61f43e4ef4d54c3b8b6218d633af017aeffdd0df76898a0d35664938eb21321d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `compute_affine` | 12 | 未由扫描证明；以证据限定范围为准 |
| `compute_matrix` | 16 | 未由扫描证明；以证据限定范围为准 |
| `forward` | 20 | 未由扫描证明；以证据限定范围为准 |
| `look_at` | 25 | 未由扫描证明；以证据限定范围为准 |
| `abs_diff_eq` | 33 | 未由扫描证明；以证据限定范围为准 |

# crates/tabloid/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：3/3/0。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

[业务功能对照](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)：{"implemented_scoped": 5, "integration_pending": 1}。此为功能行分类，不是完成率；源码缺失与原验证/库外接线分开。

定义归属：private_game_source。

发布、订阅、共享增量批次及reader生命周期；三主要接口有业务恢复。

## 接线与验证

接线：候选模块可调用，完整原游戏消费者接线仍待完成。

3个原锁合同测试、严格Clippy；Default/Drop/Deref内联接口单列复核。

## 已恢复的函数和类型范围

- tabloid::Publisher<T>::publish
- tabloid::Publisher<T>::subscribe
- tabloid::Subscriber<T>::take
- private publication retain predicate

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| tabloid::Publisher<T>::publish | [reconstruction/shared-foundation/crates/tabloid/src/lib.rs](../../reconstruction/shared-foundation/crates/tabloid/src/lib.rs)：`publish` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json), [evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md) |
| tabloid::Publisher<T>::subscribe | [reconstruction/shared-foundation/crates/tabloid/src/lib.rs](../../reconstruction/shared-foundation/crates/tabloid/src/lib.rs)：`subscribe` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json), [evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md) |
| tabloid::Subscriber<T>::take | [reconstruction/shared-foundation/crates/tabloid/src/lib.rs](../../reconstruction/shared-foundation/crates/tabloid/src/lib.rs)：`take` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json), [evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md) |
| private publication retain predicate | [reconstruction/shared-foundation/crates/tabloid/src/lib.rs](../../reconstruction/shared-foundation/crates/tabloid/src/lib.rs)：`publish` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json), [evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md) |

## 明确缺口

- 真实DirtyTiles/ColliderDiff生产者未接线
- Default/Drop/Deref独立原接口对照不足
- 原发布消费全流程运行差分未执行

下一闭包：补内联接口对照并对接真实生产者

## 证据

- [evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json)
- [evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md)
- [evidence/shared-foundation-2026-10-13/utils-tabloid/tabloid-test.log](../../evidence/shared-foundation-2026-10-13/utils-tabloid/tabloid-test.log)
- [evidence/shared-foundation-2026-10-13/utils-tabloid/tabloid-clippy.log](../../evidence/shared-foundation-2026-10-13/utils-tabloid/tabloid-clippy.log)
- [evidence/shared-foundation-2026-10-13/integration/dependency-validation.json](../../evidence/shared-foundation-2026-10-13/integration/dependency-validation.json)
- [evidence/shared-foundation-review-2026-10-13/REVIEW.md](../../evidence/shared-foundation-review-2026-10-13/REVIEW.md)
- [tracking/shared-foundation-code-review.json](../shared-foundation-code-review.json)
- [evidence/shared-foundation-review-2026-10-13/integration-validation.json](../../evidence/shared-foundation-review-2026-10-13/integration-validation.json)
- [FOUNDATION_BUSINESS_COMPARISON.md](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)
- [tracking/shared-foundation-business-comparison.json](../shared-foundation-business-comparison.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/tabloid/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/tabloid/src/lib.rs](../../reconstruction/shared-foundation/crates/tabloid/src/lib.rs)：explicit_scoped_function_binding | tabloid::Publisher<T>::publish（candidate_static_evidence；业务等价以限定证据为准）<br>tabloid::Publisher<T>::subscribe（candidate_static_evidence；业务等价以限定证据为准）<br>tabloid::Subscriber<T>::take（candidate_static_evidence；业务等价以限定证据为准）<br>private publication retain predicate（candidate_static_evidence；业务等价以限定证据为准） | candidate_present；tabloid::Publisher<T>::publish：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json), [evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md)<br>tabloid::Publisher<T>::subscribe：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json), [evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md)<br>tabloid::Subscriber<T>::take：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json), [evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md)<br>private publication retain predicate：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json), [evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/tabloid/src/lib.rs

[reconstruction/shared-foundation/crates/tabloid/src/lib.rs](../../reconstruction/shared-foundation/crates/tabloid/src/lib.rs)

SHA256：`3e1896ae4b45a7d252f5f6702d7daf659f711675e1537dbeb8a232cd644db3d2`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `empty` | 8 | 未由扫描证明；以证据限定范围为准 |
| `apply_to` | 11 | 未由扫描证明；以证据限定范围为准 |
| `clone` | 26 | 未由扫描证明；以证据限定范围为准 |
| `deref` | 38 | 未由扫描证明；以证据限定范围为准 |
| `sequence` | 43 | 未由扫描证明；以证据限定范围为准 |
| `default` | 54 | 未由扫描证明；以证据限定范围为准 |
| `new` | 63 | 未由扫描证明；以证据限定范围为准 |
| `subscribe` | 68 | 未由扫描证明；以证据限定范围为准 |
| `publish` | 80 | 未由扫描证明；以证据限定范围为准 |
| `sequence` | 108 | 未由扫描证明；以证据限定范围为准 |
| `take` | 113 | 未由扫描证明；以证据限定范围为准 |

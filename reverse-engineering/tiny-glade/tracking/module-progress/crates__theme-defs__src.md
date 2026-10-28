# crates/theme-defs/src

2026-10-14 当前交付以实际 `tiny-glade/crates/theme-defs` 为准：B已定位库内业务/合同静态补全，三库26离线测试、严格Clippy/fmt通过。历史镜像与其旧源SHA不覆盖新代码；完整原入口与库外游戏接线仍未验收。详见 [最新报告](../../evidence/theme-static-2026-10-14/REPORT.md) 和 [机器台账](../theme-static-recovery.json)。

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：0/0/0。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

[业务功能对照](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)：{"contract_only": 2}。此为功能行分类，不是完成率；源码缺失与原验证/库外接线分开。

定义归属：private_game_source。

活动数据PathBuf+两个SystemTime，以及Clone/Eq和错误状态候选。

## 接线与验证

接线：typed producer/consumer transport contract; no original plugin replacement yet。

4个已观察内联类型接口有静态证据；不是完整传输流程验收。

## 已恢复的函数和类型范围

- IPI 0x3F73 clone / 0x3F76 eq
- IPI 0x3F78 clone / 0x3F7A eq

## 明确缺口

- 错误与交换格式可观察规则尚未核实
- 原生产/消费全流程未对照

下一闭包：补生产/消费和错误语义对照

## 证据

- [evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json)
- [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json)
- [evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md](../../evidence/shared-foundation-2026-10-13/paths-language/CONTRACTS.md)
- [evidence/shared-foundation-2026-10-13/integration/dependency-validation.json](../../evidence/shared-foundation-2026-10-13/integration/dependency-validation.json)
- [evidence/shared-foundation-review-2026-10-13/REVIEW.md](../../evidence/shared-foundation-review-2026-10-13/REVIEW.md)
- [tracking/shared-foundation-code-review.json](../shared-foundation-code-review.json)
- [evidence/shared-foundation-review-2026-10-13/integration-validation.json](../../evidence/shared-foundation-review-2026-10-13/integration-validation.json)
- [FOUNDATION_BUSINESS_COMPARISON.md](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)
- [tracking/shared-foundation-business-comparison.json](../shared-foundation-business-comparison.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/theme-defs/src/lib.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/theme-defs/src/lib.rs

[reconstruction/shared-foundation/crates/theme-defs/src/lib.rs](../../reconstruction/shared-foundation/crates/theme-defs/src/lib.rs)

SHA256：`542b22e4403dfe5b86e86bea1a6f79ed7a4086a7d908188a89d1322c44146216`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_active_interval` | 31 | 未由扫描证明；以证据限定范围为准 |

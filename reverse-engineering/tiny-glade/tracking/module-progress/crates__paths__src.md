# crates/paths/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：10/10/0。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

[业务功能对照](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)：{"implemented_scoped": 5}。此为功能行分类，不是完成率；源码缺失与原验证/库外接线分开。

定义归属：private_game_source。

Windows路径选择、账号目录和顺序初始化/错误传播；10个原接口映射。

## 接线与验证

接线：lang-lib persistence consumer + independent path policy entry; original tiny-glade startup not wired。

原锁候选合同验证；静态原代码规则核对，完整消费者和原入口运行差分尚未完成。

## 已恢复的函数和类型范围

- paths::config
- paths::saves
- paths::backup_saves
- paths::tmp
- paths::deleted_saves
- paths::installed
- paths::standard_app_root_path
- paths::find_standard_app_root_path
- paths::initialize_paths
- paths::ensure_path<std::path::PathBuf>

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| paths::config | [reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)：`config` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) |
| paths::saves | [reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)：`saves` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) |
| paths::backup_saves | [reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)：`backup_saves` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) |
| paths::tmp | [reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)：`tmp` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) |
| paths::deleted_saves | [reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)：`deleted_saves` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) |
| paths::installed | [reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)：`installed` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) |
| paths::standard_app_root_path | [reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)：`standard_app_root_path` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) |
| paths::find_standard_app_root_path | [reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)：`find_standard_app_root_path` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) |
| paths::initialize_paths | [reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)：`initialize_paths` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) |
| paths::ensure_path<std::path::PathBuf> | [reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)：`ensure_path` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) |

## 明确缺口

- 其他平台分支未核实
- 原初始化消费者未完整接线
- 原入口运行时差分未执行

下一闭包：对接原初始化消费者，补原入口行为对照

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
| `crates/paths/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)：explicit_scoped_function_binding | paths::config（candidate_static_evidence；业务等价以限定证据为准）<br>paths::saves（candidate_static_evidence；业务等价以限定证据为准）<br>paths::backup_saves（candidate_static_evidence；业务等价以限定证据为准）<br>paths::tmp（candidate_static_evidence；业务等价以限定证据为准）<br>paths::deleted_saves（candidate_static_evidence；业务等价以限定证据为准）<br>paths::installed（candidate_static_evidence；业务等价以限定证据为准）<br>paths::standard_app_root_path（candidate_static_evidence；业务等价以限定证据为准）<br>paths::find_standard_app_root_path（candidate_static_evidence；业务等价以限定证据为准）<br>paths::initialize_paths（candidate_static_evidence；业务等价以限定证据为准）<br>paths::ensure_path<std::path::PathBuf>（candidate_static_evidence；业务等价以限定证据为准） | candidate_present；paths::config：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json)<br>paths::saves：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json)<br>paths::backup_saves：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json)<br>paths::tmp：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json)<br>paths::deleted_saves：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json)<br>paths::installed：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json)<br>paths::standard_app_root_path：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json)<br>paths::find_standard_app_root_path：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json)<br>paths::initialize_paths：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json)<br>paths::ensure_path<std::path::PathBuf>：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/paths-language/delivery.json](../../evidence/shared-foundation-2026-10-13/paths-language/delivery.json), [evidence/shared-foundation-2026-10-13/paths-language/validation.json](../../evidence/shared-foundation-2026-10-13/paths-language/validation.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/paths/src/lib.rs

[reconstruction/shared-foundation/crates/paths/src/lib.rs](../../reconstruction/shared-foundation/crates/paths/src/lib.rs)

SHA256：`e6d4f87783c1b879e52ad0385a4fc48a331871340e77447258b989b169251103`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `initialize_at` | 22 | 未由扫描证明；以证据限定范围为准 |
| `config` | 42 | 未由扫描证明；以证据限定范围为准 |
| `saves` | 45 | 未由扫描证明；以证据限定范围为准 |
| `beta_saves` | 49 | 未由扫描证明；以证据限定范围为准 |
| `backup_saves` | 52 | 未由扫描证明；以证据限定范围为准 |
| `tmp` | 55 | 未由扫描证明；以证据限定范围为准 |
| `deleted_saves` | 58 | 未由扫描证明；以证据限定范围为准 |
| `installed` | 61 | 未由扫描证明；以证据限定范围为准 |
| `standard_app_root_path` | 64 | 未由扫描证明；以证据限定范围为准 |
| `paths` | 70 | 未由扫描证明；以证据限定范围为准 |
| `config` | 73 | 未由扫描证明；以证据限定范围为准 |
| `saves` | 76 | 未由扫描证明；以证据限定范围为准 |
| `backup_saves` | 79 | 未由扫描证明；以证据限定范围为准 |
| `tmp` | 82 | 未由扫描证明；以证据限定范围为准 |
| `deleted_saves` | 85 | 未由扫描证明；以证据限定范围为准 |
| `installed` | 88 | 未由扫描证明；以证据限定范围为准 |
| `standard_app_root_path` | 91 | 未由扫描证明；以证据限定范围为准 |
| `ensure_path` | 95 | 未由扫描证明；以证据限定范围为准 |
| `find_standard_app_root_path` | 104 | 未由扫描证明；以证据限定范围为准 |
| `choose_standard_app_root` | 112 | 未由扫描证明；以证据限定范围为准 |
| `find_standard_app_root_path` | 125 | 未由扫描证明；以证据限定范围为准 |
| `initialize_paths` | 129 | 未由扫描证明；以证据限定范围为准 |

# crates/lang-lib/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：11/11/0。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

[业务功能对照](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)：{"implemented_scoped": 7}。此为功能行分类，不是完成率；源码缺失与原验证/库外接线分开。

定义归属：private_game_source。

语言加载、回退、热切换、缺失key和请求locale持久化；修正非Unicode环境分支。

## 接线与验证

接线：候选模块可调用，完整原游戏消费者接线仍待完成。

2单元测试、5隔离进程、15发行locale通过；原8入口静态对照、15调用边界，另3内联接口单列。

## 已恢复的函数和类型范围

- lang_lib::lang_pack_path
- lang_lib::load_locale
- lang_lib::load_locale_impl
- lang_lib::load_last_used_locale
- lang_lib::lookup
- lang_lib::game_locale
- lang_lib::language_library::LanguageLibrary::get
- lang_lib::language_library::LanguageLibrary::try_from_yml

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| lang_lib::lang_pack_path | [reconstruction/shared-foundation/crates/lang-lib/src/lib.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/lib.rs)：`lang_pack_path` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json) |
| lang_lib::load_locale | [reconstruction/shared-foundation/crates/lang-lib/src/lib.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/lib.rs)：`load_locale` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json) |
| lang_lib::load_locale_impl | [reconstruction/shared-foundation/crates/lang-lib/src/lib.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/lib.rs)：`load_locale_impl` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json) |
| lang_lib::load_last_used_locale | [reconstruction/shared-foundation/crates/lang-lib/src/lib.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/lib.rs)：`load_last_used_locale` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json) |
| lang_lib::lookup | [reconstruction/shared-foundation/crates/lang-lib/src/lib.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/lib.rs)：`lookup` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json) |
| lang_lib::game_locale | [reconstruction/shared-foundation/crates/lang-lib/src/lib.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/lib.rs)：`game_locale` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json) |
| lang_lib::language_library::LanguageLibrary::get | [reconstruction/shared-foundation/crates/lang-lib/src/language_library.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/language_library.rs)：`get` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json) |
| lang_lib::language_library::LanguageLibrary::try_from_yml | [reconstruction/shared-foundation/crates/lang-lib/src/language_library.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/language_library.rs)：`try_from_yml` | B：B限定静态业务规则和候选测试，原全接口行为等价未完成 | candidate_static_evidence：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json) |

## 明确缺口

- 原ECS/UI初始化未接线
- new/locale/try_get接口状态需逐项核实
- 开发工具和完整原运行时差分未覆盖

下一闭包：补内联接口合同并接入真实消费者

## 证据

- [evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json)
- [evidence/shared-foundation-2026-10-13/lang-small/inventory.json](../../evidence/shared-foundation-2026-10-13/lang-small/inventory.json)
- [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json)
- [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json)
- [evidence/shared-foundation-2026-10-13/lang-small/locale-policy.log](../../evidence/shared-foundation-2026-10-13/lang-small/locale-policy.log)
- [evidence/shared-foundation-2026-10-13/lang-small/original-assets.log](../../evidence/shared-foundation-2026-10-13/lang-small/original-assets.log)
- [evidence/shared-foundation-review-2026-10-13/REVIEW.md](../../evidence/shared-foundation-review-2026-10-13/REVIEW.md)
- [tracking/shared-foundation-code-review.json](../shared-foundation-code-review.json)
- [evidence/shared-foundation-review-2026-10-13/integration-validation.json](../../evidence/shared-foundation-review-2026-10-13/integration-validation.json)
- [FOUNDATION_BUSINESS_COMPARISON.md](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)
- [tracking/shared-foundation-business-comparison.json](../shared-foundation-business-comparison.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/lang-lib/src/language_library.rs` | A,B,C | [reconstruction/shared-foundation/crates/lang-lib/src/language_library.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/language_library.rs)：explicit_scoped_function_binding | lang_lib::language_library::LanguageLibrary::get（candidate_static_evidence；业务等价以限定证据为准）<br>lang_lib::language_library::LanguageLibrary::try_from_yml（candidate_static_evidence；业务等价以限定证据为准） | candidate_present；lang_lib::language_library::LanguageLibrary::get：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json)<br>lang_lib::language_library::LanguageLibrary::try_from_yml：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json) |
| `crates/lang-lib/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/lang-lib/src/lib.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/lib.rs)：explicit_scoped_function_binding | lang_lib::lang_pack_path（candidate_static_evidence；业务等价以限定证据为准）<br>lang_lib::load_locale（candidate_static_evidence；业务等价以限定证据为准）<br>lang_lib::load_locale_impl（candidate_static_evidence；业务等价以限定证据为准）<br>lang_lib::load_last_used_locale（candidate_static_evidence；业务等价以限定证据为准）<br>lang_lib::lookup（candidate_static_evidence；业务等价以限定证据为准）<br>lang_lib::game_locale（candidate_static_evidence；业务等价以限定证据为准） | candidate_present；lang_lib::lang_pack_path：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json)<br>lang_lib::load_locale：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json)<br>lang_lib::load_locale_impl：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json)<br>lang_lib::load_last_used_locale：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json)<br>lang_lib::lookup：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json)<br>lang_lib::game_locale：B限定静态业务规则和候选测试，原全接口行为等价未完成：[evidence/shared-foundation-2026-10-13/lang-small/summary.json](../../evidence/shared-foundation-2026-10-13/lang-small/summary.json), [evidence/shared-foundation-2026-10-13/lang-small/validation.json](../../evidence/shared-foundation-2026-10-13/lang-small/validation.json), [evidence/shared-foundation-2026-10-13/lang-small/native-result.json](../../evidence/shared-foundation-2026-10-13/lang-small/native-result.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/lang-lib/src/language_library.rs

[reconstruction/shared-foundation/crates/lang-lib/src/language_library.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/language_library.rs)

SHA256：`152f10cdbd909d0642693c72d0c429dcf981e609fc4887272fec980336851c78`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 22 | 未由扫描证明；以证据限定范围为准 |
| `try_from_yml` | 28 | 未由扫描证明；以证据限定范围为准 |
| `parse_yml` | 33 | 未由扫描证明；以证据限定范围为准 |
| `locale` | 53 | 未由扫描证明；以证据限定范围为准 |
| `try_get` | 56 | 未由扫描证明；以证据限定范围为准 |
| `get` | 59 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/lang-lib/src/lib.rs

[reconstruction/shared-foundation/crates/lang-lib/src/lib.rs](../../reconstruction/shared-foundation/crates/lang-lib/src/lib.rs)

SHA256：`c97f7e0429a273fd4c1ee1feb0892df59fa4e34c8b4f7c332740e772fe6fb2a6`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `lang_pack_path` | 14 | 未由扫描证明；以证据限定范围为准 |
| `load_locale_impl` | 17 | 未由扫描证明；以证据限定范围为准 |
| `load_locale` | 44 | 未由扫描证明；以证据限定范围为准 |
| `load_last_used_locale` | 55 | 未由扫描证明；以证据限定范围为准 |
| `lookup` | 63 | 未由扫描证明；以证据限定范围为准 |
| `game_locale` | 81 | 未由扫描证明；以证据限定范围为准 |

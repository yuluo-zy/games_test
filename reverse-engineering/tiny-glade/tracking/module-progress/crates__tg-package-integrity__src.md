# crates/tg-package-integrity/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

B manifest原bytes校验→schema/version→逐文件missing/corrupt→递归extra→聚合候选链，原依赖复用。

## 接线与验证

接线：加入shared-foundation workspace与只读inspect_package example；原游戏报告consumer未接线。

既有4候选tests/真实manifest/disk检查；独立review原x64 FNV/数组Hash叶4096+128组合gate零差异，跨批/Unicode/junction探针通过。无确认业务bug/造轮子；64批线程创建与全文件读取成本已测，未证明原性能回退。完整verify/罕见schema/report consumer未验收。

batch-02（2026-10-13）：Ghidra 反编译 verify/find_extra_files×2/checksum 叶与候选逐条一致；修复合成测试的 OS 平台相关失败（`build-info/manifest.json/../manifest.json` Unix=Missing/Windows=Corrupt）；新增 serde/schema 边界测试（unknown-field 忽略、version 类型错误归 ManifestParse）；报告/崩溃消费者接线证据（verify_package_integrity_and_show_report 0x2552f5→run_integrity_report_gui、on_game_crashed 0x3b040f→run_crash_reporter_gui）。7 测试过、clippy 过。证据：[batch-02 BUSINESS_RULES](../../evidence/small-libs-batch-02-2026-10-13/tg-package-integrity/BUSINESS_RULES.md)、[delivery](../../evidence/small-libs-batch-02-2026-10-13/tg-package-integrity/delivery.json)、[native-oracle](../../evidence/small-libs-batch-02-2026-10-13/tg-package-integrity/native-oracle.json)。

完成轮（2026-10-14）：新增 symlink/权限探针测试（symlink 跳过、无权限目录 read_dir 静默），8 测试过。
完成度判定（2026-10-14）：库内业务逻辑全部实现并反编译验证（校验门/verify 链/extra 扫描）；未闭合仅报告/崩溃 GUI 消费者接线与 A/C（按用户保留）。
## 已恢复的函数和类型范围

- is_valid_manifest_checksum
- verify_package_manifest/verify_manifest_bytes
- find_extra_files

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| tg_package_integrity::verify_package_manifest | [reconstruction/shared-foundation/crates/tg-package-integrity/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-package-integrity/src/lib.rs)：`verify_package_manifest` | B：原静态完整规则、原manifest样本与候选disk chain；无原入口runtime | scoped_business_comparison：[evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-package-integrity/delivery.json](../../evidence/small-libs-batch-01/tg-package-integrity/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |
| tg_package_integrity::is_valid_manifest_checksum | [reconstruction/shared-foundation/crates/tg-package-integrity/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-package-integrity/src/lib.rs)：`is_valid_manifest_checksum` | B：只恢复BLAKE3/数组Hash/FNV composition与low16 gate；通用hash库复用；独立review新增原x64 Hash叶与依赖BLAKE3组合对照，非完整verify入口 | scoped_business_comparison：[evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-package-integrity/delivery.json](../../evidence/small-libs-batch-01/tg-package-integrity/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json), [evidence/small-libs-review-2026-10-13/tg-package-integrity/REVIEW.md](../../evidence/small-libs-review-2026-10-13/tg-package-integrity/REVIEW.md) |
| tg_package_integrity::find_extra_files | [reconstruction/shared-foundation/crates/tg-package-integrity/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-package-integrity/src/lib.rs)：`find_extra_files` | B：str/PathBuf实例合并；路径/递归/排除策略静态及候选disk验证 | scoped_business_comparison：[evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-package-integrity/delivery.json](../../evidence/small-libs-batch-01/tg-package-integrity/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |

## 明确缺口

- 完整verify原入口运行对照及报告/crash消费者接线
- 罕见serde schema/其他Windows OS失败边界
- 32-bit/大端Hash长度前缀
- 64批并发时序差异；O(N)线程创建、整文件buffer压力需真实release consumer测量
- A/C行为验收

下一闭包：接真实本地report消费者，再定点核实罕见schema与release性能；哈希/线程池继续复用依赖

## 证据

- [evidence/small-libs-batch-01/tg-package-integrity/delivery.json](../../evidence/small-libs-batch-01/tg-package-integrity/delivery.json)
- [evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md)
- [evidence/small-libs-batch-01/tg-package-integrity/DIFFERENCES.md](../../evidence/small-libs-batch-01/tg-package-integrity/DIFFERENCES.md)
- [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)
- [evidence/small-libs-batch-01/integration/dependency-validation.json](../../evidence/small-libs-batch-01/integration/dependency-validation.json)
- [evidence/small-libs-review-2026-10-13/tg-package-integrity/REVIEW.md](../../evidence/small-libs-review-2026-10-13/tg-package-integrity/REVIEW.md)
- [evidence/small-libs-review-2026-10-13/tg-package-integrity/findings.json](../../evidence/small-libs-review-2026-10-13/tg-package-integrity/findings.json)
- [evidence/small-libs-review-2026-10-13/integration/summary.json](../../evidence/small-libs-review-2026-10-13/integration/summary.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/tg-package-integrity/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/tg-package-integrity/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-package-integrity/src/lib.rs)：original_pdb_rules_and_scoped_native_or_candidate_comparison | tg_package_integrity::verify_package_manifest（scoped_business_comparison；业务等价以限定证据为准）<br>tg_package_integrity::is_valid_manifest_checksum（scoped_business_comparison；业务等价以限定证据为准）<br>tg_package_integrity::find_extra_files（scoped_business_comparison；业务等价以限定证据为准） | candidate_present；原清单校验与文件检查候选链；原完整入口运行待验收。：[evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)<br>tg_package_integrity::verify_package_manifest：原静态完整规则、原manifest样本与候选disk chain；无原入口runtime：[evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-package-integrity/delivery.json](../../evidence/small-libs-batch-01/tg-package-integrity/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)<br>tg_package_integrity::is_valid_manifest_checksum：只恢复BLAKE3/数组Hash/FNV composition与low16 gate；通用hash库复用；独立review新增原x64 Hash叶与依赖BLAKE3组合对照，非完整verify入口：[evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-package-integrity/delivery.json](../../evidence/small-libs-batch-01/tg-package-integrity/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json), [evidence/small-libs-review-2026-10-13/tg-package-integrity/REVIEW.md](../../evidence/small-libs-review-2026-10-13/tg-package-integrity/REVIEW.md)<br>tg_package_integrity::find_extra_files：str/PathBuf实例合并；路径/递归/排除策略静态及候选disk验证：[evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/tg-package-integrity/BUSINESS_RULES.md), [evidence/small-libs-batch-01/tg-package-integrity/delivery.json](../../evidence/small-libs-batch-01/tg-package-integrity/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/tg-package-integrity/examples/inspect_package.rs

[reconstruction/shared-foundation/crates/tg-package-integrity/examples/inspect_package.rs](../../reconstruction/shared-foundation/crates/tg-package-integrity/examples/inspect_package.rs)

SHA256：`47f0996e8072a5bdf202a8737898219b28de269eb07112946070ded78c402481`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `main` | 6 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/tg-package-integrity/src/lib.rs

[reconstruction/shared-foundation/crates/tg-package-integrity/src/lib.rs](../../reconstruction/shared-foundation/crates/tg-package-integrity/src/lib.rs)

SHA256：`377453f8574394ec6557c8ca0a598658b306f3c52effdcf0434821c6fd7b7a04`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_valid_manifest_checksum` | 39 | 未由扫描证明；以证据限定范围为准 |
| `verify_package_manifest` | 47 | 未由扫描证明；以证据限定范围为准 |
| `verify_manifest_bytes` | 55 | 未由扫描证明；以证据限定范围为准 |
| `check_files` | 83 | 未由扫描证明；以证据限定范围为准 |
| `find_extra_files` | 128 | 未由扫描证明；以证据限定范围为准 |
| `walk` | 129 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L125：/// Unknown files, recursive directories; ignore root tmp, symlinks and failed

# tg-package-integrity

检验发行包 manifest 和文件内容，将结果分成 missing/corrupt/extra 供启动与崩溃报告消费。

## 功能与目录

src/lib.rs；fixtures/manifest-contract.json、windows-boundaries.json 和 B-manifest.json；examples/inspect_package.rs 为诊断入口。

## 来源

- 恢复源码：[shared-foundation/tg-package-integrity](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-package-integrity/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-package-integrity/`。
- 原 B 包/PDB 归属：`crates/tg-package-integrity/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`3e0387cd`、`c05fad96`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/small-libs-batch-02-2026-10-13/REPORT.md)；[BUSINESS_RULES.md](../../../reverse-engineering/tiny-glade/evidence/small-libs-batch-02-2026-10-13/tg-package-integrity/BUSINESS_RULES.md)。

## 已完成

BLAKE3→Hash 长度前缀→seeded FNV low16 门、version==1 schema、并行文件校验、extra 递归扫描、tmp/符号链接跳过及读取失败策略均恢复；报告/崩溃调用者已经映射。

2026-10-14 后续复核：11 个 Windows x64 原 verify 容器案例与当前源码零差异，并冻结为回归合同。原作忽略根 `tmp` 普通文件和目录，但 manifest 列出的 `tmp` 仍做内容校验；`Tmp` 不忽略。Windows 能打开大小写、反斜杠与 `./` 路径别名，extra 的集合仍按清单字符串精确比较，self 跳过也只接受精确原名。中文/emoji 正常处理，孤立 UTF-16 surrogate 扫描名忽略。见 [后续证据](../../../reverse-engineering/tiny-glade/evidence/integrity-followup-2026-10-14/REPORT.md)。

诊断 example 已修复 read/parse `expect` 提前 panic：先执行库内 read→checksum→parse→version→files，错误输出分类 JSON，辅助 metadata 可为空。5 个阶段案例通过；这是诊断消费者修复，未新增原库算法。

## 尚差的功能与验收

正式启动/崩溃全链仍待接线；发行进程 oracle、A/C 及特殊 OS 权限边界未全验收。32/64 位 Hash 长度前缀存在原平台相关性，不声明跨平台 manifest 一律互通。

在 workspace 根执行 `cargo test -p tg-package-integrity --all-targets`。本说明列的是实现与证据范围，当前集成验证结果以根工程报告为准。

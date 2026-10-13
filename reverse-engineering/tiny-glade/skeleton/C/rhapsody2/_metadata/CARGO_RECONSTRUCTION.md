# Cargo 依赖重建

13 个包的 Cargo.toml 已补齐；全部 88 条依赖边和 54 个 workspace 依赖身份来自 C 原始锁文件。Registry 依赖精确锁版本；外部 Git 依赖锁定原 commit；rhapsody2 内部依赖指向当前独立目录。

原始锁文件参考保存在 `C-original-Cargo.lock`，SHA-256 与原作一致。它不是这个诊断 workspace 新解出的 Cargo.lock。Cargo 自身解析器已经检查全部清单、workspace 继承及目标声明；没有执行依赖下载、构建或游戏验证。

shabriri、shabriri-rt、shaderface 已接到 C/shabriri 的本地元数据骨架。其版本与原始 Git commit 继续从对应包 metadata 和本报告核对；path 替换不表示工具实现已恢复。

原仓库的清单文本无法访问。Cargo.lock 不提供 features、可选标志、target 条件或 normal/build/dev 分类，所以 `[dependencies]` 是完整已知边的规范化汇总，不声称复原了作者分类。edition 2021/resolver 2 是诊断 workspace 的明确选择，未从 PDB 推断。包没有已记录 crate root 时使用 `_metadata/unrecovered-lib.rs` 诊断占位目标。

A 没有锁文件。B 已作为对照参考，但 C 的身份及边均完整，无需 A/B 填补；例如 C 的 gpu-allocator 依赖 thiserror 2.0.11，不能混用 B 的 thiserror 1.0.60 和旧 GUI 依赖。

身份、依赖边、B 对照及未知字段见 `cargo-dependency-provenance.json`；核对结果见 `cargo-verification.json`。这些清单不构成恢复了引擎实现或作者原 Cargo 配置的证明。

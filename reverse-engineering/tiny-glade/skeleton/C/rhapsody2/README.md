# C 版 rhapsody2 PDB 骨架

仓库：`PounceLight/rhapsody2`；原作 C 锁定 commit：`48efa87114d22e484592a0e9efe866faafa6a876`。

已按 PDB 源路径创建 85 个仓库源文件，以及 6 个相关生成文件；写入 2329 个按文件归属的函数符号/类型条目。函数实现尚未恢复；缺失的参数和返回类型仍标为 unknown。

## 目录

- `src/`：rhapsody 主包，保留 raw、rg 等原目录。
- `render-std/`：core、DOF、GI、post、shadow、TAA 等 PDB 记录的包。
- `crates/`：原仓库附属包，包括 GPU profiler、allocator、同步和诊断接口。
- `_generated/target/`：6 个原 OUT_DIR 的 shabriri.rs，保留原构建路径与校验和。
- `_unlocated-packages/`：锁文件有包、PDB 无源路径的条目（rhapsody-deferred）；该目录不是推断的作者原目录。
- `_metadata/`：仓库/锁定依赖身份、文件清单、报告及全量来源映射验证。

各 `.rs` 使用注释保存函数名、泛型/闭包作用域、TPI/IPI、源码行、地址与类型引用图。同名 `.rs.pdb.jsonl.gz` 保存逐条行表、内联操作、原始符号、公开别名与编译上下文。同名 `.rs.pdb.json` 保存文件摘要。

完整原 PDB、类型、公共符号和全部模块记录仍复用 `skeleton/_pdb/C/`，未重复提取或更改。Cargo.toml 已依据 C 原锁文件补齐依赖边和版本/commit；具体来源、未知 features 与 build/target 分类见 `_metadata/CARGO_RECONSTRUCTION.md`。 本目录是明确授权的依赖元数据骨架，不集成到 `reconstruction/`，也不计作游戏业务或渲染实现恢复。

复现入口：`skeleton/_tools/restore_rhapsody2.py`。覆盖核对见 `_metadata/verification.json`。

shabriri、shabriri-rt、shaderface 已引用 C/shabriri 中的本地元数据骨架；原 Git commit 继续保存在对应包 metadata 和 cargo-dependency-provenance.json。主生成器的函数/源码未在游戏 PDB 中记录，不能将本地 path 替换当作工具实现已恢复。

# C 版 PounceLight/shabriri 骨架

原作锁定 commit：`2212a07a7c6aaa58f47283f6fff090a6744dfbed`；仓库包：shabriri、shabriri-rt、shaderface，均 0.1.0。

从 C PDB 创建 9 个已记录的源文件（shabriri-rt 3 个，shaderface 6 个），写入 228 个按文件归属的函数符号/类型条目及可用 TPI/IPI 图。每文件保留逐条压缩记录与源校验和；完整 PDB 继续引用 skeleton/_pdb/C/。

`crates/shabriri-rt/`、`crates/shaderface/` 保留原源目录。shabriri 主包在 C PDB 没有编译单元、源路径或函数命名空间记录；它可能仅构建时运行，但此用途仍是推断。主包置于 `_unlocated-packages/shabriri/`，只恢复锁文件身份、14 条依赖边和明确的诊断目标，未编造作者源码目录或生成器实现。

3 份包 Cargo.toml 和根 workspace 清单按 C 原锁文件补齐。版本、Git commit 与依赖身份可核对；edition/resolver 是诊断选择，features、optional、build/target/dev 分类、实际 build.rs 未恢复。来源和核对结果在 `_metadata/cargo-dependency-provenance.json` 与 `_metadata/cargo-verification.json`。

函数体、真实签名参数和字段布局在 PDB 缺失时保持 unknown。此目录是独立元数据骨架，不合并到 reconstruction，不宣称已经复现 shader 编译器。已有的 8 个消费者生成文件位于 C/_generated/ 和 C/rhapsody2/_generated/。

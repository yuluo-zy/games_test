# asset-pipe

读取 texture 配置和图片，生成 mip/压缩块并维护磁盘内容缓存；编解码/压缩器复用原 image、ddsfile、intel_tex_2 fork。

## 功能与目录

src/texture.rs；tests/pipeline_contract.rs、allocation_review.rs；examples/gamma_runner.rs。

## 来源

- 恢复源码：[shared-foundation/asset-pipe](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/asset-pipe/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/asset-pipe/`。
- 原 B 包/PDB 归属：`crates/asset-pipe/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`326cb236`、`3e0387cd`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[FOUNDATION_BUSINESS_COMPARISON.md](../../../reverse-engineering/tiny-glade/FOUNDATION_BUSINESS_COMPARISON.md)；[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/foundation-review-fixes-2026-10-14/REPORT.md)。

## 已完成

图片/数组纹理处理、gamma/mip/块尺寸、压缩调用组合、缓存 metadata 和内容指纹均已恢复或按后续开发修复实现。数组所有叶文件的缓存失效、旧 metadata 迁移、零 mip/零尺寸和对齐溢出错误处理已保留。

## 尚差的功能与验收

完整原 image/cache producer 闭包与全部异常/缓存策略等价仍待验收；新参数检查与内容缓存策略属于产品改变。真实资产目录编译→country-core runtime 消费、A/C 及完整离线工具入口未闭合。

## 持续开发差异

326cb236 的缓存和参数处理修复保持默认行为；历史原版等价报告只覆盖其限定原算法范围。

在 workspace 根执行 `cargo test -p asset-pipe --all-targets`。本说明列的是实现与证据范围，当前集成验证结果以根工程报告为准。

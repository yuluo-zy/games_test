# shader-compiler

已恢复的 shader 构建子链：`preprocess` 调用原 `shader-prepper` 处理 include/pragma once/#line，`DxcRequest` 恢复 stage/入口/profile/参数顺序，`reflect` 调用原 SpirQ fork，`pack` 完成 SPIR-V 后处理与 Speedy 打包，`build` 连接完整可运行链。运行时依赖资产结构的公共包为 `shader-pipeline`。

源文件：[`rhapsody-rebuild/src/shader_compiler.rs`](../../../../../reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/src/shader_compiler.rs) 及同名目录。提交 `e4e0fbe2` 恢复实际构建，`db1ecad7` 修复 original artifact 宏与 TGV recipe 分离、独立反射和原 runtime load。来源证据见 [`shader-build-2026-10-13`](../../../../../reverse-engineering/tiny-glade/evidence/shader-build-2026-10-13/SUMMARY.md) 与 [`shader-gap-investigation-2026-10-13`](../../../../../reverse-engineering/tiny-glade/evidence/shader-gap-investigation-2026-10-13/SUMMARY.md)，逐文件记录在上级 [migration-sources.json](../../migration-sources.json)。

已完成可调用预处理→真实 DXC→反射→打包子链；源选择 recipe 不参与原 artifact key；失败保存诊断且不把旧输出当成功。后者以及临时目录/provider 适配是明确的本工程策略，不宣称与原离线工具完全相同。

还缺原 DXC 版本/fork commit、离线 CLI、provider/cache/temp 命名和清理、异常策略与完整资产状态机。原作者公开编译器补丁不能直接当本构建的来源身份。当前请求签名来源是发行 SPIR-V 的 `OpModuleProcessed`，不是凭空生成离线原工具。

历史实编译覆盖 404 包/638 阶段，失败 0，397/404 typed 反射相同；132 zero-key tuple、7 schema 残差、325 阶段完整公共接口差异与整帧仍未闭合。本包可开发实现与原整工具验收分开记录；本次迁移没有重新宣称全量原 shader 语义一致。

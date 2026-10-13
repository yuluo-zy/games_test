# crates/shader-variants/src

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：local_path_source_candidate; public_origin_unconfirmed。

B原raster variant函数clone base后追加SHADER_VARIANT_DEPTH_ONLY=0/1；完整原compute宏manifest未恢复。

## 接线与验证

接线：现有rhapsody构建API接通；完整原asset构建manifest未恢复。

234原raster资产macro/key证据；新fill/egui实际编译与GPU布局通过；不代表全部变体或shader算法等价。

## 已恢复的函数和类型范围

- raster_variants clone+append（原0x1417388a0）

## 明确缺口

- 170compute原宏tuple未独证，其中38非零define key未知
- 公开上游与原optional/default/异常策略尚未核实

下一闭包：从原pass/variant生产者恢复compute宏tuple；禁止未知空宏包冒充原canonical key

## 证据

- [evidence/shader-build-2026-10-13/pipeline/validation.json](../../evidence/shader-build-2026-10-13/pipeline/validation.json)
- [evidence/shader-build-2026-10-13/provenance/package-classification.json](../../evidence/shader-build-2026-10-13/provenance/package-classification.json)
- [evidence/shader-build-2026-10-13/dxc/merged-audit.json](../../evidence/shader-build-2026-10-13/dxc/merged-audit.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/shader-variants/src/lib.rs` | A,B | [reconstruction/rhapsody-rebuild/src/shader_compiler/build.rs](../../reconstruction/rhapsody-rebuild/src/shader_compiler/build.rs)：B_PDB_source_and_raster_variant_clone_append_rule | 全集/逐函数映射待核对 | candidate_present；仅raster_variants原0x1417388a0的clone+append depth-only=0/1子集；不代表完整原文件、170compute宏manifest或未知默认/错误策略。：[evidence/shader-build-2026-10-13/pipeline/validation.json](../../evidence/shader-build-2026-10-13/pipeline/validation.json), [evidence/shader-build-2026-10-13/pipeline/delivery.json](../../evidence/shader-build-2026-10-13/pipeline/delivery.json), [evidence/shader-build-2026-10-13/provenance/versioned-pdb-source-boundaries.json](../../evidence/shader-build-2026-10-13/provenance/versioned-pdb-source-boundaries.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/rhapsody-rebuild/src/shader_compiler/build.rs

[reconstruction/rhapsody-rebuild/src/shader_compiler/build.rs](../../reconstruction/rhapsody-rebuild/src/shader_compiler/build.rs)

SHA256：`9336953401382281eacb523e0c8ccc00f3db461859428400a67e25f2fd2320bc`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `raster_variants` | 32 | 未由扫描证明；以证据限定范围为准 |
| `build_pipeline` | 42 | 未由扫描证明；以证据限定范围为准 |
| `build_pipeline_with_recipe` | 50 | 未由扫描证明；以证据限定范围为准 |

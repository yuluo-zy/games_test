# crates/asset-pipe/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：14/13/1。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

[业务功能对照](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)：{"implemented_scoped": 9, "out_of_baseline": 1}。此为功能行分类，不是完成率；源码缺失与原验证/库外接线分开。

定义归属：private_game_source。

B两原文件的图片处理、mip/padding、DDS/数组、原压缩preset、版本缓存与输出；已接原easy-parallel。

## 接线与验证

接线：候选模块可调用，完整原游戏消费者接线仍待完成。

两资产包14测试、严格Clippy通过；OETF4364原码零差异；完整原图流水线差分未完成。

## 已恢复的函数和类型范围

- compile_or_load_texture_assets
- compile_texture_assets
- compile_texture_asset
- process_regular_image / process_decoded_image
- next_level
- with_rgba8
- srgb_oetf
- compress / bc7_settings
- process_dds_image / process_dds
- assemble_array

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| B entry  0x1417b7300?compile_or_load_texture_assets | [reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)：`compile_or_load_texture_assets` | B：index loading/conditional build; modern root argument; outer flags/logs not recreated | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x1417b7790?compile_texture_assets | [reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)：`compile_texture_assets` | B：scan, tolerant metadata, sorted names, jobs complete before metadata persistence; contract fixture | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x1417b8f50?compile_texture_asset | [reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)：`compile_texture_asset` | B：descriptor+mtime+magic/version cache and output; synthetic actual-file fixture | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x1417bbc10?process_regular_image / process_decoded_image | [reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)：`process_regular_image` | B：gamma, resize, mips, format; instruction/control-flow comparison, contracts; no full original entry differential | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x1417bc930?next_level | [reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)：`next_level` | B：private mip closure; base resize/padding/type conversion; original address is closure, new API is own port | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x1420e2bd0,0x1420e3080?with_rgba8 | [reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)：`with_rgba8` | B：RGB-only output transfer; original generic specializations; wrapper differs in API | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x1417bcd20?srgb_oetf | [reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)：`srgb_oetf` | B：4364 original private instruction cases bit-identical | reported_verified_local：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md), [evidence/shared-foundation-2026-10-13/assets-small-first/gamma-verification.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/gamma-verification.json) |
| B entry  0x1417bcd60?compress / bc7_settings | [reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)：`compress` | B：original channel selection/basic presets; dependency codecs reused; contract only, not compressed-pixel native differential | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x1417ba565..0x1417bafb7?process_dds_image / process_dds | [reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)：`process_dds_image` | B：inlined DDS route in compile_texture_asset; format/layer-zero/dimensions clauses checked | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x1417b99bc..0x1417ba6e4?assemble_array | [reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)：`assemble_array` | B：inlined array route in compile_texture_asset; invariants and layer-major join checked | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |

## 明确缺口

- 原PNG/JPG/DDS源图缺失，完整原压缩结果差分未执行
- 原CLI flags/日志等外层行为尚未覆盖
- C新增文件独立待分析

下一闭包：按接口覆盖表补未触及接口及全链原侧对照

## 证据

- [evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json)
- [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)
- [evidence/shared-foundation-2026-10-13/assets-small-first/cargo-test.log](../../evidence/shared-foundation-2026-10-13/assets-small-first/cargo-test.log)
- [evidence/shared-foundation-2026-10-13/assets-small-first/clippy.log](../../evidence/shared-foundation-2026-10-13/assets-small-first/clippy.log)
- [evidence/shared-foundation-2026-10-13/assets-small-first/gamma-verification.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/gamma-verification.json)
- [evidence/shared-foundation-2026-10-13/assets-small-first/bounded-report.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/bounded-report.json)
- [evidence/shared-foundation-review-2026-10-13/REVIEW.md](../../evidence/shared-foundation-review-2026-10-13/REVIEW.md)
- [tracking/shared-foundation-code-review.json](../shared-foundation-code-review.json)
- [evidence/shared-foundation-review-2026-10-13/integration-validation.json](../../evidence/shared-foundation-review-2026-10-13/integration-validation.json)
- [FOUNDATION_BUSINESS_COMPARISON.md](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)
- [tracking/shared-foundation-business-comparison.json](../shared-foundation-business-comparison.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/asset-pipe/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/asset-pipe/src/lib.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/lib.rs)：reviewed_B_business_mapping | 全集/逐函数映射待核对 | candidate_present；B限定恢复与对照范围见报告；不代表A/C或完整原文件：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| `crates/asset-pipe/src/mip_flood.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/asset-pipe/src/store.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/asset-pipe/src/texture.rs` | A,B,C | [reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)：reviewed_B_business_mapping | B entry  0x1417b7300?compile_or_load_texture_assets（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x1417b7790?compile_texture_assets（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x1417b8f50?compile_texture_asset（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x1417bbc10?process_regular_image / process_decoded_image（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x1417bc930?next_level（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x1420e2bd0,0x1420e3080?with_rgba8（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x1417bcd20?srgb_oetf（reported_verified_local；业务等价以限定证据为准）<br>B entry  0x1417bcd60?compress / bc7_settings（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x1417ba565..0x1417bafb7?process_dds_image / process_dds（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x1417b99bc..0x1417ba6e4?assemble_array（candidate_business_comparison；业务等价以限定证据为准） | candidate_present；B限定恢复与对照范围见报告；不代表A/C或完整原文件：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x1417b7300?compile_or_load_texture_assets：index loading/conditional build; modern root argument; outer flags/logs not recreated：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x1417b7790?compile_texture_assets：scan, tolerant metadata, sorted names, jobs complete before metadata persistence; contract fixture：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x1417b8f50?compile_texture_asset：descriptor+mtime+magic/version cache and output; synthetic actual-file fixture：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x1417bbc10?process_regular_image / process_decoded_image：gamma, resize, mips, format; instruction/control-flow comparison, contracts; no full original entry differential：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x1417bc930?next_level：private mip closure; base resize/padding/type conversion; original address is closure, new API is own port：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x1420e2bd0,0x1420e3080?with_rgba8：RGB-only output transfer; original generic specializations; wrapper differs in API：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x1417bcd20?srgb_oetf：4364 original private instruction cases bit-identical：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md), [evidence/shared-foundation-2026-10-13/assets-small-first/gamma-verification.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/gamma-verification.json)<br>B entry  0x1417bcd60?compress / bc7_settings：original channel selection/basic presets; dependency codecs reused; contract only, not compressed-pixel native differential：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x1417ba565..0x1417bafb7?process_dds_image / process_dds：inlined DDS route in compile_texture_asset; format/layer-zero/dimensions clauses checked：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x1417b99bc..0x1417ba6e4?assemble_array：inlined array route in compile_texture_asset; invariants and layer-major join checked：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| `crates/asset-pipe/src/texture_processing.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/asset-pipe/src/lib.rs

[reconstruction/shared-foundation/crates/asset-pipe/src/lib.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/lib.rs)

SHA256：`9edc23d6191c33d74a6327671ce66036425614b9e9e190ac22c99340d92ed8b9`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs

[reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs](../../reconstruction/shared-foundation/crates/asset-pipe/src/texture.rs)

SHA256：`581d6ee69449a5a33426241589d81e23775c0a0e29de1f3bde2cc42cc234dfba`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `srgb_oetf` | 22 | 未由扫描证明；以证据限定范围为准 |
| `srgb_eotf` | 29 | 未由扫描证明；以证据限定范围为准 |
| `mip_count` | 37 | 未由扫描证明；以证据限定范围为准 |
| `pad_edges` | 46 | 未由扫描证明；以证据限定范围为准 |
| `compress` | 53 | 未由扫描证明；以证据限定范围为准 |
| `bc7_settings` | 93 | 未由扫描证明；以证据限定范围为准 |
| `with_rgba8` | 102 | 未由扫描证明；以证据限定范围为准 |
| `encode_level` | 122 | 未由扫描证明；以证据限定范围为准 |
| `next_level` | 133 | 未由扫描证明；以证据限定范围为准 |
| `align` | 144 | 未由扫描证明；以证据限定范围为准 |
| `process_regular_image` | 150 | 未由扫描证明；以证据限定范围为准 |
| `process_decoded_image` | 158 | 未由扫描证明；以证据限定范围为准 |
| `process_dds_image` | 216 | 未由扫描证明；以证据限定范围为准 |
| `process_dds` | 220 | 未由扫描证明；以证据限定范围为准 |
| `assemble_array` | 267 | 未由扫描证明；以证据限定范围为准 |
| `process_texture_asset` | 291 | 未由扫描证明；以证据限定范围为准 |
| `cache_is_current` | 308 | 未由扫描证明；以证据限定范围为准 |
| `compile_texture_asset` | 321 | 未由扫描证明；以证据限定范围为准 |
| `compile_texture_assets` | 357 | 未由扫描证明；以证据限定范围为准 |
| `compile_or_load_texture_assets` | 389 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L245：R::Unknown \| R::Buffer => anyhow::bail!("DDS dimension is not a texture"),

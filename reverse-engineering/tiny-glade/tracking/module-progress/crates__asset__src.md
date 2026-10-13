# crates/asset/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：18/16/2。[逐接口进度](../../INTERFACE_PROGRESS.md)。

[业务功能对照](../../FOUNDATION_BUSINESS_COMPARISON.md)：{"partial_missing_logic": 2, "implemented_scoped": 6, "out_of_baseline": 2}。此为功能行分类，不是完成率；源码缺失与原验证/库外接线分开。

定义归属：private_game_source。

B九原文件的资源扫描、prefab继承、纹理交换格式；texture.rs是合成glue，不计C恢复。

## 接线与验证

接线：候选模块可调用，完整原游戏消费者接线仍待完成。

388原纹理字节回写一致，25prefab/28依赖通过；候选合同与原全链差分分开。

## 已恢复的函数和类型范围

- Inheritable::inherit_from / Option::inherit_from / HashMap::inherit_from
- deserialize_inheritable_option
- PrefabName::new
- PrefabName::as_name_ref / AsRef::as_ref
- PrefabConfig::inherit_from / PrefabPartConfig::inherit_from
- PrefabRtConfig::inherit_from / PrefabRenderConfig::inherit_from / AudioTriggerConfig::inherit_from
- deserialize_float3
- PrefabDependencies::add_file_to_prefab_dependency
- scan_asset_tree
- scan_asset_tree_walker
- load_prefab_config
- TextureAssetBlob transport / load / to_file_bytes
- InheritableTextureOpts::inherit_from / into_concrete
- TextureOpts / TextureAssetDesc / Gamma / MipChain / CompressionMethod
- TextureAssetDesc::source_files
- scan_asset_tree
- scan_asset_tree_walker

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| B entry  0x142689b90,0x142689ed0,0x14268a2e0,0x14268a700,0x142424d90?Inheritable::inherit_from / Option::inherit_from / HashMap::inherit_from | [reconstruction/shared-foundation/crates/asset/src/inheritable.rs](../../reconstruction/shared-foundation/crates/asset/src/inheritable.rs)：`inherit_from` | B：control-flow comparison; own semantic data types; not native ABI | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x14268aa60,0x14268aee0,0x14268b260,0x14268b770?deserialize_inheritable_option | [reconstruction/shared-foundation/crates/asset/src/inheritable.rs](../../reconstruction/shared-foundation/crates/asset/src/inheritable.rs)：`deserialize_inheritable_option` | B：typed RON thin adaptation; actual prefab plus bare/Some/None/unit/error contracts | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x141556080?PrefabName::new | [reconstruction/shared-foundation/crates/asset/src/prefab.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab.rs)：`new` | B：Arc<str> wrapper, dependency allocation semantics reused | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x141556130,0x14067dd80?PrefabName::as_name_ref / AsRef::as_ref | [reconstruction/shared-foundation/crates/asset/src/prefab.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab.rs)：`as_name_ref` | B：borrowed name/access; no native-layout claim | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x142424d90?PrefabConfig::inherit_from / PrefabPartConfig::inherit_from | [reconstruction/shared-foundation/crates/asset/src/prefab/config.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab/config.rs)：`inherit_from` | B：map merge and default resolution candidate; unresolved/default paths not exhaustive | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x142879c20,0x142879de0,0x142689ed0,0x142689b90?PrefabRtConfig::inherit_from / PrefabRenderConfig::inherit_from / AudioTriggerConfig::inherit_from | [reconstruction/shared-foundation/crates/asset/src/prefab/config.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab/config.rs)：`inherit_from` | B：field-level override candidates; semantic mappings, not native structure declarations | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x142009df0?PrefabDependencies::add_file_to_prefab_dependency | [reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs)：`add_file_to_prefab_dependency` | B：file→prefab vector relationship; duplicate registrations retained | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x14200a1b0?scan_asset_tree | [reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs)：`scan_asset_tree` | B：25 original shipped prefabs/28 dependency files parse+merge | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x14200a5f0?scan_asset_tree_walker | [reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs)：`scan_asset_tree_walker` | B：default.ron inheritance and recursion; consumer native differential remains | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x14200ba50?load_prefab_config | [reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs)：`load_prefab_config` | B：original RON delegation; modern compiler adaptation | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x142456250?InheritableTextureOpts::inherit_from / into_concrete | [reconstruction/shared-foundation/crates/asset/src/texture/config.rs](../../reconstruction/shared-foundation/crates/asset/src/texture/config.rs)：`inherit_from` | B：field resolution candidate; defaults/source file syntax validated; all panic/default cases not exhaustive | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x1420d8ba0?scan_asset_tree | [reconstruction/shared-foundation/crates/asset/src/texture/scan.rs](../../reconstruction/shared-foundation/crates/asset/src/texture/scan.rs)：`scan_asset_tree` | B：root defaults and mapping; original configs/fixture scan | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| B entry  0x1420d8d00?scan_asset_tree_walker | [reconstruction/shared-foundation/crates/asset/src/texture/scan.rs](../../reconstruction/shared-foundation/crates/asset/src/texture/scan.rs)：`scan_asset_tree_walker` | B：configuration inheritance, extension selection, consecutive array indexing; fixtures | candidate_business_comparison：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |

## 明确缺口

- prefab全部默认/错误/副作用路径未穷尽
- country-core资产热重载消费者未接线
- C新增文件须独立身份分析

下一闭包：按接口覆盖表补未触及接口及真实消费者

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
- [FOUNDATION_BUSINESS_COMPARISON.md](../../FOUNDATION_BUSINESS_COMPARISON.md)
- [tracking/shared-foundation-business-comparison.json](../shared-foundation-business-comparison.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/asset/src/foliage.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/asset/src/inheritable.rs` | A,B,C | [reconstruction/shared-foundation/crates/asset/src/inheritable.rs](../../reconstruction/shared-foundation/crates/asset/src/inheritable.rs)：reviewed_B_business_mapping | B entry  0x142689b90,0x142689ed0,0x14268a2e0,0x14268a700,0x142424d90?Inheritable::inherit_from / Option::inherit_from / HashMap::inherit_from（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x14268aa60,0x14268aee0,0x14268b260,0x14268b770?deserialize_inheritable_option（candidate_business_comparison；业务等价以限定证据为准） | candidate_present；B限定恢复与对照范围见报告；不代表A/C或完整原文件：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x142689b90,0x142689ed0,0x14268a2e0,0x14268a700,0x142424d90?Inheritable::inherit_from / Option::inherit_from / HashMap::inherit_from：control-flow comparison; own semantic data types; not native ABI：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x14268aa60,0x14268aee0,0x14268b260,0x14268b770?deserialize_inheritable_option：typed RON thin adaptation; actual prefab plus bare/Some/None/unit/error contracts：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| `crates/asset/src/legacy_mod.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/asset/src/lib.rs` | B,C | [reconstruction/shared-foundation/crates/asset/src/lib.rs](../../reconstruction/shared-foundation/crates/asset/src/lib.rs)：reviewed_B_business_mapping | 全集/逐函数映射待核对 | candidate_present；B限定恢复与对照范围见报告；不代表A/C或完整原文件：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| `crates/asset/src/mesh.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/asset/src/prefab.rs` | A,B,C | [reconstruction/shared-foundation/crates/asset/src/prefab.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab.rs)：reviewed_B_business_mapping<br>[skeleton/task-index/work/TG-a3f23bd62617630cba/abc-tools-52/crates/asset/src/prefab.rs](../../skeleton/task-index/work/TG-a3f23bd62617630cba/abc-tools-52/crates/asset/src/prefab.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-a3f23bd62617630cba/abc-tools-52/crates/asset/src/prefab.rs](../../skeleton/task-index/work/TG-a3f23bd62617630cba/abc-tools-52/crates/asset/src/prefab.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-a3f23bd62617630cba/abc-tools-52/crates/asset/src/prefab.rs](../../skeleton/task-index/work/TG-a3f23bd62617630cba/abc-tools-52/crates/asset/src/prefab.rs)：explicit_candidate_delivery_mapping | asset::prefab::impl$5::as_ref（implemented_unverified；业务等价以限定证据为准）<br>asset::prefab::PrefabName::as_name_ref（implemented_unverified；业务等价以限定证据为准）<br>asset::prefab::impl$4::as_ref（implemented_unverified；业务等价以限定证据为准）<br>B entry  0x141556080?PrefabName::new（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x141556130,0x14067dd80?PrefabName::as_name_ref / AsRef::as_ref（candidate_business_comparison；业务等价以限定证据为准） | candidate_present；B限定恢复与对照范围见报告；不代表A/C或完整原文件：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x141556080?PrefabName::new：Arc<str> wrapper, dependency allocation semantics reused：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x141556130,0x14067dd80?PrefabName::as_name_ref / AsRef::as_ref：borrowed name/access; no native-layout claim：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| `crates/asset/src/prefab/config.rs` | A,B,C | [reconstruction/shared-foundation/crates/asset/src/prefab/config.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab/config.rs)：reviewed_B_business_mapping | B entry  0x142424d90?PrefabConfig::inherit_from / PrefabPartConfig::inherit_from（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x142879c20,0x142879de0,0x142689ed0,0x142689b90?PrefabRtConfig::inherit_from / PrefabRenderConfig::inherit_from / AudioTriggerConfig::inherit_from（candidate_business_comparison；业务等价以限定证据为准） | candidate_present；B限定恢复与对照范围见报告；不代表A/C或完整原文件：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x142424d90?PrefabConfig::inherit_from / PrefabPartConfig::inherit_from：map merge and default resolution candidate; unresolved/default paths not exhaustive：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x142879c20,0x142879de0,0x142689ed0,0x142689b90?PrefabRtConfig::inherit_from / PrefabRenderConfig::inherit_from / AudioTriggerConfig::inherit_from：field-level override candidates; semantic mappings, not native structure declarations：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| `crates/asset/src/prefab/scan.rs` | A,B,C | [reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs)：reviewed_B_business_mapping | B entry  0x142009df0?PrefabDependencies::add_file_to_prefab_dependency（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x14200a1b0?scan_asset_tree（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x14200a5f0?scan_asset_tree_walker（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x14200ba50?load_prefab_config（candidate_business_comparison；业务等价以限定证据为准） | candidate_present；B限定恢复与对照范围见报告；不代表A/C或完整原文件：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x142009df0?PrefabDependencies::add_file_to_prefab_dependency：file→prefab vector relationship; duplicate registrations retained：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x14200a1b0?scan_asset_tree：25 original shipped prefabs/28 dependency files parse+merge：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x14200a5f0?scan_asset_tree_walker：default.ron inheritance and recursion; consumer native differential remains：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x14200ba50?load_prefab_config：original RON delegation; modern compiler adaptation：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| `crates/asset/src/texture.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/asset/src/texture/blob.rs` | A,B,C | [reconstruction/shared-foundation/crates/asset/src/texture/blob.rs](../../reconstruction/shared-foundation/crates/asset/src/texture/blob.rs)：reviewed_B_business_mapping | 全集/逐函数映射待核对 | candidate_present；B限定恢复与对照范围见报告；不代表A/C或完整原文件：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| `crates/asset/src/texture/config.rs` | A,B,C | [reconstruction/shared-foundation/crates/asset/src/texture/config.rs](../../reconstruction/shared-foundation/crates/asset/src/texture/config.rs)：reviewed_B_business_mapping | B entry  0x142456250?InheritableTextureOpts::inherit_from / into_concrete（candidate_business_comparison；业务等价以限定证据为准） | candidate_present；B限定恢复与对照范围见报告；不代表A/C或完整原文件：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x142456250?InheritableTextureOpts::inherit_from / into_concrete：field resolution candidate; defaults/source file syntax validated; all panic/default cases not exhaustive：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| `crates/asset/src/texture/desc.rs` | A,B,C | [reconstruction/shared-foundation/crates/asset/src/texture/desc.rs](../../reconstruction/shared-foundation/crates/asset/src/texture/desc.rs)：reviewed_B_business_mapping | 全集/逐函数映射待核对 | candidate_present；B限定恢复与对照范围见报告；不代表A/C或完整原文件：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |
| `crates/asset/src/texture/scan.rs` | A,B,C | [reconstruction/shared-foundation/crates/asset/src/texture/scan.rs](../../reconstruction/shared-foundation/crates/asset/src/texture/scan.rs)：reviewed_B_business_mapping | B entry  0x1420d8ba0?scan_asset_tree（candidate_business_comparison；业务等价以限定证据为准）<br>B entry  0x1420d8d00?scan_asset_tree_walker（candidate_business_comparison；业务等价以限定证据为准） | candidate_present；B限定恢复与对照范围见报告；不代表A/C或完整原文件：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x1420d8ba0?scan_asset_tree：root defaults and mapping; original configs/fixture scan：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md)<br>B entry  0x1420d8d00?scan_asset_tree_walker：configuration inheritance, extension selection, consecutive array indexing; fixtures：[evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json](../../evidence/shared-foundation-2026-10-13/assets-small-first/delivery.json), [evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md](../../evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/asset/src/inheritable.rs

[reconstruction/shared-foundation/crates/asset/src/inheritable.rs](../../reconstruction/shared-foundation/crates/asset/src/inheritable.rs)

SHA256：`2f64c818ae1126a90e7814240e6eed694bab413bece4328fed8354fd098a42a9`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `inherit_from` | 14 | 未由扫描证明；以证据限定范围为准 |
| `finish_inheritance` | 17 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 20 | 未由扫描证明；以证据限定范围为准 |
| `as_concrete` | 31 | 未由扫描证明；以证据限定范围为准 |
| `inherit_shallow` | 46 | 未由扫描证明；以证据限定范围为准 |
| `into_concrete` | 55 | 未由扫描证明；以证据限定范围为准 |
| `deserialize` | 69 | 未由扫描证明；以证据限定范围为准 |
| `deserialize_inheritable` | 75 | 未由扫描证明；以证据限定范围为准 |
| `deserialize_inheritable_option` | 83 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 102 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 114 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/asset/src/lib.rs

[reconstruction/shared-foundation/crates/asset/src/lib.rs](../../reconstruction/shared-foundation/crates/asset/src/lib.rs)

SHA256：`293e0615754a8491caece8a084c75fe110e4fea0e01e583190f6f5c661cf04e3`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/shared-foundation/crates/asset/src/prefab.rs

[reconstruction/shared-foundation/crates/asset/src/prefab.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab.rs)

SHA256：`6692ce67bee015587c93db7c507160e85fe396718e14b55940fb1d2fb900bcfb`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_str_ref` | 13 | 未由扫描证明；以证据限定范围为准 |
| `as_str` | 18 | 未由扫描证明；以证据限定范围为准 |
| `as_ref` | 23 | 未由扫描证明；以证据限定范围为准 |
| `new` | 30 | 未由扫描证明；以证据限定范围为准 |
| `as_str` | 33 | 未由扫描证明；以证据限定范围为准 |
| `as_name_ref` | 36 | 未由扫描证明；以证据限定范围为准 |
| `as_ref` | 41 | 未由扫描证明；以证据限定范围为准 |
| `as_ref` | 46 | 未由扫描证明；以证据限定范围为准 |
| `borrow` | 51 | 未由扫描证明；以证据限定范围为准 |
| `deref` | 57 | 未由扫描证明；以证据限定范围为准 |
| `from` | 62 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/asset/src/prefab/config.rs

[reconstruction/shared-foundation/crates/asset/src/prefab/config.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab/config.rs)

SHA256：`cc63c2d25f3851cf6433d161cd1034e2a24df52916fe71c0d7feb17a03308f9e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `deserialize_float3` | 5 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 20 | 未由扫描证明；以证据限定范围为准 |
| `resolved_defaults` | 39 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 49 | 未由扫描证明；以证据限定范围为准 |
| `concrete_default` | 65 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 70 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 81 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 97 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 100 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 103 | 未由扫描证明；以证据限定范围为准 |
| `concrete_default` | 122 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 134 | 未由扫描证明；以证据限定范围为准 |
| `finish_inheritance` | 142 | 未由扫描证明；以证据限定范围为准 |
| `concrete_default` | 150 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 155 | 未由扫描证明；以证据限定范围为准 |
| `finish_inheritance` | 158 | 未由扫描证明；以证据限定范围为准 |
| `concrete_default` | 170 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 175 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs

[reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs](../../reconstruction/shared-foundation/crates/asset/src/prefab/scan.rs)

SHA256：`ad310c2aaa55c750d80195c2ac6f9bb061bfad32e9c3f30b04ff0d9dde1cf062`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `add_file_to_prefab_dependency` | 16 | 未由扫描证明；以证据限定范围为准 |
| `load_prefab_config` | 25 | 未由扫描证明；以证据限定范围为准 |
| `scan_asset_tree` | 29 | 未由扫描证明；以证据限定范围为准 |
| `scan_asset_tree_walker` | 40 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/asset/src/texture.rs

[reconstruction/shared-foundation/crates/asset/src/texture.rs](../../reconstruction/shared-foundation/crates/asset/src/texture.rs)

SHA256：`9d107c5aa5b10184e6bb26744e13f28ebd1a38a6d0c8b8351032ba35b2921851`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/shared-foundation/crates/asset/src/texture/blob.rs

[reconstruction/shared-foundation/crates/asset/src/texture/blob.rs](../../reconstruction/shared-foundation/crates/asset/src/texture/blob.rs)

SHA256：`3c167d325ab2942eae36a6d150a28ea536ee8d80b82ce085b6e084f41901df84`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `load` | 17 | 未由扫描证明；以证据限定范围为准 |
| `to_file_bytes` | 33 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L1：//! Private archived schema. Numeric `kind` is preserved because its original Rust enum name is unknown.
### reconstruction/shared-foundation/crates/asset/src/texture/config.rs

[reconstruction/shared-foundation/crates/asset/src/texture/config.rs](../../reconstruction/shared-foundation/crates/asset/src/texture/config.rs)

SHA256：`e2066688d980b3c446fafe598dcea9f3367ac1084a75f9b2a978a0973ba67088`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `inherit_from` | 16 | 未由扫描证明；以证据限定范围为准 |
| `concrete_default` | 26 | 未由扫描证明；以证据限定范围为准 |
| `from_concrete` | 35 | 未由扫描证明；以证据限定范围为准 |
| `into_concrete` | 44 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L43：/// B 0x142456250: fields resolved independently; unknown defaults panic, no invented fallbacks.
### reconstruction/shared-foundation/crates/asset/src/texture/desc.rs

[reconstruction/shared-foundation/crates/asset/src/texture/desc.rs](../../reconstruction/shared-foundation/crates/asset/src/texture/desc.rs)

SHA256：`c87be9f80e1c2deb1d4df1a67a547582bd594a94eea9334dae2216aa42892843`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `inherit_from` | 26 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 29 | 未由扫描证明；以证据限定范围为准 |
| `inherit_from` | 32 | 未由扫描证明；以证据限定范围为准 |
| `source_files` | 49 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/asset/src/texture/scan.rs

[reconstruction/shared-foundation/crates/asset/src/texture/scan.rs](../../reconstruction/shared-foundation/crates/asset/src/texture/scan.rs)

SHA256：`fa5c157bf0cfdc58949d4869683ee7a2d21a725a9b5133ae15059dfb5550f929`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `scan_asset_tree` | 8 | 未由扫描证明；以证据限定范围为准 |
| `scan_asset_tree_walker` | 19 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-a3f23bd62617630cba/abc-tools-52/crates/asset/src/prefab.rs

[skeleton/task-index/work/TG-a3f23bd62617630cba/abc-tools-52/crates/asset/src/prefab.rs](../../skeleton/task-index/work/TG-a3f23bd62617630cba/abc-tools-52/crates/asset/src/prefab.rs)

SHA256：`fb7b89c723863ac7f5db0b8712db97d96024a97a4c693dc4b9eb983a3992ad23`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `as_name_ref` | 18 | 未由扫描证明；以证据限定范围为准 |
| `as_ref` | 25 | 未由扫描证明；以证据限定范围为准 |
| `as_ref_identity` | 33 | 未由扫描证明；以证据限定范围为准 |

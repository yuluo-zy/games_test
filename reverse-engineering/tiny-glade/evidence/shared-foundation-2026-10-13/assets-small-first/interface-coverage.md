# B asset / asset-pipe 接口覆盖对照

本表来自 B 原 11 个文件的 PDB 骨架函数/inlinee 声明、原 LLVM procedure 库存及 public alias。骨架不是源码实现；PDB `void()` 不是 ABI。按原私有定义归属去 hash、合并同一泛型重复，不把 1013 个含库 procedure 当接口分母，也不把同地址的不同原 trait 接口合并。

已观察私有接口 **61**：已业务分析 **42**、尚未触及 **19**、已有对应行为实现 **42**、原指令差分 **2**、原 country-core 消费者接线 **0**。另列 27 个 closure（4 已分析）及 6 组 typed schema 规则。库存仍是已观察下界，不计算全包完成率。

`implemented_static` 是已列行为的 Rust 实现/静态对照，不代表完整原方法等价；`native_compared` 仅 OETF 与浅继承的四种纹理字段实例。布尔维度独立：内部 asset-pipe→asset 已编译调用与原游戏消费者接线分开，原类型/错误边界/未记录接口仍可能缺失。

| 原接口（同泛型已合并） | 状态 | 原地址/形式 | 恢复 API |
|---|---|---|---|
| `asset::inheritable::Inheritable::as_concrete` | implemented_static | inlinee 声明；地址未提取 | `Inheritable::as_concrete` |
| `asset::inheritable::Inheritable::inherit_shallow` | native_compared | inlinee 声明；地址未提取 | `Inheritable::inherit_shallow` |
| `asset::inheritable::Inheritable::into_concrete` | implemented_static | inlinee 声明；地址未提取 | `Inheritable::into_concrete` |
| `asset::inheritable::deserialize_inheritable` | implemented_static | inlinee 声明；地址未提取 | `deserialize_inheritable; Inheritable<T>::deserialize delegates` |
| `asset::inheritable::deserialize_inheritable_option` | implemented_static | 0x14268aa60, 0x14268aee0, 0x14268b260, 0x14268b770 | `deserialize_inheritable_option` |
| `asset::inheritable::impl$1::inherit_from` | implemented_static | 0x142689b90, 0x142689ed0, 0x14268a2e0, 0x14268a700 | `impl Inherit for Inheritable<T>` |
| `asset::inheritable::impl$2::concrete_default` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::inheritable::impl$3::inherit_from` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::inheritable::impl$5::inherit_from` | implemented_static | 0x142424d90 | `impl Inherit for HashMap<K,V>` |
| `asset::inheritable::impl$6::concrete_default` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::PrefabName::AsRef<str>::as_ref` | implemented_static | 0x141556130 | `impl AsRef<str> for PrefabName` |
| `asset::prefab::PrefabName::Deref::deref` | implemented_static | 0x141556130 | `impl Deref<Target=PrefabNameRef> for PrefabName` |
| `asset::prefab::PrefabName::From<&str>::from` | implemented_static | 0x141556080 | `impl From<&str> for PrefabName` |
| `asset::prefab::PrefabName::as_name_ref` | implemented_static | 0x141556130 | `PrefabName::as_name_ref` |
| `asset::prefab::PrefabName::as_str` | implemented_static | 0x141556130 | `PrefabName::as_str` |
| `asset::prefab::PrefabName::new` | implemented_static | 0x141556080 | `PrefabName::new` |
| `asset::prefab::PrefabNameRef::as_str` | implemented_static | 0x14067dd80 | `PrefabNameRef::as_str` |
| `asset::prefab::config::impl$10::inherit_from` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::config::impl$11::concrete_default` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::config::impl$13::default` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::config::impl$1::inherit_from` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::config::impl$2::concrete_default` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::config::impl$3::inherit_from` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::config::impl$4::concrete_default` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::config::impl$5::inherit_from` | implemented_static | 0x142879c20 | `PrefabRenderConfig::inherit_from` |
| `asset::prefab::config::impl$6::inherit_from` | implemented_static | 0x142879de0 | `PrefabRtConfig::inherit_from` |
| `asset::prefab::config::impl$8::inherit_from` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::config::impl$9::concrete_default` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::impl$4::as_ref` | implemented_static | 0x14067dd80 | `impl AsRef<PrefabNameRef> for PrefabNameRef` |
| `asset::prefab::impl$5::as_ref` | implemented_static | 0x141556130 | `impl AsRef<PrefabNameRef> for PrefabName` |
| `asset::prefab::impl$6::borrow` | implemented_static | inlinee 声明；地址未提取 | `impl Borrow<PrefabNameRef> for PrefabName` |
| `asset::prefab::scan::PrefabDependencies::add_file_to_prefab_dependency` | implemented_static | 0x142009df0 | `add_file_to_prefab_dependency` |
| `asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::scan::impl$2::default` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::scan::impl$3::default` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::scan::load_prefab_config` | implemented_static | 0x14200ba50 | `load_prefab_config` |
| `asset::prefab::scan::load_prefab_config::do_load` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::prefab::scan::scan_asset_tree` | implemented_static | 0x14200a1b0 | `scan_asset_tree` |
| `asset::prefab::scan::scan_asset_tree_walker` | implemented_static | 0x14200a5f0 | `scan_asset_tree_walker` |
| `asset::texture::config::InheritableTextureOpts::concrete_default` | implemented_static | inlinee 声明；地址未提取 | `InheritableTextureOpts::concrete_default` |
| `asset::texture::config::InheritableTextureOpts::inherit_from` | implemented_static | inlinee 声明；地址未提取 | `InheritableTextureOpts::inherit_from` |
| `asset::texture::config::InheritableTextureOpts::into_concrete` | implemented_static | 0x142456250 | `InheritableTextureOpts::into_concrete` |
| `asset::texture::config::impl$2::default` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset::texture::scan::load_texture_group_config` | implemented_static | inlinee 声明；地址未提取 | `scan_asset_tree_walker RON loading branch` |
| `asset::texture::scan::scan_asset_tree` | implemented_static | 0x1420d8ba0 | `scan_asset_tree` |
| `asset::texture::scan::scan_asset_tree_walker` | implemented_static | 0x1420d8d00 | `scan_asset_tree_walker` |
| `asset_pipe::texture::BcMode::block_bytes` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset_pipe::texture::apply_transfer_function` | implemented_static | inlinee 声明；地址未提取 | `with_rgba8 and process_decoded_image RGB loops` |
| `asset_pipe::texture::compile_or_load_texture_assets` | implemented_static | 0x1417b7300 | `compile_or_load_texture_assets` |
| `asset_pipe::texture::compile_texture_asset` | implemented_static | 0x1417b8f50 | `compile_texture_asset` |
| `asset_pipe::texture::compile_texture_assets` | implemented_static | 0x1417b7790 | `compile_texture_assets` |
| `asset_pipe::texture::compress` | implemented_static | 0x1417bcd60 | `compress` |
| `asset_pipe::texture::impl$1::default` | untouched | inlinee 声明；地址未提取 | `未恢复；已有共享 body 分析也不补造 typed API` |
| `asset_pipe::texture::pad_image` | implemented_static | inlinee 声明；地址未提取 | `pad_edges/next_level` |
| `asset_pipe::texture::process_dds_image` | implemented_static | inlinee 声明；地址未提取 | `process_dds_image/process_dds` |
| `asset_pipe::texture::process_image_array` | implemented_static | inlinee 声明；地址未提取 | `process_texture_asset/assemble_array` |
| `asset_pipe::texture::process_regular_image` | implemented_static | 0x1417bbc10 | `process_regular_image` |
| `asset_pipe::texture::resize_image` | implemented_static | inlinee 声明；地址未提取 | `process_decoded_image/next_level resize_exact branches` |
| `asset_pipe::texture::srgb_eotf` | implemented_static | inlinee 声明；地址未提取 | `srgb_eotf` |
| `asset_pipe::texture::srgb_oetf` | native_compared | 0x1417bcd20 | `srgb_oetf` |
| `asset_pipe::texture::with_rgba8` | implemented_static | 0x1420e2bd0, 0x1420e3080 | `with_rgba8` |

## 尚未触及的接口

- `asset::inheritable::impl$2::concrete_default`
- `asset::inheritable::impl$3::inherit_from`
- `asset::inheritable::impl$6::concrete_default`
- `asset::prefab::config::impl$10::inherit_from`
- `asset::prefab::config::impl$11::concrete_default`
- `asset::prefab::config::impl$13::default`
- `asset::prefab::config::impl$1::inherit_from`
- `asset::prefab::config::impl$2::concrete_default`
- `asset::prefab::config::impl$3::inherit_from`
- `asset::prefab::config::impl$4::concrete_default`
- `asset::prefab::config::impl$8::inherit_from`
- `asset::prefab::config::impl$9::concrete_default`
- `asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency`
- `asset::prefab::scan::impl$2::default`
- `asset::prefab::scan::impl$3::default`
- `asset::prefab::scan::load_prefab_config::do_load`
- `asset::texture::config::impl$2::default`
- `asset_pipe::texture::BcMode::block_bytes`
- `asset_pipe::texture::impl$1::default`

尚未触及的 indexed concrete_default/default/部分 inherit_from、PrefabScanResult 依赖联接和 BcMode 字节数接口保持单独缺口。仅从其他 caller 见到相似规则或写了候选 Default，不算这些原接口已分析。新增继承/借用名称/纹理 concrete_default 的对照和测试见 interface-batch-02。

## Closure 和 schema 分开记录

| Closure | 状态 | 对应 port |
|---|---|---|
| `asset::inheritable::impl$5::inherit_from::closure$0` | untouched | 未分析 |
| `asset::inheritable::impl$5::inherit_from::closure$1` | untouched | 未分析 |
| `asset::prefab::scan::load_prefab_config::closure$0` | untouched | 未分析 |
| `asset::prefab::scan::scan_asset_tree_walker::closure$1` | untouched | 未分析 |
| `asset::texture::scan::scan_asset_tree_walker::closure$0` | untouched | 未分析 |
| `asset::texture::scan::scan_asset_tree_walker::closure$1` | untouched | 未分析 |
| `asset::texture::scan::scan_asset_tree_walker::closure$2` | untouched | 未分析 |
| `asset::texture::scan::scan_asset_tree_walker::closure$3` | untouched | 未分析 |
| `asset_pipe::texture::compile_or_load_texture_assets::closure$0` | untouched | 未分析 |
| `asset_pipe::texture::compile_or_load_texture_assets::closure$1` | untouched | 未分析 |
| `asset_pipe::texture::compile_or_load_texture_assets::closure$2` | untouched | 未分析 |
| `asset_pipe::texture::compile_texture_asset::closure$0` | untouched | 未分析 |
| `asset_pipe::texture::compile_texture_assets::closure$1` | untouched | 未分析 |
| `asset_pipe::texture::compile_texture_assets::closure$2` | untouched | 未分析 |
| `asset_pipe::texture::compile_texture_assets::closure$3` | untouched | 未分析 |
| `asset_pipe::texture::compile_texture_assets::closure$4` | implemented_static | Parallel::each job closure |
| `asset_pipe::texture::compress::closure$0` | untouched | 未分析 |
| `asset_pipe::texture::compress::closure$1` | untouched | 未分析 |
| `asset_pipe::texture::compress::closure$2` | untouched | 未分析 |
| `asset_pipe::texture::process_dds_image::closure$0` | untouched | 未分析 |
| `asset_pipe::texture::process_dds_image::closure$1` | untouched | 未分析 |
| `asset_pipe::texture::process_dds_image::closure$2` | untouched | 未分析 |
| `asset_pipe::texture::process_dds_image::closure$3` | untouched | 未分析 |
| `asset_pipe::texture::process_regular_image::closure$0` | untouched | 未分析 |
| `asset_pipe::texture::process_regular_image::closure$2` | implemented_static | encode_level base |
| `asset_pipe::texture::process_regular_image::closure$3` | implemented_static | next_level |
| `asset_pipe::texture::process_regular_image::closure$4` | implemented_static | encode_level appended mips |

Serde/RKYV derive 的 field visitor、serialize/resolve 胶水不计作者私有接口，但其承载的字段、enum、Option、默认值、28B texture root 等业务交换规则没有被删除：JSON 的 schema_rules 单独记录原 source、PDB 条目和验证范围。Blob schema 有 388 个原文件 byte-identical roundtrip；其余 schema 为发行 RON 和 typed fixture 对照，完整错误/原 native 类型没有因此通过。

## 版本与未验收范围

- C 的 mip_flood/store/texture_processing/foliage/mesh/legacy_mod/texture 是独立新路径；C 接口尚未盘点，数量 unknown，不拿 B 地址推断。
- 原 PNG/JPG/DDS 输入缺失，完整原 pipeline/compression 入口差分未完成。
- PrefabNameRef 已恢复为无分配 UTF-8 借用视图；原 ABI/layout 完全一致、全部 unresolved-default/坏输入副作用、原日志/CLI、country-core 热重载消费者仍未验收。
- 原 easy-parallel 3.2.0 的每资产 each→run 已接线；14 个测试覆盖正常编译、全部任务完成后错误返回、panic先join后传播、失败不持久化 meta/list。

逐记录/source/evidence/hash 见 [interface-coverage.json](interface-coverage.json)。

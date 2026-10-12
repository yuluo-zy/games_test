// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/forest/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f3c0950ae458f19af7317e44c8f8638a7866c376d2cedf0b03cf8a0c350006b4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 305, 2492, 3699, 3707, 3709
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2492,"record_offset":1444,"rva":29299840,"va":5398008960}
// 编译实例 RVAs: 0x1bf1480, 0x22792c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t13532832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398008960,"raw_hex":"e8a07ece00","size":5,"soft_fail":false,"target_va":5411542296,"va":5398009459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2492,"record_offset":1720,"parent_record_offset":1444,"ancestor_record_offset":1444,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/02492.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2492,"record_offset":1848,"parent_record_offset":1444,"ancestor_record_offset":1444,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/02492.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2492,"record_offset":1628,"parent_record_offset":1444,"ancestor_record_offset":1444,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/02492.jsonl.gz"}
//   [pdb_inline_direct] core::str::pattern::impl$36::matching
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2492,"record_offset":2000,"parent_record_offset":1444,"ancestor_record_offset":1444,"inlinee":5012,"source_line":1736,"archive":"_pdb/B/modules/02492.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2492,"record_offset":1804,"parent_record_offset":1776,"ancestor_record_offset":1444,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/02492.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2492,"record_offset":1936,"parent_record_offset":1876,"ancestor_record_offset":1444,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/02492.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2492,"record_offset":1876,"parent_record_offset":1848,"ancestor_record_offset":1444,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/02492.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2492,"record_offset":1776,"parent_record_offset":1748,"ancestor_record_offset":1444,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/02492.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2492,"record_offset":1748,"parent_record_offset":1720,"ancestor_record_offset":1444,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/02492.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2492,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":1444,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/02492.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2492,"record_offset":1680,"parent_record_offset":1628,"ancestor_record_offset":1444,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02492.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: forest::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":305,"record_offset":252,"rva":6622288,"va":5375331408}
// 编译实例 RVAs: 0x650c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140029880
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6452411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331408,"raw_hex":"e8458b9dff","size":5,"soft_fail":false,"target_va":5368879232,"va":5375331638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea68d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8739928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331408,"raw_hex":"e8585c8500","size":5,"soft_fail":false,"target_va":5384071376,"va":5375331443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8745692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331408,"raw_hex":"e8dc728500","size":5,"soft_fail":false,"target_va":5384077296,"va":5375331599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8744868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331408,"raw_hex":"e8a46f8500","size":5,"soft_fail":false,"target_va":5384076432,"va":5375331559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::intern::Interner$LT$T$GT$::intern
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8742919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331408,"raw_hex":"e807688500","size":5,"soft_fail":false,"target_va":5384074400,"va":5375331476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::schedule::config::ScheduleConfigs$LT$T$GT$::in_set_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8744792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375331408,"raw_hex":"e8586f8500","size":5,"soft_fail":false,"target_va":5384076288,"va":5375331491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":305,"record_offset":436,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00305.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs
//     evidence={"module":305,"record_offset":368,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7908,"source_line":492,"archive":"_pdb/B/modules/00305.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":305,"record_offset":464,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":18338,"source_line":100,"archive":"_pdb/B/modules/00305.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/label.rs
//     evidence={"module":305,"record_offset":404,"parent_record_offset":368,"ancestor_record_offset":252,"inlinee":5551,"source_line":139,"archive":"_pdb/B/modules/00305.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// === 完整可用 TPI 类型引用图（包含参数表、返回类型、字段等实际记录） ===
// TPI 0x1000:
// ---
// Kind:            LF_ARGLIST
// ArgList:
//   ArgIndices:      [  ]
// ...
// TPI 0x1001:
// ---
// Kind:            LF_PROCEDURE
// Procedure:
//   ReturnType:      3
//   CallConv:        NearC
//   Options:         [ None ]
//   ParameterCount:  0
//   ArgumentList:    4096
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] forest::ForestPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<forest::ForestPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["forest::ForestPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<forest::ForestPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["forest::ForestPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<forest::ForestPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["forest::ForestPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<forest::ForestPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["forest::ForestPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<forest::ForestPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["forest::ForestPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<forest::ForestPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["forest::ForestPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<forest::ForestPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["forest::ForestPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<forest::ForestPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["forest::ForestPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::add_plugins<bevy_app::plugin::sealed::PluginMarker,forest::ForestPlugin>", "constructor": "bevy_app::app::App::add_plugins", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "forest::ForestPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: core::str::pattern::TwoWaySearcher::next_back<enum2$<core::str::pattern::MatchOnly> >
// 信息 ID: 935fd15b7fefbfb868c4a7b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["core::str::pattern::MatchOnly"]; 字段布局 = unknown。
// [类型关系] core::str::pattern::TwoWaySearcher::next_back -> ["enum2$<core::str::pattern::MatchOnly>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x1bf1437 -> 0x1bf1480 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::str::impl$0::rfind<ref$<str$> > @ 0x2279277 -> 0x22792c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: forest::impl$0::build
// 信息 ID: c4cc06d08b6d0a4be6fdb516
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

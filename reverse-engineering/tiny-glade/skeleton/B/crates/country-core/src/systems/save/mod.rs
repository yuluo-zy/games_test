// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/save/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 74e6dd54e81cd5c3f6da279479685667cd270aa91f791aff99bd316a1e27f791
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 79, 250, 487, 506, 511, 600, 1608, 1629, 1699
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69E7
// 首条位置: {"module":1608,"inlinee":27111}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1608,"record_offset":34092,"parent_record_offset":34036,"ancestor_record_offset":34036,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01608.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$54::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1608,"record_offset":34064,"parent_record_offset":34036,"ancestor_record_offset":34036,"inlinee":13338,"source_line":134,"archive":"_pdb/B/modules/01608.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x296F
// 首条位置: {"module":79,"inlinee":10607}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::ser::impls::impl$41::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":79,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2692,"inlinee":8882,"source_line":11,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":79,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":2692,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$27::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":79,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":2692,"inlinee":6591,"source_line":705,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_i32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":79,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":2692,"inlinee":8883,"source_line":90,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_i64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":79,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":2692,"inlinee":8884,"source_line":94,"archive":"_pdb/B/modules/00079.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::impl$1::add_save_load_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":61656,"rva":9716656,"va":5378425776}
// 编译实例 RVAs: 0x9443b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140029880
//     owner=unresolved_namespace; count=6
//     target_aliases=53 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9547356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378425776,"raw_hex":"e8a4516eff","size":5,"soft_fail":false,"target_va":5368879232,"va":5378426583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409beac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t501172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378425776,"raw_hex":"e8b4a50700","size":5,"soft_fail":false,"target_va":5378927296,"va":5378426119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5fa20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1160605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378425776,"raw_hex":"e89db51100","size":5,"soft_fail":false,"target_va":5379586592,"va":5378425982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b218a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1954946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378425776,"raw_hex":"e882d41d00","size":5,"soft_fail":false,"target_va":5380380832,"va":5378425881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b669c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2237207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378425776,"raw_hex":"e817232200","size":5,"soft_fail":false,"target_va":5380663744,"va":5378426532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14144ff50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11582300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378425776,"raw_hex":"e85cbbb000","size":5,"soft_fail":false,"target_va":5390008144,"va":5378425839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141453000
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11594707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378425776,"raw_hex":"e8d3ebb000","size":5,"soft_fail":false,"target_va":5390020608,"va":5378425896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14174d040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14715995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378425776,"raw_hex":"e85b8ce000","size":5,"soft_fail":false,"target_va":5393141824,"va":5378425824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141750ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14730931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378425776,"raw_hex":"e8b3c6e000","size":5,"soft_fail":false,"target_va":5393156784,"va":5378425848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":511,"record_offset":62008,"parent_record_offset":61656,"ancestor_record_offset":61656,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::schedule::config::impl$3::in_set
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule/config.rs
//     evidence={"module":511,"record_offset":61804,"parent_record_offset":61656,"ancestor_record_offset":61656,"inlinee":7908,"source_line":492,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::schedule::set::SystemSet::intern
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/label.rs
//     evidence={"module":511,"record_offset":61840,"parent_record_offset":61804,"ancestor_record_offset":61656,"inlinee":5551,"source_line":139,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::impl$1::add_save_load_events_and_resources
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":62040,"rva":9717728,"va":5378426848}
// 编译实例 RVAs: 0x9447e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=4
//     target_aliases=19 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8811572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8cc8b79ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5378427679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dee70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3563736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8289fc9ff","size":5,"soft_fail":false,"target_va":5374865008,"va":5378428739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3528952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e80827caff","size":5,"soft_fail":false,"target_va":5374900736,"va":5378429683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f5c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3470311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8190ccbff","size":5,"soft_fail":false,"target_va":5374958624,"va":5378428930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3345553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e86ff3ccff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378427036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3347569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e88febccff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378429068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140876fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-844516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e81c1df3ff","size":5,"soft_fail":false,"target_va":5377585104,"va":5378429615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140877550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-843090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8ae22f3ff","size":5,"soft_fail":false,"target_va":5377586512,"va":5378429597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140878f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-836352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8003df3ff","size":5,"soft_fail":false,"target_va":5377593104,"va":5378429451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408795d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-834606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8d243f3ff","size":5,"soft_fail":false,"target_va":5377594832,"va":5378429433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087f910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-808416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e820aaf3ff","size":5,"soft_fail":false,"target_va":5377620240,"va":5378428651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6d8b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1215951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8cf8d1200","size":5,"soft_fail":false,"target_va":5379643568,"va":5378427612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5ce40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2194727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8277d2100","size":5,"soft_fail":false,"target_va":5380623936,"va":5378429204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146ca50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11696808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8a87ab200","size":5,"soft_fail":false,"target_va":5390125648,"va":5378428835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33420094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e83ef3fd01","size":5,"soft_fail":false,"target_va":5411847168,"va":5378427069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33112536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8d841f901","size":5,"soft_fail":false,"target_va":5411540992,"va":5378428451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33112438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e87641f901","size":5,"soft_fail":false,"target_va":5411540963,"va":5378428520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12568607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e81fc8bf00","size":5,"soft_fail":false,"target_va":5390996512,"va":5378427900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::registry::EventRegistry::register_event
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12437204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8d4c6bd00","size":5,"soft_fail":false,"target_va":5390864960,"va":5378427751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33114299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e8bb48f901","size":5,"soft_fail":false,"target_va":5411542880,"va":5378428576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4263517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378426848,"raw_hex":"e85d0e4100","size":5,"soft_fail":false,"target_va":5382691056,"va":5378427534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":511,"record_offset":63248,"parent_record_offset":62040,"ancestor_record_offset":62040,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":511,"record_offset":63132,"parent_record_offset":62040,"ancestor_record_offset":62040,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$56::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":511,"record_offset":62856,"parent_record_offset":62040,"ancestor_record_offset":62040,"inlinee":7957,"source_line":3466,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] bevy_app::app::App::add_event
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":511,"record_offset":63848,"parent_record_offset":62040,"ancestor_record_offset":62040,"inlinee":9230,"source_line":358,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":66896,"parent_record_offset":62040,"ancestor_record_offset":62040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":511,"record_offset":63804,"parent_record_offset":62040,"ancestor_record_offset":62040,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::load_game::LoadGameManager::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/load_game.rs
//     evidence={"module":511,"record_offset":64560,"parent_record_offset":62040,"ancestor_record_offset":62040,"inlinee":29252,"source_line":78,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::channel
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":511,"record_offset":62228,"parent_record_offset":62040,"ancestor_record_offset":62040,"inlinee":7841,"source_line":519,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":511,"record_offset":63220,"parent_record_offset":62040,"ancestor_record_offset":62040,"inlinee":12559,"source_line":289,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":62432,"parent_record_offset":62408,"ancestor_record_offset":62040,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":62456,"parent_record_offset":62432,"ancestor_record_offset":62040,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":67092,"parent_record_offset":67068,"ancestor_record_offset":62040,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":62376,"parent_record_offset":62336,"ancestor_record_offset":62040,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":62408,"parent_record_offset":62376,"ancestor_record_offset":62040,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":511,"record_offset":67068,"parent_record_offset":67040,"ancestor_record_offset":62040,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":62336,"parent_record_offset":62300,"ancestor_record_offset":62040,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":62892,"parent_record_offset":62856,"ancestor_record_offset":62040,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":62920,"parent_record_offset":62892,"ancestor_record_offset":62040,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":62952,"parent_record_offset":62920,"ancestor_record_offset":62040,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":511,"record_offset":63072,"parent_record_offset":62856,"ancestor_record_offset":62040,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":511,"record_offset":64316,"parent_record_offset":64292,"ancestor_record_offset":62040,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":511,"record_offset":63516,"parent_record_offset":63480,"ancestor_record_offset":62040,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":511,"record_offset":67040,"parent_record_offset":67016,"ancestor_record_offset":62040,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":511,"record_offset":64364,"parent_record_offset":64340,"ancestor_record_offset":62040,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":511,"record_offset":64340,"parent_record_offset":64316,"ancestor_record_offset":62040,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":511,"record_offset":63592,"parent_record_offset":63552,"ancestor_record_offset":62040,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":511,"record_offset":63552,"parent_record_offset":63516,"ancestor_record_offset":62040,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":511,"record_offset":67016,"parent_record_offset":66992,"ancestor_record_offset":62040,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":511,"record_offset":63440,"parent_record_offset":63404,"ancestor_record_offset":62040,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":511,"record_offset":63404,"parent_record_offset":63368,"ancestor_record_offset":62040,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":511,"record_offset":63332,"parent_record_offset":63284,"ancestor_record_offset":62040,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":511,"record_offset":63368,"parent_record_offset":63332,"ancestor_record_offset":62040,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":511,"record_offset":63284,"parent_record_offset":63248,"ancestor_record_offset":62040,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":511,"record_offset":67364,"parent_record_offset":67328,"ancestor_record_offset":62040,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":511,"record_offset":67176,"parent_record_offset":67152,"ancestor_record_offset":62040,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":64292,"parent_record_offset":64268,"ancestor_record_offset":62040,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":64268,"parent_record_offset":64236,"ancestor_record_offset":62040,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":63480,"parent_record_offset":63440,"ancestor_record_offset":62040,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":511,"record_offset":64236,"parent_record_offset":64212,"ancestor_record_offset":62040,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::sub_app::SubApp::add_event
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/sub_app.rs
//     evidence={"module":511,"record_offset":63872,"parent_record_offset":63848,"ancestor_record_offset":62040,"inlinee":9217,"source_line":339,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":511,"record_offset":64508,"parent_record_offset":64484,"ancestor_record_offset":62040,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":511,"record_offset":64484,"parent_record_offset":64460,"ancestor_record_offset":62040,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":511,"record_offset":63984,"parent_record_offset":63960,"ancestor_record_offset":62040,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":64032,"parent_record_offset":64008,"ancestor_record_offset":62040,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":64008,"parent_record_offset":63984,"ancestor_record_offset":62040,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":511,"record_offset":64212,"parent_record_offset":64188,"ancestor_record_offset":62040,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":511,"record_offset":63900,"parent_record_offset":63872,"ancestor_record_offset":62040,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":511,"record_offset":63960,"parent_record_offset":63932,"ancestor_record_offset":62040,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":511,"record_offset":63748,"parent_record_offset":63724,"ancestor_record_offset":62040,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":511,"record_offset":63096,"parent_record_offset":63072,"ancestor_record_offset":62040,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":511,"record_offset":64124,"parent_record_offset":64060,"ancestor_record_offset":62040,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":511,"record_offset":63932,"parent_record_offset":63900,"ancestor_record_offset":62040,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":511,"record_offset":64148,"parent_record_offset":64124,"ancestor_record_offset":62040,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":511,"record_offset":64436,"parent_record_offset":63900,"ancestor_record_offset":62040,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":511,"record_offset":64188,"parent_record_offset":64008,"ancestor_record_offset":62040,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":511,"record_offset":63724,"parent_record_offset":63440,"ancestor_record_offset":62040,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":66920,"parent_record_offset":66896,"ancestor_record_offset":62040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":511,"record_offset":67396,"parent_record_offset":67364,"ancestor_record_offset":62040,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":511,"record_offset":62484,"parent_record_offset":62456,"ancestor_record_offset":62040,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":511,"record_offset":64060,"parent_record_offset":64032,"ancestor_record_offset":62040,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":511,"record_offset":64096,"parent_record_offset":64060,"ancestor_record_offset":62040,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":511,"record_offset":63160,"parent_record_offset":63132,"ancestor_record_offset":62040,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":511,"record_offset":67212,"parent_record_offset":67176,"ancestor_record_offset":62040,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":511,"record_offset":63184,"parent_record_offset":63160,"ancestor_record_offset":62040,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":511,"record_offset":67236,"parent_record_offset":67212,"ancestor_record_offset":62040,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":511,"record_offset":67268,"parent_record_offset":67176,"ancestor_record_offset":62040,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::channel
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":511,"record_offset":62264,"parent_record_offset":62228,"ancestor_record_offset":62040,"inlinee":7806,"source_line":197,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\counter.rs
//     evidence={"module":511,"record_offset":62300,"parent_record_offset":62264,"ancestor_record_offset":62040,"inlinee":7808,"source_line":20,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::impl$1::add_save_load_events_and_resources::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7041
// 首条位置: {"module":506,"inlinee":28737}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::SaveFileVersion
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 24; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// 候选参数类型: ["tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >", "tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs> >", "bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SavingGameComplete>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::reader::EventReader<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::event::reader::EventReader<country_core::systems::prefab::SpawnPrefabCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::resources::prefab::PrefabInstance>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Changed<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::prefab::PrefabsToDelete>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Changed<country_core::systems::prefab::PrefabInstanceData> >", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::prefab::PrefabsToDelete>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::systems::prefab::PrefabInstanceData> >,bevy_ecs::query::filter::Changed<country_core::systems::prefab::PrefabInstanceData> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::prefab::PrefabsToDelete>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::_::impl$0::deserialize
// 信息 ID: 9d9f557695c07042d20ae486
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::load_game::parse_saved_game_data; module=1608, inline_record=34036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::_::impl$0::serialize
// 信息 ID: 51c68c0e57a6b9ae3852abde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game_to_file<std::fs::File,core::slice::iter::Iter<country_core::resources::history::MultiEdit> >; module=79, inline_record=2692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::save_game_to_file<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> >,core::slice::iter::Iter<country_core::resources::history::MultiEdit> >; module=1699, inline_record=160416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::impl$1::add_save_load_events_and_resources
// 信息 ID: 6ba81699430d5bb2f0115373
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x59f8fd -> 0x9447e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::impl$1::add_save_load_events_and_resources::closure$0
// 信息 ID: 8b9f1286c60a3a6acb9eb0df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::impl$1::add_save_load_events_and_resources", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- std::thread::Builder::spawn<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<country_core::systems::save::impl$1::add_save_load_events_and_resources::closure_env$0,tuple$<> >; module=506, inline_record=7196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::impl$1::add_save_load_systems
// 信息 ID: 62a60c76501174fbfcf73f1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x59f8b6 -> 0x9443b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

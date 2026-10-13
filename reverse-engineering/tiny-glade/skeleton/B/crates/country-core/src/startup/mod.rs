// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 178429bf292633cff0610b6273f6d0628462a6c43e02f3fc393fae26a78603a4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 511, 579, 587, 1596, 1611
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::impl$0::add_game_startup_systems
// 记录形式: compiled_procedure; instances=1
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":511,"record_offset":24800,"rva":9703696,"va":5378412816}
// 编译实例 RVAs: 0x941110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a97110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1400806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378412816,"raw_hex":"e8e65f1500","size":5,"soft_fail":false,"target_va":5379813648,"va":5378412837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2089953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378412816,"raw_hex":"e8e1e31f00","size":5,"soft_fail":false,"target_va":5380502864,"va":5378412906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b66480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2249476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378412816,"raw_hex":"e804532200","size":5,"soft_fail":false,"target_va":5380662400,"va":5378412919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b66830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2250446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378412816,"raw_hex":"e8ce562200","size":5,"soft_fail":false,"target_va":5380663344,"va":5378412893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b67620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2254059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378412816,"raw_hex":"e8eb642200","size":5,"soft_fail":false,"target_va":5380666912,"va":5378412848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b67b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2255308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378412816,"raw_hex":"e8cc692200","size":5,"soft_fail":false,"target_va":5380668240,"va":5378412927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141452870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11605787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378412816,"raw_hex":"e81b17b100","size":5,"soft_fail":false,"target_va":5390018672,"va":5378412880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141740a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14678238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378412816,"raw_hex":"e8def8df00","size":5,"soft_fail":false,"target_va":5393091104,"va":5378412861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140b65bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t2247210","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378412816,"raw_hex":"e92a4a2200","size":5,"soft_fail":false,"target_va":5380660160,"va":5378412945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":511,"record_offset":24944,"parent_record_offset":24800,"ancestor_record_offset":24800,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::impl$1::add_audio_startup_systems
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/mod.rs
//     evidence={"module":511,"record_offset":25056,"parent_record_offset":24800,"ancestor_record_offset":24800,"inlinee":29220,"source_line":144,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   [pdb_inline_transitive] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":511,"record_offset":25080,"parent_record_offset":25056,"ancestor_record_offset":24800,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00511.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::impl$0::add_game_startup_systems::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82B0
// 首条位置: {"module":587,"inlinee":33456}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback::on_terrain_modified
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/terrain/delayed_terrain_changed_msg.rs
//     evidence={"module":587,"record_offset":22796,"parent_record_offset":22772,"ancestor_record_offset":22772,"inlinee":29405,"source_line":39,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":587,"record_offset":22820,"parent_record_offset":22796,"ancestor_record_offset":22772,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":587,"record_offset":22844,"parent_record_offset":22820,"ancestor_record_offset":22772,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::impl$0::add_game_startup_systems::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82AB
// 首条位置: {"module":587,"inlinee":33451}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":587,"record_offset":12388,"parent_record_offset":12036,"ancestor_record_offset":12036,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":587,"record_offset":12064,"parent_record_offset":12036,"ancestor_record_offset":12036,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":12256,"parent_record_offset":12224,"ancestor_record_offset":12036,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":12304,"parent_record_offset":12280,"ancestor_record_offset":12036,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":12280,"parent_record_offset":12256,"ancestor_record_offset":12036,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":587,"record_offset":12224,"parent_record_offset":12184,"ancestor_record_offset":12036,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":587,"record_offset":12184,"parent_record_offset":12148,"ancestor_record_offset":12036,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":587,"record_offset":12120,"parent_record_offset":12088,"ancestor_record_offset":12036,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":587,"record_offset":12088,"parent_record_offset":12064,"ancestor_record_offset":12036,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":587,"record_offset":12148,"parent_record_offset":12088,"ancestor_record_offset":12036,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":587,"record_offset":12344,"parent_record_offset":12184,"ancestor_record_offset":12036,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::impl$1::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":579,"record_offset":41900,"rva":11644048,"va":5380353168}
// 编译实例 RVAs: 0xb1ac90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140b65f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t307944","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380353168,"raw_hex":"e9e8b20400","size":5,"soft_fail":false,"target_va":5380661120,"va":5380353171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":579,"record_offset":42020,"parent_record_offset":41900,"ancestor_record_offset":41900,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shader_into_library
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82C5
// 首条位置: {"module":1611,"inlinee":33477}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1611,"record_offset":1560900,"parent_record_offset":1560864,"ancestor_record_offset":1560864,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shader_into_library<alloc::string::String>
// 记录形式: compiled_procedure; instances=1
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":587,"record_offset":124584,"rva":11797008,"va":5380506128}
// 编译实例 RVAs: 0xb40210
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2341586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380506128,"raw_hex":"e8d2ba2300","size":5,"soft_fail":false,"target_va":5382847888,"va":5380506297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5424726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380506128,"raw_hex":"e8aa39adff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380506209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5425451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380506128,"raw_hex":"e8d536adff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380506950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088dec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2827974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380506128,"raw_hex":"e83ad9d4ff","size":5,"soft_fail":false,"target_va":5377679040,"va":5380507009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409383b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2129786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380506128,"raw_hex":"e88680dfff","size":5,"soft_fail":false,"target_va":5378376624,"va":5380506405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31340913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380506128,"raw_hex":"e87139de01","size":5,"soft_fail":false,"target_va":5411847168,"va":5380506250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31034150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380506128,"raw_hex":"e8268bd901","size":5,"soft_fail":false,"target_va":5411540963,"va":5380506808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2147946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380506128,"raw_hex":"e86ac62000","size":5,"soft_fail":false,"target_va":5382655024,"va":5380507073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31034892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380506128,"raw_hex":"e80c8ed901","size":5,"soft_fail":false,"target_va":5411541776,"va":5380506879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31036010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380506128,"raw_hex":"e86a92d901","size":5,"soft_fail":false,"target_va":5411542880,"va":5380506865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2135462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380506128,"raw_hex":"e85a6adfff","size":5,"soft_fail":false,"target_va":5378371312,"va":5380506769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":587,"record_offset":125916,"parent_record_offset":124584,"ancestor_record_offset":124584,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::shader::RasterShaderProgram::new_draw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/shader.rs
//     evidence={"module":587,"record_offset":125788,"parent_record_offset":124584,"ancestor_record_offset":124584,"inlinee":29033,"source_line":95,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":587,"record_offset":124748,"parent_record_offset":124584,"ancestor_record_offset":124584,"inlinee":30579,"source_line":24,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":125392,"parent_record_offset":125368,"ancestor_record_offset":124584,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":125416,"parent_record_offset":125392,"ancestor_record_offset":124584,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":125732,"parent_record_offset":125708,"ancestor_record_offset":124584,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":125368,"parent_record_offset":125184,"ancestor_record_offset":124584,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":125708,"parent_record_offset":125652,"ancestor_record_offset":124584,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":125116,"parent_record_offset":125084,"ancestor_record_offset":124584,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":125680,"parent_record_offset":125652,"ancestor_record_offset":124584,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":125652,"parent_record_offset":125628,"ancestor_record_offset":124584,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":125184,"parent_record_offset":125148,"ancestor_record_offset":124584,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":125148,"parent_record_offset":125116,"ancestor_record_offset":124584,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":125628,"parent_record_offset":125604,"ancestor_record_offset":124584,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":125228,"parent_record_offset":125184,"ancestor_record_offset":124584,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":125044,"parent_record_offset":125012,"ancestor_record_offset":124584,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":125012,"parent_record_offset":124980,"ancestor_record_offset":124584,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":124948,"parent_record_offset":124900,"ancestor_record_offset":124584,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":124980,"parent_record_offset":124948,"ancestor_record_offset":124584,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":124900,"parent_record_offset":124868,"ancestor_record_offset":124584,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":587,"record_offset":124868,"parent_record_offset":124836,"ancestor_record_offset":124584,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":587,"record_offset":124836,"parent_record_offset":124804,"ancestor_record_offset":124584,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":587,"record_offset":125084,"parent_record_offset":125044,"ancestor_record_offset":124584,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":587,"record_offset":125252,"parent_record_offset":125228,"ancestor_record_offset":124584,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":587,"record_offset":125276,"parent_record_offset":125252,"ancestor_record_offset":124584,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":587,"record_offset":124804,"parent_record_offset":124748,"ancestor_record_offset":124584,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":587,"record_offset":125492,"parent_record_offset":125468,"ancestor_record_offset":124584,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":587,"record_offset":125324,"parent_record_offset":125300,"ancestor_record_offset":124584,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":587,"record_offset":125300,"parent_record_offset":125276,"ancestor_record_offset":124584,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":587,"record_offset":125468,"parent_record_offset":125044,"ancestor_record_offset":124584,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":587,"record_offset":125556,"parent_record_offset":124748,"ancestor_record_offset":124584,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":587,"record_offset":125840,"parent_record_offset":125788,"ancestor_record_offset":124584,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shader_into_library<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":587,"record_offset":123224,"rva":11796032,"va":5380505152}
// 编译实例 RVAs: 0xb3fe40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2342569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380505152,"raw_hex":"e8a9be2300","size":5,"soft_fail":false,"target_va":5382847888,"va":5380505314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5423743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380505152,"raw_hex":"e8813dadff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380505226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5424473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380505152,"raw_hex":"e8a73aadff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380505972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088dec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2826996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380505152,"raw_hex":"e80cddd4ff","size":5,"soft_fail":false,"target_va":5377679040,"va":5380506031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409383b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2128803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380505152,"raw_hex":"e85d84dfff","size":5,"soft_fail":false,"target_va":5378376624,"va":5380505422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31341896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380505152,"raw_hex":"e8483dde01","size":5,"soft_fail":false,"target_va":5411847168,"va":5380505267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31035128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380505152,"raw_hex":"e8f88ed901","size":5,"soft_fail":false,"target_va":5411540963,"va":5380505830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2148924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380505152,"raw_hex":"e83cca2000","size":5,"soft_fail":false,"target_va":5382655024,"va":5380506095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31035870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380505152,"raw_hex":"e8de91d901","size":5,"soft_fail":false,"target_va":5411541776,"va":5380505901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31036988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380505152,"raw_hex":"e83c96d901","size":5,"soft_fail":false,"target_va":5411542880,"va":5380505887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2134486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380505152,"raw_hex":"e82a6edfff","size":5,"soft_fail":false,"target_va":5378371312,"va":5380505793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":587,"record_offset":124544,"parent_record_offset":123224,"ancestor_record_offset":123224,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::shader::RasterShaderProgram::new_draw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/shader.rs
//     evidence={"module":587,"record_offset":124416,"parent_record_offset":123224,"ancestor_record_offset":123224,"inlinee":29033,"source_line":95,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":587,"record_offset":123376,"parent_record_offset":123224,"ancestor_record_offset":123224,"inlinee":30579,"source_line":24,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":124020,"parent_record_offset":123996,"ancestor_record_offset":123224,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":124044,"parent_record_offset":124020,"ancestor_record_offset":123224,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":124360,"parent_record_offset":124336,"ancestor_record_offset":123224,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":123996,"parent_record_offset":123812,"ancestor_record_offset":123224,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":124336,"parent_record_offset":124280,"ancestor_record_offset":123224,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":123744,"parent_record_offset":123712,"ancestor_record_offset":123224,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":124308,"parent_record_offset":124280,"ancestor_record_offset":123224,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":124280,"parent_record_offset":124256,"ancestor_record_offset":123224,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":123812,"parent_record_offset":123776,"ancestor_record_offset":123224,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":123776,"parent_record_offset":123744,"ancestor_record_offset":123224,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":124256,"parent_record_offset":124232,"ancestor_record_offset":123224,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":123856,"parent_record_offset":123812,"ancestor_record_offset":123224,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":123672,"parent_record_offset":123640,"ancestor_record_offset":123224,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":123640,"parent_record_offset":123608,"ancestor_record_offset":123224,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":123576,"parent_record_offset":123528,"ancestor_record_offset":123224,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":123608,"parent_record_offset":123576,"ancestor_record_offset":123224,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":123528,"parent_record_offset":123496,"ancestor_record_offset":123224,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":587,"record_offset":123496,"parent_record_offset":123464,"ancestor_record_offset":123224,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":587,"record_offset":123464,"parent_record_offset":123432,"ancestor_record_offset":123224,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":587,"record_offset":123712,"parent_record_offset":123672,"ancestor_record_offset":123224,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":587,"record_offset":123880,"parent_record_offset":123856,"ancestor_record_offset":123224,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":587,"record_offset":123904,"parent_record_offset":123880,"ancestor_record_offset":123224,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":587,"record_offset":123432,"parent_record_offset":123376,"ancestor_record_offset":123224,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":587,"record_offset":124120,"parent_record_offset":124096,"ancestor_record_offset":123224,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":587,"record_offset":123952,"parent_record_offset":123928,"ancestor_record_offset":123224,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":587,"record_offset":123928,"parent_record_offset":123904,"ancestor_record_offset":123224,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":587,"record_offset":124096,"parent_record_offset":123672,"ancestor_record_offset":123224,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":587,"record_offset":124184,"parent_record_offset":123376,"ancestor_record_offset":123224,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":587,"record_offset":124468,"parent_record_offset":124416,"ancestor_record_offset":123224,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_shader_into_library_replace_previous<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":587,"record_offset":125956,"rva":11797984,"va":5380507104}
// 编译实例 RVAs: 0xb405e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2340617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380507104,"raw_hex":"e809b72300","size":5,"soft_fail":false,"target_va":5382847888,"va":5380507266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5425695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380507104,"raw_hex":"e8e135adff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380507178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5426425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380507104,"raw_hex":"e80733adff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380507924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088dec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2828948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380507104,"raw_hex":"e86cd5d4ff","size":5,"soft_fail":false,"target_va":5377679040,"va":5380507983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409383b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2130755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380507104,"raw_hex":"e8bd7cdfff","size":5,"soft_fail":false,"target_va":5378376624,"va":5380507374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31339944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380507104,"raw_hex":"e8a835de01","size":5,"soft_fail":false,"target_va":5411847168,"va":5380507219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31033176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380507104,"raw_hex":"e85887d901","size":5,"soft_fail":false,"target_va":5411540963,"va":5380507782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2146972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380507104,"raw_hex":"e89cc22000","size":5,"soft_fail":false,"target_va":5382655024,"va":5380508047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31033918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380507104,"raw_hex":"e83e8ad901","size":5,"soft_fail":false,"target_va":5411541776,"va":5380507853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31035036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380507104,"raw_hex":"e89c8ed901","size":5,"soft_fail":false,"target_va":5411542880,"va":5380507839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2136438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380507104,"raw_hex":"e88a66dfff","size":5,"soft_fail":false,"target_va":5378371312,"va":5380507745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":587,"record_offset":127296,"parent_record_offset":125956,"ancestor_record_offset":125956,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::shader::RasterShaderProgram::new_draw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/shader.rs
//     evidence={"module":587,"record_offset":127168,"parent_record_offset":125956,"ancestor_record_offset":125956,"inlinee":29033,"source_line":95,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":587,"record_offset":126128,"parent_record_offset":125956,"ancestor_record_offset":125956,"inlinee":30579,"source_line":24,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":126772,"parent_record_offset":126748,"ancestor_record_offset":125956,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":126796,"parent_record_offset":126772,"ancestor_record_offset":125956,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":127112,"parent_record_offset":127088,"ancestor_record_offset":125956,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":126748,"parent_record_offset":126564,"ancestor_record_offset":125956,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":127088,"parent_record_offset":127032,"ancestor_record_offset":125956,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":126496,"parent_record_offset":126464,"ancestor_record_offset":125956,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":127060,"parent_record_offset":127032,"ancestor_record_offset":125956,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":127032,"parent_record_offset":127008,"ancestor_record_offset":125956,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":126564,"parent_record_offset":126528,"ancestor_record_offset":125956,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":126528,"parent_record_offset":126496,"ancestor_record_offset":125956,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":127008,"parent_record_offset":126984,"ancestor_record_offset":125956,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":126608,"parent_record_offset":126564,"ancestor_record_offset":125956,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":126424,"parent_record_offset":126392,"ancestor_record_offset":125956,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":126392,"parent_record_offset":126360,"ancestor_record_offset":125956,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":126328,"parent_record_offset":126280,"ancestor_record_offset":125956,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":126360,"parent_record_offset":126328,"ancestor_record_offset":125956,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":126280,"parent_record_offset":126248,"ancestor_record_offset":125956,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":587,"record_offset":126248,"parent_record_offset":126216,"ancestor_record_offset":125956,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":587,"record_offset":126216,"parent_record_offset":126184,"ancestor_record_offset":125956,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":587,"record_offset":126464,"parent_record_offset":126424,"ancestor_record_offset":125956,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":587,"record_offset":126632,"parent_record_offset":126608,"ancestor_record_offset":125956,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":587,"record_offset":126656,"parent_record_offset":126632,"ancestor_record_offset":125956,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":587,"record_offset":126184,"parent_record_offset":126128,"ancestor_record_offset":125956,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":587,"record_offset":126872,"parent_record_offset":126848,"ancestor_record_offset":125956,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":587,"record_offset":126704,"parent_record_offset":126680,"ancestor_record_offset":125956,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":587,"record_offset":126680,"parent_record_offset":126656,"ancestor_record_offset":125956,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":587,"record_offset":126848,"parent_record_offset":126424,"ancestor_record_offset":125956,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":587,"record_offset":126936,"parent_record_offset":126128,"ancestor_record_offset":125956,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":587,"record_offset":127220,"parent_record_offset":127168,"ancestor_record_offset":125956,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mesh_into_library
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8147
// 首条位置: {"module":579,"inlinee":33095}
// 编译实例 RVAs: 0xb1aca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5271892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380353184,"raw_hex":"e8ac8eafff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380353375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2695926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380353184,"raw_hex":"e80addd6ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5380353569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898d40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2629910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380353184,"raw_hex":"e8eaded7ff","size":5,"soft_fail":false,"target_va":5377723712,"va":5380353617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31493752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380353184,"raw_hex":"e8788ee001","size":5,"soft_fail":false,"target_va":5411847168,"va":5380353411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31187450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380353184,"raw_hex":"e8fae1db01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380353508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2023781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380353184,"raw_hex":"e89b1ee1ff","size":5,"soft_fail":false,"target_va":5378329696,"va":5380353472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::meshwatch::MeshWatch::watch
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2491903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380353184,"raw_hex":"e801fad9ff","size":5,"soft_fail":false,"target_va":5377861344,"va":5380353242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":579,"record_offset":36676,"parent_record_offset":36608,"ancestor_record_offset":36608,"inlinee":19793,"source_line":35,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":579,"record_offset":36648,"parent_record_offset":36608,"ancestor_record_offset":36608,"inlinee":19792,"source_line":31,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":37064,"parent_record_offset":37040,"ancestor_record_offset":36608,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":37088,"parent_record_offset":37064,"ancestor_record_offset":36608,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":37040,"parent_record_offset":37012,"ancestor_record_offset":36608,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":579,"record_offset":36952,"parent_record_offset":36924,"ancestor_record_offset":36608,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":579,"record_offset":37012,"parent_record_offset":36980,"ancestor_record_offset":36608,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":579,"record_offset":36980,"parent_record_offset":36952,"ancestor_record_offset":36608,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":579,"record_offset":42584,"parent_record_offset":42548,"ancestor_record_offset":42052,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":36888,"parent_record_offset":36860,"ancestor_record_offset":36608,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":36860,"parent_record_offset":36832,"ancestor_record_offset":36608,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":36804,"parent_record_offset":36776,"ancestor_record_offset":36608,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":36832,"parent_record_offset":36804,"ancestor_record_offset":36608,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":36776,"parent_record_offset":36748,"ancestor_record_offset":36608,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":579,"record_offset":36748,"parent_record_offset":36720,"ancestor_record_offset":36608,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":579,"record_offset":36720,"parent_record_offset":36676,"ancestor_record_offset":36608,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":36924,"parent_record_offset":36888,"ancestor_record_offset":36608,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":579,"record_offset":42608,"parent_record_offset":42584,"ancestor_record_offset":42052,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":579,"record_offset":42632,"parent_record_offset":42608,"ancestor_record_offset":42052,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":579,"record_offset":37164,"parent_record_offset":37140,"ancestor_record_offset":36608,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":579,"record_offset":42680,"parent_record_offset":42656,"ancestor_record_offset":42052,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":42656,"parent_record_offset":42632,"ancestor_record_offset":42052,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":579,"record_offset":37140,"parent_record_offset":36888,"ancestor_record_offset":36608,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00579.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::MinimalStartupPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$9::add_to_app<bevy_app::plugin::sealed::PluginMarker,country_core::startup::MinimalStartupPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::logo_animation::AnimatedLogo,bevy_app::plugin::sealed::PluginMarker,country_core::resources::loading_screen::LoadingScreen,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::SplashScreenPlugin>", "constructor": "bevy_app::plugin::sealed::impl$9::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "country_core::startup::MinimalStartupPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::logo_animation::AnimatedLogo", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::loading_screen::LoadingScreen", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::SplashScreenPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::startup::MinimalStartupPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::startup::MinimalStartupPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::startup::MinimalStartupPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::startup::MinimalStartupPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::startup::MinimalStartupPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<country_core::startup::MinimalStartupPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<country_core::startup::MinimalStartupPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<country_core::startup::MinimalStartupPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["country_core::startup::MinimalStartupPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 230; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,ref$<country_core::geometry::instanced_wall::InstancedWall> >,bevy_ecs::query::filter::Changed<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,bevy_ecs::removal_detection::RemovedComponents<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_shape::Roof> > >,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,bevy_ecs::removal_detection::RemovedComponents<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Added<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::components::AutoClutterConvertableToDecoClutter> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >,bevy_ecs::removal_detection::RemovedComponents<country_core::components::AutoClutterConvertableToDecoClutter>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,ref$<country_core::geometry::instanced_wall::InstancedWall> >,bevy_ecs::query::filter::Changed<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >", "bevy_ecs::removal_detection::RemovedComponents<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_shape::Roof> > >,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >", "bevy_ecs::removal_detection::RemovedComponents<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>", "bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Added<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::components::AutoClutterConvertableToDecoClutter> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >", "bevy_ecs::removal_detection::RemovedComponents<country_core::components::AutoClutterConvertableToDecoClutter>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,ref$<country_core::geometry::instanced_wall::InstancedWall> >,bevy_ecs::query::filter::Changed<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_shape::Roof> > >,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Added<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::components::AutoClutterConvertableToDecoClutter> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>)
// 候选参数类型: ["bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::impl$0::add_game_startup_systems
// 信息 ID: e33051d94926c50bd8953193
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x5949c5 -> 0x941110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::impl$0::add_game_startup_systems::closure$0
// 信息 ID: 8b1eb6bd58e775ee6c9a07cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0c2c6fe0a741bc4ceed59613: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// [泛型宿主/闭包/直接引用方参数候选] 43e1929450c1720459b7678c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// [泛型宿主/闭包/直接引用方参数候选] 5b63364bde1be456ca8b15f8: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 5df651bd0e970d5c74e15a18: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 7e34f79a397d35d81c4a3ab4: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>)
// [泛型宿主/闭包/直接引用方参数候选] 88f196a8c00793fb11016af9: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// [泛型宿主/闭包/直接引用方参数候选] 89e0a4ce22b9cfb45244adbf: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 92ffd56d28ffe8b4c60423de: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 其余候选接口见完整附档，共 14 条。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::impl$0::add_game_startup_systems", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::schedule::config::impl$16::into_configs<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>)> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>)> >,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> >,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)> >,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>),tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)> >,country_core::startup::impl$0::add_game_startup_systems::closure_env$0,alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$0 <- bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 27 条。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0> >; module=587, inline_record=22772; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>),country_core::startup::impl$0::add_game_startup_systems::closure_env$0> > >; module=1611, inline_record=2268816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::impl$0::add_game_startup_systems::closure$1
// 信息 ID: 3e9a5dcc2aa579df81c9b157
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cc85d15c013966493a85b83e: void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] f1571263866702a2d9c5663c: void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// [泛型宿主/闭包/直接引用方参数候选] f7712984cef45b03dcc9f3e2: void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::impl$0::add_game_startup_systems", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::impl$0::add_game_startup_systems::closure_env$1 <- bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 30 条。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> >; module=587, inline_record=12036; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>),country_core::startup::impl$0::add_game_startup_systems::closure_env$1> > >; module=1611, inline_record=2025872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::impl$1::build
// 信息 ID: 5c2e3a7c2da4c87df425c382
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mesh_into_library
// 信息 ID: ec31bac23349478ddb7c22bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 87 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x7b6ad8 -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::clouds::startup_clouds @ 0x85b9cc -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_meadow::init_meadow @ 0x863e71 -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sky_and_sun::startup_sky_and_sun @ 0x8644ff -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::spawn_human_ref::spawn_human_ref @ 0x89d601 -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8efe98 -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0025 -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f00d8 -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f018b -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f023e -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f02f1 -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f03a4 -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0457 -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f050a -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f05bd -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0692 -> 0xb1aca0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::logo_animation::startup_animated_logo; module=579, inline_record=36608; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::logo_animation::startup_animated_logo; module=579, inline_record=37680; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shader_into_library
// 信息 ID: e35e339aaff2ae1170485d83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f6e723bf3a293de264db1c5c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> > >; module=1611, inline_record=1560864; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> > >; module=1611, inline_record=2307924; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> > >; module=1611, inline_record=3326844; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> > >; module=1611, inline_record=3521244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shader_into_library<alloc::string::String>
// 信息 ID: 0154f0a1f75b5dfcc448cace
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::load_shader_into_library -> ["alloc::string::String"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xa79885 -> 0xb40210 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shader_into_library<ref$<str$> >
// 信息 ID: e9403360dc1a4b28055a4e6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0a0bc4d76faec5be35e32390: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)
// [泛型宿主/闭包/直接引用方参数候选] f6e723bf3a293de264db1c5c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] country_core::startup::load_shader_into_library -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 97 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::draw_shaders::clouds::startup_clouds @ 0x85ba0c -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_meadow::init_meadow @ 0x863a79 -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_sky_and_sun::startup_sky_and_sun @ 0x86453f -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::startup_painterly @ 0x8a801e -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::startup_painterly @ 0x8a8051 -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::startup_vermeer @ 0x8aa75e -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::startup_vermeer @ 0x8aa791 -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)> > @ 0x8c3685 -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)> > @ 0x8c36c8 -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)> > @ 0x8c370b -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources>)> > @ 0x8c374d -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8efdd4 -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0a6e -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0b52 -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x8f1b83 -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x8f1c7c -> 0xb3fe40 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_shader_into_library_replace_previous<ref$<str$> >
// 信息 ID: 82f600f013844cb90a351957
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] country_core::startup::load_shader_into_library_replace_previous -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_water::startup_water @ 0x8f1e7f -> 0xb405e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

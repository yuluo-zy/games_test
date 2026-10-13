// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/critters/src/firefly.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 940fedc30ae6ffe5c647d04c337247cd3b0e1020dc737edb13f135931aec60f5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: firefly.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2380, 2410, 3606
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use perchance; // namespace candidate
//   package identities: [{"name":"perchance","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_critters; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::firefly::fireflies_send_to_gpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3606,"record_offset":252,"rva":28502464,"va":5397211584}
// 编译实例 RVAs: 0x1b2e9c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: firefly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24462184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397211584,"raw_hex":"e898bc8afe","size":5,"soft_fail":false,"target_va":5372749712,"va":5397211891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a5150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: firefly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5806343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397211584,"raw_hex":"e8f966a7ff","size":5,"soft_fail":false,"target_va":5391405392,"va":5397211730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2daf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: firefly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397211584,"raw_hex":"e877f0ffff","size":5,"soft_fail":false,"target_va":5397207792,"va":5397211764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2ddc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: firefly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397211584,"raw_hex":"e8e4f2ffff","size":5,"soft_fail":false,"target_va":5397208512,"va":5397211863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: firefly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6381513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397211584,"raw_hex":"e8c95f6100","size":5,"soft_fail":false,"target_va":5403593344,"va":5397211826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6233942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397211584,"raw_hex":"e8561f5f00","size":5,"soft_fail":false,"target_va":5403445888,"va":5397211941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5767180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397211584,"raw_hex":"e80c005800","size":5,"soft_fail":false,"target_va":5402979152,"va":5397211967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5397211584,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5397211842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3606,"record_offset":396,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3606,"record_offset":600,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3606,"record_offset":924,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3606,"record_offset":684,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3606,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":252,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3606,"record_offset":1076,"parent_record_offset":976,"ancestor_record_offset":252,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3606,"record_offset":1008,"parent_record_offset":976,"ancestor_record_offset":252,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3606,"record_offset":976,"parent_record_offset":948,"ancestor_record_offset":252,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3606,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":252,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3606,"record_offset":444,"parent_record_offset":420,"ancestor_record_offset":252,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3606,"record_offset":472,"parent_record_offset":444,"ancestor_record_offset":252,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3606,"record_offset":420,"parent_record_offset":396,"ancestor_record_offset":252,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3606,"record_offset":752,"parent_record_offset":712,"ancestor_record_offset":252,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3606,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":252,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3606,"record_offset":1040,"parent_record_offset":1008,"ancestor_record_offset":252,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3606,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":252,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3606,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":252,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3606,"record_offset":836,"parent_record_offset":808,"ancestor_record_offset":252,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3606,"record_offset":864,"parent_record_offset":836,"ancestor_record_offset":252,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3606,"record_offset":500,"parent_record_offset":472,"ancestor_record_offset":252,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3606,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":252,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3606,"record_offset":780,"parent_record_offset":752,"ancestor_record_offset":252,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3606,"record_offset":712,"parent_record_offset":684,"ancestor_record_offset":252,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::firefly::fireflies_send_to_gpu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143FA
// 首条位置: {"module":2380,"inlinee":82938}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::firefly::update_firefly_goals
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3606,"record_offset":1368,"rva":28502928,"va":5397212048}
// 编译实例 RVAs: 0x1b2eb90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbdcb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: firefly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15143466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397212048,"raw_hex":"e8d6ed18ff","size":5,"soft_fail":false,"target_va":5382069424,"va":5397212885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::uniform_f32
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12824832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397212048,"raw_hex":"e8004f3cff","size":5,"soft_fail":false,"target_va":5384387936,"va":5397212763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::uniform_sample_sphere
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20272177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397212048,"raw_hex":"e8cfabcafe","size":5,"soft_fail":false,"target_va":5376940672,"va":5397212844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3606,"record_offset":1540,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3606,"record_offset":1796,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3606,"record_offset":1852,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3606,"record_offset":1596,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3606,"record_offset":2744,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3606,"record_offset":3704,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":3320,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":3124,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":3064,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":22360,"source_line":1027,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":3224,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":3606,"record_offset":2952,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":22397,"source_line":270,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3606,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":1368,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3606,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":1368,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3606,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1368,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3606,"record_offset":2592,"parent_record_offset":2476,"ancestor_record_offset":1368,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3606,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":1368,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3606,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":1368,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3606,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":1368,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3606,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":1368,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3606,"record_offset":1624,"parent_record_offset":1596,"ancestor_record_offset":1368,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3606,"record_offset":2476,"parent_record_offset":1876,"ancestor_record_offset":1368,"inlinee":14639,"source_line":201,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3606,"record_offset":2164,"parent_record_offset":1876,"ancestor_record_offset":1368,"inlinee":14640,"source_line":192,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3606,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":1368,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3606,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":1368,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3606,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":1368,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3606,"record_offset":2412,"parent_record_offset":2188,"ancestor_record_offset":1368,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3606,"record_offset":2136,"parent_record_offset":1876,"ancestor_record_offset":1368,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3606,"record_offset":2216,"parent_record_offset":2188,"ancestor_record_offset":1368,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3606,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":1368,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3606,"record_offset":2052,"parent_record_offset":1876,"ancestor_record_offset":1368,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3606,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":1368,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3606,"record_offset":3092,"parent_record_offset":3064,"ancestor_record_offset":1368,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3606,"record_offset":3160,"parent_record_offset":3124,"ancestor_record_offset":1368,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3606,"record_offset":2820,"parent_record_offset":2792,"ancestor_record_offset":1368,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3606,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":1368,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3606,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":1368,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3606,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":1368,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3606,"record_offset":2016,"parent_record_offset":1992,"ancestor_record_offset":1368,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3606,"record_offset":1964,"parent_record_offset":1928,"ancestor_record_offset":1368,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3606,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":1368,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3606,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":1368,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3606,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":1368,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3606,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":1368,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3606,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":1368,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3606,"record_offset":1720,"parent_record_offset":1696,"ancestor_record_offset":1368,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3606,"record_offset":1992,"parent_record_offset":1928,"ancestor_record_offset":1368,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3606,"record_offset":1928,"parent_record_offset":1876,"ancestor_record_offset":1368,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3606,"record_offset":3728,"parent_record_offset":3704,"ancestor_record_offset":1368,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":2908,"parent_record_offset":2880,"ancestor_record_offset":1368,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":3408,"parent_record_offset":3380,"ancestor_record_offset":1368,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":3380,"parent_record_offset":3348,"ancestor_record_offset":1368,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":2880,"parent_record_offset":2768,"ancestor_record_offset":1368,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":3348,"parent_record_offset":3320,"ancestor_record_offset":1368,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":1368,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":3606,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":1368,"inlinee":22398,"source_line":275,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3606,"record_offset":3480,"parent_record_offset":3444,"ancestor_record_offset":1368,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::firefly::update_firefly_goals::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/firefly.rs
//     evidence={"module":3606,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":1368,"inlinee":108489,"source_line":72,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::firefly::update_firefly_goals::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A7C9
// 首条位置: {"module":3606,"inlinee":108489}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":2880,"parent_record_offset":2768,"ancestor_record_offset":2768,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":2768,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3606,"record_offset":2820,"parent_record_offset":2792,"ancestor_record_offset":2768,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":2908,"parent_record_offset":2880,"ancestor_record_offset":2768,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::firefly::set_firefly_velocities_towards_goals
// 记录形式: compiled_procedure; instances=1
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3606,"record_offset":3948,"rva":28504224,"va":5397213344}
// 编译实例 RVAs: 0x1b2f0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3606,"record_offset":5480,"parent_record_offset":3948,"ancestor_record_offset":3948,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3606,"record_offset":4316,"parent_record_offset":3948,"ancestor_record_offset":3948,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3606,"record_offset":4120,"parent_record_offset":3948,"ancestor_record_offset":3948,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3606,"record_offset":5592,"parent_record_offset":3948,"ancestor_record_offset":3948,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3606,"record_offset":5148,"parent_record_offset":3948,"ancestor_record_offset":3948,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":5272,"parent_record_offset":3948,"ancestor_record_offset":3948,"inlinee":22755,"source_line":558,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":5180,"parent_record_offset":3948,"ancestor_record_offset":3948,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3606,"record_offset":5504,"parent_record_offset":5480,"ancestor_record_offset":3948,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3606,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":3948,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3606,"record_offset":5052,"parent_record_offset":4936,"ancestor_record_offset":3948,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3606,"record_offset":4648,"parent_record_offset":4624,"ancestor_record_offset":3948,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3606,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":3948,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3606,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":3948,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3606,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":3948,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3606,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":3948,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3606,"record_offset":4936,"parent_record_offset":4340,"ancestor_record_offset":3948,"inlinee":82914,"source_line":201,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3606,"record_offset":4624,"parent_record_offset":4340,"ancestor_record_offset":3948,"inlinee":82913,"source_line":192,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3606,"record_offset":4896,"parent_record_offset":4872,"ancestor_record_offset":3948,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3606,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":3948,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3606,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":3948,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3606,"record_offset":4872,"parent_record_offset":4648,"ancestor_record_offset":3948,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3606,"record_offset":4596,"parent_record_offset":4340,"ancestor_record_offset":3948,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3606,"record_offset":4676,"parent_record_offset":4648,"ancestor_record_offset":3948,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3606,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":3948,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3606,"record_offset":4512,"parent_record_offset":4340,"ancestor_record_offset":3948,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3606,"record_offset":5076,"parent_record_offset":5052,"ancestor_record_offset":3948,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3606,"record_offset":5212,"parent_record_offset":5180,"ancestor_record_offset":3948,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3606,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":3948,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3606,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":3948,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3606,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":3948,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3606,"record_offset":4476,"parent_record_offset":4452,"ancestor_record_offset":3948,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3606,"record_offset":4424,"parent_record_offset":4388,"ancestor_record_offset":3948,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3606,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":3948,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3606,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":3948,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3606,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":3948,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3606,"record_offset":4560,"parent_record_offset":4536,"ancestor_record_offset":3948,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3606,"record_offset":4796,"parent_record_offset":4772,"ancestor_record_offset":3948,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3606,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":3948,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3606,"record_offset":4452,"parent_record_offset":4388,"ancestor_record_offset":3948,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3606,"record_offset":4388,"parent_record_offset":4340,"ancestor_record_offset":3948,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":5396,"parent_record_offset":5360,"ancestor_record_offset":3948,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":5360,"parent_record_offset":5332,"ancestor_record_offset":3948,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":5332,"parent_record_offset":5300,"ancestor_record_offset":3948,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3606,"record_offset":5300,"parent_record_offset":5272,"ancestor_record_offset":3948,"inlinee":22756,"source_line":541,"archive":"_pdb/A/modules/03606.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3606,"record_offset":5432,"parent_record_offset":5396,"ancestor_record_offset":3948,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03606.jsonl.gz"}
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
// 完整结构及原始类型记录: firefly.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: firefly.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: firefly.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>", "bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: system_critters::firefly::fireflies_send_to_gpu
// 信息 ID: 246a0bd615008273070cf3dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 90f19925995fbae465e26641: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0x1584686 -> 0x1b2e9c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> @ 0x159059b -> 0x1b2e9c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::firefly::fireflies_send_to_gpu::closure$0
// 信息 ID: c8f15abad1cc582091783350
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::firefly::fireflies_send_to_gpu", "captures": "unknown"}
// [闭包环境类型引用] system_critters::firefly::fireflies_send_to_gpu::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::resources::critters::FireflySsbo,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly> >,tuple$<> >,system_critters::firefly::fireflies_send_to_gpu::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::resources::critters::FireflySsbo,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly> >,tuple$<> >,system_critters::firefly::fireflies_send_to_gpu::closure_env$0> >; module=2410, inline_record=10988; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::resources::critters::FireflySsbo,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly> >,tuple$<> >,system_critters::firefly::fireflies_send_to_gpu::closure_env$0> >; module=2410, inline_record=12752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::firefly::set_firefly_velocities_towards_goals
// 信息 ID: dcaa65005cc4056c47441321
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b7b7e7b8a55aa669589f3e42: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)> > @ 0x15825eb -> 0x1b2f0a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)> @ 0x1590120 -> 0x1b2f0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::firefly::update_firefly_goals
// 信息 ID: ee41ced2e9c9caf79f99d76e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 924731fd5c9ecc02648d2897: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x1584e21 -> 0x1b2eb90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Firefly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x159034c -> 0x1b2eb90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::firefly::update_firefly_goals::closure$0
// 信息 ID: be40133ea52b929895df6967
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::firefly::update_firefly_goals", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::firefly::update_firefly_goals; module=3606, inline_record=2768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

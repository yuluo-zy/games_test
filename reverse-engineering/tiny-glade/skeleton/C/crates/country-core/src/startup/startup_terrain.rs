// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/startup_terrain.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 776a514649c016670f4b61617ab64c534978c966d7050ed424ad836ae87c57c2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_terrain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 537, 544
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::startup_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":537,"record_offset":26676,"rva":10825920,"va":5379535040}
// 编译实例 RVAs: 0xa530c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14096eb80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-936084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e86cb7f1ff","size":5,"soft_fail":false,"target_va":5378599808,"va":5379535887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2042309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e8c5291f00","size":5,"soft_fail":false,"target_va":5381577744,"va":5379535430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141630b90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12441478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e886d7bd00","size":5,"soft_fail":false,"target_va":5391977360,"va":5379535877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aed3d0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17408585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e849a20901","size":5,"soft_fail":false,"target_va":5396943824,"va":5379535234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2d780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17671359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e8bfa40d01","size":5,"soft_fail":false,"target_va":5397206912,"va":5379535548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2e400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17673885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e89dae0d01","size":5,"soft_fail":false,"target_va":5397210112,"va":5379536222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36815901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e81dc43102","size":5,"soft_fail":false,"target_va":5416353040,"va":5379537134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5086502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e8269d4d00","size":5,"soft_fail":false,"target_va":5384624080,"va":5379537573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37084281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e879dc3502","size":5,"soft_fail":false,"target_va":5416621600,"va":5379537314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37084534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e876dd3502","size":5,"soft_fail":false,"target_va":5416621632,"va":5379537093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_world_raster_pipelines
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e8a8caffff","size":5,"soft_fail":false,"target_va":5379521760,"va":5379535411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-256579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e8bd15fcff","size":5,"soft_fail":false,"target_va":5379278928,"va":5379535502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::rocky_terrain::RockyTerrainMesh::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17435729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e8510c0a01","size":5,"soft_fail":false,"target_va":5396970864,"va":5379535130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t114291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e873be0100","size":5,"soft_fail":false,"target_va":5379649664,"va":5379535368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3789241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379535040,"raw_hex":"e8b9d13900","size":5,"soft_fail":false,"target_va":5383324736,"va":5379535490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379535040,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5379535452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":537,"record_offset":26968,"parent_record_offset":26676,"ancestor_record_offset":26676,"inlinee":11204,"source_line":840,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":537,"record_offset":27396,"parent_record_offset":26676,"ancestor_record_offset":26676,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":537,"record_offset":26896,"parent_record_offset":26676,"ancestor_record_offset":26676,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::shader_library::render_pipelines_impl::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/shader_library.rs
//     evidence={"module":537,"record_offset":27460,"parent_record_offset":26676,"ancestor_record_offset":26676,"inlinee":32867,"source_line":61,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":537,"record_offset":27336,"parent_record_offset":26676,"ancestor_record_offset":26676,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":537,"record_offset":27432,"parent_record_offset":26676,"ancestor_record_offset":26676,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":537,"record_offset":26840,"parent_record_offset":26676,"ancestor_record_offset":26676,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":27084,"parent_record_offset":27056,"ancestor_record_offset":26676,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":27168,"parent_record_offset":27144,"ancestor_record_offset":26676,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":27144,"parent_record_offset":27112,"ancestor_record_offset":26676,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":27112,"parent_record_offset":27084,"ancestor_record_offset":26676,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":537,"record_offset":26864,"parent_record_offset":26840,"ancestor_record_offset":26676,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":27056,"parent_record_offset":27024,"ancestor_record_offset":26676,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":537,"record_offset":26996,"parent_record_offset":26968,"ancestor_record_offset":26676,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":537,"record_offset":27024,"parent_record_offset":26996,"ancestor_record_offset":26676,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":537,"record_offset":27688,"parent_record_offset":27660,"ancestor_record_offset":26676,"inlinee":5243,"source_line":2136,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":537,"record_offset":27852,"parent_record_offset":27488,"ancestor_record_offset":26676,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":26936,"parent_record_offset":26896,"ancestor_record_offset":26676,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::with_addr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":537,"record_offset":27660,"parent_record_offset":27636,"ancestor_record_offset":26676,"inlinee":5241,"source_line":195,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_offset
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":537,"record_offset":27716,"parent_record_offset":27660,"ancestor_record_offset":26676,"inlinee":5244,"source_line":487,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":537,"record_offset":27740,"parent_record_offset":27716,"ancestor_record_offset":26676,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::map_addr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":537,"record_offset":27612,"parent_record_offset":27588,"ancestor_record_offset":26676,"inlinee":5239,"source_line":375,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::with_addr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":537,"record_offset":27636,"parent_record_offset":27612,"ancestor_record_offset":26676,"inlinee":5240,"source_line":361,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":537,"record_offset":27788,"parent_record_offset":27560,"ancestor_record_offset":26676,"inlinee":12347,"source_line":3134,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":537,"record_offset":27812,"parent_record_offset":27788,"ancestor_record_offset":26676,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":537,"record_offset":27360,"parent_record_offset":27336,"ancestor_record_offset":26676,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$7::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":537,"record_offset":27488,"parent_record_offset":27460,"ancestor_record_offset":26676,"inlinee":32855,"source_line":17,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding_set::impl$26::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":537,"record_offset":28924,"parent_record_offset":28896,"ancestor_record_offset":26676,"inlinee":16899,"source_line":78,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::TaggedSlot::chunk_and_index
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":537,"record_offset":27588,"parent_record_offset":27560,"ancestor_record_offset":26676,"inlinee":5238,"source_line":471,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::respool::impl$24::clone
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":537,"record_offset":27560,"parent_record_offset":27536,"ancestor_record_offset":26676,"inlinee":12345,"source_line":766,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$44::clone
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/respool.rs
//     evidence={"module":537,"record_offset":27536,"parent_record_offset":27488,"ancestor_record_offset":26676,"inlinee":32857,"source_line":78,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::startup_glade_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":537,"record_offset":29712,"rva":10828736,"va":5379537856}
// 编译实例 RVAs: 0xa53bc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9877552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e8d04769ff","size":5,"soft_fail":false,"target_va":5369660368,"va":5379537915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096eb80
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-942364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e8e49ef1ff","size":5,"soft_fail":false,"target_va":5378599808,"va":5379542167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-803201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e87fbef3ff","size":5,"soft_fail":false,"target_va":5378736416,"va":5379539612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a52c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e8daefffff","size":5,"soft_fail":false,"target_va":5379533952,"va":5379538081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141631590
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12441050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e8dad5bd00","size":5,"soft_fail":false,"target_va":5391979920,"va":5379538865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416318f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12439246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e8cecebd00","size":5,"soft_fail":false,"target_va":5391980784,"va":5379541533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aed3d0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17404520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e868920901","size":5,"soft_fail":false,"target_va":5396943824,"va":5379539299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2c3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17662838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e876830d01","size":5,"soft_fail":false,"target_va":5397201856,"va":5379539013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2c6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17662401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e8c1810d01","size":5,"soft_fail":false,"target_va":5397202592,"va":5379540186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2c820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17662328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e878810d01","size":5,"soft_fail":false,"target_va":5397202976,"va":5379540643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2d340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17667143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e847940d01","size":5,"soft_fail":false,"target_va":5397205824,"va":5379538676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2e830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17669818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e8ba9e0d01","size":5,"soft_fail":false,"target_va":5397211184,"va":5379541361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36811418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e89ab23102","size":5,"soft_fail":false,"target_va":5416353040,"va":5379541617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5082099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e8f38b4d00","size":5,"soft_fail":false,"target_va":5384624080,"va":5379541976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37080618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e82ace3502","size":5,"soft_fail":false,"target_va":5416622208,"va":5379541585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37079948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e88ccb3502","size":5,"soft_fail":false,"target_va":5416621632,"va":5379541679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_world_raster_pipelines
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e85abfffff","size":5,"soft_fail":false,"target_va":5379521760,"va":5379538305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-259442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e88e0afcff","size":5,"soft_fail":false,"target_va":5379278928,"va":5379538365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::background_terrain_maps::generate_background_terrain_maps
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11176188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e8fc88aa00","size":5,"soft_fail":false,"target_va":5390715856,"va":5379539663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::background_terrain_maps::water_plane_footprint_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11175779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e86387aa00","size":5,"soft_fail":false,"target_va":5390715376,"va":5379539592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t111408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e830b30100","size":5,"soft_fail":false,"target_va":5379649664,"va":5379538251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37077430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e8b6c13502","size":5,"soft_fail":false,"target_va":5416619360,"va":5379541925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3786385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379537856,"raw_hex":"e891c63900","size":5,"soft_fail":false,"target_va":5383324736,"va":5379538346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":537,"record_offset":30976,"parent_record_offset":29712,"ancestor_record_offset":29712,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":537,"record_offset":30560,"parent_record_offset":29712,"ancestor_record_offset":29712,"inlinee":11204,"source_line":840,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":537,"record_offset":30828,"parent_record_offset":29712,"ancestor_record_offset":29712,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":537,"record_offset":30336,"parent_record_offset":29712,"ancestor_record_offset":29712,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":537,"record_offset":30308,"parent_record_offset":29712,"ancestor_record_offset":29712,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_terrain::generate_grid_cw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_terrain.rs
//     evidence={"module":537,"record_offset":29880,"parent_record_offset":29712,"ancestor_record_offset":29712,"inlinee":32872,"source_line":359,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":30680,"parent_record_offset":30652,"ancestor_record_offset":29712,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":30764,"parent_record_offset":30740,"ancestor_record_offset":29712,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":30740,"parent_record_offset":30708,"ancestor_record_offset":29712,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":30708,"parent_record_offset":30680,"ancestor_record_offset":29712,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":537,"record_offset":31536,"parent_record_offset":31512,"ancestor_record_offset":29712,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":30652,"parent_record_offset":30616,"ancestor_record_offset":29712,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":537,"record_offset":31000,"parent_record_offset":30976,"ancestor_record_offset":29712,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":537,"record_offset":30588,"parent_record_offset":30560,"ancestor_record_offset":29712,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":537,"record_offset":30616,"parent_record_offset":30588,"ancestor_record_offset":29712,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":537,"record_offset":30164,"parent_record_offset":30140,"ancestor_record_offset":29712,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":537,"record_offset":30216,"parent_record_offset":30192,"ancestor_record_offset":29712,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":537,"record_offset":30192,"parent_record_offset":30140,"ancestor_record_offset":29712,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":537,"record_offset":30240,"parent_record_offset":30216,"ancestor_record_offset":29712,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":537,"record_offset":30112,"parent_record_offset":30084,"ancestor_record_offset":29712,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":30872,"parent_record_offset":30828,"ancestor_record_offset":29712,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":537,"record_offset":29932,"parent_record_offset":29880,"ancestor_record_offset":29712,"inlinee":30455,"source_line":40,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":537,"record_offset":29968,"parent_record_offset":29932,"ancestor_record_offset":29712,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":30024,"parent_record_offset":29996,"ancestor_record_offset":29712,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":30140,"parent_record_offset":30084,"ancestor_record_offset":29712,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":29996,"parent_record_offset":29968,"ancestor_record_offset":29712,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":537,"record_offset":30084,"parent_record_offset":30052,"ancestor_record_offset":29712,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":537,"record_offset":30052,"parent_record_offset":30024,"ancestor_record_offset":29712,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::generate_grid_cw
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 359
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8068
// 首条位置: {"module":537,"inlinee":32872}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":537,"record_offset":29932,"parent_record_offset":29880,"ancestor_record_offset":29880,"inlinee":30455,"source_line":40,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":537,"record_offset":30164,"parent_record_offset":30140,"ancestor_record_offset":29880,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":537,"record_offset":30216,"parent_record_offset":30192,"ancestor_record_offset":29880,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":537,"record_offset":30192,"parent_record_offset":30140,"ancestor_record_offset":29880,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":537,"record_offset":30240,"parent_record_offset":30216,"ancestor_record_offset":29880,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":537,"record_offset":30112,"parent_record_offset":30084,"ancestor_record_offset":29880,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":537,"record_offset":29968,"parent_record_offset":29932,"ancestor_record_offset":29880,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":30024,"parent_record_offset":29996,"ancestor_record_offset":29880,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":30140,"parent_record_offset":30084,"ancestor_record_offset":29880,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":29996,"parent_record_offset":29968,"ancestor_record_offset":29880,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":537,"record_offset":30084,"parent_record_offset":30052,"ancestor_record_offset":29880,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":537,"record_offset":30052,"parent_record_offset":30024,"ancestor_record_offset":29880,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::grid_gen_cw
// 记录形式: compiled_procedure; instances=1
// 源码行: 376
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":537,"record_offset":23324,"rva":10824832,"va":5379533952}
// 编译实例 RVAs: 0xa52c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a52c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-94","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379533952,"raw_hex":"e8a2ffffff","size":5,"soft_fail":false,"target_va":5379533952,"va":5379534041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t36990553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379533952,"raw_hex":"e8596e3402","size":5,"soft_fail":false,"target_va":5416524832,"va":5379534274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37087114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379533952,"raw_hex":"e88ae73502","size":5,"soft_fail":false,"target_va":5416621600,"va":5379534481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":23660,"parent_record_offset":23324,"ancestor_record_offset":23324,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":537,"record_offset":23540,"parent_record_offset":23324,"ancestor_record_offset":23324,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":537,"record_offset":23568,"parent_record_offset":23324,"ancestor_record_offset":23324,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":537,"record_offset":23476,"parent_record_offset":23324,"ancestor_record_offset":23324,"inlinee":22886,"source_line":917,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":537,"record_offset":23504,"parent_record_offset":23324,"ancestor_record_offset":23324,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":23752,"parent_record_offset":23728,"ancestor_record_offset":23324,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":23800,"parent_record_offset":23776,"ancestor_record_offset":23324,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":23776,"parent_record_offset":23752,"ancestor_record_offset":23324,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":23728,"parent_record_offset":23688,"ancestor_record_offset":23324,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":23688,"parent_record_offset":23660,"ancestor_record_offset":23324,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":537,"record_offset":23624,"parent_record_offset":23596,"ancestor_record_offset":23324,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":537,"record_offset":23596,"parent_record_offset":23568,"ancestor_record_offset":23324,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":23840,"parent_record_offset":23688,"ancestor_record_offset":23324,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::generate_grid_ccw
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 406
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8098
// 首条位置: {"module":544,"inlinee":32920}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":544,"record_offset":8860,"parent_record_offset":8816,"ancestor_record_offset":8816,"inlinee":30455,"source_line":40,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":544,"record_offset":9068,"parent_record_offset":9040,"ancestor_record_offset":8816,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":544,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":8816,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":544,"record_offset":9096,"parent_record_offset":9040,"ancestor_record_offset":8816,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":544,"record_offset":9144,"parent_record_offset":9120,"ancestor_record_offset":8816,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":544,"record_offset":9012,"parent_record_offset":8984,"ancestor_record_offset":8816,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":8888,"parent_record_offset":8860,"ancestor_record_offset":8816,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":544,"record_offset":8936,"parent_record_offset":8912,"ancestor_record_offset":8816,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":544,"record_offset":9040,"parent_record_offset":8984,"ancestor_record_offset":8816,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":544,"record_offset":8912,"parent_record_offset":8888,"ancestor_record_offset":8816,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":544,"record_offset":8984,"parent_record_offset":8960,"ancestor_record_offset":8816,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":544,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":8816,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::grid_gen_ccw
// 记录形式: compiled_procedure; instances=1
// 源码行: 423
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":537,"record_offset":25000,"rva":10825376,"va":5379534496}
// 编译实例 RVAs: 0xa52ea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a52ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-94","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379534496,"raw_hex":"e8a2ffffff","size":5,"soft_fail":false,"target_va":5379534496,"va":5379534585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t36990009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379534496,"raw_hex":"e8396c3402","size":5,"soft_fail":false,"target_va":5416524832,"va":5379534818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37086571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379534496,"raw_hex":"e86be53502","size":5,"soft_fail":false,"target_va":5416621600,"va":5379535024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":25336,"parent_record_offset":25000,"ancestor_record_offset":25000,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":537,"record_offset":25216,"parent_record_offset":25000,"ancestor_record_offset":25000,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":537,"record_offset":25244,"parent_record_offset":25000,"ancestor_record_offset":25000,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":537,"record_offset":25152,"parent_record_offset":25000,"ancestor_record_offset":25000,"inlinee":22886,"source_line":917,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":537,"record_offset":25180,"parent_record_offset":25000,"ancestor_record_offset":25000,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":25428,"parent_record_offset":25404,"ancestor_record_offset":25000,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":25476,"parent_record_offset":25452,"ancestor_record_offset":25000,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":537,"record_offset":25452,"parent_record_offset":25428,"ancestor_record_offset":25000,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":25404,"parent_record_offset":25364,"ancestor_record_offset":25000,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":25364,"parent_record_offset":25336,"ancestor_record_offset":25000,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":537,"record_offset":25300,"parent_record_offset":25272,"ancestor_record_offset":25000,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":537,"record_offset":25272,"parent_record_offset":25244,"ancestor_record_offset":25000,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":25516,"parent_record_offset":25364,"ancestor_record_offset":25000,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00537.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_terrain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_terrain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_terrain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::generate_grid_ccw
// 信息 ID: 3803e6c425caa4ec2d4285f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=544, inline_record=8816; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=544, inline_record=9692; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=544, inline_record=15616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::generate_grid_cw
// 信息 ID: 069744782020e733f5bd2665
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_terrain::startup_glade_terrain; module=537, inline_record=29880; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=544, inline_record=7036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::grid_gen_ccw
// 信息 ID: eae2c938d69104c4c617a4da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_terrain::grid_gen_ccw @ 0xa52ef9 -> 0xa52ea0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0xa857d2 -> 0xa52ea0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0xa8599a -> 0xa52ea0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0xa86057 -> 0xa52ea0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::grid_gen_cw
// 信息 ID: 20c51e557f3b7ba9b5b7f8d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_terrain::grid_gen_cw @ 0xa52cd9 -> 0xa52c80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_terrain::startup_glade_terrain @ 0xa53ca1 -> 0xa52c80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0xa854ba -> 0xa52c80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::startup_glade_terrain
// 信息 ID: e6dbccffc6ea672d73de6de1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] db1613382300002a162c704d: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)> @ 0x116b81e -> 0xa53bc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::rocky_terrain::RockyTerrainMeshVertCount>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)> > @ 0x192f3aa -> 0xa53bc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::startup_terrain
// 信息 ID: a3b54d2eaed2b1787e06a9db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d7d00d9620e09b4a94fee40d: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)> @ 0x116c952 -> 0xa530c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)> > @ 0x194965c -> 0xa530c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/startup_terrain.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f61764295c3aaff733a5ea80e096a5324e2ca79d0572501c59d1c6489f72462d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_terrain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 420, 471, 1302
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::startup_terrain
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":471,"record_offset":7932,"rva":6666352,"va":5375375472}
// 编译实例 RVAs: 0x65b870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2750544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e8b007d6ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375375563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2626075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e8e5edd7ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375375766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b6e80
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-675736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e868b0f5ff","size":5,"soft_fail":false,"target_va":5374701184,"va":5375376915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065d060
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e829170000","size":5,"soft_fail":false,"target_va":5375381600,"va":5375375666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t355484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e89c6c0500","size":5,"soft_fail":false,"target_va":5375731504,"va":5375376015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t555995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e8db7b0800","size":5,"soft_fail":false,"target_va":5375936112,"va":5375380112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f12d0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t608411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e89b480900","size":5,"soft_fail":false,"target_va":5375988432,"va":5375380016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t27598704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e8701fa501","size":5,"soft_fail":false,"target_va":5402978144,"va":5375379435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27539652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e8c438a401","size":5,"soft_fail":false,"target_va":5402919072,"va":5375379415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t1983355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e87b431e00","size":5,"soft_fail":false,"target_va":5377364336,"va":5375380976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t9005839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e80f6b8900","size":5,"soft_fail":false,"target_va":5384381936,"va":5375376092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1869818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e8fa871c00","size":5,"soft_fail":false,"target_va":5377246528,"va":5375376705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27599082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e8ea20a501","size":5,"soft_fail":false,"target_va":5402978880,"va":5375379793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t27600639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e8ff26a501","size":5,"soft_fail":false,"target_va":5402980128,"va":5375379484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-809869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e873a4f3ff","size":5,"soft_fail":false,"target_va":5374566080,"va":5375375944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t313972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e874ca0400","size":5,"soft_fail":false,"target_va":5375692416,"va":5375378439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::rocky_terrain::RockyTerrainMesh::load
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1028764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e8644df0ff","size":5,"soft_fail":false,"target_va":5374347904,"va":5375376663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1990678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375375472,"raw_hex":"e816601e00","size":5,"soft_fail":false,"target_va":5377366224,"va":5375375541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":471,"record_offset":10764,"parent_record_offset":7932,"ancestor_record_offset":7932,"inlinee":12145,"source_line":598,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":471,"record_offset":14028,"parent_record_offset":7932,"ancestor_record_offset":7932,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":471,"record_offset":10692,"parent_record_offset":7932,"ancestor_record_offset":7932,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":471,"record_offset":9276,"parent_record_offset":7932,"ancestor_record_offset":7932,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mesh_into_library
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/mod.rs
//     evidence={"module":471,"record_offset":8628,"parent_record_offset":7932,"ancestor_record_offset":7932,"inlinee":22686,"source_line":234,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::startup_terrain::generate_grid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/startup_terrain.rs
//     evidence={"module":471,"record_offset":8096,"parent_record_offset":7932,"ancestor_record_offset":7932,"inlinee":24286,"source_line":253,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::identity
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":471,"record_offset":9332,"parent_record_offset":7932,"ancestor_record_offset":7932,"inlinee":22687,"source_line":14,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":471,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":7932,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":471,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":7932,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":471,"record_offset":9028,"parent_record_offset":9000,"ancestor_record_offset":7932,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":471,"record_offset":9000,"parent_record_offset":8972,"ancestor_record_offset":7932,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":471,"record_offset":8972,"parent_record_offset":8944,"ancestor_record_offset":7932,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":471,"record_offset":9148,"parent_record_offset":8972,"ancestor_record_offset":7932,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":471,"record_offset":8908,"parent_record_offset":8880,"ancestor_record_offset":7932,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":471,"record_offset":8880,"parent_record_offset":8852,"ancestor_record_offset":7932,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":471,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":7932,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":471,"record_offset":8852,"parent_record_offset":8824,"ancestor_record_offset":7932,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":471,"record_offset":8796,"parent_record_offset":8768,"ancestor_record_offset":7932,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":471,"record_offset":8768,"parent_record_offset":8740,"ancestor_record_offset":7932,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":471,"record_offset":8740,"parent_record_offset":8692,"ancestor_record_offset":7932,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":471,"record_offset":9300,"parent_record_offset":9276,"ancestor_record_offset":7932,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":471,"record_offset":8944,"parent_record_offset":8908,"ancestor_record_offset":7932,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":471,"record_offset":10788,"parent_record_offset":10764,"ancestor_record_offset":7932,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":471,"record_offset":10816,"parent_record_offset":10788,"ancestor_record_offset":7932,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":471,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":7932,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":471,"record_offset":8504,"parent_record_offset":8480,"ancestor_record_offset":7932,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":471,"record_offset":8480,"parent_record_offset":8428,"ancestor_record_offset":7932,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":471,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":7932,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":471,"record_offset":8528,"parent_record_offset":8504,"ancestor_record_offset":7932,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":471,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":7932,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":471,"record_offset":9184,"parent_record_offset":8908,"ancestor_record_offset":7932,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":471,"record_offset":10732,"parent_record_offset":10692,"ancestor_record_offset":7932,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":471,"record_offset":9104,"parent_record_offset":9076,"ancestor_record_offset":7932,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":471,"record_offset":8552,"parent_record_offset":8528,"ancestor_record_offset":7932,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":471,"record_offset":8692,"parent_record_offset":8628,"ancestor_record_offset":7932,"inlinee":22491,"source_line":35,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":471,"record_offset":8664,"parent_record_offset":8628,"ancestor_record_offset":7932,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":471,"record_offset":8140,"parent_record_offset":8096,"ancestor_record_offset":7932,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":471,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":7932,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":471,"record_offset":8192,"parent_record_offset":8164,"ancestor_record_offset":7932,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":471,"record_offset":8240,"parent_record_offset":8216,"ancestor_record_offset":7932,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":471,"record_offset":8316,"parent_record_offset":8288,"ancestor_record_offset":7932,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":471,"record_offset":8428,"parent_record_offset":8288,"ancestor_record_offset":7932,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":471,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":7932,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":471,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":7932,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":471,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":7932,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":471,"record_offset":8288,"parent_record_offset":8264,"ancestor_record_offset":7932,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":471,"record_offset":8264,"parent_record_offset":8240,"ancestor_record_offset":7932,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::generate_grid
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 253
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EDE
// 首条位置: {"module":420,"inlinee":24286}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":420,"record_offset":19308,"parent_record_offset":19264,"ancestor_record_offset":19264,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":420,"record_offset":19632,"parent_record_offset":19604,"ancestor_record_offset":19264,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":420,"record_offset":19684,"parent_record_offset":19660,"ancestor_record_offset":19264,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":420,"record_offset":19660,"parent_record_offset":19604,"ancestor_record_offset":19264,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":420,"record_offset":19708,"parent_record_offset":19684,"ancestor_record_offset":19264,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":420,"record_offset":19560,"parent_record_offset":19536,"ancestor_record_offset":19264,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":420,"record_offset":19732,"parent_record_offset":19708,"ancestor_record_offset":19264,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":420,"record_offset":19332,"parent_record_offset":19308,"ancestor_record_offset":19264,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":420,"record_offset":19360,"parent_record_offset":19332,"ancestor_record_offset":19264,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":420,"record_offset":19408,"parent_record_offset":19384,"ancestor_record_offset":19264,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":420,"record_offset":19484,"parent_record_offset":19456,"ancestor_record_offset":19264,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":420,"record_offset":19604,"parent_record_offset":19456,"ancestor_record_offset":19264,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":420,"record_offset":19384,"parent_record_offset":19360,"ancestor_record_offset":19264,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":420,"record_offset":19508,"parent_record_offset":19484,"ancestor_record_offset":19264,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":420,"record_offset":19536,"parent_record_offset":19508,"ancestor_record_offset":19264,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":420,"record_offset":19456,"parent_record_offset":19432,"ancestor_record_offset":19264,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":420,"record_offset":19432,"parent_record_offset":19408,"ancestor_record_offset":19264,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::startup_terrain::grid_gen
// 记录形式: compiled_procedure; instances=1
// 源码行: 270
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":471,"record_offset":15692,"rva":6672480,"va":5375381600}
// 编译实例 RVAs: 0x65d060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14065d060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_terrain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-94","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375381600,"raw_hex":"e8a2ffffff","size":5,"soft_fail":false,"target_va":5375381600,"va":5375381689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-4073478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375381600,"raw_hex":"e8fad7c1ff","size":5,"soft_fail":false,"target_va":5371308432,"va":5375381905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27596736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375381600,"raw_hex":"e8c017a501","size":5,"soft_fail":false,"target_va":5402978880,"va":5375382139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":471,"record_offset":16120,"parent_record_offset":15692,"ancestor_record_offset":15692,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":471,"record_offset":15928,"parent_record_offset":15692,"ancestor_record_offset":15692,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":471,"record_offset":15836,"parent_record_offset":15692,"ancestor_record_offset":15692,"inlinee":5427,"source_line":604,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":471,"record_offset":15892,"parent_record_offset":15692,"ancestor_record_offset":15692,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":471,"record_offset":16180,"parent_record_offset":16156,"ancestor_record_offset":15692,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":471,"record_offset":16156,"parent_record_offset":16120,"ancestor_record_offset":15692,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":471,"record_offset":15984,"parent_record_offset":15956,"ancestor_record_offset":15692,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":471,"record_offset":15956,"parent_record_offset":15928,"ancestor_record_offset":15692,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":471,"record_offset":15860,"parent_record_offset":15836,"ancestor_record_offset":15692,"inlinee":5428,"source_line":2074,"archive":"_pdb/A/modules/00471.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":471,"record_offset":16212,"parent_record_offset":16120,"ancestor_record_offset":15692,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00471.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::generate_grid
// 信息 ID: c50af9621835a778fdb27603
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_roof::startup_roof; module=420, inline_record=19264; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_terrain::startup_terrain; module=471, inline_record=8096; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_terrain::startup_terrain; module=471, inline_record=9360; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=1302, inline_record=17648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::grid_gen
// 信息 ID: 7a3c1781a749472cbd794bb6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_roof::startup_roof @ 0x59c4f7 -> 0x65d060 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_terrain::startup_terrain @ 0x65b932 -> 0x65d060 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_terrain::startup_terrain @ 0x65bb52 -> 0x65d060 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_terrain::grid_gen @ 0x65d0b9 -> 0x65d060 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0xd1eb3f -> 0x65d060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::startup_terrain::startup_terrain
// 信息 ID: cb81179f895e57245f1450e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] be51919f8ae808aa4892ddb7: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > @ 0xe41d96 -> 0x65b870 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> @ 0xf1c483 -> 0x65b870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

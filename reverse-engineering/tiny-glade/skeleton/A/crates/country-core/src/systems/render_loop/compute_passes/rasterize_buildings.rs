// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 024ecdea143463d2d6f6245a38df42dc6029a1a38f2fc29a042ab082d982e186
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rasterize_buildings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 421, 451, 1308
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":451,"record_offset":34220,"rva":6368656,"va":5375077776}
// 编译实例 RVAs: 0x612d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2451944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e81896daff","size":5,"soft_fail":false,"target_va":5372630832,"va":5375082771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2331786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8766bdcff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375081493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540600
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-863477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e80bd3f2ff","size":5,"soft_fail":false,"target_va":5374215680,"va":5375079152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589d00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-563234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8de67f7ff","size":5,"soft_fail":false,"target_va":5374516480,"va":5375079709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140642d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t193863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e847f50200","size":5,"soft_fail":false,"target_va":5375274368,"va":5375080500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140643320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t195825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8f1fc0200","size":5,"soft_fail":false,"target_va":5375275808,"va":5375079978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064cbb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t235254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8f6960300","size":5,"soft_fail":false,"target_va":5375314864,"va":5375079605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140661c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t323267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8c3ee0400","size":5,"soft_fail":false,"target_va":5375401104,"va":5375077832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t344858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e81a430500","size":5,"soft_fail":false,"target_va":5375423344,"va":5375078481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eec80
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t896616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e868ae0d00","size":5,"soft_fail":false,"target_va":5375978624,"va":5375082003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1160289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e861b41100","size":5,"soft_fail":false,"target_va":5376239920,"va":5375079626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1207315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8136c1200","size":5,"soft_fail":false,"target_va":5376286288,"va":5375078968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d853e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7808158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e89e247700","size":5,"soft_fail":false,"target_va":5382886368,"va":5375078205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dce0a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8105500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e81cae7b00","size":5,"soft_fail":false,"target_va":5383184544,"va":5375079039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd8bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8149246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8fe587c00","size":5,"soft_fail":false,"target_va":5383228352,"va":5375079101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dd9070
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8149692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8bc5a7c00","size":5,"soft_fail":false,"target_va":5383229552,"va":5375079855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddb0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8158722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8027e7c00","size":5,"soft_fail":false,"target_va":5383237792,"va":5375079065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27876919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8375ea901","size":5,"soft_fail":false,"target_va":5402958528,"va":5375081604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28514788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8e419b301","size":5,"soft_fail":false,"target_va":5403593344,"va":5375078551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28514179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e88317b301","size":5,"soft_fail":false,"target_va":5403593360,"va":5375079176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27897349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e805aea901","size":5,"soft_fail":false,"target_va":5402978880,"va":5375081526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27897260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e8acada901","size":5,"soft_fail":false,"target_va":5402978944,"va":5375081679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27898256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e890b1a901","size":5,"soft_fail":false,"target_va":5402980128,"va":5375081867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1263915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e82b491300","size":5,"soft_fail":false,"target_va":5376341760,"va":5375077840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1260847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e82f3d1300","size":5,"soft_fail":false,"target_va":5376340512,"va":5375079660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1139798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375077776,"raw_hex":"e856641100","size":5,"soft_fail":false,"target_va":5376219552,"va":5375079749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5375077776,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5375078571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":451,"record_offset":42068,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":451,"record_offset":36408,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":41848,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":451,"record_offset":42096,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":451,"record_offset":38032,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":451,"record_offset":40200,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":451,"record_offset":40112,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":451,"record_offset":40632,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":451,"record_offset":39840,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":14705,"source_line":356,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":451,"record_offset":34440,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":451,"record_offset":40532,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":9081,"source_line":40,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":451,"record_offset":36004,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":42024,"parent_record_offset":42000,"ancestor_record_offset":34220,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":42000,"parent_record_offset":41900,"ancestor_record_offset":34220,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":41932,"parent_record_offset":41900,"ancestor_record_offset":34220,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":41900,"parent_record_offset":41872,"ancestor_record_offset":34220,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":36432,"parent_record_offset":36408,"ancestor_record_offset":34220,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":451,"record_offset":34496,"parent_record_offset":34468,"ancestor_record_offset":34220,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":451,"record_offset":35928,"parent_record_offset":35896,"ancestor_record_offset":34220,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":451,"record_offset":35872,"parent_record_offset":35484,"ancestor_record_offset":34220,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":451,"record_offset":35896,"parent_record_offset":35872,"ancestor_record_offset":34220,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":451,"record_offset":35512,"parent_record_offset":35484,"ancestor_record_offset":34220,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":451,"record_offset":35560,"parent_record_offset":35536,"ancestor_record_offset":34220,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":451,"record_offset":35536,"parent_record_offset":35512,"ancestor_record_offset":34220,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":451,"record_offset":35652,"parent_record_offset":35588,"ancestor_record_offset":34220,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":451,"record_offset":35740,"parent_record_offset":35716,"ancestor_record_offset":34220,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":451,"record_offset":34468,"parent_record_offset":34440,"ancestor_record_offset":34220,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":451,"record_offset":35484,"parent_record_offset":34468,"ancestor_record_offset":34220,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":451,"record_offset":35216,"parent_record_offset":35192,"ancestor_record_offset":34220,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":451,"record_offset":35264,"parent_record_offset":35240,"ancestor_record_offset":34220,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":451,"record_offset":35192,"parent_record_offset":35168,"ancestor_record_offset":34220,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":451,"record_offset":40600,"parent_record_offset":40532,"ancestor_record_offset":34220,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":451,"record_offset":36264,"parent_record_offset":36236,"ancestor_record_offset":34220,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":451,"record_offset":34684,"parent_record_offset":34660,"ancestor_record_offset":34220,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":451,"record_offset":34768,"parent_record_offset":34744,"ancestor_record_offset":34220,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":451,"record_offset":34708,"parent_record_offset":34684,"ancestor_record_offset":34220,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":451,"record_offset":41964,"parent_record_offset":41932,"ancestor_record_offset":34220,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":451,"record_offset":34880,"parent_record_offset":34856,"ancestor_record_offset":34220,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":35676,"parent_record_offset":35652,"ancestor_record_offset":34220,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":35452,"parent_record_offset":34496,"ancestor_record_offset":34220,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":35716,"parent_record_offset":35536,"ancestor_record_offset":34220,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":35972,"parent_record_offset":34440,"ancestor_record_offset":34220,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":451,"record_offset":35820,"parent_record_offset":35796,"ancestor_record_offset":34220,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":40300,"parent_record_offset":40232,"ancestor_record_offset":34220,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":451,"record_offset":35104,"parent_record_offset":35080,"ancestor_record_offset":34220,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":451,"record_offset":35024,"parent_record_offset":35000,"ancestor_record_offset":34220,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":451,"record_offset":40008,"parent_record_offset":39940,"ancestor_record_offset":34220,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":451,"record_offset":38184,"parent_record_offset":38156,"ancestor_record_offset":34220,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":451,"record_offset":38212,"parent_record_offset":38184,"ancestor_record_offset":34220,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":451,"record_offset":35588,"parent_record_offset":35560,"ancestor_record_offset":34220,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":451,"record_offset":39880,"parent_record_offset":39840,"ancestor_record_offset":34220,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":451,"record_offset":35624,"parent_record_offset":35588,"ancestor_record_offset":34220,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":451,"record_offset":35796,"parent_record_offset":35764,"ancestor_record_offset":34220,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":451,"record_offset":38128,"parent_record_offset":38100,"ancestor_record_offset":34220,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":451,"record_offset":40052,"parent_record_offset":39840,"ancestor_record_offset":34220,"inlinee":5322,"source_line":2440,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":451,"record_offset":40076,"parent_record_offset":40052,"ancestor_record_offset":34220,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":451,"record_offset":35240,"parent_record_offset":35216,"ancestor_record_offset":34220,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":451,"record_offset":38060,"parent_record_offset":38032,"ancestor_record_offset":34220,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":451,"record_offset":40232,"parent_record_offset":40200,"ancestor_record_offset":34220,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":451,"record_offset":39908,"parent_record_offset":39840,"ancestor_record_offset":34220,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":451,"record_offset":39940,"parent_record_offset":39908,"ancestor_record_offset":34220,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":451,"record_offset":35168,"parent_record_offset":35144,"ancestor_record_offset":34220,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":451,"record_offset":34520,"parent_record_offset":34496,"ancestor_record_offset":34220,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":451,"record_offset":34544,"parent_record_offset":34520,"ancestor_record_offset":34220,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":451,"record_offset":36028,"parent_record_offset":36004,"ancestor_record_offset":34220,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":451,"record_offset":35144,"parent_record_offset":34948,"ancestor_record_offset":34220,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":451,"record_offset":36176,"parent_record_offset":36084,"ancestor_record_offset":34220,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":451,"record_offset":36056,"parent_record_offset":36028,"ancestor_record_offset":34220,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":451,"record_offset":36148,"parent_record_offset":36084,"ancestor_record_offset":34220,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":451,"record_offset":36120,"parent_record_offset":36084,"ancestor_record_offset":34220,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":451,"record_offset":36084,"parent_record_offset":36056,"ancestor_record_offset":34220,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":35056,"parent_record_offset":34972,"ancestor_record_offset":34220,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":35000,"parent_record_offset":34972,"ancestor_record_offset":34220,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":35400,"parent_record_offset":34620,"ancestor_record_offset":34220,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":34972,"parent_record_offset":34948,"ancestor_record_offset":34220,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":34596,"parent_record_offset":34572,"ancestor_record_offset":34220,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":34572,"parent_record_offset":34544,"ancestor_record_offset":34220,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":34620,"parent_record_offset":34596,"ancestor_record_offset":34220,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":451,"record_offset":34828,"parent_record_offset":34800,"ancestor_record_offset":34220,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":451,"record_offset":34856,"parent_record_offset":34828,"ancestor_record_offset":34220,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":451,"record_offset":34916,"parent_record_offset":34800,"ancestor_record_offset":34220,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":451,"record_offset":34800,"parent_record_offset":34620,"ancestor_record_offset":34220,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":451,"record_offset":34948,"parent_record_offset":34620,"ancestor_record_offset":34220,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":451,"record_offset":34660,"parent_record_offset":34620,"ancestor_record_offset":34220,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":451,"record_offset":34744,"parent_record_offset":34620,"ancestor_record_offset":34220,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":451,"record_offset":35316,"parent_record_offset":34620,"ancestor_record_offset":34220,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":451,"record_offset":36236,"parent_record_offset":36176,"ancestor_record_offset":34220,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":451,"record_offset":36292,"parent_record_offset":36236,"ancestor_record_offset":34220,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":451,"record_offset":35764,"parent_record_offset":35740,"ancestor_record_offset":34220,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":451,"record_offset":39980,"parent_record_offset":39940,"ancestor_record_offset":34220,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":451,"record_offset":40572,"parent_record_offset":40532,"ancestor_record_offset":34220,"inlinee":9072,"source_line":67,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F3D
// 首条位置: {"module":421,"inlinee":24381}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1308,"record_offset":364364,"parent_record_offset":364324,"ancestor_record_offset":364324,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$39::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1308,"record_offset":364388,"parent_record_offset":364364,"ancestor_record_offset":364324,"inlinee":17202,"source_line":1266,"archive":"_pdb/A/modules/01308.jsonl.gz"}
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
// 完整结构及原始类型记录: rasterize_buildings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rasterize_buildings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: rasterize_buildings.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":451,"record_offset":40112,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/00451.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":451,"record_offset":40632,"parent_record_offset":34220,"ancestor_record_offset":34220,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/00451.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"3dd71ad4215fdc42358b9c0a","relative_path":"_rasterize_buildings.cs.hlsl_cs_main_0.bin","sha256":"c7506477a37540f7f7c9b2947f315cce63738250625148763da385421c295579","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"rasterize_buildings","prefix_candidates":["rasterize_buildings"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"158aa922a9d1f958852f5cb5","relative_path":"_rasterize_buildings_animate.cs.hlsl_cs_main_0.bin","sha256":"cbe53867213f9d6949ea1c4340d924e2121839f3a6227cf6a3c410c2dfa80e7e","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"rasterize_buildings","prefix_candidates":["rasterize_buildings"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _rasterize_buildings_animate.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"frame_constants","storage_class":"Uniform","type":{"id":22,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4]},"decorations":[],"pointee":{"id":4,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[18,18,18,18,18,18,18,18,13,11,11,13,19,13,11,14,11,14,13,14,13,18,9]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":18,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":18,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":18,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":18,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":18,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":18,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":18,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":18,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":13,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":11,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":11,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":13,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":19,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":13,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":11,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":14,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":11,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":14,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":13,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":14,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":13,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"debug_tweakers","type_id":18,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"view_constants","type_id":9,"decorations":[{"decoration":"Offset","operands":[224]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out_mask","storage_class":"UniformConstant","type":{"id":23,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,7]},"decorations":[],"pointee":{"id":7,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[11,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":25,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,24]},"decorations":[],"pointee":{"id":24,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[10,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _rasterize_buildings.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"$Globals","storage_class":"Uniform","type":{"id":31,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,4]},"decorations":[],"pointee":{"id":4,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[30,30,19]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"out_mask_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"out_mask2_dims","type_id":30,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"buildings_count","type_id":19,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":0,"binding":9,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[9]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out_mask","storage_class":"UniformConstant","type":{"id":32,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,6]},"decorations":[],"pointee":{"id":6,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":10,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[10]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out_mask2","storage_class":"UniformConstant","type":{"id":33,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,8]},"decorations":[],"pointee":{"id":8,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[14,1,2,0,0,2,15]},"decorations":[]}},"descriptor_set":0,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[11]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":12,"name":"buildings_buffer","storage_class":"Uniform","type":{"id":34,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,10]},"decorations":[],"pointee":{"id":10,"name":"type.RWStructuredBuffer.BuildingShape","record":{"opcode":"OpTypeStruct","operands":[13]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":13,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":12,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[12]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":36,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,35]},"decorations":[],"pointee":{"id":35,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[19,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3138172,"position_among_mapped_render_targets":13,"mapped_render_target_count":70,"previous_callsite":3138150,"next_callsite":3138194,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/draw_path_mask.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rasterize_buildings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings
// 信息 ID: 215bd896230331d56c818247
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fe27c -> 0x612d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure$0
// 信息 ID: 76f6a8d8ffffd66818ac4372
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >; module=1308, inline_record=364324; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >; module=1308, inline_record=365676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c3a45ca099a311397b1a140c7de501c86e7dc15b2111f8a112dc6179c5999dbb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rasterize_buildings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 518, 543, 1585
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":543,"record_offset":45720,"rva":10692464,"va":5379401584}
// 编译实例 RVAs: 0xa32770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=3
//     target_aliases=19 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9785576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e818af6aff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379401683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4323406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8b207beff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379404905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407b1810
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2629960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8b8ded7ff","size":5,"soft_fail":false,"target_va":5376776208,"va":5379406163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1689336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e80839e6ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5379405475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408edd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1331567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e891aeebff","size":5,"soft_fail":false,"target_va":5378071808,"va":5379403370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408ee240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1330780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8a4b1ebff","size":5,"soft_fail":false,"target_va":5378073152,"va":5379403927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140905260
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1235138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e83e27edff","size":5,"soft_fail":false,"target_va":5378167392,"va":5379402525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-823773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8236ef3ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379401976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-816487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8998af3ff","size":5,"soft_fail":false,"target_va":5378585968,"va":5379402450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096b850
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-816525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8738af3ff","size":5,"soft_fail":false,"target_va":5378586704,"va":5379403224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409dd4d0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-350221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8f3a7faff","size":5,"soft_fail":false,"target_va":5379052752,"va":5379402968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a59890
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t159971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8e3700200","size":5,"soft_fail":false,"target_va":5379561616,"va":5379401640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1720225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8a13f1a00","size":5,"soft_fail":false,"target_va":5381123296,"va":5379403066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1767358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8bef71a00","size":5,"soft_fail":false,"target_va":5381170352,"va":5379402989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beefa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1819996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e85cc51b00","size":5,"soft_fail":false,"target_va":5381222304,"va":5379402303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fc510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9215114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e88a9c8c00","size":5,"soft_fail":false,"target_va":5388616976,"va":5379401857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12388373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e81508bd00","size":5,"soft_fail":false,"target_va":5391790752,"va":5379402374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160cba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12427515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8fba0bd00","size":5,"soft_fail":false,"target_va":5391829920,"va":5379402400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32445100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8ac12ef01","size":5,"soft_fail":false,"target_va":5411847152,"va":5379402047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: rasterize_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32444685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e80d11ef01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379402478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32136784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8505eea01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379404987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32077458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e89276e901","size":5,"soft_fail":false,"target_va":5411482433,"va":5379404970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32136811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e86b5eea01","size":5,"soft_fail":false,"target_va":5411541904,"va":5379405088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32137611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e88b61ea01","size":5,"soft_fail":false,"target_va":5411542880,"va":5379405264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1716187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e8db2f1a00","size":5,"soft_fail":false,"target_va":5381117840,"va":5379401648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1713548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e88c251a00","size":5,"soft_fail":false,"target_va":5381116576,"va":5379403023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1795945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379401584,"raw_hex":"e869671b00","size":5,"soft_fail":false,"target_va":5381199056,"va":5379403106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379401584,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5379402067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":543,"record_offset":51588,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":543,"record_offset":47184,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":51352,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":543,"record_offset":51616,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":543,"record_offset":48076,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":543,"record_offset":49532,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":543,"record_offset":49444,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":543,"record_offset":50064,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":543,"record_offset":49372,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":543,"record_offset":49068,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":25075,"source_line":357,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":543,"record_offset":45944,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":543,"record_offset":49864,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":543,"record_offset":46784,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":51540,"parent_record_offset":51516,"ancestor_record_offset":45720,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":51516,"parent_record_offset":51424,"ancestor_record_offset":45720,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":543,"record_offset":46400,"parent_record_offset":46376,"ancestor_record_offset":45720,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":543,"record_offset":51452,"parent_record_offset":51424,"ancestor_record_offset":45720,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":543,"record_offset":51424,"parent_record_offset":51400,"ancestor_record_offset":45720,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":543,"record_offset":46448,"parent_record_offset":46424,"ancestor_record_offset":45720,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":543,"record_offset":46424,"parent_record_offset":46400,"ancestor_record_offset":45720,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":543,"record_offset":51400,"parent_record_offset":51376,"ancestor_record_offset":45720,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":543,"record_offset":46376,"parent_record_offset":46352,"ancestor_record_offset":45720,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":543,"record_offset":46352,"parent_record_offset":46312,"ancestor_record_offset":45720,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":543,"record_offset":47208,"parent_record_offset":47184,"ancestor_record_offset":45720,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":543,"record_offset":46312,"parent_record_offset":46288,"ancestor_record_offset":45720,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":543,"record_offset":46700,"parent_record_offset":46676,"ancestor_record_offset":45720,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":543,"record_offset":46676,"parent_record_offset":46652,"ancestor_record_offset":45720,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":543,"record_offset":46624,"parent_record_offset":46592,"ancestor_record_offset":45720,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":543,"record_offset":46568,"parent_record_offset":46036,"ancestor_record_offset":45720,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":543,"record_offset":46592,"parent_record_offset":46568,"ancestor_record_offset":45720,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":543,"record_offset":46064,"parent_record_offset":46036,"ancestor_record_offset":45720,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":543,"record_offset":46652,"parent_record_offset":46592,"ancestor_record_offset":45720,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":543,"record_offset":46112,"parent_record_offset":46088,"ancestor_record_offset":45720,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":543,"record_offset":46088,"parent_record_offset":46064,"ancestor_record_offset":45720,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":543,"record_offset":46288,"parent_record_offset":46264,"ancestor_record_offset":45720,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":543,"record_offset":46000,"parent_record_offset":45972,"ancestor_record_offset":45720,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":543,"record_offset":45972,"parent_record_offset":45944,"ancestor_record_offset":45720,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":543,"record_offset":46036,"parent_record_offset":45972,"ancestor_record_offset":45720,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":48144,"parent_record_offset":48104,"ancestor_record_offset":45720,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":543,"record_offset":49932,"parent_record_offset":49864,"ancestor_record_offset":45720,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":543,"record_offset":47040,"parent_record_offset":47012,"ancestor_record_offset":45720,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":543,"record_offset":47100,"parent_record_offset":47068,"ancestor_record_offset":45720,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":543,"record_offset":49236,"parent_record_offset":49212,"ancestor_record_offset":45720,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":543,"record_offset":51484,"parent_record_offset":51452,"ancestor_record_offset":45720,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":543,"record_offset":46200,"parent_record_offset":46140,"ancestor_record_offset":45720,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":543,"record_offset":49412,"parent_record_offset":49372,"ancestor_record_offset":45720,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":543,"record_offset":46224,"parent_record_offset":46200,"ancestor_record_offset":45720,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":543,"record_offset":46264,"parent_record_offset":46088,"ancestor_record_offset":45720,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":543,"record_offset":46752,"parent_record_offset":45944,"ancestor_record_offset":45720,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":543,"record_offset":47296,"parent_record_offset":47268,"ancestor_record_offset":45720,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":543,"record_offset":46984,"parent_record_offset":46948,"ancestor_record_offset":45720,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":49632,"parent_record_offset":49564,"ancestor_record_offset":45720,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":543,"record_offset":49268,"parent_record_offset":49168,"ancestor_record_offset":45720,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":48228,"parent_record_offset":48200,"ancestor_record_offset":45720,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":543,"record_offset":48256,"parent_record_offset":48228,"ancestor_record_offset":45720,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":543,"record_offset":46140,"parent_record_offset":46112,"ancestor_record_offset":45720,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":543,"record_offset":49108,"parent_record_offset":49068,"ancestor_record_offset":45720,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":46516,"parent_record_offset":46492,"ancestor_record_offset":45720,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":46172,"parent_record_offset":46140,"ancestor_record_offset":45720,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":46492,"parent_record_offset":46312,"ancestor_record_offset":45720,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":543,"record_offset":48172,"parent_record_offset":48144,"ancestor_record_offset":45720,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":49312,"parent_record_offset":49068,"ancestor_record_offset":45720,"inlinee":7965,"source_line":2725,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":49336,"parent_record_offset":49312,"ancestor_record_offset":45720,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":543,"record_offset":48104,"parent_record_offset":48076,"ancestor_record_offset":45720,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":543,"record_offset":49564,"parent_record_offset":49532,"ancestor_record_offset":45720,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":543,"record_offset":49136,"parent_record_offset":49068,"ancestor_record_offset":45720,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":543,"record_offset":49168,"parent_record_offset":49136,"ancestor_record_offset":45720,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":543,"record_offset":47012,"parent_record_offset":46948,"ancestor_record_offset":45720,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":543,"record_offset":47068,"parent_record_offset":47012,"ancestor_record_offset":45720,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":543,"record_offset":46808,"parent_record_offset":46784,"ancestor_record_offset":45720,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":543,"record_offset":46948,"parent_record_offset":46860,"ancestor_record_offset":45720,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":543,"record_offset":46836,"parent_record_offset":46808,"ancestor_record_offset":45720,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":543,"record_offset":46920,"parent_record_offset":46860,"ancestor_record_offset":45720,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":543,"record_offset":46892,"parent_record_offset":46860,"ancestor_record_offset":45720,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":543,"record_offset":46860,"parent_record_offset":46836,"ancestor_record_offset":45720,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":543,"record_offset":49212,"parent_record_offset":49168,"ancestor_record_offset":45720,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":543,"record_offset":49904,"parent_record_offset":49864,"ancestor_record_offset":45720,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":543,"record_offset":49960,"parent_record_offset":49864,"ancestor_record_offset":45720,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73C8
// 首条位置: {"module":518,"inlinee":29640}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1585,"record_offset":193284,"parent_record_offset":193244,"ancestor_record_offset":193244,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1585,"record_offset":193308,"parent_record_offset":193284,"ancestor_record_offset":193244,"inlinee":4583,"source_line":1811,"archive":"_pdb/B/modules/01585.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo", "core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer", "arguments": ["country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo", "status": "evidence_association; original field ownership unknown"}]
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
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":543,"record_offset":49372,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00543.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_write_api","symbol":"country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture","relationship":"pdb_inline_direct","evidence":{"module":543,"record_offset":49444,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00543.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture","relationship":"pdb_inline_direct","evidence":{"module":543,"record_offset":50064,"parent_record_offset":45720,"ancestor_record_offset":45720,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00543.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"24849f3a060e8a996ab4a4a5","relative_path":"_rasterize_buildings.cs.hlsl_cs_main_0.bin","sha256":"bba210b286ed13b2bbb4061c974815696d29fe2d0d7cdbdac676e2c068decf65","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"rasterize_buildings","prefix_candidates":["rasterize_buildings"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"c448b149dc1e893bbf3ca514","relative_path":"_rasterize_buildings_animate.cs.hlsl_cs_main_0.bin","sha256":"652ef8635b31b30a9887776e73fb07525ddc8c816ec61e7cb3bb633c11ee68b1","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"rasterize_buildings","prefix_candidates":["rasterize_buildings"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _rasterize_buildings.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":14,"name":"$Globals","storage_class":"Uniform","type":{"id":40,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,13]},"decorations":[],"pointee":{"id":13,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[39,39,23]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"out_mask_dims","type_id":39,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"out_mask2_dims","type_id":39,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"buildings_count","type_id":23,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":16,"name":"out_mask","storage_class":"UniformConstant","type":{"id":41,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,15]},"decorations":[],"pointee":{"id":15,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[27,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":18,"name":"out_mask2","storage_class":"UniformConstant","type":{"id":42,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,17]},"decorations":[],"pointee":{"id":17,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[27,1,2,0,0,2,15]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"buildings_buffer","storage_class":"Uniform","type":{"id":43,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,19]},"decorations":[],"pointee":{"id":19,"name":"type.StructuredBuffer.BuildingShape","record":{"opcode":"OpTypeStruct","operands":[22]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":22,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":45,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,44]},"decorations":[],"pointee":{"id":44,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[23,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _rasterize_buildings_animate.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":19,"name":"frame_constants","storage_class":"Uniform","type":{"id":35,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,17]},"decorations":[],"pointee":{"id":17,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[31,31,31,31,31,31,31,31,26,24,24,26,32,26,24,27,24,27,26,27,26,24,26,26,26,31,22]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":31,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":31,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":31,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":31,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":31,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":31,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":31,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":31,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":26,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":24,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":24,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":26,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":32,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":26,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":24,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":27,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":24,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":27,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":26,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":27,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":26,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":24,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":26,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":26,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":26,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":31,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":22,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":21,"name":"out_mask","storage_class":"UniformConstant","type":{"id":36,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,20]},"decorations":[],"pointee":{"id":20,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[24,1,2,0,0,2,7]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":38,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,37]},"decorations":[],"pointee":{"id":37,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[23,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161817,"position_among_mapped_render_targets":16,"mapped_render_target_count":59,"previous_callsite":5161806,"next_callsite":5161828,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_path_masks.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/compute_garden.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rasterize_buildings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings
// 信息 ID: c7310c97ba2832dec2b06382
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec359 -> 0xa32770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure$0
// 信息 ID: 6d322b72c14c9d676f23f64f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >; module=1585, inline_record=193244; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >; module=1585, inline_record=194636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

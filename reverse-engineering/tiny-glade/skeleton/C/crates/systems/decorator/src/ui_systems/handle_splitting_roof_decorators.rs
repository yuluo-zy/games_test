// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d483e87bb4b9026f00f7a3784da4000cf23a8f5836c0e329f7df952a4baa8c72
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: handle_splitting_roof_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1576, 1616
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1576,"record_offset":7784,"rva":21844416,"va":5390553536}
// 编译实例 RVAs: 0x14d51c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-14876419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e8fd001dff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390555102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-14876571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e865001dff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390555270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21832900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e83cdbb2fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390555087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be90d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9356599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e8c93a71ff","size":5,"soft_fail":false,"target_va":5381198032,"va":5390554626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26064682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e82ab78d01","size":5,"soft_fail":false,"target_va":5416620769,"va":5390556082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5760204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e8341ba8ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5390555063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26066094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e8aebc8d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5390556109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::get_position_from_mesh_uv
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11339771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e805f852ff","size":5,"soft_fail":false,"target_va":5379215056,"va":5390554822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12428278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e80a5c42ff","size":5,"soft_fail":false,"target_va":5378126480,"va":5390554753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11044179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e8ad7a57ff","size":5,"soft_fail":false,"target_va":5379509504,"va":5390553678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9651246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e8d2bb6cff","size":5,"soft_fail":false,"target_va":5380902448,"va":5390553689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11619444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e88cb34eff","size":5,"soft_fail":false,"target_va":5378935744,"va":5390555183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10343269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e89b2c62ff","size":5,"soft_fail":false,"target_va":5380211968,"va":5390555232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10958271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e841ca58ff","size":5,"soft_fail":false,"target_va":5379596368,"va":5390554634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10957198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e872ce58ff","size":5,"soft_fail":false,"target_va":5379597536,"va":5390554729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::edge_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7245895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e8b96f91ff","size":5,"soft_fail":false,"target_va":5383308784,"va":5390554674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7238508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553536,"raw_hex":"e8948c91ff","size":5,"soft_fail":false,"target_va":5383316160,"va":5390554663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1576,"record_offset":7992,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":1576,"record_offset":10920,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":80522,"source_line":63,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13228,"parent_record_offset":13204,"ancestor_record_offset":7784,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13252,"parent_record_offset":13228,"ancestor_record_offset":7784,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13596,"parent_record_offset":13572,"ancestor_record_offset":7784,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13620,"parent_record_offset":13596,"ancestor_record_offset":7784,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13276,"parent_record_offset":13252,"ancestor_record_offset":7784,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":7784,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13204,"parent_record_offset":13172,"ancestor_record_offset":7784,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13572,"parent_record_offset":13516,"ancestor_record_offset":7784,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":13172,"parent_record_offset":13136,"ancestor_record_offset":7784,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":13136,"parent_record_offset":13104,"ancestor_record_offset":7784,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1576,"record_offset":12992,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1576,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":7784,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":7784,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":13544,"parent_record_offset":13516,"ancestor_record_offset":7784,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":13516,"parent_record_offset":13492,"ancestor_record_offset":7784,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":8748,"parent_record_offset":8724,"ancestor_record_offset":7784,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":8724,"parent_record_offset":8700,"ancestor_record_offset":7784,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":13492,"parent_record_offset":13468,"ancestor_record_offset":7784,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1576,"record_offset":13104,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":8676,"parent_record_offset":8652,"ancestor_record_offset":7784,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":8652,"parent_record_offset":8620,"ancestor_record_offset":7784,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":8620,"parent_record_offset":8596,"ancestor_record_offset":7784,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":1576,"record_offset":8408,"parent_record_offset":8128,"ancestor_record_offset":7784,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1576,"record_offset":8172,"parent_record_offset":8128,"ancestor_record_offset":7784,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1576,"record_offset":8196,"parent_record_offset":8172,"ancestor_record_offset":7784,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1576,"record_offset":8312,"parent_record_offset":8280,"ancestor_record_offset":7784,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1576,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":7784,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1576,"record_offset":10760,"parent_record_offset":10736,"ancestor_record_offset":7784,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1576,"record_offset":8828,"parent_record_offset":8128,"ancestor_record_offset":7784,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1576,"record_offset":8852,"parent_record_offset":8828,"ancestor_record_offset":7784,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1576,"record_offset":10736,"parent_record_offset":9500,"ancestor_record_offset":7784,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1576,"record_offset":10784,"parent_record_offset":10760,"ancestor_record_offset":7784,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":1576,"record_offset":9304,"parent_record_offset":9280,"ancestor_record_offset":7784,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1576,"record_offset":8952,"parent_record_offset":8928,"ancestor_record_offset":7784,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1576,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":7784,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1576,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":7784,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1576,"record_offset":8904,"parent_record_offset":8876,"ancestor_record_offset":7784,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1576,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":7784,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1576,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":7784,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1576,"record_offset":8596,"parent_record_offset":8128,"ancestor_record_offset":7784,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":1576,"record_offset":9280,"parent_record_offset":9256,"ancestor_record_offset":7784,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1576,"record_offset":9256,"parent_record_offset":9232,"ancestor_record_offset":7784,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1576,"record_offset":8016,"parent_record_offset":7992,"ancestor_record_offset":7784,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1576,"record_offset":8064,"parent_record_offset":8040,"ancestor_record_offset":7784,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1576,"record_offset":8128,"parent_record_offset":7992,"ancestor_record_offset":7784,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1576,"record_offset":8088,"parent_record_offset":8064,"ancestor_record_offset":7784,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1576,"record_offset":8040,"parent_record_offset":8016,"ancestor_record_offset":7784,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":1576,"record_offset":8444,"parent_record_offset":8408,"ancestor_record_offset":7784,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1576,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":7784,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1576,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":7784,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":11620,"parent_record_offset":11200,"ancestor_record_offset":7784,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":11528,"parent_record_offset":11472,"ancestor_record_offset":7784,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":11296,"parent_record_offset":11272,"ancestor_record_offset":7784,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":11328,"parent_record_offset":11244,"ancestor_record_offset":7784,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":11392,"parent_record_offset":11200,"ancestor_record_offset":7784,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1576,"record_offset":14184,"parent_record_offset":14160,"ancestor_record_offset":7784,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1576,"record_offset":13076,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1576,"record_offset":12120,"parent_record_offset":12096,"ancestor_record_offset":7784,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1576,"record_offset":12096,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1576,"record_offset":12924,"parent_record_offset":12896,"ancestor_record_offset":7784,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1576,"record_offset":12036,"parent_record_offset":12008,"ancestor_record_offset":7784,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1576,"record_offset":11856,"parent_record_offset":11832,"ancestor_record_offset":7784,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1576,"record_offset":9040,"parent_record_offset":8980,"ancestor_record_offset":7784,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":9064,"parent_record_offset":9040,"ancestor_record_offset":7784,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":8280,"parent_record_offset":8196,"ancestor_record_offset":7784,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":9104,"parent_record_offset":8928,"ancestor_record_offset":7784,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":7784,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":8380,"parent_record_offset":8128,"ancestor_record_offset":7784,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":12216,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":13420,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1576,"record_offset":8548,"parent_record_offset":8516,"ancestor_record_offset":7784,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1576,"record_offset":8224,"parent_record_offset":8196,"ancestor_record_offset":7784,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1576,"record_offset":8248,"parent_record_offset":8224,"ancestor_record_offset":7784,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1576,"record_offset":9184,"parent_record_offset":9152,"ancestor_record_offset":7784,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1576,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":7784,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1576,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":7784,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1576,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":7784,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1576,"record_offset":11176,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1576,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":7784,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1576,"record_offset":11648,"parent_record_offset":11200,"ancestor_record_offset":7784,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1576,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":7784,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1576,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":7784,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":14032,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":11976,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":12008,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":12896,"parent_record_offset":12248,"ancestor_record_offset":7784,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":11832,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1576,"record_offset":11272,"parent_record_offset":11244,"ancestor_record_offset":7784,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1576,"record_offset":11244,"parent_record_offset":11200,"ancestor_record_offset":7784,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1576,"record_offset":10840,"parent_record_offset":10808,"ancestor_record_offset":7784,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1576,"record_offset":9152,"parent_record_offset":9128,"ancestor_record_offset":7784,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1576,"record_offset":14160,"parent_record_offset":14124,"ancestor_record_offset":7784,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":1576,"record_offset":12180,"parent_record_offset":10920,"ancestor_record_offset":7784,"inlinee":27125,"source_line":32,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":1576,"record_offset":12248,"parent_record_offset":12216,"ancestor_record_offset":7784,"inlinee":80524,"source_line":127,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A8A
// 首条位置: {"module":1576,"inlinee":80522}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1576,"record_offset":12992,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1576,"record_offset":13104,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1576,"record_offset":13076,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1576,"record_offset":12096,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":12216,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":13420,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1576,"record_offset":11176,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":14032,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":11976,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":12008,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":13332,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":11832,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":1576,"record_offset":12180,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":27125,"source_line":32,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13228,"parent_record_offset":13204,"ancestor_record_offset":10920,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13252,"parent_record_offset":13228,"ancestor_record_offset":10920,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13596,"parent_record_offset":13572,"ancestor_record_offset":10920,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13620,"parent_record_offset":13596,"ancestor_record_offset":10920,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13276,"parent_record_offset":13252,"ancestor_record_offset":10920,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":10920,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13204,"parent_record_offset":13172,"ancestor_record_offset":10920,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":13572,"parent_record_offset":13516,"ancestor_record_offset":10920,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":13172,"parent_record_offset":13136,"ancestor_record_offset":10920,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":13136,"parent_record_offset":13104,"ancestor_record_offset":10920,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1576,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":10920,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":13544,"parent_record_offset":13516,"ancestor_record_offset":10920,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":13516,"parent_record_offset":13492,"ancestor_record_offset":10920,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":13492,"parent_record_offset":13468,"ancestor_record_offset":10920,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1576,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":10920,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1576,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":10920,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":11620,"parent_record_offset":11200,"ancestor_record_offset":10920,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":11528,"parent_record_offset":11472,"ancestor_record_offset":10920,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":11296,"parent_record_offset":11272,"ancestor_record_offset":10920,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":11328,"parent_record_offset":11244,"ancestor_record_offset":10920,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":11392,"parent_record_offset":11200,"ancestor_record_offset":10920,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1576,"record_offset":14184,"parent_record_offset":14160,"ancestor_record_offset":10920,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1576,"record_offset":12120,"parent_record_offset":12096,"ancestor_record_offset":10920,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1576,"record_offset":12924,"parent_record_offset":12896,"ancestor_record_offset":10920,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1576,"record_offset":12036,"parent_record_offset":12008,"ancestor_record_offset":10920,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1576,"record_offset":11856,"parent_record_offset":11832,"ancestor_record_offset":10920,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":10920,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":13444,"parent_record_offset":13420,"ancestor_record_offset":10920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1576,"record_offset":12272,"parent_record_offset":12248,"ancestor_record_offset":10920,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1576,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":10920,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1576,"record_offset":11648,"parent_record_offset":11200,"ancestor_record_offset":10920,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1576,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":10920,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1576,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":10920,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":14124,"parent_record_offset":14056,"ancestor_record_offset":10920,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":12896,"parent_record_offset":12248,"ancestor_record_offset":10920,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":14056,"parent_record_offset":14032,"ancestor_record_offset":10920,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1576,"record_offset":11272,"parent_record_offset":11244,"ancestor_record_offset":10920,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1576,"record_offset":11244,"parent_record_offset":11200,"ancestor_record_offset":10920,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1576,"record_offset":14160,"parent_record_offset":14124,"ancestor_record_offset":10920,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":1576,"record_offset":12248,"parent_record_offset":12216,"ancestor_record_offset":10920,"inlinee":80524,"source_line":127,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A8C
// 首条位置: {"module":1576,"inlinee":80524}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1576,"record_offset":12272,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1576,"record_offset":12896,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1576,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":12248,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":12620,"parent_record_offset":12380,"ancestor_record_offset":12248,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":12528,"parent_record_offset":12472,"ancestor_record_offset":12248,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":12420,"parent_record_offset":12380,"ancestor_record_offset":12248,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":12708,"parent_record_offset":12648,"ancestor_record_offset":12248,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1576,"record_offset":12768,"parent_record_offset":12648,"ancestor_record_offset":12248,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1576,"record_offset":12924,"parent_record_offset":12896,"ancestor_record_offset":12248,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":1576,"record_offset":12380,"parent_record_offset":12272,"ancestor_record_offset":12248,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1576,"record_offset":12648,"parent_record_offset":12380,"ancestor_record_offset":12248,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1576,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":12248,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1576,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":12248,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1576,"record_offset":12472,"parent_record_offset":12448,"ancestor_record_offset":12248,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":1576,"record_offset":12448,"parent_record_offset":12380,"ancestor_record_offset":12248,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1576,"record_offset":4208,"rva":21839728,"va":5390548848}
// 编译实例 RVAs: 0x14d3f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14870224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e830191dff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390548907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14870573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e8d3171dff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390549272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21826705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e86ff3b2fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390548892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26071562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e80ad28d01","size":5,"soft_fail":false,"target_va":5416620769,"va":5390549202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26071571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e813d28d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5390549218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11612774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e89acd4eff","size":5,"soft_fail":false,"target_va":5378936272,"va":5390549041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11610405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e8dbd64eff","size":5,"soft_fail":false,"target_va":5378938736,"va":5390549136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11612672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e800ce4eff","size":5,"soft_fail":false,"target_va":5378936352,"va":5390549019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11602357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e84bf64eff","size":5,"soft_fail":false,"target_va":5378946752,"va":5390549104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11602412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548848,"raw_hex":"e814f64eff","size":5,"soft_fail":false,"target_va":5378946656,"va":5390549063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":5144,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":4432,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1576,"record_offset":4612,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":5228,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":4516,"parent_record_offset":4492,"ancestor_record_offset":4208,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":4208,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":5300,"parent_record_offset":5276,"ancestor_record_offset":4208,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":5324,"parent_record_offset":5300,"ancestor_record_offset":4208,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":4564,"parent_record_offset":4540,"ancestor_record_offset":4208,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":4208,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":4492,"parent_record_offset":4460,"ancestor_record_offset":4208,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":4208,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":4460,"parent_record_offset":4432,"ancestor_record_offset":4208,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":5252,"parent_record_offset":5228,"ancestor_record_offset":4208,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":4844,"parent_record_offset":4816,"ancestor_record_offset":4208,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":4904,"parent_record_offset":4872,"ancestor_record_offset":4208,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":4872,"parent_record_offset":4844,"ancestor_record_offset":4208,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":5168,"parent_record_offset":5144,"ancestor_record_offset":4208,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":4724,"parent_record_offset":4696,"ancestor_record_offset":4208,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":4752,"parent_record_offset":4724,"ancestor_record_offset":4208,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":4780,"parent_record_offset":4752,"ancestor_record_offset":4208,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":4696,"parent_record_offset":4668,"ancestor_record_offset":4208,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1576,"record_offset":4668,"parent_record_offset":4640,"ancestor_record_offset":4208,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1576,"record_offset":4640,"parent_record_offset":4612,"ancestor_record_offset":4208,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":5192,"parent_record_offset":5168,"ancestor_record_offset":4208,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":4816,"parent_record_offset":4780,"ancestor_record_offset":4208,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1576,"record_offset":5060,"parent_record_offset":4780,"ancestor_record_offset":4208,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":5084,"parent_record_offset":5060,"ancestor_record_offset":4208,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1576,"record_offset":6592,"rva":21839280,"va":5390548400}
// 编译实例 RVAs: 0x14d3db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14869776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e8f01a1dff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390548459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14870125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e893191dff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390548824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21826257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e82ff5b2fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390548444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26072010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e8cad38d01","size":5,"soft_fail":false,"target_va":5416620769,"va":5390548754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26072019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e8d3d38d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5390548770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11612326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e85acf4eff","size":5,"soft_fail":false,"target_va":5378936272,"va":5390548593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11609957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e89bd84eff","size":5,"soft_fail":false,"target_va":5378938736,"va":5390548688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11612224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e8c0cf4eff","size":5,"soft_fail":false,"target_va":5378936352,"va":5390548571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11601909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e80bf84eff","size":5,"soft_fail":false,"target_va":5378946752,"va":5390548656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11601964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390548400,"raw_hex":"e8d4f74eff","size":5,"soft_fail":false,"target_va":5378946656,"va":5390548615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":7528,"parent_record_offset":6592,"ancestor_record_offset":6592,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":6816,"parent_record_offset":6592,"ancestor_record_offset":6592,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1576,"record_offset":6996,"parent_record_offset":6592,"ancestor_record_offset":6592,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":7612,"parent_record_offset":6592,"ancestor_record_offset":6592,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":6592,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":6592,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":6592,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":7708,"parent_record_offset":7684,"ancestor_record_offset":6592,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":6948,"parent_record_offset":6924,"ancestor_record_offset":6592,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":7732,"parent_record_offset":7708,"ancestor_record_offset":6592,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":6876,"parent_record_offset":6844,"ancestor_record_offset":6592,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":6592,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":6844,"parent_record_offset":6816,"ancestor_record_offset":6592,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":6592,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":7228,"parent_record_offset":7200,"ancestor_record_offset":6592,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":7288,"parent_record_offset":7256,"ancestor_record_offset":6592,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":7256,"parent_record_offset":7228,"ancestor_record_offset":6592,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":7552,"parent_record_offset":7528,"ancestor_record_offset":6592,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":7108,"parent_record_offset":7080,"ancestor_record_offset":6592,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":7136,"parent_record_offset":7108,"ancestor_record_offset":6592,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":7164,"parent_record_offset":7136,"ancestor_record_offset":6592,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":7080,"parent_record_offset":7052,"ancestor_record_offset":6592,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1576,"record_offset":7052,"parent_record_offset":7024,"ancestor_record_offset":6592,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1576,"record_offset":7024,"parent_record_offset":6996,"ancestor_record_offset":6592,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":7576,"parent_record_offset":7552,"ancestor_record_offset":6592,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":7200,"parent_record_offset":7164,"ancestor_record_offset":6592,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1576,"record_offset":7444,"parent_record_offset":7164,"ancestor_record_offset":6592,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":7468,"parent_record_offset":7444,"ancestor_record_offset":6592,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1576,"record_offset":5400,"rva":21840176,"va":5390549296}
// 编译实例 RVAs: 0x14d4130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14870672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e870171dff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390549355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14871021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e813161dff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390549720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21827153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e8aff1b2fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390549340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26071114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e84ad08d01","size":5,"soft_fail":false,"target_va":5416620769,"va":5390549650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26071123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e853d08d01","size":5,"soft_fail":false,"target_va":5416620794,"va":5390549666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11613222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e8dacb4eff","size":5,"soft_fail":false,"target_va":5378936272,"va":5390549489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11610853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e81bd54eff","size":5,"soft_fail":false,"target_va":5378938736,"va":5390549584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11613120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e840cc4eff","size":5,"soft_fail":false,"target_va":5378936352,"va":5390549467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11602805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e88bf44eff","size":5,"soft_fail":false,"target_va":5378946752,"va":5390549552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11602860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549296,"raw_hex":"e854f44eff","size":5,"soft_fail":false,"target_va":5378946656,"va":5390549511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":6336,"parent_record_offset":5400,"ancestor_record_offset":5400,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":5624,"parent_record_offset":5400,"ancestor_record_offset":5400,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1576,"record_offset":5804,"parent_record_offset":5400,"ancestor_record_offset":5400,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":6420,"parent_record_offset":5400,"ancestor_record_offset":5400,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":5708,"parent_record_offset":5684,"ancestor_record_offset":5400,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":5400,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":5400,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":5400,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":5400,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":6540,"parent_record_offset":6516,"ancestor_record_offset":5400,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":5684,"parent_record_offset":5652,"ancestor_record_offset":5400,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":5400,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":5652,"parent_record_offset":5624,"ancestor_record_offset":5400,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1576,"record_offset":6444,"parent_record_offset":6420,"ancestor_record_offset":5400,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":6036,"parent_record_offset":6008,"ancestor_record_offset":5400,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":6096,"parent_record_offset":6064,"ancestor_record_offset":5400,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":6064,"parent_record_offset":6036,"ancestor_record_offset":5400,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":5400,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":5916,"parent_record_offset":5888,"ancestor_record_offset":5400,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":5944,"parent_record_offset":5916,"ancestor_record_offset":5400,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":5972,"parent_record_offset":5944,"ancestor_record_offset":5400,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1576,"record_offset":5888,"parent_record_offset":5860,"ancestor_record_offset":5400,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1576,"record_offset":5860,"parent_record_offset":5832,"ancestor_record_offset":5400,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1576,"record_offset":5832,"parent_record_offset":5804,"ancestor_record_offset":5400,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":5400,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":6008,"parent_record_offset":5972,"ancestor_record_offset":5400,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1576,"record_offset":6252,"parent_record_offset":5972,"ancestor_record_offset":5400,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":5400,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part
// 记录形式: compiled_procedure; instances=1
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1576,"record_offset":272,"rva":21840736,"va":5390549856}
// 编译实例 RVAs: 0x14d4360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-14873790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e8420b1dff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390552489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ae60
//     owner=unresolved_namespace; count=1
//     target_aliases=234 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21339576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e84862bafe","size":5,"soft_fail":false,"target_va":5369212512,"va":5390552083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12423355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e8456f42ff","size":5,"soft_fail":false,"target_va":5378126944,"va":5390550294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-209782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e88accfcff","size":5,"soft_fail":false,"target_va":5390340576,"va":5390550353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a9670
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-178405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e81b47fdff","size":5,"soft_fail":false,"target_va":5390374512,"va":5390552912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd0f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26200143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e84fc88f01","size":5,"soft_fail":false,"target_va":5416750912,"va":5390550764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25830678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e816258a01","size":5,"soft_fail":false,"target_va":5416381792,"va":5390551109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26068988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e8fcc78d01","size":5,"soft_fail":false,"target_va":5416621600,"va":5390552607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26069614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e86eca8d01","size":5,"soft_fail":false,"target_va":5416622208,"va":5390552589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26069511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e807ca8d01","size":5,"soft_fail":false,"target_va":5416622240,"va":5390552724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12423674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e8066e42ff","size":5,"soft_fail":false,"target_va":5378126480,"va":5390550149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::TransientDecoratorIdGen::generate
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12423467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e8d56e42ff","size":5,"soft_fail":false,"target_va":5378128064,"va":5390551526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11058630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e83a4257ff","size":5,"soft_fail":false,"target_va":5379491392,"va":5390550017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8899456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e8803478ff","size":5,"soft_fail":false,"target_va":5381650864,"va":5390550315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8895875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e87d4278ff","size":5,"soft_fail":false,"target_va":5381654320,"va":5390550190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::apply_decorator_op::apply_decorator_op
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-15716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e89cc2ffff","size":5,"soft_fail":false,"target_va":5390535728,"va":5390551439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390549856,"raw_hex":"e8c6070000","size":5,"soft_fail":false,"target_va":5390553168,"va":5390551173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":2016,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":2144,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":1576,"record_offset":1176,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7939,"source_line":97,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1576,"record_offset":1704,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":2720,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":2628,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":528,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":2804,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1576,"record_offset":628,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26697,"source_line":291,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1576,"record_offset":896,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1576,"record_offset":1648,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":21757,"source_line":1343,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1576,"record_offset":804,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":564,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26280,"source_line":779,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":988,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26281,"source_line":1114,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":2044,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26593,"source_line":2237,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":1768,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26589,"source_line":2290,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":1576,"record_offset":592,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27125,"source_line":32,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1576,"record_offset":776,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1576,"record_offset":3152,"parent_record_offset":3068,"ancestor_record_offset":272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":3096,"parent_record_offset":3068,"ancestor_record_offset":272,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":272,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1576,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":272,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":2200,"parent_record_offset":2168,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1576,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":272,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1576,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":272,"inlinee":5092,"source_line":257,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1576,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":272,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1576,"record_offset":1356,"parent_record_offset":1272,"ancestor_record_offset":272,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1576,"record_offset":1200,"parent_record_offset":1176,"ancestor_record_offset":272,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1576,"record_offset":1504,"parent_record_offset":1356,"ancestor_record_offset":272,"inlinee":5484,"source_line":2944,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1576,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":272,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1576,"record_offset":2972,"parent_record_offset":2944,"ancestor_record_offset":272,"inlinee":4688,"source_line":971,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1576,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":272,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1576,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":272,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1576,"record_offset":928,"parent_record_offset":896,"ancestor_record_offset":272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1576,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":272,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1576,"record_offset":836,"parent_record_offset":804,"ancestor_record_offset":272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1576,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":272,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1576,"record_offset":1300,"parent_record_offset":1272,"ancestor_record_offset":272,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1576,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":272,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":2744,"parent_record_offset":2720,"ancestor_record_offset":272,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1576,"record_offset":704,"parent_record_offset":664,"ancestor_record_offset":272,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1576,"record_offset":2380,"parent_record_offset":2348,"ancestor_record_offset":272,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1576,"record_offset":1444,"parent_record_offset":1384,"ancestor_record_offset":272,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":2920,"parent_record_offset":2804,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1576,"record_offset":1044,"parent_record_offset":988,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":272,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1576,"record_offset":2312,"parent_record_offset":2168,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1576,"record_offset":1384,"parent_record_offset":1356,"ancestor_record_offset":272,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1576,"record_offset":1416,"parent_record_offset":1384,"ancestor_record_offset":272,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1576,"record_offset":664,"parent_record_offset":628,"ancestor_record_offset":272,"inlinee":26698,"source_line":223,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1576,"record_offset":736,"parent_record_offset":704,"ancestor_record_offset":272,"inlinee":26707,"source_line":226,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":2080,"parent_record_offset":2044,"ancestor_record_offset":272,"inlinee":26594,"source_line":1716,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":1828,"parent_record_offset":1768,"ancestor_record_offset":272,"inlinee":26590,"source_line":952,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":1984,"parent_record_offset":1768,"ancestor_record_offset":272,"inlinee":26591,"source_line":919,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":1112,"parent_record_offset":1080,"ancestor_record_offset":272,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":1876,"parent_record_offset":1828,"ancestor_record_offset":272,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":1080,"parent_record_offset":988,"ancestor_record_offset":272,"inlinee":15353,"source_line":984,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":1144,"parent_record_offset":988,"ancestor_record_offset":272,"inlinee":4829,"source_line":659,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":272,"inlinee":4820,"source_line":2118,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":2832,"parent_record_offset":2804,"ancestor_record_offset":272,"inlinee":26609,"source_line":2228,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1576,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":272,"inlinee":26593,"source_line":2237,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":1576,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":272,"inlinee":80520,"source_line":271,"archive":"_pdb/C/modules/01576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 271
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A88
// 首条位置: {"module":1576,"inlinee":80520}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 311
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1576,"record_offset":4020,"rva":21844048,"va":5390553168}
// 编译实例 RVAs: 0x14d5050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ae60
//     owner=unresolved_namespace; count=1
//     target_aliases=234 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21340961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553168,"raw_hex":"e8df5cbafe","size":5,"soft_fail":false,"target_va":5369212512,"va":5390553468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be90d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9355249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553168,"raw_hex":"e80f4071ff","size":5,"soft_fail":false,"target_va":5381198032,"va":5390553276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10956921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553168,"raw_hex":"e887cf58ff","size":5,"soft_fail":false,"target_va":5379596368,"va":5390553284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10955872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553168,"raw_hex":"e8a0d358ff","size":5,"soft_fail":false,"target_va":5379597536,"va":5390553403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::edge_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7244553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553168,"raw_hex":"e8f77491ff","size":5,"soft_fail":false,"target_va":5383308784,"va":5390553332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7237166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390553168,"raw_hex":"e8d29191ff","size":5,"soft_fail":false,"target_va":5383316160,"va":5390553321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: handle_splitting_roof_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2>", "constructor": "array$", "arguments": ["system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "smallvec::IntoIter<array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2> >", "constructor": "smallvec::IntoIter", "arguments": ["array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<smallvec::IntoIter<array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["smallvec::IntoIter<array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "smallvec::SmallVec::reserve_one_unchecked<array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2> >", "constructor": "smallvec::SmallVec::reserve_one_unchecked", "arguments": ["array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: handle_splitting_roof_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: handle_splitting_roof_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id
// 信息 ID: 0bc35d7dc3b0af6a573d6ca7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id; module=1576, inline_record=10920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
// 信息 ID: 24178adbe363eb883ddea99a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id; module=1576, inline_record=12248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$1
// 信息 ID: c3027b1737214c4a74dbc2cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$1 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x14d4310 -> 0x14d3f70 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$2
// 信息 ID: 393bb486136ebdcddaa9b9d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x14d42f0 -> 0x14d3db0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$3
// 信息 ID: f991055298506c34c2fad7e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$3 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x14d4300 -> 0x14d4130 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part
// 信息 ID: 1bf75a4768b7347ef9b73e92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x861e88 -> 0x14d4360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$0
// 信息 ID: 56e1cca49797a1944f4eed4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part; module=1576, inline_record=1552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1
// 信息 ID: d642a397ad2e0c100a2d4d53
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x14d4885 -> 0x14d5050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x14d4a32 -> 0x14d5050 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x14d4be5 -> 0x14d5050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id
// 信息 ID: f6cac51dc46db29bf0164363
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x86254f -> 0x14d51c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

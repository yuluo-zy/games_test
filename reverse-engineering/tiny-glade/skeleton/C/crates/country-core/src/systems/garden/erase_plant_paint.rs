// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/garden/erase_plant_paint.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 15f72e4a96f67fd1950a99e2b958b3b4d1d215069c15d4619e0975addf5249f0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: erase_plant_paint.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 507
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::erase_plant_paint::erase_plant_paint
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":507,"record_offset":63396,"rva":9574496,"va":5378283616}
// 编译实例 RVAs: 0x921860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2606036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e82c3cd8ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378284719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2606410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e8b63ad8ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378285109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: erase_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9562513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e86f166eff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378284700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: erase_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3293104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e8b03f3200","size":5,"soft_fail":false,"target_va":5381577744,"va":5378284635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: erase_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4990032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e850244c00","size":5,"soft_fail":false,"target_va":5383273968,"va":5378283931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: erase_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38065670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e806d64402","size":5,"soft_fail":false,"target_va":5416350496,"va":5378284821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: erase_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38772693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e8d59f4f02","size":5,"soft_fail":false,"target_va":5417057040,"va":5378284342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: erase_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38772269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e82d9e4f02","size":5,"soft_fail":false,"target_va":5417057056,"va":5378284782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38096006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e8864c4502","size":5,"soft_fail":false,"target_va":5416380224,"va":5378284213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38336383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e87ff74802","size":5,"soft_fail":false,"target_va":5416620794,"va":5378284406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38336568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e838f84802","size":5,"soft_fail":false,"target_va":5416621600,"va":5378285027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::copy_from_slice_impl::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38337748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e8d4fc4802","size":5,"soft_fail":false,"target_va":5416622768,"va":5378285015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::plant_sample_raster::PlantSampleRaster::accumulate_stroke
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1715264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e8402c1a00","size":5,"soft_fail":false,"target_va":5379999584,"va":5378284315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::plant_sample_raster::PlantSampleRaster::begin_stroke
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1712653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e80d221a00","size":5,"soft_fail":false,"target_va":5379997248,"va":5378284590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::plant_sample_raster::PlantSampleRaster::copy_to_staging
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1713058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e8a2231a00","size":5,"soft_fail":false,"target_va":5379997744,"va":5378284681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-162327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378283616,"raw_hex":"e8e985fdff","size":5,"soft_fail":false,"target_va":5378121712,"va":5378284034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378283616,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5378284656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":65492,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":64448,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":64928,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":507,"record_offset":64652,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":507,"record_offset":66364,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":64272,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":63744,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":507,"record_offset":64244,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":507,"record_offset":64004,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":28649,"source_line":35,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::commit_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":507,"record_offset":67188,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":29019,"source_line":137,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::copy_to_staging_ssbo
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":507,"record_offset":67388,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":29020,"source_line":307,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::stroke_changed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":507,"record_offset":64772,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":29018,"source_line":143,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::paths::draw_path::DrawState::update
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":507,"record_offset":64148,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":29012,"source_line":64,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::temp_eraser::TempEraser::brush_size_override
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/temp_eraser.rs
//     evidence={"module":507,"record_offset":63616,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":28976,"source_line":48,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::temp_eraser::TempEraser::effective_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/temp_eraser.rs
//     evidence={"module":507,"record_offset":63664,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":29007,"source_line":38,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":507,"record_offset":63920,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":28328,"source_line":110,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":507,"record_offset":64060,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":28143,"source_line":287,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":507,"record_offset":65752,"parent_record_offset":63396,"ancestor_record_offset":63396,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":65320,"parent_record_offset":65296,"ancestor_record_offset":63396,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":65344,"parent_record_offset":65320,"ancestor_record_offset":63396,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":66996,"parent_record_offset":66972,"ancestor_record_offset":63396,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":67020,"parent_record_offset":66996,"ancestor_record_offset":63396,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":65368,"parent_record_offset":65344,"ancestor_record_offset":63396,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":67044,"parent_record_offset":67020,"ancestor_record_offset":63396,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":65296,"parent_record_offset":65112,"ancestor_record_offset":63396,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":507,"record_offset":66972,"parent_record_offset":66884,"ancestor_record_offset":63396,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":64528,"parent_record_offset":64504,"ancestor_record_offset":63396,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":65048,"parent_record_offset":65020,"ancestor_record_offset":63396,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":66912,"parent_record_offset":66884,"ancestor_record_offset":63396,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":66884,"parent_record_offset":66860,"ancestor_record_offset":63396,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":64576,"parent_record_offset":64552,"ancestor_record_offset":63396,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":64552,"parent_record_offset":64528,"ancestor_record_offset":63396,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":65112,"parent_record_offset":65076,"ancestor_record_offset":63396,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":65076,"parent_record_offset":65048,"ancestor_record_offset":63396,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":66860,"parent_record_offset":66836,"ancestor_record_offset":63396,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":507,"record_offset":65152,"parent_record_offset":65112,"ancestor_record_offset":63396,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":507,"record_offset":64956,"parent_record_offset":64928,"ancestor_record_offset":63396,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":507,"record_offset":64984,"parent_record_offset":64956,"ancestor_record_offset":63396,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":64504,"parent_record_offset":64472,"ancestor_record_offset":63396,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":66504,"parent_record_offset":66460,"ancestor_record_offset":63396,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":64472,"parent_record_offset":64448,"ancestor_record_offset":63396,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":65020,"parent_record_offset":64984,"ancestor_record_offset":63396,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":507,"record_offset":64796,"parent_record_offset":64772,"ancestor_record_offset":63396,"inlinee":13752,"source_line":17,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":507,"record_offset":64680,"parent_record_offset":64652,"ancestor_record_offset":63396,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":507,"record_offset":66392,"parent_record_offset":66364,"ancestor_record_offset":63396,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":507,"record_offset":66460,"parent_record_offset":66392,"ancestor_record_offset":63396,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":507,"record_offset":65176,"parent_record_offset":65152,"ancestor_record_offset":63396,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":507,"record_offset":64824,"parent_record_offset":64796,"ancestor_record_offset":63396,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":507,"record_offset":65256,"parent_record_offset":65200,"ancestor_record_offset":63396,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":65200,"parent_record_offset":65176,"ancestor_record_offset":63396,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":65228,"parent_record_offset":65200,"ancestor_record_offset":63396,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":507,"record_offset":66940,"parent_record_offset":66912,"ancestor_record_offset":63396,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":63880,"parent_record_offset":63800,"ancestor_record_offset":63396,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":63852,"parent_record_offset":63800,"ancestor_record_offset":63396,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":63824,"parent_record_offset":63800,"ancestor_record_offset":63396,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":66296,"parent_record_offset":66160,"ancestor_record_offset":63396,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":507,"record_offset":64028,"parent_record_offset":64004,"ancestor_record_offset":63396,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":507,"record_offset":65424,"parent_record_offset":64984,"ancestor_record_offset":63396,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":65448,"parent_record_offset":65424,"ancestor_record_offset":63396,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":66740,"parent_record_offset":66540,"ancestor_record_offset":63396,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":507,"record_offset":64616,"parent_record_offset":64472,"ancestor_record_offset":63396,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":507,"record_offset":64852,"parent_record_offset":64824,"ancestor_record_offset":63396,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":507,"record_offset":64880,"parent_record_offset":64852,"ancestor_record_offset":63396,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":507,"record_offset":67256,"parent_record_offset":67224,"ancestor_record_offset":63396,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":507,"record_offset":67224,"parent_record_offset":67188,"ancestor_record_offset":63396,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":507,"record_offset":66236,"parent_record_offset":66160,"ancestor_record_offset":63396,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":507,"record_offset":66260,"parent_record_offset":66236,"ancestor_record_offset":63396,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":507,"record_offset":66332,"parent_record_offset":66160,"ancestor_record_offset":63396,"inlinee":28513,"source_line":306,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::garden::erase_plant_paint::erase_plant_paint::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/garden/erase_plant_paint.rs
//     evidence={"module":507,"record_offset":63772,"parent_record_offset":63744,"ancestor_record_offset":63396,"inlinee":29010,"source_line":45,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::garden::erase_plant_paint::erase_plant_paint::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/garden/erase_plant_paint.rs
//     evidence={"module":507,"record_offset":64300,"parent_record_offset":64272,"ancestor_record_offset":63396,"inlinee":29014,"source_line":71,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::paths::draw_path::DrawState::finish
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":507,"record_offset":64212,"parent_record_offset":64148,"ancestor_record_offset":63396,"inlinee":29016,"source_line":43,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::paths::draw_path::DrawState::start_or_continue
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":507,"record_offset":64184,"parent_record_offset":64148,"ancestor_record_offset":63396,"inlinee":29013,"source_line":50,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::temp_eraser::TempEraser::eraser_layer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/temp_eraser.rs
//     evidence={"module":507,"record_offset":63704,"parent_record_offset":63664,"ancestor_record_offset":63396,"inlinee":29008,"source_line":25,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":507,"record_offset":63944,"parent_record_offset":63920,"ancestor_record_offset":63396,"inlinee":28117,"source_line":101,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":507,"record_offset":64084,"parent_record_offset":64060,"ancestor_record_offset":63396,"inlinee":28114,"source_line":165,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":507,"record_offset":64112,"parent_record_offset":64084,"ancestor_record_offset":63396,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":507,"record_offset":63968,"parent_record_offset":63944,"ancestor_record_offset":63396,"inlinee":28119,"source_line":183,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":507,"record_offset":64324,"parent_record_offset":64300,"ancestor_record_offset":63396,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":507,"record_offset":64404,"parent_record_offset":64348,"ancestor_record_offset":63396,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":507,"record_offset":64348,"parent_record_offset":64324,"ancestor_record_offset":63396,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":507,"record_offset":63800,"parent_record_offset":63772,"ancestor_record_offset":63396,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::erase_plant_paint::erase_plant_paint::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7152
// 首条位置: {"module":507,"inlinee":29010}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":507,"record_offset":63800,"parent_record_offset":63772,"ancestor_record_offset":63772,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":63880,"parent_record_offset":63800,"ancestor_record_offset":63772,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":63852,"parent_record_offset":63800,"ancestor_record_offset":63772,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":63824,"parent_record_offset":63800,"ancestor_record_offset":63772,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::erase_plant_paint::erase_plant_paint::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7156
// 首条位置: {"module":507,"inlinee":29014}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":507,"record_offset":64324,"parent_record_offset":64300,"ancestor_record_offset":64300,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":507,"record_offset":64376,"parent_record_offset":64348,"ancestor_record_offset":64300,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":507,"record_offset":64404,"parent_record_offset":64348,"ancestor_record_offset":64300,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":507,"record_offset":64348,"parent_record_offset":64324,"ancestor_record_offset":64300,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00507.jsonl.gz"}
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
// 完整结构及原始类型记录: erase_plant_paint.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: erase_plant_paint.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: erase_plant_paint.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::erase_plant_paint::erase_plant_paint
// 信息 ID: e9ae46cb0787dfbac11d80b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8381fe098ab13feeb1d16738: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>)> > @ 0x194a2a0 -> 0x921860 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>)> @ 0x1ac24ad -> 0x921860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::erase_plant_paint::erase_plant_paint::closure$0
// 信息 ID: 452aa24c9a9496ee57e1db94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::garden::erase_plant_paint::erase_plant_paint", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::garden::erase_plant_paint::erase_plant_paint; module=507, inline_record=63772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::erase_plant_paint::erase_plant_paint::closure$1
// 信息 ID: 73071b00ebbc153eef707334
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::garden::erase_plant_paint::erase_plant_paint", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::garden::erase_plant_paint::erase_plant_paint; module=507, inline_record=64300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

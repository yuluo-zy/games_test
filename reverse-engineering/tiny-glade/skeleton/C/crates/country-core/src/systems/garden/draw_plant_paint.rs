// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/garden/draw_plant_paint.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 63cb8ccf400f77a5f626d8445bcb275febb8a37152d8c94cf9603c89df0be768
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: draw_plant_paint.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 580
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
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::draw_plant_paint::draw_plant_paint
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":81296,"rva":12345808,"va":5381054928}
// 编译实例 RVAs: 0xbc61d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5377376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e8a0f2adff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381056059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5377712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e850f1adff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381056411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: draw_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12333853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e8e3cc43ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381056040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c45c10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: draw_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t522091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e86bf70700","size":5,"soft_fail":false,"target_va":5381577744,"va":5381055648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35294643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e8b38d1a02","size":5,"soft_fail":false,"target_va":5416350384,"va":5381055736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: draw_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35294354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e8928c1a02","size":5,"soft_fail":false,"target_va":5416350496,"va":5381056137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: draw_plant_paint.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36001165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e88d552502","size":5,"soft_fail":false,"target_va":5417057056,"va":5381055886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35324859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e8bb031b02","size":5,"soft_fail":false,"target_va":5416380224,"va":5381055360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35564767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e8dfac1e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5381056022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35565275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e8dbae1e02","size":5,"soft_fail":false,"target_va":5416621600,"va":5381056320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::copy_from_slice_impl::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35566455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e877b31e02","size":5,"soft_fail":false,"target_va":5416622768,"va":5381056308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::plant_sample_raster::PlantSampleRaster::accumulate_stroke
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1056051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e8cde2efff","size":5,"soft_fail":false,"target_va":5379999584,"va":5381055630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::plant_sample_raster::PlantSampleRaster::begin_stroke
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1058222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e852daefff","size":5,"soft_fail":false,"target_va":5379997248,"va":5381055465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::plant_sample_raster::PlantSampleRaster::copy_to_staging
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1057962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e856dbefff","size":5,"soft_fail":false,"target_va":5379997744,"va":5381055701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::paths::draw_path::DrawState::update_bounded_by_glade
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t355780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e8c46d0500","size":5,"soft_fail":false,"target_va":5381411104,"va":5381055319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2933491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e80d3dd3ff","size":5,"soft_fail":false,"target_va":5378121712,"va":5381055198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2250631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381054928,"raw_hex":"e887572200","size":5,"soft_fail":false,"target_va":5383305920,"va":5381055284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381054928,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5381055669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":81788,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":81816,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":83984,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":580,"record_offset":82312,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":580,"record_offset":83148,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":82456,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":82428,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":580,"record_offset":81640,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":28649,"source_line":35,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::commit_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":580,"record_offset":83628,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":29019,"source_line":137,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::plant_sample_raster::PlantSampleRaster::copy_to_staging_ssbo
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/plant_sample_raster.rs
//     evidence={"module":580,"record_offset":82992,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":29020,"source_line":307,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::temp_eraser::TempEraser::effective_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/temp_eraser.rs
//     evidence={"module":580,"record_offset":81500,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":29007,"source_line":38,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":580,"record_offset":81556,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":28328,"source_line":110,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":580,"record_offset":81696,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":28143,"source_line":287,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":82020,"parent_record_offset":81296,"ancestor_record_offset":81296,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":84408,"parent_record_offset":84384,"ancestor_record_offset":81296,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":84432,"parent_record_offset":84408,"ancestor_record_offset":81296,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":85212,"parent_record_offset":85188,"ancestor_record_offset":81296,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":85236,"parent_record_offset":85212,"ancestor_record_offset":81296,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":84456,"parent_record_offset":84432,"ancestor_record_offset":81296,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":85260,"parent_record_offset":85236,"ancestor_record_offset":81296,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":84384,"parent_record_offset":84200,"ancestor_record_offset":81296,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":85188,"parent_record_offset":85100,"ancestor_record_offset":81296,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":81896,"parent_record_offset":81872,"ancestor_record_offset":81296,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":84144,"parent_record_offset":84120,"ancestor_record_offset":81296,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":85128,"parent_record_offset":85100,"ancestor_record_offset":81296,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":85100,"parent_record_offset":85076,"ancestor_record_offset":81296,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":81944,"parent_record_offset":81920,"ancestor_record_offset":81296,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":81920,"parent_record_offset":81896,"ancestor_record_offset":81296,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":84200,"parent_record_offset":84168,"ancestor_record_offset":81296,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":84168,"parent_record_offset":84144,"ancestor_record_offset":81296,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":85076,"parent_record_offset":85052,"ancestor_record_offset":81296,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":84240,"parent_record_offset":84200,"ancestor_record_offset":81296,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":580,"record_offset":84080,"parent_record_offset":84016,"ancestor_record_offset":81296,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":81872,"parent_record_offset":81840,"ancestor_record_offset":81296,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":84052,"parent_record_offset":84016,"ancestor_record_offset":81296,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":83272,"parent_record_offset":83236,"ancestor_record_offset":81296,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":81840,"parent_record_offset":81816,"ancestor_record_offset":81296,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":84120,"parent_record_offset":84080,"ancestor_record_offset":81296,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":84016,"parent_record_offset":83984,"ancestor_record_offset":81296,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":580,"record_offset":82340,"parent_record_offset":82312,"ancestor_record_offset":81296,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":580,"record_offset":83172,"parent_record_offset":83148,"ancestor_record_offset":81296,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":580,"record_offset":83236,"parent_record_offset":83172,"ancestor_record_offset":81296,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":580,"record_offset":84264,"parent_record_offset":84240,"ancestor_record_offset":81296,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":84344,"parent_record_offset":84288,"ancestor_record_offset":81296,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":84288,"parent_record_offset":84264,"ancestor_record_offset":81296,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":84316,"parent_record_offset":84288,"ancestor_record_offset":81296,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":85156,"parent_record_offset":85128,"ancestor_record_offset":81296,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":82048,"parent_record_offset":82020,"ancestor_record_offset":81296,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":82560,"parent_record_offset":82532,"ancestor_record_offset":81296,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":82960,"parent_record_offset":82896,"ancestor_record_offset":81296,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":81664,"parent_record_offset":81640,"ancestor_record_offset":81296,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":580,"record_offset":84512,"parent_record_offset":84080,"ancestor_record_offset":81296,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":83732,"parent_record_offset":83696,"ancestor_record_offset":81296,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":84956,"parent_record_offset":84756,"ancestor_record_offset":81296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":81984,"parent_record_offset":81840,"ancestor_record_offset":81296,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":83696,"parent_record_offset":83664,"ancestor_record_offset":81296,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":83664,"parent_record_offset":83628,"ancestor_record_offset":81296,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":580,"record_offset":83028,"parent_record_offset":82992,"ancestor_record_offset":81296,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":580,"record_offset":83052,"parent_record_offset":83028,"ancestor_record_offset":81296,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":580,"record_offset":83116,"parent_record_offset":82992,"ancestor_record_offset":81296,"inlinee":28513,"source_line":306,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::garden::draw_plant_paint::draw_plant_paint::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/garden/draw_plant_paint.rs
//     evidence={"module":580,"record_offset":82484,"parent_record_offset":82456,"ancestor_record_offset":81296,"inlinee":36992,"source_line":84,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::temp_eraser::TempEraser::eraser_layer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/temp_eraser.rs
//     evidence={"module":580,"record_offset":81524,"parent_record_offset":81500,"ancestor_record_offset":81296,"inlinee":29008,"source_line":25,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":580,"record_offset":81580,"parent_record_offset":81556,"ancestor_record_offset":81296,"inlinee":28117,"source_line":101,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":580,"record_offset":81720,"parent_record_offset":81696,"ancestor_record_offset":81296,"inlinee":28114,"source_line":165,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":580,"record_offset":81752,"parent_record_offset":81720,"ancestor_record_offset":81296,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":580,"record_offset":81604,"parent_record_offset":81580,"ancestor_record_offset":81296,"inlinee":28119,"source_line":183,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":82508,"parent_record_offset":82484,"ancestor_record_offset":81296,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":82588,"parent_record_offset":82532,"ancestor_record_offset":81296,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":82532,"parent_record_offset":82508,"ancestor_record_offset":81296,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::garden::draw_plant_paint::draw_plant_paint::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9080
// 首条位置: {"module":580,"inlinee":36992}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":82508,"parent_record_offset":82484,"ancestor_record_offset":82484,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":82560,"parent_record_offset":82532,"ancestor_record_offset":82484,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":82588,"parent_record_offset":82532,"ancestor_record_offset":82484,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":82532,"parent_record_offset":82508,"ancestor_record_offset":82484,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
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
// 完整结构及原始类型记录: draw_plant_paint.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: draw_plant_paint.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: draw_plant_paint.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::draw_plant_paint::draw_plant_paint
// 信息 ID: 805ab33dbc12167004bf86f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] af3deec2c1bf57af113519f1: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)> @ 0x9ea91a -> 0xbc61d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::record_plant_paint_history::RecordPlantPaintEditToHistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::garden::PlantDrawEvent>)> > @ 0x1921ee5 -> 0xbc61d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::garden::draw_plant_paint::draw_plant_paint::closure$0
// 信息 ID: cb068ffc2773e165e8512e67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::garden::draw_plant_paint::draw_plant_paint", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::garden::draw_plant_paint::draw_plant_paint; module=580, inline_record=82484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/shapes/rectangle/create_rectangle.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5b399f2a9a60dcaa87e8a3f32b62bfac99de62bc182748cf73c2b95c997a59f1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: create_rectangle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1461
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1461,"record_offset":55000,"rva":18010672,"va":5386719792}
// 编译实例 RVAs: 0x112d230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: create_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17108784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e8d0f0fafe","size":5,"soft_fail":false,"target_va":5369613520,"va":5386722299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: create_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13972824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e8a8ca2aff","size":5,"soft_fail":false,"target_va":5372749696,"va":5386722515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: create_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13973843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e8adc62aff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386723550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f6420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: create_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11762249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e8b7854cff","size":5,"soft_fail":false,"target_va":5374960672,"va":5386722916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6f6d0
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: create_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7069613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e8532094ff","size":5,"soft_fail":false,"target_va":5379651280,"va":5386720888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f373c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: create_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2056870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e85a9de0ff","size":5,"soft_fail":false,"target_va":5384664000,"va":5386720865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: create_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16870781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e87d6d0101","size":5,"soft_fail":false,"target_va":5403593360,"va":5386722574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16254786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e84207f800","size":5,"soft_fail":false,"target_va":5402978144,"va":5386723353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-15413052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e8c4d014ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5386720711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9196256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e820ad73ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5386723371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16255537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e8310af800","size":5,"soft_fail":false,"target_va":5402978880,"va":5386723338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11756725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e84b9b4cff","size":5,"soft_fail":false,"target_va":5374966368,"va":5386723088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::new_wall_w_params
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11435396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e87c8251ff","size":5,"soft_fail":false,"target_va":5375285440,"va":5386720831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e8f6090000","size":5,"soft_fail":false,"target_va":5386723664,"va":5386721109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-9367280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e8101171ff","size":5,"soft_fail":false,"target_va":5377353776,"va":5386721051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9783948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e874b56aff","size":5,"soft_fail":false,"target_va":5376938064,"va":5386722007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8766747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386719792,"raw_hex":"e8e53a7aff","size":5,"soft_fail":false,"target_va":5377954064,"va":5386720806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1461,"record_offset":61684,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1461,"record_offset":56600,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1461,"record_offset":55212,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1461,"record_offset":55752,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1461,"record_offset":56028,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::RoofShapeParams::from_wall_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":1461,"record_offset":58576,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":27399,"source_line":320,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::ridge_length_01_from_rectangle_ratio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":1461,"record_offset":58388,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":27713,"source_line":281,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$14::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1461,"record_offset":60164,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":24541,"source_line":60,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1461,"record_offset":59100,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":22067,"source_line":105,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_elevation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1461,"record_offset":56712,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":26489,"source_line":131,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_style
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1461,"record_offset":57116,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":26492,"source_line":149,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1461,"record_offset":58040,"parent_record_offset":55000,"ancestor_record_offset":55000,"inlinee":24554,"source_line":173,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1461,"record_offset":60648,"parent_record_offset":60624,"ancestor_record_offset":55000,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1461,"record_offset":60672,"parent_record_offset":60648,"ancestor_record_offset":55000,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1461,"record_offset":61548,"parent_record_offset":61524,"ancestor_record_offset":55000,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1461,"record_offset":61740,"parent_record_offset":61712,"ancestor_record_offset":55000,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1461,"record_offset":60624,"parent_record_offset":60516,"ancestor_record_offset":55000,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1461,"record_offset":61524,"parent_record_offset":61432,"ancestor_record_offset":55000,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1461,"record_offset":61712,"parent_record_offset":61684,"ancestor_record_offset":55000,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":61460,"parent_record_offset":61432,"ancestor_record_offset":55000,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":55900,"parent_record_offset":55868,"ancestor_record_offset":55000,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":60516,"parent_record_offset":60480,"ancestor_record_offset":55000,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":60480,"parent_record_offset":60444,"ancestor_record_offset":55000,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":60744,"parent_record_offset":60480,"ancestor_record_offset":55000,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1461,"record_offset":61432,"parent_record_offset":61408,"ancestor_record_offset":55000,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1461,"record_offset":60400,"parent_record_offset":60360,"ancestor_record_offset":55000,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1461,"record_offset":60360,"parent_record_offset":60320,"ancestor_record_offset":55000,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1461,"record_offset":60320,"parent_record_offset":60280,"ancestor_record_offset":55000,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":55868,"parent_record_offset":55824,"ancestor_record_offset":55000,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":55824,"parent_record_offset":55780,"ancestor_record_offset":55000,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":60444,"parent_record_offset":60400,"ancestor_record_offset":55000,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1461,"record_offset":60280,"parent_record_offset":60228,"ancestor_record_offset":55000,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1461,"record_offset":55992,"parent_record_offset":55968,"ancestor_record_offset":55000,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1461,"record_offset":55968,"parent_record_offset":55752,"ancestor_record_offset":55000,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1461,"record_offset":55300,"parent_record_offset":55276,"ancestor_record_offset":55000,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1461,"record_offset":55780,"parent_record_offset":55752,"ancestor_record_offset":55000,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1461,"record_offset":55276,"parent_record_offset":55236,"ancestor_record_offset":55000,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":1461,"record_offset":56052,"parent_record_offset":56028,"ancestor_record_offset":55000,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":1461,"record_offset":55236,"parent_record_offset":55212,"ancestor_record_offset":55000,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1461,"record_offset":60560,"parent_record_offset":60516,"ancestor_record_offset":55000,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1461,"record_offset":60588,"parent_record_offset":60560,"ancestor_record_offset":55000,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1461,"record_offset":56968,"parent_record_offset":56736,"ancestor_record_offset":55000,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1461,"record_offset":59916,"parent_record_offset":59892,"ancestor_record_offset":55000,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1461,"record_offset":59384,"parent_record_offset":59360,"ancestor_record_offset":55000,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1461,"record_offset":59468,"parent_record_offset":59444,"ancestor_record_offset":55000,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1461,"record_offset":58508,"parent_record_offset":58480,"ancestor_record_offset":55000,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1461,"record_offset":58424,"parent_record_offset":58388,"ancestor_record_offset":55000,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1461,"record_offset":56996,"parent_record_offset":56736,"ancestor_record_offset":55000,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1461,"record_offset":59408,"parent_record_offset":59384,"ancestor_record_offset":55000,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1461,"record_offset":56104,"parent_record_offset":56080,"ancestor_record_offset":55000,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1461,"record_offset":56080,"parent_record_offset":56052,"ancestor_record_offset":55000,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1461,"record_offset":56184,"parent_record_offset":56160,"ancestor_record_offset":55000,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1461,"record_offset":55344,"parent_record_offset":55300,"ancestor_record_offset":55000,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1461,"record_offset":61488,"parent_record_offset":61460,"ancestor_record_offset":55000,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1461,"record_offset":59580,"parent_record_offset":59556,"ancestor_record_offset":55000,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1461,"record_offset":58688,"parent_record_offset":58636,"ancestor_record_offset":55000,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1461,"record_offset":58744,"parent_record_offset":58636,"ancestor_record_offset":55000,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1461,"record_offset":58660,"parent_record_offset":58636,"ancestor_record_offset":55000,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":56128,"parent_record_offset":56104,"ancestor_record_offset":55000,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":56160,"parent_record_offset":56080,"ancestor_record_offset":55000,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":60132,"parent_record_offset":59100,"ancestor_record_offset":55000,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1461,"record_offset":60784,"parent_record_offset":60400,"ancestor_record_offset":55000,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":61384,"parent_record_offset":60228,"ancestor_record_offset":55000,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1461,"record_offset":55644,"parent_record_offset":55620,"ancestor_record_offset":55000,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1461,"record_offset":59828,"parent_record_offset":59804,"ancestor_record_offset":55000,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1461,"record_offset":59720,"parent_record_offset":59696,"ancestor_record_offset":55000,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1461,"record_offset":56284,"parent_record_offset":56260,"ancestor_record_offset":55000,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":60700,"parent_record_offset":60672,"ancestor_record_offset":55000,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1461,"record_offset":55932,"parent_record_offset":55824,"ancestor_record_offset":55000,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1461,"record_offset":55408,"parent_record_offset":55300,"ancestor_record_offset":55000,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1461,"record_offset":55596,"parent_record_offset":55300,"ancestor_record_offset":55000,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1461,"record_offset":55468,"parent_record_offset":55444,"ancestor_record_offset":55000,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1461,"record_offset":55444,"parent_record_offset":55408,"ancestor_record_offset":55000,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1461,"record_offset":55620,"parent_record_offset":55596,"ancestor_record_offset":55000,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":56260,"parent_record_offset":56236,"ancestor_record_offset":55000,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1461,"record_offset":56236,"parent_record_offset":56184,"ancestor_record_offset":55000,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":56908,"parent_record_offset":56884,"ancestor_record_offset":55000,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1461,"record_offset":56932,"parent_record_offset":56908,"ancestor_record_offset":55000,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::roof_height_01_from_wall_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":1461,"record_offset":58600,"parent_record_offset":58576,"ancestor_record_offset":55000,"inlinee":27401,"source_line":345,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1461,"record_offset":59940,"parent_record_offset":59916,"ancestor_record_offset":55000,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::_try_update_state
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1461,"record_offset":56736,"parent_record_offset":56712,"ancestor_record_offset":55000,"inlinee":24555,"source_line":185,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::impl$0::update_elevation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1461,"record_offset":56856,"parent_record_offset":56736,"ancestor_record_offset":55000,"inlinee":26185,"source_line":132,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::impl$0::update_style::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1461,"record_offset":57260,"parent_record_offset":57140,"ancestor_record_offset":55000,"inlinee":26181,"source_line":150,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::impl$0::update_wall_state_enclosure::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1461,"record_offset":58184,"parent_record_offset":58064,"ancestor_record_offset":55000,"inlinee":24557,"source_line":174,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1461,"record_offset":61596,"parent_record_offset":60164,"ancestor_record_offset":55000,"inlinee":24553,"source_line":11,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1461,"record_offset":59892,"parent_record_offset":59868,"ancestor_record_offset":55000,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1461,"record_offset":59156,"parent_record_offset":59132,"ancestor_record_offset":55000,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1461,"record_offset":59180,"parent_record_offset":59156,"ancestor_record_offset":55000,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1461,"record_offset":59868,"parent_record_offset":59648,"ancestor_record_offset":55000,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1461,"record_offset":59212,"parent_record_offset":59180,"ancestor_record_offset":55000,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":59780,"parent_record_offset":59756,"ancestor_record_offset":55000,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":59756,"parent_record_offset":59648,"ancestor_record_offset":55000,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":59696,"parent_record_offset":59672,"ancestor_record_offset":55000,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":60076,"parent_record_offset":59288,"ancestor_record_offset":55000,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":59672,"parent_record_offset":59648,"ancestor_record_offset":55000,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":59264,"parent_record_offset":59240,"ancestor_record_offset":55000,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":59240,"parent_record_offset":59180,"ancestor_record_offset":55000,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":59288,"parent_record_offset":59264,"ancestor_record_offset":55000,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1461,"record_offset":59528,"parent_record_offset":59500,"ancestor_record_offset":55000,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1461,"record_offset":59556,"parent_record_offset":59528,"ancestor_record_offset":55000,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1461,"record_offset":59616,"parent_record_offset":59500,"ancestor_record_offset":55000,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1461,"record_offset":59500,"parent_record_offset":59288,"ancestor_record_offset":55000,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":59332,"parent_record_offset":59288,"ancestor_record_offset":55000,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":59648,"parent_record_offset":59288,"ancestor_record_offset":55000,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1461,"record_offset":59360,"parent_record_offset":59288,"ancestor_record_offset":55000,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1461,"record_offset":59444,"parent_record_offset":59288,"ancestor_record_offset":55000,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1461,"record_offset":59984,"parent_record_offset":59288,"ancestor_record_offset":55000,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":1461,"record_offset":56824,"parent_record_offset":56800,"ancestor_record_offset":55000,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1461,"record_offset":58636,"parent_record_offset":58600,"ancestor_record_offset":55000,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1461,"record_offset":57024,"parent_record_offset":56736,"ancestor_record_offset":55000,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1461,"record_offset":56884,"parent_record_offset":56736,"ancestor_record_offset":55000,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1461,"record_offset":59132,"parent_record_offset":59100,"ancestor_record_offset":55000,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1461,"record_offset":56772,"parent_record_offset":56736,"ancestor_record_offset":55000,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1461,"record_offset":56800,"parent_record_offset":56736,"ancestor_record_offset":55000,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1461,"record_offset":60228,"parent_record_offset":60164,"ancestor_record_offset":55000,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1461,"record_offset":58480,"parent_record_offset":58388,"ancestor_record_offset":55000,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1461,"record_offset":62164,"rva":18014544,"va":5386723664}
// 编译实例 RVAs: 0x112e150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: create_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17110254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386723664,"raw_hex":"e812ebfafe","size":5,"soft_fail":false,"target_va":5369613520,"va":5386723769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16254138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386723664,"raw_hex":"e8ba04f800","size":5,"soft_fail":false,"target_va":5402978880,"va":5386724737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16254422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386723664,"raw_hex":"e8d605f800","size":5,"soft_fail":false,"target_va":5402979152,"va":5386724725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16254638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386723664,"raw_hex":"e8ae06f800","size":5,"soft_fail":false,"target_va":5402979344,"va":5386724701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16255309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386723664,"raw_hex":"e84d09f800","size":5,"soft_fail":false,"target_va":5402980128,"va":5386724814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":1461,"record_offset":64624,"parent_record_offset":62164,"ancestor_record_offset":62164,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":63428,"parent_record_offset":62164,"ancestor_record_offset":62164,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":66548,"parent_record_offset":62164,"ancestor_record_offset":62164,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1461,"record_offset":66584,"parent_record_offset":62164,"ancestor_record_offset":62164,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":1461,"record_offset":62364,"parent_record_offset":62164,"ancestor_record_offset":62164,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":1461,"record_offset":63516,"parent_record_offset":62164,"ancestor_record_offset":62164,"inlinee":23583,"source_line":67,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":1461,"record_offset":64836,"parent_record_offset":64692,"ancestor_record_offset":62164,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":1461,"record_offset":64692,"parent_record_offset":64652,"ancestor_record_offset":62164,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1461,"record_offset":65112,"parent_record_offset":64652,"ancestor_record_offset":62164,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1461,"record_offset":65136,"parent_record_offset":65112,"ancestor_record_offset":62164,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":1461,"record_offset":64652,"parent_record_offset":64624,"ancestor_record_offset":62164,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1461,"record_offset":65440,"parent_record_offset":65416,"ancestor_record_offset":62164,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1461,"record_offset":65212,"parent_record_offset":65188,"ancestor_record_offset":62164,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1461,"record_offset":65188,"parent_record_offset":65164,"ancestor_record_offset":62164,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1461,"record_offset":65260,"parent_record_offset":65236,"ancestor_record_offset":62164,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1461,"record_offset":65348,"parent_record_offset":65324,"ancestor_record_offset":62164,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1461,"record_offset":65416,"parent_record_offset":65136,"ancestor_record_offset":62164,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1461,"record_offset":65164,"parent_record_offset":65136,"ancestor_record_offset":62164,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1461,"record_offset":65084,"parent_record_offset":64652,"ancestor_record_offset":62164,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":1461,"record_offset":64900,"parent_record_offset":64868,"ancestor_record_offset":62164,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1461,"record_offset":64804,"parent_record_offset":64780,"ancestor_record_offset":62164,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1461,"record_offset":63180,"parent_record_offset":63156,"ancestor_record_offset":62164,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1461,"record_offset":62648,"parent_record_offset":62624,"ancestor_record_offset":62164,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1461,"record_offset":62732,"parent_record_offset":62708,"ancestor_record_offset":62164,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1461,"record_offset":62672,"parent_record_offset":62648,"ancestor_record_offset":62164,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1461,"record_offset":62844,"parent_record_offset":62820,"ancestor_record_offset":62164,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1461,"record_offset":64780,"parent_record_offset":64692,"ancestor_record_offset":62164,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":65284,"parent_record_offset":65260,"ancestor_record_offset":62164,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":64592,"parent_record_offset":63516,"ancestor_record_offset":62164,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1461,"record_offset":65236,"parent_record_offset":65212,"ancestor_record_offset":62164,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1461,"record_offset":65008,"parent_record_offset":64976,"ancestor_record_offset":62164,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1461,"record_offset":63092,"parent_record_offset":63068,"ancestor_record_offset":62164,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1461,"record_offset":62984,"parent_record_offset":62960,"ancestor_record_offset":62164,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1461,"record_offset":65056,"parent_record_offset":64652,"ancestor_record_offset":62164,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1461,"record_offset":64724,"parent_record_offset":64692,"ancestor_record_offset":62164,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1461,"record_offset":64748,"parent_record_offset":64724,"ancestor_record_offset":62164,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1461,"record_offset":63204,"parent_record_offset":63180,"ancestor_record_offset":62164,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1461,"record_offset":63480,"parent_record_offset":63456,"ancestor_record_offset":62164,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/shapes/rectangle/create_rectangle.rs
//     evidence={"module":1461,"record_offset":63456,"parent_record_offset":63428,"ancestor_record_offset":62164,"inlinee":67135,"source_line":153,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1461,"record_offset":64868,"parent_record_offset":64652,"ancestor_record_offset":62164,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1461,"record_offset":64952,"parent_record_offset":64928,"ancestor_record_offset":62164,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1461,"record_offset":64976,"parent_record_offset":64952,"ancestor_record_offset":62164,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1461,"record_offset":64928,"parent_record_offset":64900,"ancestor_record_offset":62164,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1461,"record_offset":63156,"parent_record_offset":63132,"ancestor_record_offset":62164,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1461,"record_offset":62420,"parent_record_offset":62396,"ancestor_record_offset":62164,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1461,"record_offset":62444,"parent_record_offset":62420,"ancestor_record_offset":62164,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1461,"record_offset":63132,"parent_record_offset":62912,"ancestor_record_offset":62164,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1461,"record_offset":62476,"parent_record_offset":62444,"ancestor_record_offset":62164,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":63044,"parent_record_offset":63020,"ancestor_record_offset":62164,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":63020,"parent_record_offset":62912,"ancestor_record_offset":62164,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":62960,"parent_record_offset":62936,"ancestor_record_offset":62164,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":63340,"parent_record_offset":62552,"ancestor_record_offset":62164,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":62936,"parent_record_offset":62912,"ancestor_record_offset":62164,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":62528,"parent_record_offset":62504,"ancestor_record_offset":62164,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":62504,"parent_record_offset":62444,"ancestor_record_offset":62164,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":62552,"parent_record_offset":62528,"ancestor_record_offset":62164,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1461,"record_offset":62792,"parent_record_offset":62764,"ancestor_record_offset":62164,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1461,"record_offset":62820,"parent_record_offset":62792,"ancestor_record_offset":62164,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1461,"record_offset":62880,"parent_record_offset":62764,"ancestor_record_offset":62164,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1461,"record_offset":62764,"parent_record_offset":62552,"ancestor_record_offset":62164,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":62596,"parent_record_offset":62552,"ancestor_record_offset":62164,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1461,"record_offset":62912,"parent_record_offset":62552,"ancestor_record_offset":62164,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1461,"record_offset":62624,"parent_record_offset":62552,"ancestor_record_offset":62164,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1461,"record_offset":62708,"parent_record_offset":62552,"ancestor_record_offset":62164,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1461,"record_offset":63248,"parent_record_offset":62552,"ancestor_record_offset":62164,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1461,"record_offset":65372,"parent_record_offset":65348,"ancestor_record_offset":62164,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1461,"record_offset":62396,"parent_record_offset":62364,"ancestor_record_offset":62164,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1461,"record_offset":63396,"parent_record_offset":62364,"ancestor_record_offset":62164,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1063F
// 首条位置: {"module":1461,"inlinee":67135}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1461,"record_offset":63480,"parent_record_offset":63456,"ancestor_record_offset":63456,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01461.jsonl.gz"}
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
// 完整结构及原始类型记录: create_rectangle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: create_rectangle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: create_rectangle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierId>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierId>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierId>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierMinimumLifespan> > > >,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierAnimation> > > >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierAnimation> > > >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>,enum2$<core::option::Option<ref$<country_core::systems::ui::signifiers::SignifierAnimation> > > >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>", "bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle
// 信息 ID: 9507d6ed98281b31db0a1a8d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ea5b4728a78321e0770a5709: void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>)> @ 0xd260a7 -> 0x112d230 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::CreateRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestToolbarCategoryChange>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_default_customization::GladeDefaultCustomization>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>)> > @ 0xe820cb -> 0x112d230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors
// 信息 ID: 33646c86c6cdf10b127f3a95
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::create_circle::create_circle @ 0x69eb88 -> 0x112e150 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle @ 0x112d755 -> 0x112e150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors::closure$0
// 信息 ID: 6971f00bfa13a7a1457ebdef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors; module=1461, inline_record=63456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

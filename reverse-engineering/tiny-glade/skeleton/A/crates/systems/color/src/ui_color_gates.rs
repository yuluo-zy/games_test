// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/color/src/ui_color_gates.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 1740b05ec52e18a441aae3e93e7c6f8cafda0528e639bd3e3e5e4f9410ddecfd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_color_gates.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3477, 3496
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_color; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3477,"record_offset":2592,"rva":27975248,"va":5396684368}
// 编译实例 RVAs: 0x1aade50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23936261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e8fbc292fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5396685952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23936393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e877c292fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5396686100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac6100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t97605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e8457d0100","size":5,"soft_fail":false,"target_va":5396783360,"va":5396685750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6906922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e82a646900","size":5,"soft_fail":false,"target_va":5403593360,"va":5396686433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6290919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e8e7fd5f00","size":5,"soft_fail":false,"target_va":5402978144,"va":5396687220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-19155740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e8e4b4dbfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5396685623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6412228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e8c4d76100","size":5,"soft_fail":false,"target_va":5403097408,"va":5396685175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-25382038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e86ab37cfe","size":5,"soft_fail":false,"target_va":5371304880,"va":5396686913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19160136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e8b8a3dbfe","size":5,"soft_fail":false,"target_va":5377527120,"va":5396687251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t6291717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e805016000","size":5,"soft_fail":false,"target_va":5402978880,"va":5396687158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6292259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e823036000","size":5,"soft_fail":false,"target_va":5402979344,"va":5396687080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22117488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e89083aefe","size":5,"soft_fail":false,"target_va":5374567584,"va":5396685067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12116166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e83a1f47ff","size":5,"soft_fail":false,"target_va":5384568944,"va":5396685105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21136563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e84d7bbdfe","size":5,"soft_fail":false,"target_va":5375550288,"va":5396686846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21135340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e81480bdfe","size":5,"soft_fail":false,"target_va":5375550448,"va":5396685783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20914561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e87fdec0fe","size":5,"soft_fail":false,"target_va":5375771168,"va":5396685724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21330277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e89b86bafe","size":5,"soft_fail":false,"target_va":5375355744,"va":5396686016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14412443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396684368,"raw_hex":"e8651524ff","size":5,"soft_fail":false,"target_va":5382273632,"va":5396686070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3477,"record_offset":6952,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3477,"record_offset":7120,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":4016,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":8660,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":4044,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":5152,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3477,"record_offset":6896,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3477,"record_offset":3112,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3477,"record_offset":4280,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3477,"record_offset":7620,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3477,"record_offset":5116,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3477,"record_offset":7336,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3477,"record_offset":7536,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3477,"record_offset":4156,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3477,"record_offset":2780,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3477,"record_offset":3388,"parent_record_offset":2592,"ancestor_record_offset":2592,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":7232,"parent_record_offset":7208,"ancestor_record_offset":2592,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":7256,"parent_record_offset":7232,"ancestor_record_offset":2592,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":7488,"parent_record_offset":7464,"ancestor_record_offset":2592,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":7176,"parent_record_offset":7148,"ancestor_record_offset":2592,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":7208,"parent_record_offset":7176,"ancestor_record_offset":2592,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":7464,"parent_record_offset":7408,"ancestor_record_offset":2592,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3477,"record_offset":7148,"parent_record_offset":7120,"ancestor_record_offset":2592,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3477,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":2592,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":2592,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":7436,"parent_record_offset":7408,"ancestor_record_offset":2592,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":5308,"parent_record_offset":5280,"ancestor_record_offset":2592,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":3260,"parent_record_offset":3228,"ancestor_record_offset":2592,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":2592,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":8904,"parent_record_offset":8864,"ancestor_record_offset":2592,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":8864,"parent_record_offset":8824,"ancestor_record_offset":2592,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":9136,"parent_record_offset":8864,"ancestor_record_offset":2592,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":2592,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3477,"record_offset":8780,"parent_record_offset":8740,"ancestor_record_offset":2592,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3477,"record_offset":8740,"parent_record_offset":8700,"ancestor_record_offset":2592,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3477,"record_offset":8700,"parent_record_offset":8660,"ancestor_record_offset":2592,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":3228,"parent_record_offset":3184,"ancestor_record_offset":2592,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":5216,"parent_record_offset":5184,"ancestor_record_offset":2592,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":3184,"parent_record_offset":3140,"ancestor_record_offset":2592,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":5256,"parent_record_offset":5216,"ancestor_record_offset":2592,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":8824,"parent_record_offset":8780,"ancestor_record_offset":2592,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":2592,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3477,"record_offset":5436,"parent_record_offset":5216,"ancestor_record_offset":2592,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3477,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":2592,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3477,"record_offset":5184,"parent_record_offset":5152,"ancestor_record_offset":2592,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3477,"record_offset":4492,"parent_record_offset":4344,"ancestor_record_offset":2592,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3477,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":2592,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3477,"record_offset":3328,"parent_record_offset":3112,"ancestor_record_offset":2592,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3477,"record_offset":4344,"parent_record_offset":4304,"ancestor_record_offset":2592,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3477,"record_offset":3140,"parent_record_offset":3112,"ancestor_record_offset":2592,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3477,"record_offset":8568,"parent_record_offset":7736,"ancestor_record_offset":2592,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3477,"record_offset":4740,"parent_record_offset":4304,"ancestor_record_offset":2592,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3477,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":2592,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3477,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":2592,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3477,"record_offset":5068,"parent_record_offset":5044,"ancestor_record_offset":2592,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3477,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":2592,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3477,"record_offset":4816,"parent_record_offset":4792,"ancestor_record_offset":2592,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3477,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":2592,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3477,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":2592,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3477,"record_offset":5044,"parent_record_offset":4764,"ancestor_record_offset":2592,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3477,"record_offset":4792,"parent_record_offset":4764,"ancestor_record_offset":2592,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3477,"record_offset":4712,"parent_record_offset":4304,"ancestor_record_offset":2592,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3477,"record_offset":8592,"parent_record_offset":8568,"ancestor_record_offset":2592,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3477,"record_offset":8952,"parent_record_offset":8904,"ancestor_record_offset":2592,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3477,"record_offset":8980,"parent_record_offset":8952,"ancestor_record_offset":2592,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3477,"record_offset":4556,"parent_record_offset":4524,"ancestor_record_offset":2592,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3477,"record_offset":4460,"parent_record_offset":4436,"ancestor_record_offset":2592,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3477,"record_offset":5628,"parent_record_offset":5600,"ancestor_record_offset":2592,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3477,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":2592,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3477,"record_offset":3028,"parent_record_offset":2996,"ancestor_record_offset":2592,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3477,"record_offset":9200,"parent_record_offset":9176,"ancestor_record_offset":2592,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3477,"record_offset":5796,"parent_record_offset":5768,"ancestor_record_offset":2592,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3477,"record_offset":5496,"parent_record_offset":5464,"ancestor_record_offset":2592,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3477,"record_offset":5768,"parent_record_offset":5700,"ancestor_record_offset":2592,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3477,"record_offset":5672,"parent_record_offset":5528,"ancestor_record_offset":2592,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3477,"record_offset":5600,"parent_record_offset":5572,"ancestor_record_offset":2592,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3477,"record_offset":5572,"parent_record_offset":5528,"ancestor_record_offset":2592,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3477,"record_offset":5528,"parent_record_offset":5496,"ancestor_record_offset":2592,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3477,"record_offset":5464,"parent_record_offset":5216,"ancestor_record_offset":2592,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3477,"record_offset":6644,"parent_record_offset":5672,"ancestor_record_offset":2592,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3477,"record_offset":9708,"parent_record_offset":9680,"ancestor_record_offset":2592,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3477,"record_offset":3604,"parent_record_offset":3580,"ancestor_record_offset":2592,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3477,"record_offset":4436,"parent_record_offset":4344,"ancestor_record_offset":2592,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3477,"record_offset":5944,"parent_record_offset":5916,"ancestor_record_offset":2592,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3477,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":2592,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3477,"record_offset":5884,"parent_record_offset":5796,"ancestor_record_offset":2592,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3477,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":2592,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3477,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":2592,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3477,"record_offset":6556,"parent_record_offset":5700,"ancestor_record_offset":2592,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3477,"record_offset":3064,"parent_record_offset":2912,"ancestor_record_offset":2592,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3477,"record_offset":9176,"parent_record_offset":8780,"ancestor_record_offset":2592,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3477,"record_offset":6732,"parent_record_offset":5528,"ancestor_record_offset":2592,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3477,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":2592,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3477,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":2592,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3477,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":2592,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3477,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":2592,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3477,"record_offset":4192,"parent_record_offset":4156,"ancestor_record_offset":2592,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3477,"record_offset":7284,"parent_record_offset":7256,"ancestor_record_offset":2592,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3477,"record_offset":3292,"parent_record_offset":3184,"ancestor_record_offset":2592,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3477,"record_offset":8144,"parent_record_offset":7736,"ancestor_record_offset":2592,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3477,"record_offset":4380,"parent_record_offset":4344,"ancestor_record_offset":2592,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3477,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":2592,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3477,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":2592,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3477,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":2592,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3477,"record_offset":4220,"parent_record_offset":4156,"ancestor_record_offset":2592,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3477,"record_offset":5820,"parent_record_offset":5796,"ancestor_record_offset":2592,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$0::positions::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":3477,"record_offset":5972,"parent_record_offset":5944,"ancestor_record_offset":2592,"inlinee":24870,"source_line":147,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3477,"record_offset":4524,"parent_record_offset":4304,"ancestor_record_offset":2592,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3477,"record_offset":4608,"parent_record_offset":4584,"ancestor_record_offset":2592,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3477,"record_offset":4632,"parent_record_offset":4608,"ancestor_record_offset":2592,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3477,"record_offset":4584,"parent_record_offset":4556,"ancestor_record_offset":2592,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3477,"record_offset":2804,"parent_record_offset":2780,"ancestor_record_offset":2592,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3477,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":2592,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3477,"record_offset":3804,"parent_record_offset":3464,"ancestor_record_offset":2592,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3477,"record_offset":2912,"parent_record_offset":2852,"ancestor_record_offset":2592,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3477,"record_offset":3464,"parent_record_offset":3436,"ancestor_record_offset":2592,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3477,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2592,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3477,"record_offset":2884,"parent_record_offset":2852,"ancestor_record_offset":2592,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3477,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2592,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3477,"record_offset":3548,"parent_record_offset":3512,"ancestor_record_offset":2592,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3477,"record_offset":3580,"parent_record_offset":3548,"ancestor_record_offset":2592,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3477,"record_offset":3640,"parent_record_offset":3512,"ancestor_record_offset":2592,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3477,"record_offset":3512,"parent_record_offset":3464,"ancestor_record_offset":2592,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3477,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":2592,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3477,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":2592,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3477,"record_offset":2996,"parent_record_offset":2940,"ancestor_record_offset":2592,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3477,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":2592,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] system_color::ui_color_gates::ui_color_gates::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/color/src/ui_color_gates.rs
//     evidence={"module":3477,"record_offset":5700,"parent_record_offset":5672,"ancestor_record_offset":2592,"inlinee":106684,"source_line":54,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A0BC
// 首条位置: {"module":3477,"inlinee":106684}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3477,"record_offset":5768,"parent_record_offset":5700,"ancestor_record_offset":5700,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3477,"record_offset":6556,"parent_record_offset":5700,"ancestor_record_offset":5700,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3477,"record_offset":5796,"parent_record_offset":5768,"ancestor_record_offset":5700,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3477,"record_offset":5944,"parent_record_offset":5916,"ancestor_record_offset":5700,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3477,"record_offset":5884,"parent_record_offset":5796,"ancestor_record_offset":5700,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3477,"record_offset":5916,"parent_record_offset":5768,"ancestor_record_offset":5700,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3477,"record_offset":5852,"parent_record_offset":5820,"ancestor_record_offset":5700,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3477,"record_offset":5820,"parent_record_offset":5796,"ancestor_record_offset":5700,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$0::positions::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":3477,"record_offset":5972,"parent_record_offset":5944,"ancestor_record_offset":5700,"inlinee":24870,"source_line":147,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates::closure$1
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A0BB
// 首条位置: {"module":3477,"inlinee":106683}
// 编译实例 RVAs: 0x9afba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6116441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e8a7aba2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5378866132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6116567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e829aba2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5378866274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24111907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e823eb6f01","size":5,"soft_fail":false,"target_va":5402978144,"va":5378866232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_color::ui_color_wall::color_cursor_icon
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11511626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378866080,"raw_hex":"e84aa7af00","size":5,"soft_fail":false,"target_va":5390377808,"va":5378866177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":1696,"parent_record_offset":1664,"ancestor_record_offset":1664,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3477,"record_offset":1840,"parent_record_offset":1664,"ancestor_record_offset":1664,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3477,"record_offset":1924,"parent_record_offset":1664,"ancestor_record_offset":1664,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":1664,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":1664,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":1664,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":1724,"parent_record_offset":1696,"ancestor_record_offset":1664,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":1664,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3477,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":1664,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3477,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":1664,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":1664,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3477,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":1664,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::ui_color_gates::ui_color_gates::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3477,"record_offset":11820,"rva":27978928,"va":5396688048}
// 编译实例 RVAs: 0x1aaecb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23938424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396688048,"raw_hex":"e888ba92fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5396688115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23938899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396688048,"raw_hex":"e8adb892fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5396688606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_color_gates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6905123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396688048,"raw_hex":"e8235d6900","size":5,"soft_fail":false,"target_va":5403593360,"va":5396688232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6289613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396688048,"raw_hex":"e8cdf85f00","size":5,"soft_fail":false,"target_va":5402978144,"va":5396688526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6230703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396688048,"raw_hex":"e8af125f00","size":5,"soft_fail":false,"target_va":5402919072,"va":5396688364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19161416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396688048,"raw_hex":"e8b89edbfe","size":5,"soft_fail":false,"target_va":5377527120,"va":5396688531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19441856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396688048,"raw_hex":"e84057d7fe","size":5,"soft_fail":false,"target_va":5377246528,"va":5396688379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21331588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396688048,"raw_hex":"e87c81bafe","size":5,"soft_fail":false,"target_va":5375356704,"va":5396688287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21331427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396688048,"raw_hex":"e81d82bafe","size":5,"soft_fail":false,"target_va":5375356896,"va":5396688318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21332292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396688048,"raw_hex":"e8bc7ebafe","size":5,"soft_fail":false,"target_va":5375355984,"va":5396688271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":11976,"parent_record_offset":11820,"ancestor_record_offset":11820,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3477,"record_offset":13180,"parent_record_offset":11820,"ancestor_record_offset":11820,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":12124,"parent_record_offset":11820,"ancestor_record_offset":11820,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3477,"record_offset":12624,"parent_record_offset":11820,"ancestor_record_offset":11820,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3477,"record_offset":13264,"parent_record_offset":11820,"ancestor_record_offset":11820,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":11820,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":11820,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":13336,"parent_record_offset":13312,"ancestor_record_offset":11820,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":12008,"parent_record_offset":11976,"ancestor_record_offset":11820,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3477,"record_offset":13312,"parent_record_offset":13288,"ancestor_record_offset":11820,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3477,"record_offset":13288,"parent_record_offset":13264,"ancestor_record_offset":11820,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":12852,"parent_record_offset":12824,"ancestor_record_offset":11820,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":12824,"parent_record_offset":12792,"ancestor_record_offset":11820,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":12792,"parent_record_offset":12764,"ancestor_record_offset":11820,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":12300,"parent_record_offset":12272,"ancestor_record_offset":11820,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":11820,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3477,"record_offset":12512,"parent_record_offset":12272,"ancestor_record_offset":11820,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3477,"record_offset":12208,"parent_record_offset":12180,"ancestor_record_offset":11820,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3477,"record_offset":13204,"parent_record_offset":13180,"ancestor_record_offset":11820,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3477,"record_offset":12180,"parent_record_offset":12152,"ancestor_record_offset":11820,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3477,"record_offset":12152,"parent_record_offset":12124,"ancestor_record_offset":11820,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":13228,"parent_record_offset":13204,"ancestor_record_offset":11820,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":12904,"parent_record_offset":12736,"ancestor_record_offset":11820,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":12764,"parent_record_offset":12736,"ancestor_record_offset":11820,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3477,"record_offset":12244,"parent_record_offset":12208,"ancestor_record_offset":11820,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3477,"record_offset":12672,"parent_record_offset":12648,"ancestor_record_offset":11820,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3477,"record_offset":12700,"parent_record_offset":12672,"ancestor_record_offset":11820,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3477,"record_offset":12648,"parent_record_offset":12624,"ancestor_record_offset":11820,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3477,"record_offset":12936,"parent_record_offset":12700,"ancestor_record_offset":11820,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3477,"record_offset":12736,"parent_record_offset":12700,"ancestor_record_offset":11820,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3477,"record_offset":12336,"parent_record_offset":12300,"ancestor_record_offset":11820,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3477,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":11820,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3477,"record_offset":12576,"parent_record_offset":12552,"ancestor_record_offset":11820,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3477,"record_offset":12552,"parent_record_offset":12208,"ancestor_record_offset":11820,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03477.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3477,"record_offset":12080,"parent_record_offset":12056,"ancestor_record_offset":11820,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03477.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_color_gates.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_color_gates.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_color_gates.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>", "bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >", "bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::components::WoodenGate>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates
// 信息 ID: 582c57bd804dabf68bfb02b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b0e16e5d203d307e8e30b115: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> > @ 0x149ba7c -> 0x1aade50 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,bevy_ecs::query::filter::With<country_core::components::WoodenGate> >,bevy_ecs::system::query::Query<ref$<country_core::components::WoodenGate>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::event::EventWriter<system_color::show_color_menu::ShowColorMenuCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)> @ 0x14acc71 -> 0x1aade50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates::closure$0
// 信息 ID: 56f009d7d8989a510651bdae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_color::ui_color_gates::ui_color_gates", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_color::ui_color_gates::ui_color_gates; module=3477, inline_record=5700; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates::closure$1
// 信息 ID: edc24f8d63f94c492d2e2757
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_color::ui_color_gates::ui_color_gates", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$1 <- core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$1 <- core::ptr::drop_in_place<system_color::ui_color_gates::ui_color_gates::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; module=3477, inline_record=1664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::ui_color_gates::ui_color_gates::closure$2
// 信息 ID: 347ef923967843dab25e85ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_color::ui_color_gates::ui_color_gates", "captures": "unknown"}
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$2 <- core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_color::ui_color_gates::ui_color_gates::closure_env$2 <- core::ptr::drop_in_place<system_color::ui_color_gates::ui_color_gates::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_color::ui_color_gates::ui_color_gates::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x1aaddd3 -> 0x1aaecb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

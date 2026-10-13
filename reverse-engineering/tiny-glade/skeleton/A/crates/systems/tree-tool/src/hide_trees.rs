// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/tree-tool/src/hide_trees.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2040cf2d2efddd0f0f8936d46195fce79f1dbbf5599866a38a323dcc5749b190
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: hide_trees.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3384, 4216
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
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_tree::hide_trees::hide_trees
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3384,"record_offset":248,"rva":27862192,"va":5396571312}
// 编译实例 RVAs: 0x1a924b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23822761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e8577e94fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5396572452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21858935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e88975b2fe","size":5,"soft_fail":false,"target_va":5374712880,"va":5396571810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d69f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20694488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e8283ac4fe","size":5,"soft_fail":false,"target_va":5375879664,"va":5396574147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141024ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10934836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e8cc2559ff","size":5,"soft_fail":false,"target_va":5385637584,"va":5396572415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dbbba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3314509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e84d933200","size":5,"soft_fail":false,"target_va":5399886752,"va":5396572238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: hide_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7020870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e846216b00","size":5,"soft_fail":false,"target_va":5403593360,"va":5396572485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t6404098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e802b86100","size":5,"soft_fail":false,"target_va":5402978144,"va":5396574041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-25267449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e807737efe","size":5,"soft_fail":false,"target_va":5371304880,"va":5396572324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19046911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e8015eddfe","size":5,"soft_fail":false,"target_va":5377527120,"va":5396574026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::get_unchecked_manual
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6725717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e8ab5f99ff","size":5,"soft_fail":false,"target_va":5389846320,"va":5396572032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21606364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e82450b6fe","size":5,"soft_fail":false,"target_va":5374966368,"va":5396572727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::placement_data::PlacementDataSsbo::position
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22292138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e856d9abfe","size":5,"soft_fail":false,"target_va":5374280016,"va":5396572149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::contains_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18603431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e85922e4fe","size":5,"soft_fail":false,"target_va":5377970048,"va":5396573474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18604330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e8d61ee4fe","size":5,"soft_fail":false,"target_va":5377967856,"va":5396572181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::with_expanded_diameter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18603439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e85122e4fe","size":5,"soft_fail":false,"target_va":5377970016,"va":5396573450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18613826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e8bef9e3fe","size":5,"soft_fail":false,"target_va":5377959584,"va":5396573405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::with_expanded_size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18613546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396571312,"raw_hex":"e8d6fae3fe","size":5,"soft_fail":false,"target_va":5377959840,"va":5396573381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3384,"record_offset":3036,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3384,"record_offset":2412,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3384,"record_offset":2040,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3384,"record_offset":404,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3384,"record_offset":2152,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3384,"record_offset":948,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3384,"record_offset":2012,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14710,"source_line":963,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3384,"record_offset":2468,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3384,"record_offset":1260,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$162::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":3384,"record_offset":4964,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":27674,"source_line":35,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3384,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":248,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3384,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":248,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3384,"record_offset":3112,"parent_record_offset":3072,"ancestor_record_offset":248,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3384,"record_offset":2792,"parent_record_offset":2756,"ancestor_record_offset":248,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3384,"record_offset":3072,"parent_record_offset":3036,"ancestor_record_offset":248,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3384,"record_offset":2340,"parent_record_offset":2308,"ancestor_record_offset":248,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3384,"record_offset":2756,"parent_record_offset":2728,"ancestor_record_offset":248,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3384,"record_offset":2728,"parent_record_offset":2700,"ancestor_record_offset":248,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3384,"record_offset":2912,"parent_record_offset":2728,"ancestor_record_offset":248,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3384,"record_offset":2664,"parent_record_offset":2636,"ancestor_record_offset":248,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3384,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":248,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3384,"record_offset":2580,"parent_record_offset":2552,"ancestor_record_offset":248,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3384,"record_offset":2608,"parent_record_offset":2580,"ancestor_record_offset":248,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3384,"record_offset":2552,"parent_record_offset":2524,"ancestor_record_offset":248,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3384,"record_offset":2524,"parent_record_offset":2496,"ancestor_record_offset":248,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3384,"record_offset":2496,"parent_record_offset":2468,"ancestor_record_offset":248,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3384,"record_offset":2308,"parent_record_offset":2268,"ancestor_record_offset":248,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3384,"record_offset":2268,"parent_record_offset":2232,"ancestor_record_offset":248,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3384,"record_offset":2700,"parent_record_offset":2664,"ancestor_record_offset":248,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3384,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":248,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3384,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3384,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":248,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3384,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":248,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3384,"record_offset":2232,"parent_record_offset":2152,"ancestor_record_offset":248,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3384,"record_offset":468,"parent_record_offset":428,"ancestor_record_offset":248,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3384,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":248,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3384,"record_offset":428,"parent_record_offset":404,"ancestor_record_offset":248,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3384,"record_offset":1632,"parent_record_offset":1600,"ancestor_record_offset":248,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3384,"record_offset":1696,"parent_record_offset":1660,"ancestor_record_offset":248,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3384,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":248,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3384,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":248,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3384,"record_offset":1000,"parent_record_offset":972,"ancestor_record_offset":248,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3384,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":248,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3384,"record_offset":540,"parent_record_offset":492,"ancestor_record_offset":248,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3384,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":248,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3384,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":248,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3384,"record_offset":1976,"parent_record_offset":1944,"ancestor_record_offset":248,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3384,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":248,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3384,"record_offset":1944,"parent_record_offset":1260,"ancestor_record_offset":248,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3384,"record_offset":1080,"parent_record_offset":1000,"ancestor_record_offset":248,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3384,"record_offset":1816,"parent_record_offset":1388,"ancestor_record_offset":248,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3384,"record_offset":2948,"parent_record_offset":2664,"ancestor_record_offset":248,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3384,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3384,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":248,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3384,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":248,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3384,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":248,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3384,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":248,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3384,"record_offset":2372,"parent_record_offset":2268,"ancestor_record_offset":248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3384,"record_offset":604,"parent_record_offset":492,"ancestor_record_offset":248,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3384,"record_offset":792,"parent_record_offset":492,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3384,"record_offset":664,"parent_record_offset":640,"ancestor_record_offset":248,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3384,"record_offset":640,"parent_record_offset":604,"ancestor_record_offset":248,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3384,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3384,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":248,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3384,"record_offset":1156,"parent_record_offset":1104,"ancestor_record_offset":248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3384,"record_offset":1320,"parent_record_offset":1288,"ancestor_record_offset":248,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3384,"record_offset":1732,"parent_record_offset":1388,"ancestor_record_offset":248,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3384,"record_offset":1388,"parent_record_offset":1352,"ancestor_record_offset":248,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3384,"record_offset":1472,"parent_record_offset":1436,"ancestor_record_offset":248,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3384,"record_offset":1504,"parent_record_offset":1472,"ancestor_record_offset":248,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3384,"record_offset":1564,"parent_record_offset":1436,"ancestor_record_offset":248,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3384,"record_offset":1436,"parent_record_offset":1388,"ancestor_record_offset":248,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3384,"record_offset":1352,"parent_record_offset":1320,"ancestor_record_offset":248,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3384,"record_offset":1600,"parent_record_offset":1388,"ancestor_record_offset":248,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3384,"record_offset":1660,"parent_record_offset":1600,"ancestor_record_offset":248,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3384,"record_offset":1288,"parent_record_offset":1260,"ancestor_record_offset":248,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03384.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_tree::hide_trees::hide_trees::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D2E4
// 首条位置: {"module":4216,"inlinee":119524}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":4216,"record_offset":2920,"parent_record_offset":2884,"ancestor_record_offset":2884,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":4216,"record_offset":2988,"parent_record_offset":2884,"ancestor_record_offset":2884,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_direct] utils::distance_segment_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4216,"record_offset":3016,"parent_record_offset":2884,"ancestor_record_offset":2884,"inlinee":22272,"source_line":166,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":4216,"record_offset":3360,"parent_record_offset":3044,"ancestor_record_offset":2884,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":4216,"record_offset":3328,"parent_record_offset":3072,"ancestor_record_offset":2884,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4216,"record_offset":3468,"parent_record_offset":3440,"ancestor_record_offset":2884,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4216,"record_offset":3496,"parent_record_offset":3440,"ancestor_record_offset":2884,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":4216,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":2884,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":4216,"record_offset":2956,"parent_record_offset":2920,"ancestor_record_offset":2884,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4216,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":2884,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4216,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":2884,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4216,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":2884,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4216,"record_offset":3104,"parent_record_offset":3072,"ancestor_record_offset":2884,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":4216,"record_offset":3440,"parent_record_offset":3416,"ancestor_record_offset":2884,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":4216,"record_offset":3676,"parent_record_offset":3644,"ancestor_record_offset":2884,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4216,"record_offset":3216,"parent_record_offset":3072,"ancestor_record_offset":2884,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4216,"record_offset":3596,"parent_record_offset":3016,"ancestor_record_offset":2884,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4216,"record_offset":3392,"parent_record_offset":3044,"ancestor_record_offset":2884,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4216,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":2884,"inlinee":21612,"source_line":181,"archive":"_pdb/A/modules/04216.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":4216,"record_offset":3072,"parent_record_offset":3044,"ancestor_record_offset":2884,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/04216.jsonl.gz"}
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
// 完整结构及原始类型记录: hide_trees.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: hide_trees.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: hide_trees.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesinAreaCmd>", "bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>

// ------------------------------------------------------------------
// 函数: system_tree::hide_trees::hide_trees
// 信息 ID: 13b300cf1370614200f9d6ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 67ca0a47bb43607b898227ac: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> > @ 0x1414fb4 -> 0x1a924b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::EventReader<country_core::resources::tree_storage::HideTreesNextToWallCmd>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::event::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)> @ 0x142cad9 -> 0x1a924b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_tree::hide_trees::hide_trees::closure$0
// 信息 ID: d33b569fe425c549de910219
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_tree::hide_trees::hide_trees", "captures": "unknown"}
// [闭包环境类型引用] system_tree::hide_trees::hide_trees::closure_env$0 <- country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0>; module=4216, inline_record=2884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

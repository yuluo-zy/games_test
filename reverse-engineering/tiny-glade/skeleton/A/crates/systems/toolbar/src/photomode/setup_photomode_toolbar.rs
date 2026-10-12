// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/photomode/setup_photomode_toolbar.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: aab607760dd67f4ca1f12fa357d2133bfa7bb5ebbf7b7bfed67296942553a6f2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: setup_photomode_toolbar.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 4303, 4304
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4304,"record_offset":244,"rva":31459792,"va":5400168912}
// 编译实例 RVAs: 0x1e009d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=25
//     target_aliases=1 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27419323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e8459d5dfe","size":5,"soft_fail":false,"target_va":5372749696,"va":5400169014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27425088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e8c0865dfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5400174795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5d1a0
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20599426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e87eadc5fe","size":5,"soft_fail":false,"target_va":5379576224,"va":5400175645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5f230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20590217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e877d1c5fe","size":5,"soft_fail":false,"target_va":5379584560,"va":5400174772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5f5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20592386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e8fec8c5fe","size":5,"soft_fail":false,"target_va":5379585472,"va":5400177853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a65bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20563935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e82138c6fe","size":5,"soft_fail":false,"target_va":5379611632,"va":5400175562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a65db0
//     owner=unresolved_namespace; count=16
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20558479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e8714dc6fe","size":5,"soft_fail":false,"target_va":5379612080,"va":5400170554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dff7c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e879d7ffff","size":5,"soft_fail":false,"target_va":5400164288,"va":5400174658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e004c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup_photomode_toolbar.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e8c7e3ffff","size":5,"soft_fail":false,"target_va":5400167616,"va":5400174836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=28
//     evidence={"assembly":"\tcallq\t2803200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e800c62a00","size":5,"soft_fail":false,"target_va":5402978144,"va":5400174939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_contextual_menu
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e8c9a5ffff","size":5,"soft_fail":false,"target_va":5400151632,"va":5400174722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::setup_screen_and_toolbar_nodes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400168912,"raw_hex":"e8cbb6ffff","size":5,"soft_fail":false,"target_va":5400150256,"va":5400168992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4304,"record_offset":480,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4304,"record_offset":6244,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4304,"record_offset":5352,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":4304,"record_offset":5408,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":4304,"record_offset":424,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4304,"record_offset":2216,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::button
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":4304,"record_offset":780,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":120065,"source_line":82,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::row
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":4304,"record_offset":1820,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":108701,"source_line":124,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4304,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":244,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4304,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":244,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4304,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":244,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4304,"record_offset":844,"parent_record_offset":816,"ancestor_record_offset":244,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4304,"record_offset":512,"parent_record_offset":480,"ancestor_record_offset":244,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4304,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":244,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4304,"record_offset":816,"parent_record_offset":780,"ancestor_record_offset":244,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4304,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":244,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":4304,"record_offset":5648,"parent_record_offset":5616,"ancestor_record_offset":244,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":4304,"record_offset":5616,"parent_record_offset":5588,"ancestor_record_offset":244,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":4304,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":244,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":4304,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":244,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":4304,"record_offset":448,"parent_record_offset":424,"ancestor_record_offset":244,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":4304,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":244,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":4304,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":244,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4304,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":244,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4304,"record_offset":5816,"parent_record_offset":5616,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4304,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":244,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":4304,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":244,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":4304,"record_offset":5444,"parent_record_offset":5408,"ancestor_record_offset":244,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4304,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":244,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04304.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D4FF
// 首条位置: {"module":4303,"inlinee":120063}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":11692,"parent_record_offset":11484,"ancestor_record_offset":11484,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":12004,"parent_record_offset":11976,"ancestor_record_offset":11484,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":11976,"parent_record_offset":11940,"ancestor_record_offset":11484,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":12108,"parent_record_offset":12084,"ancestor_record_offset":11484,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":11940,"parent_record_offset":11908,"ancestor_record_offset":11484,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":12084,"parent_record_offset":11864,"ancestor_record_offset":11484,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":12056,"parent_record_offset":11864,"ancestor_record_offset":11484,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":11908,"parent_record_offset":11864,"ancestor_record_offset":11484,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":14136,"parent_record_offset":13772,"ancestor_record_offset":11484,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":11772,"parent_record_offset":11732,"ancestor_record_offset":11484,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":11816,"parent_record_offset":11772,"ancestor_record_offset":11484,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":11732,"parent_record_offset":11692,"ancestor_record_offset":11484,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":4303,"record_offset":12228,"parent_record_offset":11816,"ancestor_record_offset":11484,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=22
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":4303,"record_offset":11864,"parent_record_offset":11816,"ancestor_record_offset":11484,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4303,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":11484,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4303,"record_offset":12140,"parent_record_offset":11864,"ancestor_record_offset":11484,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4303,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":11484,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 251
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D500
// 首条位置: {"module":4303,"inlinee":120064}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":24084,"parent_record_offset":24048,"ancestor_record_offset":24048,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":24388,"parent_record_offset":24360,"ancestor_record_offset":24048,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":24360,"parent_record_offset":24324,"ancestor_record_offset":24048,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":24492,"parent_record_offset":24468,"ancestor_record_offset":24048,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":24324,"parent_record_offset":24292,"ancestor_record_offset":24048,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":24468,"parent_record_offset":24244,"ancestor_record_offset":24048,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":24440,"parent_record_offset":24244,"ancestor_record_offset":24048,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":24292,"parent_record_offset":24244,"ancestor_record_offset":24048,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":24608,"parent_record_offset":24244,"ancestor_record_offset":24048,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":24156,"parent_record_offset":24120,"ancestor_record_offset":24048,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":24196,"parent_record_offset":24156,"ancestor_record_offset":24048,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":24120,"parent_record_offset":24084,"ancestor_record_offset":24048,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":4303,"record_offset":24640,"parent_record_offset":24196,"ancestor_record_offset":24048,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":4303,"record_offset":24244,"parent_record_offset":24196,"ancestor_record_offset":24048,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4303,"record_offset":24572,"parent_record_offset":24548,"ancestor_record_offset":24048,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4303,"record_offset":24524,"parent_record_offset":24244,"ancestor_record_offset":24048,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4303,"record_offset":24548,"parent_record_offset":24524,"ancestor_record_offset":24048,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/04303.jsonl.gz"}
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
// 完整结构及原始类型记录: setup_photomode_toolbar.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: setup_photomode_toolbar.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: setup_photomode_toolbar.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar
// 信息 ID: 19d5d1bb4b7703dcdf9fe1ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::toolbar_startup::toolbar_startup @ 0x1b55c86 -> 0x1e009d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure$0
// 信息 ID: a60f9a53804c59a9d0ec3173
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar", "captures": "unknown"}
// [闭包环境类型引用] toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$0 <- toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$0>; module=4303, inline_record=11484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure$1
// 信息 ID: a1caac55c54cf2ac8801c7ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar", "captures": "unknown"}
// [闭包环境类型引用] toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$1 <- toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_invisible_category_button<toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_invisible_category_button<toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar::closure_env$1>; module=4303, inline_record=24048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

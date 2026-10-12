// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: bcc250d31f4e1a85fd28e3aae06f48cb3107ed19627fdb3dc3c2b156d13da531
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: adjust_wall_ui_mode.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 453, 463, 1302, 1308, 1321
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1302,"record_offset":42560,"rva":13768128,"va":5382477248}
// 编译实例 RVAs: 0xd215c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10828321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382477248,"raw_hex":"e8dfc55aff","size":5,"soft_fail":false,"target_va":5371650384,"va":5382478700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9727726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382477248,"raw_hex":"e812916bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382477417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9728613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382477248,"raw_hex":"e89b8d6bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382478320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405caa20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7695774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382477248,"raw_hex":"e862928aff","size":5,"soft_fail":false,"target_va":5374781984,"va":5382477753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21115633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382477248,"raw_hex":"e8f1324201","size":5,"soft_fail":false,"target_va":5403593360,"va":5382477722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t20499782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382477248,"raw_hex":"e846cd3801","size":5,"soft_fail":false,"target_va":5402978144,"va":5382478357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4947504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382477248,"raw_hex":"e8d081b4ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5382477387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4951270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382477248,"raw_hex":"e81a73b4ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382478385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":42768,"parent_record_offset":42560,"ancestor_record_offset":42560,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1302,"record_offset":42848,"parent_record_offset":42560,"ancestor_record_offset":42560,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":45344,"parent_record_offset":42560,"ancestor_record_offset":42560,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":45400,"parent_record_offset":42560,"ancestor_record_offset":42560,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1302,"record_offset":45252,"parent_record_offset":42560,"ancestor_record_offset":42560,"inlinee":23070,"source_line":243,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":43064,"parent_record_offset":42560,"ancestor_record_offset":42560,"inlinee":22967,"source_line":102,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":1302,"record_offset":45372,"parent_record_offset":42560,"ancestor_record_offset":42560,"inlinee":22969,"source_line":24,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":1302,"record_offset":43852,"parent_record_offset":42560,"ancestor_record_offset":42560,"inlinee":26432,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":42960,"parent_record_offset":42936,"ancestor_record_offset":42560,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":42984,"parent_record_offset":42960,"ancestor_record_offset":42560,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":45552,"parent_record_offset":45528,"ancestor_record_offset":42560,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":42904,"parent_record_offset":42876,"ancestor_record_offset":42560,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":42936,"parent_record_offset":42904,"ancestor_record_offset":42560,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":45528,"parent_record_offset":45472,"ancestor_record_offset":42560,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1302,"record_offset":42876,"parent_record_offset":42848,"ancestor_record_offset":42560,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":42816,"parent_record_offset":42792,"ancestor_record_offset":42560,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":45500,"parent_record_offset":45472,"ancestor_record_offset":42560,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":43416,"parent_record_offset":43388,"ancestor_record_offset":42560,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":43388,"parent_record_offset":43360,"ancestor_record_offset":42560,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":43572,"parent_record_offset":43388,"ancestor_record_offset":42560,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":45472,"parent_record_offset":45448,"ancestor_record_offset":42560,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":43320,"parent_record_offset":43284,"ancestor_record_offset":42560,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":43284,"parent_record_offset":43248,"ancestor_record_offset":42560,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":43212,"parent_record_offset":43176,"ancestor_record_offset":42560,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":43248,"parent_record_offset":43212,"ancestor_record_offset":42560,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":43176,"parent_record_offset":43140,"ancestor_record_offset":42560,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1302,"record_offset":43140,"parent_record_offset":43104,"ancestor_record_offset":42560,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1302,"record_offset":43104,"parent_record_offset":43064,"ancestor_record_offset":42560,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1302,"record_offset":43720,"parent_record_offset":43696,"ancestor_record_offset":42560,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":44592,"parent_record_offset":44560,"ancestor_record_offset":42560,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":44156,"parent_record_offset":44100,"ancestor_record_offset":42560,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":43360,"parent_record_offset":43320,"ancestor_record_offset":42560,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":44004,"parent_record_offset":43956,"ancestor_record_offset":42560,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":43924,"parent_record_offset":43900,"ancestor_record_offset":42560,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":45008,"parent_record_offset":44976,"ancestor_record_offset":42560,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1302,"record_offset":45084,"parent_record_offset":45008,"ancestor_record_offset":42560,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1302,"record_offset":44560,"parent_record_offset":44100,"ancestor_record_offset":42560,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1302,"record_offset":44052,"parent_record_offset":44004,"ancestor_record_offset":42560,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1302,"record_offset":44100,"parent_record_offset":44052,"ancestor_record_offset":42560,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1302,"record_offset":44340,"parent_record_offset":44288,"ancestor_record_offset":42560,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1302,"record_offset":44364,"parent_record_offset":44340,"ancestor_record_offset":42560,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1302,"record_offset":43636,"parent_record_offset":43612,"ancestor_record_offset":42560,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1302,"record_offset":44656,"parent_record_offset":44624,"ancestor_record_offset":42560,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1302,"record_offset":44832,"parent_record_offset":44720,"ancestor_record_offset":42560,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1302,"record_offset":44688,"parent_record_offset":44656,"ancestor_record_offset":42560,"inlinee":6055,"source_line":99,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1302,"record_offset":44720,"parent_record_offset":44688,"ancestor_record_offset":42560,"inlinee":6056,"source_line":276,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1302,"record_offset":45124,"parent_record_offset":44720,"ancestor_record_offset":42560,"inlinee":6062,"source_line":268,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1302,"record_offset":44792,"parent_record_offset":44760,"ancestor_record_offset":42560,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1302,"record_offset":44760,"parent_record_offset":44720,"ancestor_record_offset":42560,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":43956,"parent_record_offset":43852,"ancestor_record_offset":42560,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":44624,"parent_record_offset":44592,"ancestor_record_offset":42560,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":44976,"parent_record_offset":44832,"ancestor_record_offset":42560,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":43900,"parent_record_offset":43852,"ancestor_record_offset":42560,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1302,"record_offset":46060,"parent_record_offset":46032,"ancestor_record_offset":42560,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1302,"record_offset":44932,"parent_record_offset":44900,"ancestor_record_offset":42560,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$50::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1302,"record_offset":44900,"parent_record_offset":44868,"ancestor_record_offset":42560,"inlinee":59719,"source_line":35,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":42792,"parent_record_offset":42768,"ancestor_record_offset":42560,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1302,"record_offset":43612,"parent_record_offset":43320,"ancestor_record_offset":42560,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":43696,"parent_record_offset":43064,"ancestor_record_offset":42560,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1302,"record_offset":45172,"parent_record_offset":45148,"ancestor_record_offset":42560,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":43012,"parent_record_offset":42984,"ancestor_record_offset":42560,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":45048,"parent_record_offset":45008,"ancestor_record_offset":42560,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1302,"record_offset":45148,"parent_record_offset":45124,"ancestor_record_offset":42560,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1302,"record_offset":43756,"parent_record_offset":43720,"ancestor_record_offset":42560,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1302,"record_offset":43780,"parent_record_offset":43756,"ancestor_record_offset":42560,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1302,"record_offset":43812,"parent_record_offset":43720,"ancestor_record_offset":42560,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1302,"record_offset":45276,"parent_record_offset":45252,"ancestor_record_offset":42560,"inlinee":22961,"source_line":152,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1302,"record_offset":45308,"parent_record_offset":45276,"ancestor_record_offset":42560,"inlinee":22151,"source_line":10,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":1302,"record_offset":44868,"parent_record_offset":44832,"ancestor_record_offset":42560,"inlinee":60342,"source_line":159,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1302,"record_offset":39448,"rva":13766992,"va":5382476112}
// 编译实例 RVAs: 0xd21150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9726581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382476112,"raw_hex":"e88b956bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382476272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140685f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6927571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382476112,"raw_hex":"e82d4b96ff","size":5,"soft_fail":false,"target_va":5375549312,"va":5382476878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20500994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382476112,"raw_hex":"e802d23801","size":5,"soft_fail":false,"target_va":5402978144,"va":5382477145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4946842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382476112,"raw_hex":"e86684b4ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5382476725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4950017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382476112,"raw_hex":"e8ff77b4ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382477132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::ui::resources::widget_params::WidgetVolumeShape> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7260937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382476112,"raw_hex":"e8f73491ff","size":5,"soft_fail":false,"target_va":5375216304,"va":5382477236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":42432,"parent_record_offset":39448,"ancestor_record_offset":39448,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1302,"record_offset":42404,"parent_record_offset":39448,"ancestor_record_offset":39448,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":1302,"record_offset":42516,"parent_record_offset":39448,"ancestor_record_offset":39448,"inlinee":24894,"source_line":87,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":1302,"record_offset":39636,"parent_record_offset":39448,"ancestor_record_offset":39448,"inlinee":26432,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":40384,"parent_record_offset":40360,"ancestor_record_offset":39448,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":40408,"parent_record_offset":40384,"ancestor_record_offset":39448,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":40360,"parent_record_offset":40268,"ancestor_record_offset":39448,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":42480,"parent_record_offset":42456,"ancestor_record_offset":39448,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":39944,"parent_record_offset":39912,"ancestor_record_offset":39448,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":39912,"parent_record_offset":39880,"ancestor_record_offset":39448,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":40480,"parent_record_offset":40240,"ancestor_record_offset":39448,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":40548,"parent_record_offset":40520,"ancestor_record_offset":39448,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":39848,"parent_record_offset":39816,"ancestor_record_offset":39448,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":39880,"parent_record_offset":39848,"ancestor_record_offset":39448,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":39752,"parent_record_offset":39676,"ancestor_record_offset":39448,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":39720,"parent_record_offset":39676,"ancestor_record_offset":39448,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":40948,"parent_record_offset":40916,"ancestor_record_offset":39448,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1302,"record_offset":41024,"parent_record_offset":40948,"ancestor_record_offset":39448,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1302,"record_offset":40520,"parent_record_offset":40144,"ancestor_record_offset":39448,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1302,"record_offset":39784,"parent_record_offset":39752,"ancestor_record_offset":39448,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1302,"record_offset":39816,"parent_record_offset":39784,"ancestor_record_offset":39448,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1302,"record_offset":40304,"parent_record_offset":40268,"ancestor_record_offset":39448,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1302,"record_offset":40328,"parent_record_offset":40304,"ancestor_record_offset":39448,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1302,"record_offset":40604,"parent_record_offset":40576,"ancestor_record_offset":39448,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1302,"record_offset":40772,"parent_record_offset":40660,"ancestor_record_offset":39448,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1302,"record_offset":40632,"parent_record_offset":40604,"ancestor_record_offset":39448,"inlinee":6055,"source_line":99,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1302,"record_offset":40660,"parent_record_offset":40632,"ancestor_record_offset":39448,"inlinee":6056,"source_line":276,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1302,"record_offset":41064,"parent_record_offset":40660,"ancestor_record_offset":39448,"inlinee":6062,"source_line":268,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1302,"record_offset":40732,"parent_record_offset":40700,"ancestor_record_offset":39448,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1302,"record_offset":40700,"parent_record_offset":40660,"ancestor_record_offset":39448,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":40048,"parent_record_offset":40016,"ancestor_record_offset":39448,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":40576,"parent_record_offset":40548,"ancestor_record_offset":39448,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":40916,"parent_record_offset":40772,"ancestor_record_offset":39448,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":39676,"parent_record_offset":39636,"ancestor_record_offset":39448,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1302,"record_offset":40872,"parent_record_offset":40840,"ancestor_record_offset":39448,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$50::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1302,"record_offset":40840,"parent_record_offset":40808,"ancestor_record_offset":39448,"inlinee":59719,"source_line":35,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":42456,"parent_record_offset":42432,"ancestor_record_offset":39448,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1302,"record_offset":41112,"parent_record_offset":41088,"ancestor_record_offset":39448,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":40436,"parent_record_offset":40408,"ancestor_record_offset":39448,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":40988,"parent_record_offset":40948,"ancestor_record_offset":39448,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1302,"record_offset":41088,"parent_record_offset":41064,"ancestor_record_offset":39448,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":1302,"record_offset":40808,"parent_record_offset":40772,"ancestor_record_offset":39448,"inlinee":60342,"source_line":159,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x658B
// 首条位置: {"module":453,"inlinee":25995}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1302,"record_offset":47924,"rva":13769872,"va":5382478992}
// 编译实例 RVAs: 0xd21c90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9729368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382478992,"raw_hex":"e8a88a6bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382479059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9729984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382478992,"raw_hex":"e840886bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382479691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7139333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382478992,"raw_hex":"e8fb0f93ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5382479760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21114182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382478992,"raw_hex":"e8462d4201","size":5,"soft_fail":false,"target_va":5403593360,"va":5382479173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20498536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382478992,"raw_hex":"e868c83801","size":5,"soft_fail":false,"target_va":5402978144,"va":5382479603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20439623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382478992,"raw_hex":"e847e23701","size":5,"soft_fail":false,"target_va":5402919072,"va":5382479444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4952510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382478992,"raw_hex":"e8426eb4ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382479625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5232936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382478992,"raw_hex":"e8d826b0ff","size":5,"soft_fail":false,"target_va":5377246528,"va":5382479459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7122507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382478992,"raw_hex":"e8b55193ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5382479398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":48132,"parent_record_offset":47924,"ancestor_record_offset":47924,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":49940,"parent_record_offset":47924,"ancestor_record_offset":47924,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":48280,"parent_record_offset":47924,"ancestor_record_offset":47924,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":49384,"parent_record_offset":47924,"ancestor_record_offset":47924,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":50024,"parent_record_offset":47924,"ancestor_record_offset":47924,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::disable_default_animation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":49356,"parent_record_offset":47924,"ancestor_record_offset":47924,"inlinee":26725,"source_line":196,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":48780,"parent_record_offset":47924,"ancestor_record_offset":47924,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":48188,"parent_record_offset":48164,"ancestor_record_offset":47924,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":48212,"parent_record_offset":48188,"ancestor_record_offset":47924,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":50096,"parent_record_offset":50072,"ancestor_record_offset":47924,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":48164,"parent_record_offset":48132,"ancestor_record_offset":47924,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":50072,"parent_record_offset":50048,"ancestor_record_offset":47924,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1302,"record_offset":50048,"parent_record_offset":50024,"ancestor_record_offset":47924,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":49612,"parent_record_offset":49584,"ancestor_record_offset":47924,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":49584,"parent_record_offset":49552,"ancestor_record_offset":47924,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":49552,"parent_record_offset":49524,"ancestor_record_offset":47924,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":48456,"parent_record_offset":48428,"ancestor_record_offset":47924,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":48428,"parent_record_offset":48400,"ancestor_record_offset":47924,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":48668,"parent_record_offset":48428,"ancestor_record_offset":47924,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":48364,"parent_record_offset":48336,"ancestor_record_offset":47924,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":49964,"parent_record_offset":49940,"ancestor_record_offset":47924,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":48336,"parent_record_offset":48308,"ancestor_record_offset":47924,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":48908,"parent_record_offset":48880,"ancestor_record_offset":47924,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":48308,"parent_record_offset":48280,"ancestor_record_offset":47924,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":48880,"parent_record_offset":48852,"ancestor_record_offset":47924,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1302,"record_offset":48852,"parent_record_offset":48824,"ancestor_record_offset":47924,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1302,"record_offset":48824,"parent_record_offset":48780,"ancestor_record_offset":47924,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":49988,"parent_record_offset":49964,"ancestor_record_offset":47924,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":49664,"parent_record_offset":49496,"ancestor_record_offset":47924,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":49524,"parent_record_offset":49496,"ancestor_record_offset":47924,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":48400,"parent_record_offset":48364,"ancestor_record_offset":47924,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":49432,"parent_record_offset":49408,"ancestor_record_offset":47924,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":49460,"parent_record_offset":49432,"ancestor_record_offset":47924,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":49408,"parent_record_offset":49384,"ancestor_record_offset":47924,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1302,"record_offset":49696,"parent_record_offset":49460,"ancestor_record_offset":47924,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1302,"record_offset":49496,"parent_record_offset":49460,"ancestor_record_offset":47924,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1302,"record_offset":48492,"parent_record_offset":48456,"ancestor_record_offset":47924,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1302,"record_offset":48516,"parent_record_offset":48492,"ancestor_record_offset":47924,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1302,"record_offset":48732,"parent_record_offset":48708,"ancestor_record_offset":47924,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1302,"record_offset":48708,"parent_record_offset":48364,"ancestor_record_offset":47924,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":48236,"parent_record_offset":48212,"ancestor_record_offset":47924,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 137
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1302,"record_offset":50140,"rva":13770672,"va":5382479792}
// 编译实例 RVAs: 0xd21fb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9730164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382479792,"raw_hex":"e88c876bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382479855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9730748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382479792,"raw_hex":"e844856bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382480455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: adjust_wall_ui_mode.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7139969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382479792,"raw_hex":"e87f0d93ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5382480396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20497799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382479792,"raw_hex":"e887c53801","size":5,"soft_fail":false,"target_va":5402978144,"va":5382480340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20438937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382479792,"raw_hex":"e899df3701","size":5,"soft_fail":false,"target_va":5402919072,"va":5382480130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1880147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382479792,"raw_hex":"e853b01c00","size":5,"soft_fail":false,"target_va":5384360432,"va":5382480280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5233622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382479792,"raw_hex":"e82a24b0ff","size":5,"soft_fail":false,"target_va":5377246528,"va":5382480145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7123193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382479792,"raw_hex":"e8074f93ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5382480084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":50348,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":51676,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":51092,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":51760,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::disable_default_animation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":51064,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":26725,"source_line":196,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":50496,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::adjust_wall_height::signifier_animation::WallHeightSignifier::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/signifier_animation.rs
//     evidence={"module":1302,"record_offset":51648,"parent_record_offset":50140,"ancestor_record_offset":50140,"inlinee":26452,"source_line":26,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":50404,"parent_record_offset":50380,"ancestor_record_offset":50140,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":50428,"parent_record_offset":50404,"ancestor_record_offset":50140,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":51832,"parent_record_offset":51808,"ancestor_record_offset":50140,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":50380,"parent_record_offset":50348,"ancestor_record_offset":50140,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":51808,"parent_record_offset":51784,"ancestor_record_offset":50140,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1302,"record_offset":51784,"parent_record_offset":51760,"ancestor_record_offset":50140,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":51320,"parent_record_offset":51292,"ancestor_record_offset":50140,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":51292,"parent_record_offset":51260,"ancestor_record_offset":50140,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":51260,"parent_record_offset":51232,"ancestor_record_offset":50140,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":50796,"parent_record_offset":50768,"ancestor_record_offset":50140,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":50768,"parent_record_offset":50740,"ancestor_record_offset":50140,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":50940,"parent_record_offset":50768,"ancestor_record_offset":50140,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":50704,"parent_record_offset":50676,"ancestor_record_offset":50140,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":51700,"parent_record_offset":51676,"ancestor_record_offset":50140,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":50676,"parent_record_offset":50648,"ancestor_record_offset":50140,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":50620,"parent_record_offset":50592,"ancestor_record_offset":50140,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":50648,"parent_record_offset":50620,"ancestor_record_offset":50140,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":50592,"parent_record_offset":50564,"ancestor_record_offset":50140,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1302,"record_offset":50564,"parent_record_offset":50536,"ancestor_record_offset":50140,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1302,"record_offset":50536,"parent_record_offset":50496,"ancestor_record_offset":50140,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":51724,"parent_record_offset":51700,"ancestor_record_offset":50140,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":51372,"parent_record_offset":51204,"ancestor_record_offset":50140,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":51232,"parent_record_offset":51204,"ancestor_record_offset":50140,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":50740,"parent_record_offset":50704,"ancestor_record_offset":50140,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":51140,"parent_record_offset":51116,"ancestor_record_offset":50140,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":51168,"parent_record_offset":51140,"ancestor_record_offset":50140,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":51116,"parent_record_offset":51092,"ancestor_record_offset":50140,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1302,"record_offset":51404,"parent_record_offset":51168,"ancestor_record_offset":50140,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1302,"record_offset":51204,"parent_record_offset":51168,"ancestor_record_offset":50140,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1302,"record_offset":51000,"parent_record_offset":50976,"ancestor_record_offset":50140,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1302,"record_offset":50976,"parent_record_offset":50704,"ancestor_record_offset":50140,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":50452,"parent_record_offset":50428,"ancestor_record_offset":50140,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6740
// 首条位置: {"module":463,"inlinee":26432}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":463,"record_offset":15532,"parent_record_offset":15228,"ancestor_record_offset":15228,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":463,"record_offset":15336,"parent_record_offset":15228,"ancestor_record_offset":15228,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":463,"record_offset":15252,"parent_record_offset":15228,"ancestor_record_offset":15228,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":40384,"parent_record_offset":40360,"ancestor_record_offset":40016,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":40408,"parent_record_offset":40384,"ancestor_record_offset":40016,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":40360,"parent_record_offset":40268,"ancestor_record_offset":40016,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":39944,"parent_record_offset":39912,"ancestor_record_offset":39636,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":39912,"parent_record_offset":39880,"ancestor_record_offset":39636,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":40480,"parent_record_offset":40240,"ancestor_record_offset":40016,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":40548,"parent_record_offset":40520,"ancestor_record_offset":40016,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":39848,"parent_record_offset":39816,"ancestor_record_offset":39636,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":39880,"parent_record_offset":39848,"ancestor_record_offset":39636,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":15556,"parent_record_offset":15532,"ancestor_record_offset":15228,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":15228,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":40948,"parent_record_offset":40916,"ancestor_record_offset":40016,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1302,"record_offset":41024,"parent_record_offset":40948,"ancestor_record_offset":40016,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1302,"record_offset":40520,"parent_record_offset":40144,"ancestor_record_offset":40016,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1302,"record_offset":39784,"parent_record_offset":39752,"ancestor_record_offset":39636,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1302,"record_offset":39816,"parent_record_offset":39784,"ancestor_record_offset":39636,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1302,"record_offset":40304,"parent_record_offset":40268,"ancestor_record_offset":40016,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1302,"record_offset":40328,"parent_record_offset":40304,"ancestor_record_offset":40016,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1302,"record_offset":40604,"parent_record_offset":40576,"ancestor_record_offset":40016,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1302,"record_offset":40772,"parent_record_offset":40660,"ancestor_record_offset":40016,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":463,"record_offset":15472,"parent_record_offset":15336,"ancestor_record_offset":15228,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1302,"record_offset":40632,"parent_record_offset":40604,"ancestor_record_offset":40016,"inlinee":6055,"source_line":99,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1302,"record_offset":40660,"parent_record_offset":40632,"ancestor_record_offset":40016,"inlinee":6056,"source_line":276,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1302,"record_offset":41064,"parent_record_offset":40660,"ancestor_record_offset":40016,"inlinee":6062,"source_line":268,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":463,"record_offset":15496,"parent_record_offset":15472,"ancestor_record_offset":15228,"inlinee":11986,"source_line":300,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1302,"record_offset":40732,"parent_record_offset":40700,"ancestor_record_offset":40016,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1302,"record_offset":40700,"parent_record_offset":40660,"ancestor_record_offset":40016,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":463,"record_offset":15580,"parent_record_offset":15556,"ancestor_record_offset":15228,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":40576,"parent_record_offset":40548,"ancestor_record_offset":40016,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":40916,"parent_record_offset":40772,"ancestor_record_offset":40016,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1302,"record_offset":40872,"parent_record_offset":40840,"ancestor_record_offset":40016,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$50::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1302,"record_offset":40840,"parent_record_offset":40808,"ancestor_record_offset":40016,"inlinee":59719,"source_line":35,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":463,"record_offset":15300,"parent_record_offset":15276,"ancestor_record_offset":15228,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":40436,"parent_record_offset":40408,"ancestor_record_offset":40016,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":40988,"parent_record_offset":40948,"ancestor_record_offset":40016,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":463,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":15228,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":463,"record_offset":15276,"parent_record_offset":15252,"ancestor_record_offset":15228,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1302,"record_offset":41088,"parent_record_offset":41064,"ancestor_record_offset":40016,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs
//     evidence={"module":1302,"record_offset":40808,"parent_record_offset":40772,"ancestor_record_offset":40016,"inlinee":60342,"source_line":159,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEBB6
// 首条位置: {"module":1302,"inlinee":60342}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$50::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1302,"record_offset":40840,"parent_record_offset":40808,"ancestor_record_offset":40808,"inlinee":59719,"source_line":35,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1302,"record_offset":40872,"parent_record_offset":40840,"ancestor_record_offset":40808,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01302.jsonl.gz"}
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
// 完整结构及原始类型记录: adjust_wall_ui_mode.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: adjust_wall_ui_mode.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: adjust_wall_ui_mode.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
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
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::new
// 信息 ID: 3b4615edd8bbd628158d10a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height; module=453, inline_record=20440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier
// 信息 ID: e63cd73adbcb08fda2a4ce73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height @ 0x61d54e -> 0xd215c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget
// 信息 ID: be0349fed0ded3f90098d894
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height @ 0x61d4f5 -> 0xd21150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve
// 信息 ID: e57e8ea34de89b234b4ad5d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::signifier_animation::ui_adjust_wall_height_signifier_animation; module=463, inline_record=15228; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=1302, inline_record=39636; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=1302, inline_record=40016; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=1302, inline_record=41192; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier; module=1302, inline_record=43852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure$0
// 信息 ID: e301cc6494d799c31edbce19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=1302, inline_record=40808; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget; module=1302, inline_record=42020; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier; module=1302, inline_record=44868; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::adjust_wall_height_curve::closure_env$0> >; module=1308, inline_record=152336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$0
// 信息 ID: b902e5dcd36c69c9b725cdb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xd1e673 -> 0xd21c90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure$1
// 信息 ID: 0089f9b9204216e5f7c760e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$1 <- core::ptr::drop_in_place<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::adjust_wall_height::adjust_wall_ui_mode::impl$0::signifier::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xd1e57d -> 0xd21fb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

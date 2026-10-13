// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/toolbar_startup.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 47afd2845a9dbeaf3559849bcd12d52aed0f483ff35aa89763db22a10225b709
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: toolbar_startup.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3639, 4303
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
// 函数符号: toolbar::toolbar_startup::toolbar_startup
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3639,"record_offset":244,"rva":28657888,"va":5397367008}
// 编译实例 RVAs: 0x1b548e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24617538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e8be5d88fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5397367229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24617434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e8265e88fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5397367141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5d1a0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17796962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e89e70f0fe","size":5,"soft_fail":false,"target_va":5379576224,"va":5397373181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5f230
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17787340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e83496f0fe","size":5,"soft_fail":false,"target_va":5379584560,"va":5397371895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5f450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17789067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e8758ff0fe","size":5,"soft_fail":false,"target_va":5379585104,"va":5397374166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5f5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17789151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e8218ff0fe","size":5,"soft_fail":false,"target_va":5379585472,"va":5397374618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5ff10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17779245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e8d3b5f0fe","size":5,"soft_fail":false,"target_va":5379587856,"va":5397367096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a65bf0
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17761375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e8a1fbf0fe","size":5,"soft_fail":false,"target_va":5379611632,"va":5397373002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a65db0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17761298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e8eefbf0fe","size":5,"soft_fail":false,"target_va":5379612080,"va":5397373373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a89cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17607795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e88d53f3fe","size":5,"soft_fail":false,"target_va":5379759280,"va":5397367070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dfed30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2789802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e8aa912a00","size":5,"soft_fail":false,"target_va":5400161584,"va":5397371777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: toolbar_startup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6226196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e814015f00","size":5,"soft_fail":false,"target_va":5403593360,"va":5397367159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t5605592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e8d8885500","size":5,"soft_fail":false,"target_va":5402978144,"va":5397372547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5547490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e8e2a55400","size":5,"soft_fail":false,"target_va":5402918880,"va":5397371385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17651078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e87aaaf2fe","size":5,"soft_fail":false,"target_va":5379721104,"va":5397372177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20125879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e849e7ccfe","size":5,"soft_fail":false,"target_va":5377246528,"va":5397372402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::photomode::setup_photomode_toolbar::setup_photomode_toolbar
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2796869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e845ad2a00","size":5,"soft_fail":false,"target_va":5400168912,"va":5397372038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_contextual_menu
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2779785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e8896a2a00","size":5,"soft_fail":false,"target_va":5400151632,"va":5397371842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::setup_screen_and_toolbar_nodes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2783044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e844772a00","size":5,"soft_fail":false,"target_va":5400150256,"va":5397367207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout::setup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2805897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397367008,"raw_hex":"e889d02a00","size":5,"soft_fail":false,"target_va":5400177984,"va":5397372082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3639,"record_offset":416,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3639,"record_offset":5620,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":12145,"source_line":598,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3639,"record_offset":472,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3639,"record_offset":388,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":17514,"source_line":51,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] toolbar::toolbar_startup::setup_toolbar
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/toolbar_startup.rs
//     evidence={"module":3639,"record_offset":936,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":108699,"source_line":70,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":244,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":244,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":664,"parent_record_offset":640,"ancestor_record_offset":244,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":1304,"parent_record_offset":936,"ancestor_record_offset":244,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":1336,"parent_record_offset":1304,"ancestor_record_offset":244,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":640,"parent_record_offset":548,"ancestor_record_offset":244,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3639,"record_offset":5108,"parent_record_offset":936,"ancestor_record_offset":244,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":3512,"parent_record_offset":3484,"ancestor_record_offset":244,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":244,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":3484,"parent_record_offset":3452,"ancestor_record_offset":244,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":3616,"parent_record_offset":3592,"ancestor_record_offset":244,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":3452,"parent_record_offset":3428,"ancestor_record_offset":244,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":244,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3639,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":244,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3639,"record_offset":1608,"parent_record_offset":936,"ancestor_record_offset":244,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":3592,"parent_record_offset":3388,"ancestor_record_offset":244,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":3388,"parent_record_offset":3356,"ancestor_record_offset":244,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":244,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":5860,"parent_record_offset":5680,"ancestor_record_offset":244,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":3428,"parent_record_offset":3388,"ancestor_record_offset":244,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":6028,"parent_record_offset":5680,"ancestor_record_offset":244,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":4124,"parent_record_offset":936,"ancestor_record_offset":244,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":3296,"parent_record_offset":936,"ancestor_record_offset":244,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":244,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3639,"record_offset":4432,"parent_record_offset":4396,"ancestor_record_offset":244,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3639,"record_offset":4396,"parent_record_offset":4368,"ancestor_record_offset":244,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3639,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":244,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3639,"record_offset":4180,"parent_record_offset":936,"ancestor_record_offset":244,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3639,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":244,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3639,"record_offset":3564,"parent_record_offset":3388,"ancestor_record_offset":244,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3639,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":244,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3639,"record_offset":3356,"parent_record_offset":3296,"ancestor_record_offset":244,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3639,"record_offset":440,"parent_record_offset":416,"ancestor_record_offset":244,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3639,"record_offset":1248,"parent_record_offset":936,"ancestor_record_offset":244,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3639,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":244,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3639,"record_offset":6060,"parent_record_offset":5644,"ancestor_record_offset":244,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3639,"record_offset":5680,"parent_record_offset":5644,"ancestor_record_offset":244,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3639,"record_offset":3328,"parent_record_offset":3296,"ancestor_record_offset":244,"inlinee":14752,"source_line":57,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3639,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":244,"inlinee":6065,"source_line":262,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3639,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":244,"inlinee":6073,"source_line":267,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3639,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":244,"inlinee":11998,"source_line":356,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3639,"record_offset":5992,"parent_record_offset":5968,"ancestor_record_offset":244,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":3639,"record_offset":3700,"parent_record_offset":3672,"ancestor_record_offset":244,"inlinee":6067,"source_line":40,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":3639,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":244,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":3639,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":244,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3639,"record_offset":3648,"parent_record_offset":3388,"ancestor_record_offset":244,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3639,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":244,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3639,"record_offset":3724,"parent_record_offset":3700,"ancestor_record_offset":244,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3639,"record_offset":604,"parent_record_offset":576,"ancestor_record_offset":244,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3639,"record_offset":3756,"parent_record_offset":3724,"ancestor_record_offset":244,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\try_trait.rs
//     evidence={"module":3639,"record_offset":3788,"parent_record_offset":3724,"ancestor_record_offset":244,"inlinee":6071,"source_line":390,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3639,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":244,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3639,"record_offset":500,"parent_record_offset":472,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3639,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":244,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3639,"record_offset":5944,"parent_record_offset":5680,"ancestor_record_offset":244,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3639,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":244,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3639,"record_offset":4220,"parent_record_offset":4180,"ancestor_record_offset":244,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3639,"record_offset":1412,"parent_record_offset":1384,"ancestor_record_offset":244,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3639,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":244,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3639,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":244,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::row
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":3639,"record_offset":2300,"parent_record_offset":936,"ancestor_record_offset":244,"inlinee":108701,"source_line":124,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_startup::setup_toolbar
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A89B
// 首条位置: {"module":3639,"inlinee":108699}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":1304,"parent_record_offset":936,"ancestor_record_offset":936,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3639,"record_offset":5108,"parent_record_offset":936,"ancestor_record_offset":936,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3639,"record_offset":1608,"parent_record_offset":936,"ancestor_record_offset":936,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":4124,"parent_record_offset":936,"ancestor_record_offset":936,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":3296,"parent_record_offset":936,"ancestor_record_offset":936,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3639,"record_offset":4180,"parent_record_offset":936,"ancestor_record_offset":936,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3639,"record_offset":1248,"parent_record_offset":936,"ancestor_record_offset":936,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3639,"record_offset":4368,"parent_record_offset":936,"ancestor_record_offset":936,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarItem::row
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":3639,"record_offset":2300,"parent_record_offset":936,"ancestor_record_offset":936,"inlinee":108701,"source_line":124,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":936,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":936,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":936,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":1336,"parent_record_offset":1304,"ancestor_record_offset":936,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3639,"record_offset":4764,"parent_record_offset":4672,"ancestor_record_offset":936,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":3512,"parent_record_offset":3484,"ancestor_record_offset":936,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":4700,"parent_record_offset":4672,"ancestor_record_offset":936,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":3484,"parent_record_offset":3452,"ancestor_record_offset":936,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":3616,"parent_record_offset":3592,"ancestor_record_offset":936,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":3452,"parent_record_offset":3428,"ancestor_record_offset":936,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3639,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":936,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3639,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":936,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":3592,"parent_record_offset":3388,"ancestor_record_offset":936,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":3388,"parent_record_offset":3356,"ancestor_record_offset":936,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":936,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":3428,"parent_record_offset":3388,"ancestor_record_offset":936,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3639,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":936,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3639,"record_offset":4432,"parent_record_offset":4396,"ancestor_record_offset":936,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3639,"record_offset":4396,"parent_record_offset":4368,"ancestor_record_offset":936,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3639,"record_offset":4624,"parent_record_offset":4600,"ancestor_record_offset":936,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3639,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":936,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3639,"record_offset":3564,"parent_record_offset":3388,"ancestor_record_offset":936,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3639,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":936,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3639,"record_offset":3356,"parent_record_offset":3296,"ancestor_record_offset":936,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3639,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":936,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3639,"record_offset":3328,"parent_record_offset":3296,"ancestor_record_offset":936,"inlinee":14752,"source_line":57,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3639,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":936,"inlinee":6065,"source_line":262,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3639,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":936,"inlinee":6073,"source_line":267,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3639,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":936,"inlinee":11998,"source_line":356,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":3639,"record_offset":3700,"parent_record_offset":3672,"ancestor_record_offset":936,"inlinee":6067,"source_line":40,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":3639,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":936,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":3639,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":936,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3639,"record_offset":3648,"parent_record_offset":3388,"ancestor_record_offset":936,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3639,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":936,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3639,"record_offset":3724,"parent_record_offset":3700,"ancestor_record_offset":936,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3639,"record_offset":4728,"parent_record_offset":4700,"ancestor_record_offset":936,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3639,"record_offset":3756,"parent_record_offset":3724,"ancestor_record_offset":936,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\try_trait.rs
//     evidence={"module":3639,"record_offset":3788,"parent_record_offset":3724,"ancestor_record_offset":936,"inlinee":6071,"source_line":390,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3639,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":936,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3639,"record_offset":3936,"parent_record_offset":3672,"ancestor_record_offset":936,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3639,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":936,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3639,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":936,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3639,"record_offset":4220,"parent_record_offset":4180,"ancestor_record_offset":936,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3639,"record_offset":1412,"parent_record_offset":1384,"ancestor_record_offset":936,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3639,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":936,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03639.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_startup::setup_toolbar::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D4FD
// 首条位置: {"module":4303,"inlinee":120061}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":2336,"parent_record_offset":2172,"ancestor_record_offset":2172,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":2752,"parent_record_offset":2724,"ancestor_record_offset":2172,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":2724,"parent_record_offset":2688,"ancestor_record_offset":2172,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":2172,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4303,"record_offset":2688,"parent_record_offset":2656,"ancestor_record_offset":2172,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":2832,"parent_record_offset":2608,"ancestor_record_offset":2172,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":2804,"parent_record_offset":2608,"ancestor_record_offset":2172,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":2656,"parent_record_offset":2608,"ancestor_record_offset":2172,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4303,"record_offset":2972,"parent_record_offset":2608,"ancestor_record_offset":2172,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":2464,"parent_record_offset":2400,"ancestor_record_offset":2172,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":2532,"parent_record_offset":2464,"ancestor_record_offset":2172,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":4303,"record_offset":2400,"parent_record_offset":2336,"ancestor_record_offset":2172,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":4303,"record_offset":3004,"parent_record_offset":2532,"ancestor_record_offset":2172,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":4303,"record_offset":2608,"parent_record_offset":2532,"ancestor_record_offset":2172,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4303,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":2172,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4303,"record_offset":2888,"parent_record_offset":2608,"ancestor_record_offset":2172,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/04303.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4303,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":2172,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/04303.jsonl.gz"}
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
// 完整结构及原始类型记录: toolbar_startup.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: toolbar_startup.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: toolbar_startup.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_startup::setup_toolbar
// 信息 ID: 5e6e573c560198fabad7767d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::toolbar_startup::toolbar_startup; module=3639, inline_record=936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_startup::setup_toolbar::closure$0
// 信息 ID: 5ff52fbf4d8960601932e126
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::toolbar_startup::setup_toolbar", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_startup::setup_toolbar::closure_env$0 <- toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::toolbar_startup::setup_toolbar::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::add_category_button<toolbar::toolbar_startup::setup_toolbar::closure_env$0>; module=4303, inline_record=2172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_startup::toolbar_startup
// 信息 ID: 0ff2797619d24180ab0f28ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 626ca7c1fa93979407fff589: void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::commands::Commands)> > @ 0x15e8f5e -> 0x1b548e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::commands::Commands)> @ 0x1617014 -> 0x1b548e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

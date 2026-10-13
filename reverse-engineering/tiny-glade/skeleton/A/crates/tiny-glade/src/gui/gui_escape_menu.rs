// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/gui/gui_escape_menu.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3198519fa9399d609f699cc87985c338b01fefb90b8f4e199bb48c3f078100b7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_escape_menu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 148, 220
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_state; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use obfstr; // namespace candidate
//   package identities: [{"name":"obfstr","source":"PDB_registry_path_only","version":"0.4.3"}]
// use toolbar; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":29764,"rva":3401872,"va":5372110992}
// 编译实例 RVAs: 0x33e890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007edd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2882423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372110992,"raw_hex":"e88904d4ff","size":5,"soft_fail":false,"target_va":5369228752,"va":5372111170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-812338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372110992,"raw_hex":"e8ce9af3ff","size":5,"soft_fail":false,"target_va":5371298848,"va":5372111181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t638343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372110992,"raw_hex":"e887bd0900","size":5,"soft_fail":false,"target_va":5372749712,"va":5372111364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1068098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372110992,"raw_hex":"e8424c1000","size":5,"soft_fail":false,"target_va":5373179248,"va":5372111145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5418758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372110992,"raw_hex":"e806af5200","size":5,"soft_fail":false,"target_va":5377529888,"va":5372111125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":220,"record_offset":29920,"parent_record_offset":29764,"ancestor_record_offset":29764,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":30148,"parent_record_offset":29764,"ancestor_record_offset":29764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":220,"record_offset":30000,"parent_record_offset":29764,"ancestor_record_offset":29764,"inlinee":5600,"source_line":196,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":30300,"parent_record_offset":30276,"ancestor_record_offset":29764,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":30276,"parent_record_offset":30220,"ancestor_record_offset":29764,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":220,"record_offset":29968,"parent_record_offset":29944,"ancestor_record_offset":29764,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":30248,"parent_record_offset":30220,"ancestor_record_offset":29764,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":30220,"parent_record_offset":30196,"ancestor_record_offset":29764,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":220,"record_offset":29944,"parent_record_offset":29920,"ancestor_record_offset":29764,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":30172,"parent_record_offset":30148,"ancestor_record_offset":29764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":148,"record_offset":232,"rva":2634736,"va":5371343856}
// 编译实例 RVAs: 0x2833f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400693c0
//     owner=unresolved_namespace; count=2
//     target_aliases=33 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2204901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e81b5bdeff","size":5,"soft_fail":false,"target_va":5369140160,"va":5371345056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033a790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t748360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8486b0b00","size":5,"soft_fail":false,"target_va":5372094352,"va":5371345987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033ab20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t748468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8b46b0b00","size":5,"soft_fail":false,"target_va":5372095264,"va":5371346791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033aeb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t751292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8bc760b00","size":5,"soft_fail":false,"target_va":5372096176,"va":5371344879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033b0a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t750076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8fc710b00","size":5,"soft_fail":false,"target_va":5372096672,"va":5371346591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033b260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t751333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8e5760b00","size":5,"soft_fail":false,"target_va":5372097120,"va":5371345782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033bbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t753421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e80d7f0b00","size":5,"soft_fail":false,"target_va":5372099504,"va":5371346078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033bf50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t754378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8ca820b00","size":5,"soft_fail":false,"target_va":5372100432,"va":5371346049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033c340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t757082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e85a8d0b00","size":5,"soft_fail":false,"target_va":5372101440,"va":5371344353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033c6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t755989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e815890b00","size":5,"soft_fail":false,"target_va":5372102336,"va":5371346342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033cc10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t757584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8508f0b00","size":5,"soft_fail":false,"target_va":5372103696,"va":5371346107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033cfb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t757429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8b58e0b00","size":5,"soft_fail":false,"target_va":5372104624,"va":5371347190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033d1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t757848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e858900b00","size":5,"soft_fail":false,"target_va":5372105120,"va":5371347267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033d360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t758344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e848920b00","size":5,"soft_fail":false,"target_va":5372105568,"va":5371347219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033d760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t759573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e815970b00","size":5,"soft_fail":false,"target_va":5372106592,"va":5371347014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033d990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t759904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e860980b00","size":5,"soft_fail":false,"target_va":5372107152,"va":5371347243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033dd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t761675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e84b9f0b00","size":5,"soft_fail":false,"target_va":5372108048,"va":5371346368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043d460
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1810233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8399f1b00","size":5,"soft_fail":false,"target_va":5373154400,"va":5371344162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043e590
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1814749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8ddb01b00","size":5,"soft_fail":false,"target_va":5373158800,"va":5371344046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3054267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8bb9a2e00","size":5,"soft_fail":false,"target_va":5374398944,"va":5371344672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140597160
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3226433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e8413b3100","size":5,"soft_fail":false,"target_va":5374570848,"va":5371344410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t-35304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e81876ffff","size":5,"soft_fail":false,"target_va":5371308800,"va":5371344099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3438649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e839783400","size":5,"soft_fail":false,"target_va":5374783040,"va":5371344386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3054654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e83e9c2e00","size":5,"soft_fail":false,"target_va":5374399200,"va":5371344541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::history_process::HistoryCmd::interrupt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4232344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e898944000","size":5,"soft_fail":false,"target_va":5375576768,"va":5371344419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t1815314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371343856,"raw_hex":"e812b31b00","size":5,"soft_fail":false,"target_va":5373159392,"va":5371344073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":148,"record_offset":388,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":148,"record_offset":740,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":148,"record_offset":1004,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::impl$2::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_state/src/state/resources.rs
//     evidence={"module":148,"record_offset":644,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":11418,"source_line":77,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":148,"record_offset":3896,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":7454,"source_line":349,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::wide_safe_frame
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":148,"record_offset":448,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":8028,"source_line":355,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":148,"record_offset":572,"parent_record_offset":540,"ancestor_record_offset":232,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":148,"record_offset":540,"parent_record_offset":508,"ancestor_record_offset":232,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":148,"record_offset":508,"parent_record_offset":472,"ancestor_record_offset":232,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=22
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":148,"record_offset":416,"parent_record_offset":388,"ancestor_record_offset":232,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":148,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":232,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":148,"record_offset":824,"parent_record_offset":740,"ancestor_record_offset":232,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":148,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":232,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":148,"record_offset":604,"parent_record_offset":508,"ancestor_record_offset":232,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] country_core::app_loading_state::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/app_loading_state.rs
//     evidence={"module":148,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":232,"inlinee":7175,"source_line":1,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":148,"record_offset":472,"parent_record_offset":448,"ancestor_record_offset":232,"inlinee":7455,"source_line":398,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 137
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":30752,"rva":3402400,"va":5372111520}
// 编译实例 RVAs: 0x33eaa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1038234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111520,"raw_hex":"e89ad70f00","size":5,"soft_fail":false,"target_va":5373149856,"va":5372111617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2287211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111520,"raw_hex":"e86be62200","size":5,"soft_fail":false,"target_va":5374398944,"va":5372111728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-806617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111520,"raw_hex":"e827b1f3ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5372111668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1047234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111520,"raw_hex":"e8c2fa0f00","size":5,"soft_fail":false,"target_va":5373158864,"va":5372111625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::PillButtonView::inactive_opacity
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1037102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111520,"raw_hex":"e82ed30f00","size":5,"soft_fail":false,"target_va":5373148704,"va":5372111597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::back_button
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1036569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111520,"raw_hex":"e819d10f00","size":5,"soft_fail":false,"target_va":5373148160,"va":5372111586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":31168,"parent_record_offset":30752,"ancestor_record_offset":30752,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":30904,"parent_record_offset":30752,"ancestor_record_offset":30752,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":31092,"parent_record_offset":31068,"ancestor_record_offset":30752,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":31068,"parent_record_offset":31028,"ancestor_record_offset":30752,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":31028,"parent_record_offset":30992,"ancestor_record_offset":30752,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":30964,"parent_record_offset":30928,"ancestor_record_offset":30752,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":30928,"parent_record_offset":30904,"ancestor_record_offset":30752,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":30992,"parent_record_offset":30928,"ancestor_record_offset":30752,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":31124,"parent_record_offset":31028,"ancestor_record_offset":30752,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32FE
// 首条位置: {"module":220,"inlinee":13054}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3303
// 首条位置: {"module":220,"inlinee":13059}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3308
// 首条位置: {"module":220,"inlinee":13064}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3307
// 首条位置: {"module":220,"inlinee":13063}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 288
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3305
// 首条位置: {"module":220,"inlinee":13061}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$7
// 记录形式: compiled_procedure; instances=1
// 源码行: 300
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":31232,"rva":3402704,"va":5372111824}
// 编译实例 RVAs: 0x33ebd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t637308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e87cb90900","size":5,"soft_fail":false,"target_va":5372749696,"va":5372112383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t637748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e834bb0900","size":5,"soft_fail":false,"target_va":5372749712,"va":5372111959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043ad70
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1032520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e848c10f00","size":5,"soft_fail":false,"target_va":5373144432,"va":5372111907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1033375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e89fc40f00","size":5,"soft_fail":false,"target_va":5373145504,"va":5372112124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1037927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e867d60f00","size":5,"soft_fail":false,"target_va":5373149856,"va":5372111924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1067520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e8004a1000","size":5,"soft_fail":false,"target_va":5373179424,"va":5372111899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2286882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e822e52200","size":5,"soft_fail":false,"target_va":5374398944,"va":5372112057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30865646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e8eef8d601","size":5,"soft_fail":false,"target_va":5402978144,"va":5372112493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-807124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e82caff3ff","size":5,"soft_fail":false,"target_va":5371304880,"va":5372111999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_uppercase
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5420944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e890b75200","size":5,"soft_fail":false,"target_va":5377532832,"va":5372111883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4041007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e82fa93d00","size":5,"soft_fail":false,"target_va":5376152880,"va":5372111868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1046927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372111824,"raw_hex":"e88ff90f00","size":5,"soft_fail":false,"target_va":5373158864,"va":5372111932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":31848,"parent_record_offset":31232,"ancestor_record_offset":31232,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":31584,"parent_record_offset":31232,"ancestor_record_offset":31232,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":31392,"parent_record_offset":31232,"ancestor_record_offset":31232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$28::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":220,"record_offset":32436,"parent_record_offset":31232,"ancestor_record_offset":31232,"inlinee":13070,"source_line":211,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":32660,"parent_record_offset":32636,"ancestor_record_offset":31232,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":32684,"parent_record_offset":32660,"ancestor_record_offset":31232,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":31536,"parent_record_offset":31512,"ancestor_record_offset":31232,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":32604,"parent_record_offset":32576,"ancestor_record_offset":31232,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":32636,"parent_record_offset":32604,"ancestor_record_offset":31232,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":31512,"parent_record_offset":31456,"ancestor_record_offset":31232,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":220,"record_offset":32576,"parent_record_offset":32548,"ancestor_record_offset":31232,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":31484,"parent_record_offset":31456,"ancestor_record_offset":31232,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":31776,"parent_record_offset":31744,"ancestor_record_offset":31232,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":31456,"parent_record_offset":31432,"ancestor_record_offset":31232,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":220,"record_offset":32548,"parent_record_offset":32464,"ancestor_record_offset":31232,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":220,"record_offset":32464,"parent_record_offset":32436,"ancestor_record_offset":31232,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":31744,"parent_record_offset":31704,"ancestor_record_offset":31232,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":31704,"parent_record_offset":31668,"ancestor_record_offset":31232,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":31636,"parent_record_offset":31608,"ancestor_record_offset":31232,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":31608,"parent_record_offset":31584,"ancestor_record_offset":31232,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":220,"record_offset":31668,"parent_record_offset":31584,"ancestor_record_offset":31232,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":31412,"parent_record_offset":31392,"ancestor_record_offset":31232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":32712,"parent_record_offset":32684,"ancestor_record_offset":31232,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":31808,"parent_record_offset":31704,"ancestor_record_offset":31232,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$27::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":220,"record_offset":32516,"parent_record_offset":32492,"ancestor_record_offset":31232,"inlinee":12031,"source_line":167,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":220,"record_offset":32492,"parent_record_offset":32464,"ancestor_record_offset":31232,"inlinee":9750,"source_line":422,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8
// 记录形式: compiled_procedure; instances=1
// 源码行: 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":33388,"rva":3403584,"va":5372112704}
// 编译实例 RVAs: 0x33ef40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f7d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2881441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372112704,"raw_hex":"e85f08d4ff","size":5,"soft_fail":false,"target_va":5369231312,"va":5372112748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1032635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372112704,"raw_hex":"e8bbc10f00","size":5,"soft_fail":false,"target_va":5373145504,"va":5372112864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=8 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1037083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372112704,"raw_hex":"e81bd30f00","size":5,"soft_fail":false,"target_va":5373149856,"va":5372112768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2286123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372112704,"raw_hex":"e82be22200","size":5,"soft_fail":false,"target_va":5374398944,"va":5372112816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1046083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372112704,"raw_hex":"e843f60f00","size":5,"soft_fail":false,"target_va":5373158864,"va":5372112776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":33536,"parent_record_offset":33388,"ancestor_record_offset":33388,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":33560,"parent_record_offset":33536,"ancestor_record_offset":33388,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$9
// 记录形式: compiled_procedure; instances=1
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":33716,"rva":3403936,"va":5372113056}
// 编译实例 RVAs: 0x33f0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2884016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113056,"raw_hex":"e850fed3ff","size":5,"soft_fail":false,"target_va":5369229136,"va":5372113147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-814320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113056,"raw_hex":"e81093f3ff","size":5,"soft_fail":false,"target_va":5371298848,"va":5372113163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t636493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113056,"raw_hex":"e84db60900","size":5,"soft_fail":false,"target_va":5372749712,"va":5372113214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1036671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113056,"raw_hex":"e87fd10f00","size":5,"soft_fail":false,"target_va":5373149856,"va":5372113180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1066289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113056,"raw_hex":"e831451000","size":5,"soft_fail":false,"target_va":5373179424,"va":5372113130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2285692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113056,"raw_hex":"e87ce02200","size":5,"soft_fail":false,"target_va":5374398944,"va":5372113247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_uppercase
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5419717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113056,"raw_hex":"e8c5b25200","size":5,"soft_fail":false,"target_va":5377532832,"va":5372113110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4039780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113056,"raw_hex":"e864a43d00","size":5,"soft_fail":false,"target_va":5376152880,"va":5372113095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1045671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113056,"raw_hex":"e8a7f40f00","size":5,"soft_fail":false,"target_va":5373158864,"va":5372113188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":34108,"parent_record_offset":33716,"ancestor_record_offset":33716,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":33908,"parent_record_offset":33716,"ancestor_record_offset":33716,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":220,"record_offset":33872,"parent_record_offset":33716,"ancestor_record_offset":33716,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":34060,"parent_record_offset":34036,"ancestor_record_offset":33716,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":34036,"parent_record_offset":33980,"ancestor_record_offset":33716,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":34008,"parent_record_offset":33980,"ancestor_record_offset":33716,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":33980,"parent_record_offset":33956,"ancestor_record_offset":33716,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":34136,"parent_record_offset":34108,"ancestor_record_offset":33716,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":33932,"parent_record_offset":33908,"ancestor_record_offset":33716,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$10
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 363
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32FC
// 首条位置: {"module":220,"inlinee":13052}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 372
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3302
// 首条位置: {"module":220,"inlinee":13058}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$12
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 391
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3301
// 首条位置: {"module":220,"inlinee":13057}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$13
// 记录形式: compiled_procedure; instances=1
// 源码行: 400
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":220,"record_offset":34372,"rva":3404208,"va":5372113328}
// 编译实例 RVAs: 0x33f1b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t636241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113328,"raw_hex":"e851b50900","size":5,"soft_fail":false,"target_va":5372749712,"va":5372113466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1032085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113328,"raw_hex":"e895bf0f00","size":5,"soft_fail":false,"target_va":5373145504,"va":5372113414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1036420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113328,"raw_hex":"e884d00f00","size":5,"soft_fail":false,"target_va":5373149856,"va":5372113431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1066016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113328,"raw_hex":"e820441000","size":5,"soft_fail":false,"target_va":5373179424,"va":5372113403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14056d1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2285439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113328,"raw_hex":"e87fdf2200","size":5,"soft_fail":false,"target_va":5374398944,"va":5372113500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30805326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113328,"raw_hex":"e84e0dd601","size":5,"soft_fail":false,"target_va":5402919072,"va":5372113741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_uppercase
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5419440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113328,"raw_hex":"e8b0b15200","size":5,"soft_fail":false,"target_va":5377532832,"va":5372113387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5132853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113328,"raw_hex":"e835524e00","size":5,"soft_fail":false,"target_va":5377246528,"va":5372113670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4039503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113328,"raw_hex":"e84fa33d00","size":5,"soft_fail":false,"target_va":5376152880,"va":5372113372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1045420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372113328,"raw_hex":"e8acf30f00","size":5,"soft_fail":false,"target_va":5373158864,"va":5372113439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":34720,"parent_record_offset":34372,"ancestor_record_offset":34372,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":220,"record_offset":34980,"parent_record_offset":34372,"ancestor_record_offset":34372,"inlinee":12145,"source_line":598,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":34528,"parent_record_offset":34372,"ancestor_record_offset":34372,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":34672,"parent_record_offset":34648,"ancestor_record_offset":34372,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":220,"record_offset":34648,"parent_record_offset":34592,"ancestor_record_offset":34372,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":35196,"parent_record_offset":35168,"ancestor_record_offset":34372,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":34620,"parent_record_offset":34592,"ancestor_record_offset":34372,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":35168,"parent_record_offset":35132,"ancestor_record_offset":34372,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":35300,"parent_record_offset":35276,"ancestor_record_offset":34372,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":35132,"parent_record_offset":35100,"ancestor_record_offset":34372,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":220,"record_offset":34592,"parent_record_offset":34568,"ancestor_record_offset":34372,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":35276,"parent_record_offset":35052,"ancestor_record_offset":34372,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":35248,"parent_record_offset":35052,"ancestor_record_offset":34372,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":35100,"parent_record_offset":35052,"ancestor_record_offset":34372,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":220,"record_offset":35416,"parent_record_offset":35052,"ancestor_record_offset":34372,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":34748,"parent_record_offset":34720,"ancestor_record_offset":34372,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":220,"record_offset":35008,"parent_record_offset":34980,"ancestor_record_offset":34372,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":220,"record_offset":35448,"parent_record_offset":35008,"ancestor_record_offset":34372,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":220,"record_offset":35052,"parent_record_offset":35008,"ancestor_record_offset":34372,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":220,"record_offset":35380,"parent_record_offset":35356,"ancestor_record_offset":34372,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":34548,"parent_record_offset":34528,"ancestor_record_offset":34372,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":220,"record_offset":35332,"parent_record_offset":35052,"ancestor_record_offset":34372,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":220,"record_offset":35356,"parent_record_offset":35332,"ancestor_record_offset":34372,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$14
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 417
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32FD
// 首条位置: {"module":220,"inlinee":13053}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$16
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 439
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32FF
// 首条位置: {"module":220,"inlinee":13055}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":7840,"parent_record_offset":7812,"ancestor_record_offset":7812,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":220,"record_offset":7868,"parent_record_offset":7840,"ancestor_record_offset":7812,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::wishlist_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 452
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":148,"record_offset":5356,"rva":2638208,"va":5371347328}
// 编译实例 RVAs: 0x284180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ef50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2118439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e8d9acdfff","size":5,"soft_fail":false,"target_va":5369229136,"va":5371347570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2116752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e870b3dfff","size":5,"soft_fail":false,"target_va":5369230672,"va":5371347419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140132550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1383819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e875e2eaff","size":5,"soft_fail":false,"target_va":5369963856,"va":5371347670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140278420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-48742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e89a41ffff","size":5,"soft_fail":false,"target_va":5371298848,"va":5371347585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1402013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e89d641500","size":5,"soft_fail":false,"target_va":5372749712,"va":5371347694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1802214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e8e67f1b00","size":5,"soft_fail":false,"target_va":5373149856,"va":5371347637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140443620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1831866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e8baf31b00","size":5,"soft_fail":false,"target_va":5373179424,"va":5371347553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_uppercase
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6185376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e8a0615e00","size":5,"soft_fail":false,"target_va":5377532832,"va":5371347451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4805439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e83f534900","size":5,"soft_fail":false,"target_va":5376152880,"va":5371347436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1811214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e80ea31b00","size":5,"soft_fail":false,"target_va":5373158864,"va":5371347645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::align
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1798279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e887701b00","size":5,"soft_fail":false,"target_va":5373145904,"va":5371347620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::max_width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1798210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e842701b00","size":5,"soft_fail":false,"target_va":5373145824,"va":5371347609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5592412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e85c555500","size":5,"soft_fail":false,"target_va":5376939952,"va":5371347535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::linearstep
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5592404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347328,"raw_hex":"e854555500","size":5,"soft_fail":false,"target_va":5376939920,"va":5371347511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":148,"record_offset":5532,"parent_record_offset":5356,"ancestor_record_offset":5356,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":148,"record_offset":5496,"parent_record_offset":5356,"ancestor_record_offset":5356,"inlinee":5602,"source_line":283,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":148,"record_offset":5688,"parent_record_offset":5664,"ancestor_record_offset":5356,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":148,"record_offset":5664,"parent_record_offset":5608,"ancestor_record_offset":5356,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":148,"record_offset":5636,"parent_record_offset":5608,"ancestor_record_offset":5356,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":148,"record_offset":5608,"parent_record_offset":5580,"ancestor_record_offset":5356,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":148,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":5356,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::open_community_server
// 记录形式: compiled_procedure; instances=1
// 源码行: 471
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":148,"record_offset":5944,"rva":2638688,"va":5371347808}
// 编译实例 RVAs: 0x284360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t966052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347808,"raw_hex":"e8a4bd0e00","size":5,"soft_fail":false,"target_va":5372314208,"va":5371348151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1401766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347808,"raw_hex":"e8a6631500","size":5,"soft_fail":false,"target_va":5372749696,"va":5371347925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1401475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347808,"raw_hex":"e883621500","size":5,"soft_fail":false,"target_va":5372749712,"va":5371348232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31629873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347808,"raw_hex":"e831a2e201","size":5,"soft_fail":false,"target_va":5402978144,"va":5371348266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-43141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347808,"raw_hex":"e87b57ffff","size":5,"soft_fail":false,"target_va":5371305056,"va":5371348192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6005629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347808,"raw_hex":"e87da35b00","size":5,"soft_fail":false,"target_va":5377353776,"va":5371348142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] obfstr::xref::inner
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t60142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347808,"raw_hex":"e8eeea0000","size":5,"soft_fail":false,"target_va":5371408032,"va":5371347885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] webbrowser::open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4645227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371347808,"raw_hex":"e86be14600","size":5,"soft_fail":false,"target_va":5375993248,"va":5371348016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":148,"record_offset":6304,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":148,"record_offset":7020,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":148,"record_offset":6936,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":148,"record_offset":6964,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":148,"record_offset":7276,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":148,"record_offset":6992,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":148,"record_offset":6852,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] obfstr::bytes::deobfuscate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/obfstr-0.4.3/src/bytes.rs
//     evidence={"module":148,"record_offset":6172,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":11422,"source_line":171,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_direct] obfstr::xref::xref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/obfstr-0.4.3/src/xref.rs
//     evidence={"module":148,"record_offset":6116,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":11420,"source_line":78,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":148,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":5944,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":148,"record_offset":6664,"parent_record_offset":6640,"ancestor_record_offset":5944,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":148,"record_offset":7396,"parent_record_offset":7372,"ancestor_record_offset":5944,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":148,"record_offset":6616,"parent_record_offset":6588,"ancestor_record_offset":5944,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":148,"record_offset":7372,"parent_record_offset":7348,"ancestor_record_offset":5944,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":148,"record_offset":7204,"parent_record_offset":7180,"ancestor_record_offset":5944,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":148,"record_offset":6588,"parent_record_offset":6560,"ancestor_record_offset":5944,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":148,"record_offset":6560,"parent_record_offset":6532,"ancestor_record_offset":5944,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":148,"record_offset":6732,"parent_record_offset":6560,"ancestor_record_offset":5944,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":148,"record_offset":7348,"parent_record_offset":7324,"ancestor_record_offset":5944,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":148,"record_offset":6496,"parent_record_offset":6464,"ancestor_record_offset":5944,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":148,"record_offset":6464,"parent_record_offset":6432,"ancestor_record_offset":5944,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":148,"record_offset":6400,"parent_record_offset":6368,"ancestor_record_offset":5944,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":148,"record_offset":6432,"parent_record_offset":6400,"ancestor_record_offset":5944,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":148,"record_offset":6368,"parent_record_offset":6336,"ancestor_record_offset":5944,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":148,"record_offset":6336,"parent_record_offset":6304,"ancestor_record_offset":5944,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":148,"record_offset":7180,"parent_record_offset":7140,"ancestor_record_offset":5944,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":148,"record_offset":7140,"parent_record_offset":7104,"ancestor_record_offset":5944,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":148,"record_offset":6532,"parent_record_offset":6496,"ancestor_record_offset":5944,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":148,"record_offset":7072,"parent_record_offset":7044,"ancestor_record_offset":5944,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":148,"record_offset":7044,"parent_record_offset":7020,"ancestor_record_offset":5944,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":148,"record_offset":7104,"parent_record_offset":7020,"ancestor_record_offset":5944,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::black_box
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hint.rs
//     evidence={"module":148,"record_offset":6140,"parent_record_offset":6116,"ancestor_record_offset":5944,"inlinee":5511,"source_line":336,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":148,"record_offset":6792,"parent_record_offset":6768,"ancestor_record_offset":5944,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":148,"record_offset":6768,"parent_record_offset":6496,"ancestor_record_offset":5944,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":148,"record_offset":7300,"parent_record_offset":7276,"ancestor_record_offset":5944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":148,"record_offset":6216,"parent_record_offset":6172,"ancestor_record_offset":5944,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":148,"record_offset":7236,"parent_record_offset":7140,"ancestor_record_offset":5944,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":148,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":5944,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00148.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":148,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":5944,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00148.jsonl.gz"}
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
// 完整结构及原始类型记录: gui_escape_menu.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gui_escape_menu.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gui_escape_menu.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >", "tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>", "bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "country_core::systems::steam::SteamOverlayOpenResources", "bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>", "bevy_ecs::system::commands::Commands", "tuple$<bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu
// 信息 ID: 2f8367d7468ddde7619a8bb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e1cdcf0bee9a3f78687bc519: void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)> > @ 0xbe3e4 -> 0x2833f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)> @ 0xeb034 -> 0x2833f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$0
// 信息 ID: 618e0ab20152611a2fef1b08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$0,ref$<str$> > @ 0x33c4da -> 0x33e890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$1
// 信息 ID: a78f103a5ff8b820c0ee358d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$1,ref$<str$> > @ 0x33b23a -> 0x33eaa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$10
// 信息 ID: b3f1706acb191cac247642bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$10 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$10,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$10,ref$<str$> >; module=220, inline_record=4064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$11
// 信息 ID: bc57dd9fe63d661433dd3b55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 11, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11,ref$<str$> >; module=220, inline_record=12272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$12
// 信息 ID: 827bf260d5d48170b446d8fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 12, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12,ref$<str$> >; module=220, inline_record=11032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$13
// 信息 ID: f5e2399d6b512598e7acfe42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 13, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$15,ref$<str$> > @ 0x33cdcf -> 0x33f1b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$14
// 信息 ID: a77d15823ad3620b7a458882
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 14, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$14 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$14,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$14,ref$<str$> >; module=220, inline_record=5316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$16
// 信息 ID: c1bd39b743c5fd08a5667d14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 16, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$16 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$16,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$16,ref$<str$> >; module=220, inline_record=7812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$2
// 信息 ID: 9b73abc4ac3ab5f9aad3e86c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$2,ref$<str$> >; module=220, inline_record=6568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$3
// 信息 ID: 09cb6291d9051d7fbee60498
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3,ref$<str$> >; module=220, inline_record=14768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$4
// 信息 ID: 3cb4d5fd2be0a66cb76e8a00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4,ref$<str$> >; module=220, inline_record=22440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$5
// 信息 ID: 097ad191ba0d8456c57b0a4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5,ref$<str$> >; module=220, inline_record=20568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$6
// 信息 ID: bbd7d0db82dd23f2aabc25b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6,ref$<str$> >; module=220, inline_record=18016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$7
// 信息 ID: e1e132722987dcf055780910
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$7 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$7,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$7,ref$<str$> > @ 0x33d51f -> 0x33ebd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8
// 信息 ID: 423df4769da0ead5b141fa3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$8 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$8,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$8,ref$<str$> > @ 0x33db2a -> 0x33ef40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$9
// 信息 ID: 2ff6200aba63bd4c73be2f5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$9 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$9,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$9,ref$<str$> > @ 0x33d33a -> 0x33f0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::open_community_server
// 信息 ID: 47c9686e69d5603f49130bd4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11,ref$<str$> > @ 0x33c15e -> 0x284360 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3,ref$<str$> > @ 0x33c8ce -> 0x284360 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5,ref$<str$> > @ 0x33d96e -> 0x284360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::wishlist_button
// 信息 ID: af6ac1b15a89ce093d37bf2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12,ref$<str$> > @ 0x33bd52 -> 0x284180 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4,ref$<str$> > @ 0x33deb2 -> 0x284180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

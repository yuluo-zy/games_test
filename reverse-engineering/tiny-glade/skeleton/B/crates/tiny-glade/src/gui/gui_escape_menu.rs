// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/gui/gui_escape_menu.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f37bf0d09cc7260807cafefbccd9e50617002f814ea5148a9af7826d7300e9f4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_escape_menu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 149, 170, 204
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_state; // namespace candidate
//   package identities: [{"name":"bevy_state","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use obfstr; // namespace candidate
//   package identities: [{"name":"obfstr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.3"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":204,"record_offset":38568,"rva":5261296,"va":5373970416}
// 编译实例 RVAs: 0x5047f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106350
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4187486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373970416,"raw_hex":"e8a21ac0ff","size":5,"soft_fail":false,"target_va":5369783120,"va":5373970601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1110719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373970416,"raw_hex":"e8bff21000","size":5,"soft_fail":false,"target_va":5375081504,"va":5373970780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068bf00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1603182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373970416,"raw_hex":"e86e761800","size":5,"soft_fail":false,"target_va":5375573760,"va":5373970573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8869654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373970416,"raw_hex":"e816578700","size":5,"soft_fail":false,"target_va":5382840208,"va":5373970549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::content_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1561741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373970416,"raw_hex":"e88dd41700","size":5,"soft_fail":false,"target_va":5375532336,"va":5373970590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":204,"record_offset":38724,"parent_record_offset":38568,"ancestor_record_offset":38568,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":38944,"parent_record_offset":38568,"ancestor_record_offset":38568,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":204,"record_offset":38804,"parent_record_offset":38568,"ancestor_record_offset":38568,"inlinee":10727,"source_line":249,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":39120,"parent_record_offset":39096,"ancestor_record_offset":38568,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":39096,"parent_record_offset":39040,"ancestor_record_offset":38568,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":204,"record_offset":38772,"parent_record_offset":38748,"ancestor_record_offset":38568,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":39068,"parent_record_offset":39040,"ancestor_record_offset":38568,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":39040,"parent_record_offset":39016,"ancestor_record_offset":38568,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":39016,"parent_record_offset":38992,"ancestor_record_offset":38568,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":204,"record_offset":38748,"parent_record_offset":38724,"ancestor_record_offset":38568,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":38968,"parent_record_offset":38944,"ancestor_record_offset":38568,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":10016,"rva":3996080,"va":5372705200}
// 编译实例 RVAs: 0x3cf9b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1404fe790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1236979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8f3df1200","size":5,"soft_fail":false,"target_va":5373945744,"va":5372708760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404ff7e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1241659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e83bf21200","size":5,"soft_fail":false,"target_va":5373949920,"va":5372708256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404ffeb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1244033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e881fb1200","size":5,"soft_fail":false,"target_va":5373951664,"va":5372707626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140500370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1244144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8f0fb1200","size":5,"soft_fail":false,"target_va":5373952880,"va":5372708731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405005e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1246890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8aa061300","size":5,"soft_fail":false,"target_va":5373953504,"va":5372706609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140500a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1246870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e896061300","size":5,"soft_fail":false,"target_va":5373954592,"va":5372707717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140500c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1246339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e883041300","size":5,"soft_fail":false,"target_va":5373955152,"va":5372708808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140500e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1246907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8bb061300","size":5,"soft_fail":false,"target_va":5373955696,"va":5372708784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140501640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1249647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e86f111300","size":5,"soft_fail":false,"target_va":5373957696,"va":5372708044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140501e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1251215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e88f171300","size":5,"soft_fail":false,"target_va":5373959744,"va":5372708524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405028a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1254649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8f9241300","size":5,"soft_fail":false,"target_va":5373962400,"va":5372707746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140503240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1259158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e896361300","size":5,"soft_fail":false,"target_va":5373964864,"va":5372705701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140503460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1257385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8a92f1300","size":5,"soft_fail":false,"target_va":5373965408,"va":5372708018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140503950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1258979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8e3351300","size":5,"soft_fail":false,"target_va":5373966672,"va":5372707688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140503c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1258855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e867351300","size":5,"soft_fail":false,"target_va":5373967360,"va":5372708500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140503eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1261709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e88d401300","size":5,"soft_fail":false,"target_va":5373968048,"va":5372706334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140683c10
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2834842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e89a412b00","size":5,"soft_fail":false,"target_va":5375540240,"va":5372705393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068bee0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2868196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8e4c32b00","size":5,"soft_fail":false,"target_va":5375573728,"va":5372705527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14092e6b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5630660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8c4ea5500","size":5,"soft_fail":false,"target_va":5378336432,"va":5372705767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fad0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5701063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8c7fd5600","size":5,"soft_fail":false,"target_va":5378407120,"va":5372706052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a86b30
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7039806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e83e6b6b00","size":5,"soft_fail":false,"target_va":5379746608,"va":5372706797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=17
//     evidence={"assembly":"\tcallq\t2498334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e81e1f2600","size":5,"soft_fail":false,"target_va":5375203792,"va":5372705453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5701308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8bcfe5600","size":5,"soft_fail":false,"target_va":5378407056,"va":5372705743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5701499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e87bff5600","size":5,"soft_fail":false,"target_va":5378407392,"va":5372705888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::history_process::HistoryCmd::interrupt
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7495462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e8265f7200","size":5,"soft_fail":false,"target_va":5380201248,"va":5372705781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2835503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372705200,"raw_hex":"e82f442b00","size":5,"soft_fail":false,"target_va":5375540928,"va":5372705420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":170,"record_offset":10172,"parent_record_offset":10016,"ancestor_record_offset":10016,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":170,"record_offset":10576,"parent_record_offset":10016,"ancestor_record_offset":10016,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":170,"record_offset":10904,"parent_record_offset":10016,"ancestor_record_offset":10016,"inlinee":9992,"source_line":95,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::impl$2::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_state","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_state-0.16.0/src/state/resources.rs
//     evidence={"module":170,"record_offset":10492,"parent_record_offset":10016,"ancestor_record_offset":10016,"inlinee":12345,"source_line":81,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":170,"record_offset":14516,"parent_record_offset":10016,"ancestor_record_offset":10016,"inlinee":10705,"source_line":445,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::wide_safe_frame
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":170,"record_offset":10232,"parent_record_offset":10016,"ancestor_record_offset":10016,"inlinee":12679,"source_line":451,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":10364,"parent_record_offset":10332,"ancestor_record_offset":10016,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":10016,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":10016,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":10332,"parent_record_offset":10296,"ancestor_record_offset":10016,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":10296,"parent_record_offset":10256,"ancestor_record_offset":10016,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":170,"record_offset":10200,"parent_record_offset":10172,"ancestor_record_offset":10016,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":170,"record_offset":10600,"parent_record_offset":10576,"ancestor_record_offset":10016,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":170,"record_offset":11012,"parent_record_offset":10988,"ancestor_record_offset":10016,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":170,"record_offset":10988,"parent_record_offset":10904,"ancestor_record_offset":10016,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":170,"record_offset":10664,"parent_record_offset":10600,"ancestor_record_offset":10016,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":10516,"parent_record_offset":10492,"ancestor_record_offset":10016,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":10452,"parent_record_offset":10296,"ancestor_record_offset":10016,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] country_core::app_loading_state::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/app_loading_state.rs
//     evidence={"module":170,"record_offset":10540,"parent_record_offset":10516,"ancestor_record_offset":10016,"inlinee":12347,"source_line":1,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":170,"record_offset":10256,"parent_record_offset":10232,"ancestor_record_offset":10016,"inlinee":10701,"source_line":494,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":204,"record_offset":39632,"rva":5261808,"va":5373970928}
// 编译实例 RVAs: 0x5049f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140681d50
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1561323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373970928,"raw_hex":"e8ebd21700","size":5,"soft_fail":false,"target_va":5375532368,"va":5373971040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4435958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373970928,"raw_hex":"e8f6af4300","size":5,"soft_fail":false,"target_va":5378407120,"va":5373971157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t384465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373970928,"raw_hex":"e8d1dd0500","size":5,"soft_fail":false,"target_va":5374355568,"va":5373971098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1569299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373970928,"raw_hex":"e813f21700","size":5,"soft_fail":false,"target_va":5375540352,"va":5373971048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::PillButtonView::inactive_opacity
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1559187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373970928,"raw_hex":"e893ca1700","size":5,"soft_fail":false,"target_va":5375530224,"va":5373971032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::back_button
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1558390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373970928,"raw_hex":"e876c71700","size":5,"soft_fail":false,"target_va":5375529408,"va":5373971013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":40152,"parent_record_offset":39632,"ancestor_record_offset":39632,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":204,"record_offset":39784,"parent_record_offset":39632,"ancestor_record_offset":39632,"inlinee":9992,"source_line":95,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":40016,"parent_record_offset":39992,"ancestor_record_offset":39632,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":40064,"parent_record_offset":40040,"ancestor_record_offset":39632,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":40040,"parent_record_offset":40016,"ancestor_record_offset":39632,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":204,"record_offset":39992,"parent_record_offset":39952,"ancestor_record_offset":39632,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":204,"record_offset":39952,"parent_record_offset":39916,"ancestor_record_offset":39632,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":39836,"parent_record_offset":39808,"ancestor_record_offset":39632,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":39808,"parent_record_offset":39784,"ancestor_record_offset":39632,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":204,"record_offset":39892,"parent_record_offset":39868,"ancestor_record_offset":39632,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":204,"record_offset":39868,"parent_record_offset":39784,"ancestor_record_offset":39632,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":204,"record_offset":39916,"parent_record_offset":39892,"ancestor_record_offset":39632,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":40104,"parent_record_offset":39952,"ancestor_record_offset":39632,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A1A
// 首条位置: {"module":204,"inlinee":14874}
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
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A17
// 首条位置: {"module":204,"inlinee":14871}
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
// 源码行: 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A11
// 首条位置: {"module":204,"inlinee":14865}
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
// 源码行: 272
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A19
// 首条位置: {"module":204,"inlinee":14873}
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
// 源码行: 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A13
// 首条位置: {"module":204,"inlinee":14867}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 312
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A0E
// 首条位置: {"module":204,"inlinee":14862}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8
// 记录形式: compiled_procedure; instances=1
// 源码行: 324
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":204,"record_offset":40216,"rva":5262144,"va":5373971264}
// 编译实例 RVAs: 0x504b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1109592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e858ee1000","size":5,"soft_fail":false,"target_va":5375081488,"va":5373971891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1110046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e81ef01000","size":5,"soft_fail":false,"target_va":5375081504,"va":5373971453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1544663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e8d7911700","size":5,"soft_fail":false,"target_va":5375516032,"va":5373971364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067df20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1545034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e84a931700","size":5,"soft_fail":false,"target_va":5375516448,"va":5373971409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067e5b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1546478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e8ee981700","size":5,"soft_fail":false,"target_va":5375518128,"va":5373971645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140681d50
//     owner=unresolved_namespace; count=3
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1560945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e871d11700","size":5,"soft_fail":false,"target_va":5375532368,"va":5373971418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140681e50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1561247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e89fd21700","size":5,"soft_fail":false,"target_va":5375532624,"va":5373971372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068bfc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1602592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e820741800","size":5,"soft_fail":false,"target_va":5375573952,"va":5373971355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fad0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4435555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e863ae4300","size":5,"soft_fail":false,"target_va":5378407120,"va":5373971560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37568954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e8ba413d02","size":5,"soft_fail":false,"target_va":5411540992,"va":5373972033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t383487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e8ffd90500","size":5,"soft_fail":false,"target_va":5374354992,"va":5373971500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_uppercase
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t8872376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e8b8618700","size":5,"soft_fail":false,"target_va":5382843712,"va":5373971331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6916359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e807896900","size":5,"soft_fail":false,"target_va":5380887680,"va":5373971316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::default_focus
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1568919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e897f01700","size":5,"soft_fail":false,"target_va":5375540304,"va":5373971380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1568921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373971264,"raw_hex":"e899f01700","size":5,"soft_fail":false,"target_va":5375540352,"va":5373971426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":40996,"parent_record_offset":40216,"ancestor_record_offset":40216,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":204,"record_offset":40668,"parent_record_offset":40216,"ancestor_record_offset":40216,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":40440,"parent_record_offset":40216,"ancestor_record_offset":40216,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$29::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":204,"record_offset":41632,"parent_record_offset":40216,"ancestor_record_offset":40216,"inlinee":14879,"source_line":255,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::with_node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":204,"record_offset":40380,"parent_record_offset":40216,"ancestor_record_offset":40216,"inlinee":13179,"source_line":230,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":41808,"parent_record_offset":41784,"ancestor_record_offset":40216,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":41832,"parent_record_offset":41808,"ancestor_record_offset":40216,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":40616,"parent_record_offset":40592,"ancestor_record_offset":40216,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":41784,"parent_record_offset":41760,"ancestor_record_offset":40216,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":40592,"parent_record_offset":40536,"ancestor_record_offset":40216,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":204,"record_offset":41700,"parent_record_offset":41660,"ancestor_record_offset":40216,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":204,"record_offset":41728,"parent_record_offset":41700,"ancestor_record_offset":40216,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":204,"record_offset":41760,"parent_record_offset":41728,"ancestor_record_offset":40216,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":204,"record_offset":41936,"parent_record_offset":41660,"ancestor_record_offset":40216,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":40864,"parent_record_offset":40832,"ancestor_record_offset":40216,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":40564,"parent_record_offset":40536,"ancestor_record_offset":40216,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":40536,"parent_record_offset":40512,"ancestor_record_offset":40216,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":40912,"parent_record_offset":40888,"ancestor_record_offset":40216,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":40888,"parent_record_offset":40864,"ancestor_record_offset":40216,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":40512,"parent_record_offset":40488,"ancestor_record_offset":40216,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$56::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":204,"record_offset":41660,"parent_record_offset":41632,"ancestor_record_offset":40216,"inlinee":7957,"source_line":3466,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":204,"record_offset":40832,"parent_record_offset":40792,"ancestor_record_offset":40216,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":204,"record_offset":40792,"parent_record_offset":40756,"ancestor_record_offset":40216,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":40728,"parent_record_offset":40692,"ancestor_record_offset":40216,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":40692,"parent_record_offset":40668,"ancestor_record_offset":40216,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":204,"record_offset":40756,"parent_record_offset":40692,"ancestor_record_offset":40216,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":204,"record_offset":41960,"parent_record_offset":41936,"ancestor_record_offset":40216,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":40464,"parent_record_offset":40440,"ancestor_record_offset":40216,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":40952,"parent_record_offset":40792,"ancestor_record_offset":40216,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$28::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":204,"record_offset":41904,"parent_record_offset":41880,"ancestor_record_offset":40216,"inlinee":12273,"source_line":214,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":204,"record_offset":41880,"parent_record_offset":41660,"ancestor_record_offset":40216,"inlinee":10633,"source_line":422,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_escape_menu.rs
//     evidence={"module":204,"record_offset":40408,"parent_record_offset":40380,"ancestor_record_offset":40216,"inlinee":13171,"source_line":327,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3373
// 首条位置: {"module":149,"inlinee":13171}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$9
// 记录形式: compiled_procedure; instances=1
// 源码行: 356
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":204,"record_offset":42688,"rva":5263120,"va":5373972240}
// 编译实例 RVAs: 0x504f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14032e5d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1927553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972240,"raw_hex":"e87f96e2ff","size":5,"soft_fail":false,"target_va":5372044752,"va":5373972300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067e5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1545695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972240,"raw_hex":"e8df951700","size":5,"soft_fail":false,"target_va":5375518128,"va":5373972428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140681d50
//     owner=unresolved_namespace; count=3
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1560055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972240,"raw_hex":"e8f7cd1700","size":5,"soft_fail":false,"target_va":5375532368,"va":5373972308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fad0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4434759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972240,"raw_hex":"e847ab4300","size":5,"soft_fail":false,"target_va":5378407120,"va":5373972356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1568031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972240,"raw_hex":"e81fed1700","size":5,"soft_fail":false,"target_va":5375540352,"va":5373972316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":42836,"parent_record_offset":42688,"ancestor_record_offset":42688,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":42860,"parent_record_offset":42836,"ancestor_record_offset":42688,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$10
// 记录形式: compiled_procedure; instances=1
// 源码行: 374
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":204,"record_offset":43016,"rva":5263488,"va":5373972608}
// 编译实例 RVAs: 0x505080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4189602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972608,"raw_hex":"e85e12c0ff","size":5,"soft_fail":false,"target_va":5369783120,"va":5373972717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1108724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972608,"raw_hex":"e8f4ea1000","size":5,"soft_fail":false,"target_va":5375081504,"va":5373972775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140681d50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1559622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972608,"raw_hex":"e846cc1700","size":5,"soft_fail":false,"target_va":5375532368,"va":5373972741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068bfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1601255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972608,"raw_hex":"e8e76e1800","size":5,"soft_fail":false,"target_va":5375573952,"va":5373972692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4434307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972608,"raw_hex":"e883a94300","size":5,"soft_fail":false,"target_va":5378407120,"va":5373972808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_uppercase
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8871039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972608,"raw_hex":"e87f5c8700","size":5,"soft_fail":false,"target_va":5382843712,"va":5373972668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6915022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972608,"raw_hex":"e8ce836900","size":5,"soft_fail":false,"target_va":5380887680,"va":5373972653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1567598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972608,"raw_hex":"e86eeb1700","size":5,"soft_fail":false,"target_va":5375540352,"va":5373972749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1559646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972608,"raw_hex":"e85ecc1700","size":5,"soft_fail":false,"target_va":5375532352,"va":5373972701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":43432,"parent_record_offset":43016,"ancestor_record_offset":43016,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":43204,"parent_record_offset":43016,"ancestor_record_offset":43016,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":204,"record_offset":43172,"parent_record_offset":43016,"ancestor_record_offset":43016,"inlinee":10725,"source_line":364,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":43380,"parent_record_offset":43356,"ancestor_record_offset":43016,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":43356,"parent_record_offset":43300,"ancestor_record_offset":43016,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":43328,"parent_record_offset":43300,"ancestor_record_offset":43016,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":43300,"parent_record_offset":43276,"ancestor_record_offset":43016,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":43276,"parent_record_offset":43252,"ancestor_record_offset":43016,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":43460,"parent_record_offset":43432,"ancestor_record_offset":43016,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":43228,"parent_record_offset":43204,"ancestor_record_offset":43016,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 390
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A0C
// 首条位置: {"module":204,"inlinee":14860}
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
// 源码行: 399
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A18
// 首条位置: {"module":204,"inlinee":14872}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 418
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A0F
// 首条位置: {"module":204,"inlinee":14863}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$14
// 记录形式: compiled_procedure; instances=1
// 源码行: 427
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":204,"record_offset":43724,"rva":5263776,"va":5373972896}
// 编译实例 RVAs: 0x5051a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1108430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e8cee91000","size":5,"soft_fail":false,"target_va":5375081504,"va":5373973069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067e5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1545122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e8a2931700","size":5,"soft_fail":false,"target_va":5375518128,"va":5373973001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140681d50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1559195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e89bca1700","size":5,"soft_fail":false,"target_va":5375532368,"va":5373973168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140681e50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1559593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e829cc1700","size":5,"soft_fail":false,"target_va":5375532624,"va":5373973026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068bfc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1600962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e8c26d1800","size":5,"soft_fail":false,"target_va":5375573952,"va":5373972985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4434012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e85ca84300","size":5,"soft_fail":false,"target_va":5378407120,"va":5373973103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37344931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e8a3d63902","size":5,"soft_fail":false,"target_va":5411318240,"va":5373973304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_uppercase
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8870746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e85a5b8700","size":5,"soft_fail":false,"target_va":5382843712,"va":5373972961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6914729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e8a9826900","size":5,"soft_fail":false,"target_va":5380887680,"va":5373972946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::default_focus
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1567265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e821ea1700","size":5,"soft_fail":false,"target_va":5375540304,"va":5373973034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1567305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e849ea1700","size":5,"soft_fail":false,"target_va":5375540352,"va":5373973042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1545394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373972896,"raw_hex":"e8b2941700","size":5,"soft_fail":false,"target_va":5375518416,"va":5373973017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":44108,"parent_record_offset":43724,"ancestor_record_offset":43724,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":204,"record_offset":44396,"parent_record_offset":43724,"ancestor_record_offset":43724,"inlinee":9431,"source_line":851,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":43880,"parent_record_offset":43724,"ancestor_record_offset":43724,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":44056,"parent_record_offset":44032,"ancestor_record_offset":43724,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":204,"record_offset":44032,"parent_record_offset":43976,"ancestor_record_offset":43724,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":44512,"parent_record_offset":44484,"ancestor_record_offset":43724,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":44592,"parent_record_offset":44568,"ancestor_record_offset":43724,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":44004,"parent_record_offset":43976,"ancestor_record_offset":43724,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":43976,"parent_record_offset":43952,"ancestor_record_offset":43724,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":44568,"parent_record_offset":44540,"ancestor_record_offset":43724,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":44540,"parent_record_offset":44512,"ancestor_record_offset":43724,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":204,"record_offset":43952,"parent_record_offset":43928,"ancestor_record_offset":43724,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":204,"record_offset":44484,"parent_record_offset":44452,"ancestor_record_offset":43724,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":204,"record_offset":44136,"parent_record_offset":44108,"ancestor_record_offset":43724,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":204,"record_offset":44424,"parent_record_offset":44396,"ancestor_record_offset":43724,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":204,"record_offset":44452,"parent_record_offset":44424,"ancestor_record_offset":43724,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":204,"record_offset":43904,"parent_record_offset":43880,"ancestor_record_offset":43724,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00204.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$15
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 446
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3A0A
// 首条位置: {"module":204,"inlinee":14858}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::wishlist_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 473
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":16044,"rva":3999728,"va":5372708848}
// 编译实例 RVAs: 0x3d07f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2925973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e86b5ad3ff","size":5,"soft_fail":false,"target_va":5369783120,"va":5372709088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14026d3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1455495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e879cae9ff","size":5,"soft_fail":false,"target_va":5371253696,"va":5372709186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14032e0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-665518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e852d8f5ff","size":5,"soft_fail":false,"target_va":5372043440,"va":5372708953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2372290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e8c2322400","size":5,"soft_fail":false,"target_va":5375081504,"va":5372709209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140681d50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2823210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e82a142b00","size":5,"soft_fail":false,"target_va":5375532368,"va":5372709153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068bfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2864883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e8f3b62b00","size":5,"soft_fail":false,"target_va":5375573952,"va":5372709064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_uppercase
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10134722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e8c2a49a00","size":5,"soft_fail":false,"target_va":5382843712,"va":5372708985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8178705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e811cc7c00","size":5,"soft_fail":false,"target_va":5380887680,"va":5372708970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2831186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e852332b00","size":5,"soft_fail":false,"target_va":5375540352,"va":5372709161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::align
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2809923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e843e02a00","size":5,"soft_fail":false,"target_va":5375519072,"va":5372709144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2823274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e86a142b00","size":5,"soft_fail":false,"target_va":5375532352,"va":5372709073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2809859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e803e02a00","size":5,"soft_fail":false,"target_va":5375518992,"va":5372709128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9174044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e81cfc8b00","size":5,"soft_fail":false,"target_va":5381883088,"va":5372709039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::linearstep
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9174036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372708848,"raw_hex":"e814fc8b00","size":5,"soft_fail":false,"target_va":5381883056,"va":5372709015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":16220,"parent_record_offset":16044,"ancestor_record_offset":16044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":170,"record_offset":16184,"parent_record_offset":16044,"ancestor_record_offset":16044,"inlinee":10725,"source_line":364,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":16396,"parent_record_offset":16372,"ancestor_record_offset":16044,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":16372,"parent_record_offset":16316,"ancestor_record_offset":16044,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":16344,"parent_record_offset":16316,"ancestor_record_offset":16044,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":16316,"parent_record_offset":16292,"ancestor_record_offset":16044,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":16292,"parent_record_offset":16268,"ancestor_record_offset":16044,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":16244,"parent_record_offset":16220,"ancestor_record_offset":16044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::open_community_server
// 记录形式: compiled_procedure; instances=1
// 源码行: 492
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":16680,"rva":4000208,"va":5372709328}
// 编译实例 RVAs: 0x3d09d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14026d580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1455419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372709328,"raw_hex":"e8c5cae9ff","size":5,"soft_fail":false,"target_va":5371254144,"va":5372709558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cf2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372709328,"raw_hex":"e8cce8ffff","size":5,"soft_fail":false,"target_va":5372703456,"va":5372709391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2372052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372709328,"raw_hex":"e8d4312400","size":5,"soft_fail":false,"target_va":5375081488,"va":5372709431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38831363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372709328,"raw_hex":"e803855002","size":5,"soft_fail":false,"target_va":5411540963,"va":5372709595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":17048,"parent_record_offset":16680,"ancestor_record_offset":16680,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] obfstr::bytes::deobfuscate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/obfstr-0.4.3/src/bytes.rs
//     evidence={"module":170,"record_offset":16884,"parent_record_offset":16680,"ancestor_record_offset":16680,"inlinee":13899,"source_line":171,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] obfstr::xref::xref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/obfstr-0.4.3/src/xref.rs
//     evidence={"module":170,"record_offset":16828,"parent_record_offset":16680,"ancestor_record_offset":16680,"inlinee":13897,"source_line":78,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":17428,"parent_record_offset":17404,"ancestor_record_offset":16680,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":17452,"parent_record_offset":17428,"ancestor_record_offset":16680,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":17404,"parent_record_offset":17376,"ancestor_record_offset":16680,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":17316,"parent_record_offset":17288,"ancestor_record_offset":16680,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":17376,"parent_record_offset":17344,"ancestor_record_offset":16680,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":17344,"parent_record_offset":17316,"ancestor_record_offset":16680,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":17252,"parent_record_offset":17220,"ancestor_record_offset":16680,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":17220,"parent_record_offset":17188,"ancestor_record_offset":16680,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":17156,"parent_record_offset":17112,"ancestor_record_offset":16680,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":17188,"parent_record_offset":17156,"ancestor_record_offset":16680,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":17112,"parent_record_offset":17080,"ancestor_record_offset":16680,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":170,"record_offset":17080,"parent_record_offset":17048,"ancestor_record_offset":16680,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":17288,"parent_record_offset":17252,"ancestor_record_offset":16680,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::black_box
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":170,"record_offset":16852,"parent_record_offset":16828,"ancestor_record_offset":16680,"inlinee":8073,"source_line":476,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":170,"record_offset":17528,"parent_record_offset":17504,"ancestor_record_offset":16680,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":170,"record_offset":17504,"parent_record_offset":17252,"ancestor_record_offset":16680,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":16932,"parent_record_offset":16884,"ancestor_record_offset":16680,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":17016,"parent_record_offset":16884,"ancestor_record_offset":16680,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::open_newsletter
// 记录形式: compiled_procedure; instances=1
// 源码行: 497
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":17592,"rva":4000496,"va":5372709616}
// 编译实例 RVAs: 0x3d0af0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14026d580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1455802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372709616,"raw_hex":"e846c9e9ff","size":5,"soft_fail":false,"target_va":5371254144,"va":5372709941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cf310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372709616,"raw_hex":"e8d7e7ffff","size":5,"soft_fail":false,"target_va":5372703504,"va":5372709684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2371738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372709616,"raw_hex":"e89a302400","size":5,"soft_fail":false,"target_va":5375081488,"va":5372709745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38830977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372709616,"raw_hex":"e881835002","size":5,"soft_fail":false,"target_va":5411540963,"va":5372709981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":17888,"parent_record_offset":17592,"ancestor_record_offset":17592,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] obfstr::bytes::deobfuscate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/obfstr-0.4.3/src/bytes.rs
//     evidence={"module":170,"record_offset":17792,"parent_record_offset":17592,"ancestor_record_offset":17592,"inlinee":13899,"source_line":171,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] obfstr::xref::xref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/obfstr-0.4.3/src/xref.rs
//     evidence={"module":170,"record_offset":17736,"parent_record_offset":17592,"ancestor_record_offset":17592,"inlinee":13897,"source_line":78,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":18268,"parent_record_offset":18244,"ancestor_record_offset":17592,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":18292,"parent_record_offset":18268,"ancestor_record_offset":17592,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":18244,"parent_record_offset":18216,"ancestor_record_offset":17592,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":18156,"parent_record_offset":18128,"ancestor_record_offset":17592,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":18216,"parent_record_offset":18184,"ancestor_record_offset":17592,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":18184,"parent_record_offset":18156,"ancestor_record_offset":17592,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":18092,"parent_record_offset":18060,"ancestor_record_offset":17592,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":18060,"parent_record_offset":18028,"ancestor_record_offset":17592,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":17996,"parent_record_offset":17952,"ancestor_record_offset":17592,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":18028,"parent_record_offset":17996,"ancestor_record_offset":17592,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":17952,"parent_record_offset":17920,"ancestor_record_offset":17592,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":170,"record_offset":17920,"parent_record_offset":17888,"ancestor_record_offset":17592,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":18128,"parent_record_offset":18092,"ancestor_record_offset":17592,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::black_box
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":170,"record_offset":17760,"parent_record_offset":17736,"ancestor_record_offset":17592,"inlinee":8073,"source_line":476,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":170,"record_offset":18368,"parent_record_offset":18344,"ancestor_record_offset":17592,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":170,"record_offset":18344,"parent_record_offset":18092,"ancestor_record_offset":17592,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":17828,"parent_record_offset":17792,"ancestor_record_offset":17592,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00170.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >", "tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "country_core::systems::steam::SteamOverlayOpenResources", "bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>", "bevy_ecs::system::commands::Commands", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu
// 信息 ID: b13d303b213311142aa2e9b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 10d4d77f1b227b6d8f5aaf49: void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)> > @ 0xaee13 -> 0x3cf9b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::event::writer::EventWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)> > > @ 0x442b65 -> 0x3cf9b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$0
// 信息 ID: 9f20ddf59700a31f5609a939
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$0,ref$<str$> > @ 0x503442 -> 0x5047f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$1
// 信息 ID: eb5cf4ce701617d1531c1e75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$1,ref$<str$> > @ 0x5007e2 -> 0x5049f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$10
// 信息 ID: 2d2482196edd19cc67133d17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$10 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$10,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$10,ref$<str$> > @ 0x500e52 -> 0x505080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$11
// 信息 ID: 9a3f0075394ac7a57c48e913
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 11, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11,ref$<str$> >; module=204, inline_record=10836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$12
// 信息 ID: 17fd63c4d1373a72c0a4617d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 12, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12,ref$<str$> >; module=204, inline_record=31052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$13
// 信息 ID: 2597d7d1de1b58421f4fb88d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 13, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$13 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$13,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$13,ref$<str$> >; module=204, inline_record=14612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$14
// 信息 ID: 466931a13e1287895023ae41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 14, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$16,ref$<str$> > @ 0x502ab7 -> 0x5051a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$15
// 信息 ID: 4ff72e4e95cdcd5f810af1f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 15, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$15 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$15,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$15,ref$<str$> >; module=204, inline_record=8580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$2
// 信息 ID: af62ad8b67c81884a04574de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$2,ref$<str$> >; module=204, inline_record=32596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$3
// 信息 ID: a0724ea5c7bb684685cad258
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3,ref$<str$> >; module=204, inline_record=29556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$4
// 信息 ID: 990e0b2aacd11b1773dd902f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4,ref$<str$> >; module=204, inline_record=19144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$5
// 信息 ID: 4465405ad33761f47bd3e90d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5,ref$<str$> >; module=204, inline_record=31824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$6
// 信息 ID: 1b55cd93302e76c64929ba64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6,ref$<str$> >; module=204, inline_record=22204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$7
// 信息 ID: 140c5210e88cda1a20d0eda1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$7 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$7,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$7,ref$<str$> >; module=204, inline_record=12408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8
// 信息 ID: 351189be3e0a2f3bb6588d84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$8 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$8,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$8,ref$<str$> > @ 0x4fe9a7 -> 0x504b40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8::closure$0
// 信息 ID: d60e92a148f739474923cc23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8; module=204, inline_record=40408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$9
// 信息 ID: 96e8611e47935e5585784062
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$9 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$9,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$9,ref$<str$> > @ 0x501072 -> 0x504f10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::open_community_server
// 信息 ID: 81f939c5283e370a92b07a6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6,ref$<str$> > @ 0x5020cc -> 0x3d09d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3,ref$<str$> > @ 0x5036ec -> 0x3d09d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12,ref$<str$> > @ 0x503bdc -> 0x3d09d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::open_newsletter
// 信息 ID: c90d66a3a67ef90fd5031741
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5,ref$<str$> > @ 0x503e8c -> 0x3d0af0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::wishlist_button
// 信息 ID: fcde80b6558f14012464aa94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$13,ref$<str$> > @ 0x500c2a -> 0x3d07f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4,ref$<str$> > @ 0x50184a -> 0x3d07f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/crash-reporter-gui/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 536a6edc1bae489cec3043943fbe053723460afc6adb25bed5d13a5f593c5e6d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 68, 261, 716, 717, 719, 725, 1867
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arboard; // namespace candidate
//   package identities: [{"name":"arboard","source":"PDB_registry_path_only","version":"3.4.0"}]
// use core; // namespace candidate
//   package identities: []
// use crash_reporter_gui; // namespace candidate
//   package identities: []
// use iced_core; // namespace candidate
//   package identities: []
// use iced_runtime; // namespace candidate
//   package identities: []
// use iced_style; // namespace candidate
//   package identities: []
// use iced_widget; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// use winit; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.4.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::Controls::main_view
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":261,"record_offset":22788,"rva":4078896,"va":5372788016}
// 编译实例 RVAs: 0x3e3d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$core..iter..adapters..chain..Chain$LT$A$C$B$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5103642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e81ae04d00","size":5,"soft_fail":false,"target_va":5377908944,"va":5372805297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=30
//     evidence={"assembly":"\tcallq\t4800104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8683e4900","size":5,"soft_fail":false,"target_va":5377588464,"va":5372788355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140045d50
//     owner=unresolved_namespace; count=58
//     target_aliases=101 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3811656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8b8d6c5ff","size":5,"soft_fail":false,"target_va":5368995152,"va":5372806803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cf70
//     owner=unresolved_namespace; count=22
//     target_aliases=31 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-580845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e81323f7ff","size":5,"soft_fail":false,"target_va":5372235632,"va":5372816472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=70
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8c568ffff","size":5,"soft_fail":false,"target_va":5372749696,"va":5372788406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=73
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-41767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8d95cffff","size":5,"soft_fail":false,"target_va":5372749712,"va":5372791474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5040104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8e8e74c00","size":5,"soft_fail":false,"target_va":5377856000,"va":5372815891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9350
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5039331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8e3e44c00","size":5,"soft_fail":false,"target_va":5377856336,"va":5372817000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bf3b0
//     owner=unresolved_namespace; count=11
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5086545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8519d4d00","size":5,"soft_fail":false,"target_va":5377881008,"va":5372794458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bf3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=22 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5075192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8f8704d00","size":5,"soft_fail":false,"target_va":5377881024,"va":5372805827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30801146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8fafcd501","size":5,"soft_fail":false,"target_va":5403593360,"va":5372792209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=70
//     evidence={"assembly":"\tcallq\t30167372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e84c51cc01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372810767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4738807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8f74e4800","size":5,"soft_fail":false,"target_va":5377529888,"va":5372791076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30126762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8aab2cb01","size":5,"soft_fail":false,"target_va":5402919072,"va":5372792305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=30
//     evidence={"assembly":"\tcallq\t-1483690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8565ce9ff","size":5,"soft_fail":false,"target_va":5371304880,"va":5372788565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4715196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8bcf24700","size":5,"soft_fail":false,"target_va":5377527120,"va":5372811919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_core..element..Map$LT$tg_integrity_report..controls..Message$C$crash_reporter_gui..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5041374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e8deec4c00","size":5,"soft_fail":false,"target_va":5377857152,"va":5372815773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$crash_reporter_gui..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=20
//     evidence={"assembly":"\tcallq\t5037105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e831dc4c00","size":5,"soft_fail":false,"target_va":5377855616,"va":5372818506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=30
//     evidence={"assembly":"\tcallq\t30169208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e87858cc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372810915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::pattern::StrSearcher::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4801578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e82a444900","size":5,"soft_fail":false,"target_va":5377593648,"va":5372792065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e88abdffff","size":5,"soft_fail":false,"target_va":5372775232,"va":5372792241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5075300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e864714d00","size":5,"soft_fail":false,"target_va":5377885328,"va":5372810023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t5086476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e80c9d4d00","size":5,"soft_fail":false,"target_va":5377891056,"va":5372804575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::row
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5092951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e857b64d00","size":5,"soft_fail":false,"target_va":5377890448,"va":5372797492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::row::Row$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t5106999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e837ed4d00","size":5,"soft_fail":false,"target_va":5377902688,"va":5372795684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t3364752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e890573300","size":5,"soft_fail":false,"target_va":5376152880,"va":5372788123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::controls::Controls::verification_status
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5072675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372788016,"raw_hex":"e823674d00","size":5,"soft_fail":false,"target_va":5377862560,"va":5372789880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5372788016,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5372806727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":27568,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":261,"record_offset":77448,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":261,"record_offset":30376,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":261,"record_offset":28720,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":261,"record_offset":32944,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14514,"source_line":271,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":23032,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":24116,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":261,"record_offset":23552,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":31616,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=21
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":23116,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14471,"source_line":231,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::wide_button
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":38836,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14487,"source_line":415,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_core::element::Element::map
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":261,"record_offset":28804,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14510,"source_line":212,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":261,"record_offset":49980,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":40712,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14495,"source_line":112,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::padding
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":51652,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14525,"source_line":104,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":51624,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14494,"source_line":127,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":41952,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14497,"source_line":307,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::checkbox::Checkbox::on_toggle
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/checkbox.rs
//     evidence={"module":261,"record_offset":63800,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14529,"source_line":103,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::align_items
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":75288,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14484,"source_line":100,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::padding
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":77016,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14535,"source_line":76,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":65656,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14498,"source_line":70,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":65688,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14486,"source_line":333,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::style
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":261,"record_offset":68188,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14534,"source_line":133,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":261,"record_offset":50924,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14488,"source_line":116,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::checkbox
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":261,"record_offset":63164,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14526,"source_line":159,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":261,"record_offset":66488,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14478,"source_line":68,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":261,"record_offset":67668,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14501,"source_line":57,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::align_items
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":261,"record_offset":42520,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14521,"source_line":91,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":261,"record_offset":42552,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14522,"source_line":67,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/space.rs
//     evidence={"module":261,"record_offset":67376,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":14533,"source_line":95,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":261,"record_offset":40072,"parent_record_offset":22788,"ancestor_record_offset":22788,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":23768,"parent_record_offset":23744,"ancestor_record_offset":22788,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":23792,"parent_record_offset":23768,"ancestor_record_offset":22788,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=73
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":23492,"parent_record_offset":23464,"ancestor_record_offset":22788,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=64
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":23712,"parent_record_offset":23672,"ancestor_record_offset":22788,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":23744,"parent_record_offset":23712,"ancestor_record_offset":22788,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=73
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":23464,"parent_record_offset":23400,"ancestor_record_offset":22788,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=64
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":261,"record_offset":23672,"parent_record_offset":23632,"ancestor_record_offset":22788,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":261,"record_offset":69768,"parent_record_offset":69740,"ancestor_record_offset":22788,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":261,"record_offset":30424,"parent_record_offset":30400,"ancestor_record_offset":22788,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":33368,"parent_record_offset":33344,"ancestor_record_offset":22788,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":23432,"parent_record_offset":23400,"ancestor_record_offset":22788,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":33344,"parent_record_offset":33316,"ancestor_record_offset":22788,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":24192,"parent_record_offset":24160,"ancestor_record_offset":22788,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":33316,"parent_record_offset":33292,"ancestor_record_offset":22788,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":40400,"parent_record_offset":40368,"ancestor_record_offset":22788,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":40368,"parent_record_offset":40336,"ancestor_record_offset":22788,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":40556,"parent_record_offset":40368,"ancestor_record_offset":22788,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":23400,"parent_record_offset":23372,"ancestor_record_offset":22788,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":261,"record_offset":40300,"parent_record_offset":40268,"ancestor_record_offset":22788,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":261,"record_offset":28744,"parent_record_offset":28720,"ancestor_record_offset":22788,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":261,"record_offset":40268,"parent_record_offset":40236,"ancestor_record_offset":22788,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":261,"record_offset":63340,"parent_record_offset":63312,"ancestor_record_offset":22788,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":261,"record_offset":40236,"parent_record_offset":40204,"ancestor_record_offset":22788,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":261,"record_offset":63312,"parent_record_offset":63284,"ancestor_record_offset":22788,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":261,"record_offset":63284,"parent_record_offset":63256,"ancestor_record_offset":22788,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":23204,"parent_record_offset":23152,"ancestor_record_offset":22788,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":33096,"parent_record_offset":32944,"ancestor_record_offset":22788,"inlinee":4956,"source_line":1066,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$21::fmt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":30628,"parent_record_offset":30604,"ancestor_record_offset":22788,"inlinee":4475,"source_line":2366,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":23152,"parent_record_offset":23116,"ancestor_record_offset":22788,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":63256,"parent_record_offset":63228,"ancestor_record_offset":22788,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":33180,"parent_record_offset":33152,"ancestor_record_offset":22788,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":24160,"parent_record_offset":24116,"ancestor_record_offset":22788,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":33124,"parent_record_offset":33096,"ancestor_record_offset":22788,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":28768,"parent_record_offset":28744,"ancestor_record_offset":22788,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":33416,"parent_record_offset":33180,"ancestor_record_offset":22788,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":33292,"parent_record_offset":33180,"ancestor_record_offset":22788,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":40336,"parent_record_offset":40300,"ancestor_record_offset":22788,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":40204,"parent_record_offset":40168,"ancestor_record_offset":22788,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":261,"record_offset":33152,"parent_record_offset":33124,"ancestor_record_offset":22788,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":261,"record_offset":69800,"parent_record_offset":69768,"ancestor_record_offset":22788,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":261,"record_offset":40668,"parent_record_offset":40168,"ancestor_record_offset":22788,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":261,"record_offset":39412,"parent_record_offset":39360,"ancestor_record_offset":22788,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":261,"record_offset":23232,"parent_record_offset":23152,"ancestor_record_offset":22788,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":261,"record_offset":23260,"parent_record_offset":23152,"ancestor_record_offset":22788,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":261,"record_offset":40620,"parent_record_offset":40596,"ancestor_record_offset":22788,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":261,"record_offset":66648,"parent_record_offset":66616,"ancestor_record_offset":22788,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":261,"record_offset":69828,"parent_record_offset":69800,"ancestor_record_offset":22788,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":261,"record_offset":33512,"parent_record_offset":33484,"ancestor_record_offset":22788,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":261,"record_offset":30400,"parent_record_offset":30376,"ancestor_record_offset":22788,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":261,"record_offset":40596,"parent_record_offset":40300,"ancestor_record_offset":22788,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=274
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":23324,"parent_record_offset":23152,"ancestor_record_offset":22788,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":23820,"parent_record_offset":23792,"ancestor_record_offset":22788,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":24224,"parent_record_offset":24116,"ancestor_record_offset":22788,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":23288,"parent_record_offset":23152,"ancestor_record_offset":22788,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":261,"record_offset":33552,"parent_record_offset":32944,"ancestor_record_offset":22788,"inlinee":12626,"source_line":507,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::match_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":261,"record_offset":33040,"parent_record_offset":32944,"ancestor_record_offset":22788,"inlinee":14515,"source_line":1882,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::MatchIndicesInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":261,"record_offset":33484,"parent_record_offset":33460,"ancestor_record_offset":22788,"inlinee":14519,"source_line":1054,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$91::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":261,"record_offset":33460,"parent_record_offset":32944,"ancestor_record_offset":22788,"inlinee":14517,"source_line":506,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":261,"record_offset":33064,"parent_record_offset":33040,"ancestor_record_offset":22788,"inlinee":7267,"source_line":956,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":261,"record_offset":33576,"parent_record_offset":33552,"ancestor_record_offset":22788,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":38884,"parent_record_offset":38836,"ancestor_record_offset":22788,"inlinee":14471,"source_line":231,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=64
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":261,"record_offset":23632,"parent_record_offset":23592,"ancestor_record_offset":22788,"inlinee":14476,"source_line":32,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Map::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":261,"record_offset":28840,"parent_record_offset":28804,"ancestor_record_offset":22788,"inlinee":14512,"source_line":270,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::length::Length::fluid
//     owner=git_dependency_definition; count=15
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/length.rs
//     evidence={"module":261,"record_offset":39976,"parent_record_offset":39360,"ancestor_record_offset":22788,"inlinee":14492,"source_line":51,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=25
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":261,"record_offset":23592,"parent_record_offset":23552,"ancestor_record_offset":22788,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::new
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":39360,"parent_record_offset":39320,"ancestor_record_offset":22788,"inlinee":14490,"source_line":74,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":40040,"parent_record_offset":38836,"ancestor_record_offset":22788,"inlinee":14494,"source_line":127,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::width
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":40012,"parent_record_offset":38836,"ancestor_record_offset":22788,"inlinee":14493,"source_line":92,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":45548,"parent_record_offset":45512,"ancestor_record_offset":22788,"inlinee":14497,"source_line":307,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::checkbox::Checkbox::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/checkbox.rs
//     evidence={"module":261,"record_offset":63192,"parent_record_offset":63164,"ancestor_record_offset":22788,"inlinee":14528,"source_line":75,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::checkbox::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/checkbox.rs
//     evidence={"module":261,"record_offset":63864,"parent_record_offset":63828,"ancestor_record_offset":22788,"inlinee":14531,"source_line":372,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":66616,"parent_record_offset":66520,"ancestor_record_offset":22788,"inlinee":14483,"source_line":145,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":66580,"parent_record_offset":66552,"ancestor_record_offset":22788,"inlinee":14482,"source_line":50,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":66552,"parent_record_offset":66520,"ancestor_record_offset":22788,"inlinee":14481,"source_line":32,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":66520,"parent_record_offset":66488,"ancestor_record_offset":22788,"inlinee":14480,"source_line":37,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":66764,"parent_record_offset":66728,"ancestor_record_offset":22788,"inlinee":14486,"source_line":333,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":261,"record_offset":67708,"parent_record_offset":67668,"ancestor_record_offset":22788,"inlinee":14503,"source_line":50,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":261,"record_offset":68256,"parent_record_offset":68220,"ancestor_record_offset":22788,"inlinee":14509,"source_line":313,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::helpers::button
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":261,"record_offset":39320,"parent_record_offset":38836,"ancestor_record_offset":22788,"inlinee":14488,"source_line":116,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":261,"record_offset":42616,"parent_record_offset":42580,"ancestor_record_offset":22788,"inlinee":14524,"source_line":322,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/space.rs
//     evidence={"module":261,"record_offset":64460,"parent_record_offset":64432,"ancestor_record_offset":22788,"inlinee":14533,"source_line":95,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":261,"record_offset":40104,"parent_record_offset":40072,"ancestor_record_offset":22788,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":261,"record_offset":40136,"parent_record_offset":40104,"ancestor_record_offset":22788,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":261,"record_offset":40168,"parent_record_offset":40136,"ancestor_record_offset":22788,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$2::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":261,"record_offset":10688,"rva":4068176,"va":5372777296}
// 编译实例 RVAs: 0x3e1350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400056b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4046180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e89c42c2ff","size":5,"soft_fail":false,"target_va":5368731312,"va":5372777487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140005a60
//     owner=unresolved_namespace; count=1
//     target_aliases=118 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4047133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e8e33ec2ff","size":5,"soft_fail":false,"target_va":5368732256,"va":5372779384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140045d50
//     owner=unresolved_namespace; count=1
//     target_aliases=101 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3784500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e8cc40c6ff","size":5,"soft_fail":false,"target_va":5368995152,"va":5372779647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=3
//     target_aliases=89 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-463783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e859ecf8ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5372777986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e89a92ffff","size":5,"soft_fail":false,"target_va":5372749712,"va":5372777713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30210818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e802fbcc01","size":5,"soft_fail":false,"target_va":5402989616,"va":5372778793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4586116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e884fa4500","size":5,"soft_fail":false,"target_va":5377364336,"va":5372778215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arboard::Clipboard::set
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t55284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e8f4d70000","size":5,"soft_fail":false,"target_va":5372833328,"va":5372778039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arboard::platform::windows::Set::text
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5100112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e850d24d00","size":5,"soft_fail":false,"target_va":5377878240,"va":5372778123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30200174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e86ed1cc01","size":5,"soft_fail":false,"target_va":5402979152,"va":5372778973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$std..thread..JoinHandle$LT$core..result..Result$LT$alloc..string..String$C$anyhow..Error$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5074913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e8e16f4d00","size":5,"soft_fail":false,"target_va":5377854768,"va":5372779850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<crash_reporter_gui::CurrentState> >
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-4795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e845edffff","size":5,"soft_fail":false,"target_va":5372772960,"va":5372777750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30201068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e8ecd4cc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372779055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4575377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e891d04500","size":5,"soft_fail":false,"target_va":5377353776,"va":5372778394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] open::that
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5097679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e8cfc84d00","size":5,"soft_fail":false,"target_va":5377875648,"va":5372777964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::JoinHandle$LT$T$GT$::join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5100709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e8a5d44d00","size":5,"soft_fail":false,"target_va":5377879264,"va":5372778550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4664773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e8c52d4700","size":5,"soft_fail":false,"target_va":5377442944,"va":5372778166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] webbrowser::open
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3215720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372777296,"raw_hex":"e868113100","size":5,"soft_fail":false,"target_va":5375993248,"va":5372777523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5372777296,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5372777675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$47::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":261,"record_offset":11116,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":5521,"source_line":2019,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":261,"record_offset":12024,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":261,"record_offset":12052,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":261,"record_offset":11000,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":10912,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":11780,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":12372,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::copy_to_clipboard
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":11440,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":14463,"source_line":409,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::none
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/command.rs
//     evidence={"module":261,"record_offset":12472,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":14470,"source_line":30,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::window::close
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/window.rs
//     evidence={"module":261,"record_offset":10856,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":14461,"source_line":50,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":261,"record_offset":12080,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":261,"record_offset":11940,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] std::thread::JoinHandle::is_finished
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":261,"record_offset":12108,"parent_record_offset":10688,"ancestor_record_offset":10688,"inlinee":8522,"source_line":1672,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":11256,"parent_record_offset":11232,"ancestor_record_offset":10688,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":11232,"parent_record_offset":11144,"ancestor_record_offset":10688,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":261,"record_offset":11144,"parent_record_offset":11116,"ancestor_record_offset":10688,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":11612,"parent_record_offset":11588,"ancestor_record_offset":10688,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":261,"record_offset":12156,"parent_record_offset":12132,"ancestor_record_offset":10688,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::strong_count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":261,"record_offset":12132,"parent_record_offset":12108,"ancestor_record_offset":10688,"inlinee":8523,"source_line":1703,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] arboard::Clipboard::set_text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arboard","version":"3.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arboard-3.4.0/src/lib.rs
//     evidence={"module":261,"record_offset":11476,"parent_record_offset":11440,"ancestor_record_offset":10688,"inlinee":14465,"source_line":93,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] arboard::Set::text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arboard","version":"3.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arboard-3.4.0/src/lib.rs
//     evidence={"module":261,"record_offset":11512,"parent_record_offset":11476,"ancestor_record_offset":10688,"inlinee":14467,"source_line":203,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":261,"record_offset":11176,"parent_record_offset":11144,"ancestor_record_offset":10688,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":261,"record_offset":11024,"parent_record_offset":11000,"ancestor_record_offset":10688,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":261,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":10688,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":10936,"parent_record_offset":10912,"ancestor_record_offset":10688,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":261,"record_offset":12188,"parent_record_offset":12156,"ancestor_record_offset":10688,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":11052,"parent_record_offset":11024,"ancestor_record_offset":10688,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":11080,"parent_record_offset":11024,"ancestor_record_offset":10688,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":11712,"parent_record_offset":11440,"ancestor_record_offset":10688,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":11748,"parent_record_offset":11440,"ancestor_record_offset":10688,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":261,"record_offset":11964,"parent_record_offset":11940,"ancestor_record_offset":10688,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":261,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":10688,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$2::update::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":11812,"parent_record_offset":11780,"ancestor_record_offset":10688,"inlinee":14469,"source_line":286,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::single
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/command.rs
//     evidence={"module":261,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":10688,"inlinee":14462,"source_line":35,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$2::view
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":261,"record_offset":12812,"rva":4070832,"va":5372779952}
// 编译实例 RVAs: 0x3e1db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t4807446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e8165b4900","size":5,"soft_fail":false,"target_va":5377588464,"va":5372781013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140045d50
//     owner=unresolved_namespace; count=9
//     target_aliases=101 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3790680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e8a828c6ff","size":5,"soft_fail":false,"target_va":5368995152,"va":5372785827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3148061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e8e3f6cfff","size":5,"soft_fail":false,"target_va":5369632448,"va":5372780504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cf70
//     owner=unresolved_namespace; count=4
//     target_aliases=31 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-550335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e8419af7ff","size":5,"soft_fail":false,"target_va":5372235632,"va":5372785962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e85a87ffff","size":5,"soft_fail":false,"target_va":5372749696,"va":5372780577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e88087ffff","size":5,"soft_fail":false,"target_va":5372749712,"va":5372780555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9350
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5070440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e8685e4d00","size":5,"soft_fail":false,"target_va":5377856336,"va":5372785891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9490
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5070891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e82b604d00","size":5,"soft_fail":false,"target_va":5377856656,"va":5372785760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bf3b0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5098144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e8a0ca4d00","size":5,"soft_fail":false,"target_va":5377881008,"va":5372782859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bf3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=22 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5096283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e85bc34d00","size":5,"soft_fail":false,"target_va":5377881024,"va":5372784736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bf590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5100018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e8f2d14d00","size":5,"soft_fail":false,"target_va":5377881488,"va":5372781465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t30193177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e819b6cc01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372784962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4749398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e856784800","size":5,"soft_fail":false,"target_va":5377529888,"va":5372780485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-1475855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e8f17ae9ff","size":5,"soft_fail":false,"target_va":5371304880,"va":5372780730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30194136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e8d8b9cc01","size":5,"soft_fail":false,"target_va":5402979152,"va":5372785011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$crash_reporter_gui..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5068824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e818584d00","size":5,"soft_fail":false,"target_va":5377855616,"va":5372786787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30195069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e87dbdcc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372785054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_reporter_gui::Controls::main_view
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e8201f0000","size":5,"soft_fail":false,"target_va":5372788016,"va":5372780043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5101961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e889d94d00","size":5,"soft_fail":false,"target_va":5377885328,"va":5372783362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5106728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e828ec4d00","size":5,"soft_fail":false,"target_va":5377891056,"va":5372784323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t3372669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e87d763300","size":5,"soft_fail":false,"target_va":5376152880,"va":5372780206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4663665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e871294700","size":5,"soft_fail":false,"target_va":5377443760,"va":5372780090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4662874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372779952,"raw_hex":"e85a264700","size":5,"soft_fail":false,"target_va":5377442944,"va":5372780065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":261,"record_offset":13108,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":13080,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":14256,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":261,"record_offset":13720,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":261,"record_offset":12996,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":261,"record_offset":13048,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":12632,"source_line":855,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":13192,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14471,"source_line":231,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::wide_button
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":17836,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14487,"source_line":415,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":261,"record_offset":16396,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":19016,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14495,"source_line":112,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":19048,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14497,"source_line":307,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::align_items
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":15432,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14484,"source_line":100,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":19320,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14498,"source_line":70,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_x
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":261,"record_offset":22392,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14506,"source_line":121,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_y
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":261,"record_offset":22420,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14507,"source_line":127,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":261,"record_offset":15264,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14478,"source_line":68,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":261,"record_offset":21980,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14501,"source_line":57,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::scrollable::Scrollable::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/scrollable.rs
//     evidence={"module":261,"record_offset":21952,"parent_record_offset":12812,"ancestor_record_offset":12812,"inlinee":14500,"source_line":51,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":12812,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":13944,"parent_record_offset":13920,"ancestor_record_offset":12812,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":13464,"parent_record_offset":13440,"ancestor_record_offset":12812,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":13864,"parent_record_offset":13828,"ancestor_record_offset":12812,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":13896,"parent_record_offset":13864,"ancestor_record_offset":12812,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":13440,"parent_record_offset":13384,"ancestor_record_offset":12812,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":261,"record_offset":13248,"parent_record_offset":13224,"ancestor_record_offset":12812,"inlinee":5388,"source_line":88,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":261,"record_offset":13828,"parent_record_offset":13792,"ancestor_record_offset":12812,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":261,"record_offset":13156,"parent_record_offset":13132,"ancestor_record_offset":12812,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":13412,"parent_record_offset":13384,"ancestor_record_offset":12812,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":15880,"parent_record_offset":15848,"ancestor_record_offset":12812,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":13384,"parent_record_offset":13356,"ancestor_record_offset":12812,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":14388,"parent_record_offset":14332,"ancestor_record_offset":12812,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$21::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":17060,"parent_record_offset":17036,"ancestor_record_offset":12812,"inlinee":4475,"source_line":2366,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":14332,"parent_record_offset":14296,"ancestor_record_offset":12812,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":13224,"parent_record_offset":13192,"ancestor_record_offset":12812,"inlinee":5386,"source_line":2659,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":261,"record_offset":15848,"parent_record_offset":15804,"ancestor_record_offset":12812,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":261,"record_offset":18384,"parent_record_offset":18336,"ancestor_record_offset":12812,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":261,"record_offset":14416,"parent_record_offset":14332,"ancestor_record_offset":12812,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":261,"record_offset":14444,"parent_record_offset":14332,"ancestor_record_offset":12812,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":261,"record_offset":15392,"parent_record_offset":15368,"ancestor_record_offset":12812,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":261,"record_offset":13020,"parent_record_offset":12996,"ancestor_record_offset":12812,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":261,"record_offset":13132,"parent_record_offset":13108,"ancestor_record_offset":12812,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":13308,"parent_record_offset":13192,"ancestor_record_offset":12812,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":13972,"parent_record_offset":13944,"ancestor_record_offset":12812,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":15912,"parent_record_offset":15804,"ancestor_record_offset":12812,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":14472,"parent_record_offset":14332,"ancestor_record_offset":12812,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":17880,"parent_record_offset":17836,"ancestor_record_offset":12812,"inlinee":14471,"source_line":231,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":261,"record_offset":13792,"parent_record_offset":13756,"ancestor_record_offset":12812,"inlinee":14476,"source_line":32,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::length::Length::fluid
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/length.rs
//     evidence={"module":261,"record_offset":18920,"parent_record_offset":18336,"ancestor_record_offset":12812,"inlinee":14492,"source_line":51,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":261,"record_offset":13280,"parent_record_offset":13192,"ancestor_record_offset":12812,"inlinee":14473,"source_line":41,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":261,"record_offset":13756,"parent_record_offset":13720,"ancestor_record_offset":12812,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":18336,"parent_record_offset":18300,"ancestor_record_offset":12812,"inlinee":14490,"source_line":74,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":18984,"parent_record_offset":17836,"ancestor_record_offset":12812,"inlinee":14494,"source_line":127,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::width
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":18956,"parent_record_offset":17836,"ancestor_record_offset":12812,"inlinee":14493,"source_line":92,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":21132,"parent_record_offset":21096,"ancestor_record_offset":12812,"inlinee":14497,"source_line":307,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":15368,"parent_record_offset":15288,"ancestor_record_offset":12812,"inlinee":14483,"source_line":145,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":15336,"parent_record_offset":15312,"ancestor_record_offset":12812,"inlinee":14482,"source_line":50,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":15312,"parent_record_offset":15288,"ancestor_record_offset":12812,"inlinee":14481,"source_line":32,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":15288,"parent_record_offset":15264,"ancestor_record_offset":12812,"inlinee":14480,"source_line":37,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":261,"record_offset":15500,"parent_record_offset":15464,"ancestor_record_offset":12812,"inlinee":14486,"source_line":333,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":261,"record_offset":22016,"parent_record_offset":21980,"ancestor_record_offset":12812,"inlinee":14503,"source_line":50,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":261,"record_offset":22480,"parent_record_offset":22448,"ancestor_record_offset":12812,"inlinee":14509,"source_line":313,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::helpers::button
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":261,"record_offset":18300,"parent_record_offset":17836,"ancestor_record_offset":12812,"inlinee":14488,"source_line":116,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::scrollable::impl$6::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/scrollable.rs
//     evidence={"module":261,"record_offset":22104,"parent_record_offset":22068,"ancestor_record_offset":12812,"inlinee":14505,"source_line":420,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6>
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":68,"record_offset":1492,"rva":879504,"va":5369588624}
// 编译实例 RVAs: 0xd6b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t3675562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8aa153800","size":5,"soft_fail":false,"target_va":5373265456,"va":5369589889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3675947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e82b173800","size":5,"soft_fail":false,"target_va":5373265408,"va":5369589456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3662913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e841e43700","size":5,"soft_fail":false,"target_va":5373252112,"va":5369589194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400056b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-862006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8cad8f2ff","size":5,"soft_fail":false,"target_va":5368731312,"va":5369593313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400057e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-861417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e817dbf2ff","size":5,"soft_fail":false,"target_va":5368731616,"va":5369593028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d84b0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e896080000","size":5,"soft_fail":false,"target_va":5369595056,"va":5369592853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14030be80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2312185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8f9472300","size":5,"soft_fail":false,"target_va":5371903616,"va":5369591426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2655396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8a4842800","size":5,"soft_fail":false,"target_va":5372249392,"va":5369593991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2682477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e86dee2800","size":5,"soft_fail":false,"target_va":5372274416,"va":5369591934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=4
//     target_aliases=89 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2724350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8fe912900","size":5,"soft_fail":false,"target_va":5372314208,"va":5369589853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403706c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2722228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8b4892900","size":5,"soft_fail":false,"target_va":5372315328,"va":5369593095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140374040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2736861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8ddc22900","size":5,"soft_fail":false,"target_va":5372330048,"va":5369593182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140375190
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2742096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e850d72900","size":5,"soft_fail":false,"target_va":5372334480,"va":5369592379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140377f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2753949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e89d052a00","size":5,"soft_fail":false,"target_va":5372346176,"va":5369592222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403785b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2754456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e898072a00","size":5,"soft_fail":false,"target_va":5372347824,"va":5369593363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140383620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2801507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e863bf2a00","size":5,"soft_fail":false,"target_va":5372392992,"va":5369591480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3160693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8753a3000","size":5,"soft_fail":false,"target_va":5372749696,"va":5369588998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3160802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8e23a3000","size":5,"soft_fail":false,"target_va":5372749712,"va":5369588905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34004139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8abdc0602","size":5,"soft_fail":false,"target_va":5403593360,"va":5369589216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33386526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e81e70fd01","size":5,"soft_fail":false,"target_va":5402978144,"va":5369591613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7935318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e856157900","size":5,"soft_fail":false,"target_va":5377527120,"va":5369591797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7772870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8c69a7600","size":5,"soft_fail":false,"target_va":5377364336,"va":5369591461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33388537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8f977fd01","size":5,"soft_fail":false,"target_va":5402980128,"va":5369591586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_reporter_gui::Controls::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3184622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8ee973000","size":5,"soft_fail":false,"target_va":5372775760,"va":5369591133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6563563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8eb266400","size":5,"soft_fail":false,"target_va":5376152880,"va":5369589312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33384972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e80c6afd01","size":5,"soft_fail":false,"target_va":5402975328,"va":5369590351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33385171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8d36afd01","size":5,"soft_fail":false,"target_va":5402975504,"va":5369590328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33385248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8206bfd01","size":5,"soft_fail":false,"target_va":5402975696,"va":5369590443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2605169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e871c02700","size":5,"soft_fail":false,"target_va":5372193920,"va":5369588746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2613912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e898e22700","size":5,"soft_fail":false,"target_va":5372204800,"va":5369590883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3676177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e811183800","size":5,"soft_fail":false,"target_va":5373265840,"va":5369589658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::primary_monitor
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3676174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e80e183800","size":5,"soft_fail":false,"target_va":5373265808,"va":5369589629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::is_minimized
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3650486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e8b6b33700","size":5,"soft_fail":false,"target_va":5373241248,"va":5369590757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3662640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e830e33700","size":5,"soft_fail":false,"target_va":5373253168,"va":5369590523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::force_window_active
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3663192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e858e53700","size":5,"soft_fail":false,"target_va":5373254016,"va":5369590819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::init
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1117450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369588624,"raw_hex":"e80a0d1100","size":5,"soft_fail":false,"target_va":5370707680,"va":5369590225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5369588624,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5369588876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":68,"record_offset":8312,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":68,"record_offset":8520,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":68,"record_offset":4468,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":68,"record_offset":4700,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8210,"source_line":1547,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":68,"record_offset":8792,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":68,"record_offset":6992,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":68,"record_offset":2412,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::impl$18::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":68,"record_offset":5168,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8219,"source_line":302,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":68,"record_offset":1776,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8182,"source_line":200,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":68,"record_offset":4272,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8203,"source_line":329,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":68,"record_offset":5700,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8226,"source_line":305,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":68,"record_offset":4756,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8211,"source_line":125,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":68,"record_offset":4328,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8207,"source_line":118,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::focus_window
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":7556,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8251,"source_line":1253,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::request_user_attention
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":8188,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8261,"source_line":1281,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":7060,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8246,"source_line":568,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::build
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":5728,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8227,"source_line":508,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":2512,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8190,"source_line":205,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":4020,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8196,"source_line":222,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":5368,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8220,"source_line":274,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":3140,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8195,"source_line":307,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":4244,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8201,"source_line":340,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":68,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":1492,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":68,"record_offset":2940,"parent_record_offset":2916,"ancestor_record_offset":1492,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":68,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":1492,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":68,"record_offset":8356,"parent_record_offset":8312,"ancestor_record_offset":1492,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":68,"record_offset":2892,"parent_record_offset":2864,"ancestor_record_offset":1492,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":68,"record_offset":2188,"parent_record_offset":2104,"ancestor_record_offset":1492,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":68,"record_offset":8556,"parent_record_offset":8520,"ancestor_record_offset":1492,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":68,"record_offset":2104,"parent_record_offset":2076,"ancestor_record_offset":1492,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":68,"record_offset":3916,"parent_record_offset":3888,"ancestor_record_offset":1492,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":68,"record_offset":2864,"parent_record_offset":2836,"ancestor_record_offset":1492,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":68,"record_offset":2836,"parent_record_offset":2808,"ancestor_record_offset":1492,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":68,"record_offset":3012,"parent_record_offset":2836,"ancestor_record_offset":1492,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":68,"record_offset":3888,"parent_record_offset":3860,"ancestor_record_offset":1492,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":68,"record_offset":2772,"parent_record_offset":2744,"ancestor_record_offset":1492,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":68,"record_offset":2744,"parent_record_offset":2716,"ancestor_record_offset":1492,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":68,"record_offset":2688,"parent_record_offset":2660,"ancestor_record_offset":1492,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":68,"record_offset":2716,"parent_record_offset":2688,"ancestor_record_offset":1492,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":68,"record_offset":2660,"parent_record_offset":2632,"ancestor_record_offset":1492,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":68,"record_offset":2632,"parent_record_offset":2596,"ancestor_record_offset":1492,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":68,"record_offset":3212,"parent_record_offset":3184,"ancestor_record_offset":1492,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":68,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":1492,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":68,"record_offset":5932,"parent_record_offset":5892,"ancestor_record_offset":1492,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":68,"record_offset":2808,"parent_record_offset":2772,"ancestor_record_offset":1492,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":68,"record_offset":3508,"parent_record_offset":3472,"ancestor_record_offset":1492,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":68,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":1492,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":68,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":1492,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":68,"record_offset":3184,"parent_record_offset":3140,"ancestor_record_offset":1492,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$77::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":68,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":1492,"inlinee":8217,"source_line":76,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":68,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":1492,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":68,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":1492,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":68,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":1492,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":68,"record_offset":3048,"parent_record_offset":2772,"ancestor_record_offset":1492,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":68,"record_offset":1984,"parent_record_offset":1776,"ancestor_record_offset":1492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":68,"record_offset":5980,"parent_record_offset":5956,"ancestor_record_offset":1492,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":68,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":1492,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":68,"record_offset":6136,"parent_record_offset":6076,"ancestor_record_offset":1492,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":68,"record_offset":4816,"parent_record_offset":4780,"ancestor_record_offset":1492,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":68,"record_offset":4416,"parent_record_offset":4368,"ancestor_record_offset":1492,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":68,"record_offset":4852,"parent_record_offset":4780,"ancestor_record_offset":1492,"inlinee":8213,"source_line":1407,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":68,"record_offset":1948,"parent_record_offset":1864,"ancestor_record_offset":1492,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":68,"record_offset":6960,"parent_record_offset":5728,"ancestor_record_offset":1492,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":68,"record_offset":6404,"parent_record_offset":6376,"ancestor_record_offset":1492,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":68,"record_offset":6764,"parent_record_offset":6588,"ancestor_record_offset":1492,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":68,"record_offset":1892,"parent_record_offset":1864,"ancestor_record_offset":1492,"inlinee":8187,"source_line":669,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":68,"record_offset":6076,"parent_record_offset":6048,"ancestor_record_offset":1492,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":68,"record_offset":6852,"parent_record_offset":6824,"ancestor_record_offset":1492,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":68,"record_offset":6308,"parent_record_offset":6280,"ancestor_record_offset":1492,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":68,"record_offset":6108,"parent_record_offset":6076,"ancestor_record_offset":1492,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":68,"record_offset":6332,"parent_record_offset":6308,"ancestor_record_offset":1492,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":68,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":1492,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":68,"record_offset":1916,"parent_record_offset":1892,"ancestor_record_offset":1492,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":68,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":1492,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":68,"record_offset":6256,"parent_record_offset":6232,"ancestor_record_offset":1492,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":68,"record_offset":6016,"parent_record_offset":5892,"ancestor_record_offset":1492,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":68,"record_offset":6172,"parent_record_offset":6016,"ancestor_record_offset":1492,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":68,"record_offset":6556,"parent_record_offset":6528,"ancestor_record_offset":1492,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":68,"record_offset":6588,"parent_record_offset":6556,"ancestor_record_offset":1492,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":68,"record_offset":6376,"parent_record_offset":6200,"ancestor_record_offset":1492,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":68,"record_offset":6200,"parent_record_offset":6172,"ancestor_record_offset":1492,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":68,"record_offset":6048,"parent_record_offset":6016,"ancestor_record_offset":1492,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":68,"record_offset":6824,"parent_record_offset":6556,"ancestor_record_offset":1492,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":68,"record_offset":6232,"parent_record_offset":6200,"ancestor_record_offset":1492,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":68,"record_offset":5452,"parent_record_offset":5428,"ancestor_record_offset":1492,"inlinee":8224,"source_line":281,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalSize::cast
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":68,"record_offset":4104,"parent_record_offset":4080,"ancestor_record_offset":1492,"inlinee":8200,"source_line":439,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":68,"record_offset":4140,"parent_record_offset":4104,"ancestor_record_offset":1492,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$19::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":68,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":1492,"inlinee":8215,"source_line":308,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":68,"record_offset":4496,"parent_record_offset":4468,"ancestor_record_offset":1492,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$33::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":68,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":1492,"inlinee":8198,"source_line":531,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":68,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":1492,"inlinee":8222,"source_line":573,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":68,"record_offset":1864,"parent_record_offset":1776,"ancestor_record_offset":1492,"inlinee":8186,"source_line":120,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":68,"record_offset":1808,"parent_record_offset":1776,"ancestor_record_offset":1492,"inlinee":8184,"source_line":69,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":68,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":1492,"inlinee":8185,"source_line":80,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/event_loop.rs
//     evidence={"module":68,"record_offset":4296,"parent_record_offset":4272,"ancestor_record_offset":1492,"inlinee":8205,"source_line":494,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":68,"record_offset":4780,"parent_record_offset":4756,"ancestor_record_offset":1492,"inlinee":8212,"source_line":211,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":68,"record_offset":4368,"parent_record_offset":4328,"ancestor_record_offset":1492,"inlinee":8209,"source_line":185,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::focus_window
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":68,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":1492,"inlinee":8254,"source_line":998,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":68,"record_offset":6920,"parent_record_offset":5892,"ancestor_record_offset":1492,"inlinee":8244,"source_line":344,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":68,"record_offset":5824,"parent_record_offset":5728,"ancestor_record_offset":1492,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":68,"record_offset":7084,"parent_record_offset":7060,"ancestor_record_offset":1492,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":68,"record_offset":5788,"parent_record_offset":5728,"ancestor_record_offset":1492,"inlinee":8229,"source_line":96,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":68,"record_offset":5892,"parent_record_offset":5860,"ancestor_record_offset":1492,"inlinee":8233,"source_line":160,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_user_attention
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":68,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":1492,"inlinee":8264,"source_line":929,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":68,"record_offset":7132,"parent_record_offset":7108,"ancestor_record_offset":1492,"inlinee":8250,"source_line":452,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::WindowFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.4.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-2.4.2/src/lib.rs
//     evidence={"module":68,"record_offset":8108,"parent_record_offset":7628,"ancestor_record_offset":1492,"inlinee":8258,"source_line":698,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::WindowState::window_flags
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window_state.rs
//     evidence={"module":68,"record_offset":7672,"parent_record_offset":7628,"ancestor_record_offset":1492,"inlinee":8256,"source_line":180,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.4.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-2.4.2/src/lib.rs
//     evidence={"module":68,"record_offset":8136,"parent_record_offset":8108,"ancestor_record_offset":1492,"inlinee":8260,"source_line":698,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":1492,"inlinee":8249,"source_line":569,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::focus_window::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":1492,"inlinee":8253,"source_line":1254,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::request_user_attention::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":1492,"inlinee":8263,"source_line":1283,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$30::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":2548,"parent_record_offset":2512,"ancestor_record_offset":1492,"inlinee":8192,"source_line":116,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":2596,"parent_record_offset":2548,"ancestor_record_offset":1492,"inlinee":8194,"source_line":162,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$9::build::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":68,"record_offset":5860,"parent_record_offset":5824,"ancestor_record_offset":1492,"inlinee":8232,"source_line":514,"archive":"_pdb/A/modules/00068.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$11::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12BE5
// 首条位置: {"module":1867,"inlinee":76773}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::clone::impls::impl$3::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":1867,"record_offset":9144,"parent_record_offset":8988,"ancestor_record_offset":8988,"inlinee":32536,"source_line":263,"archive":"_pdb/A/modules/01867.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1867,"record_offset":9032,"parent_record_offset":8988,"ancestor_record_offset":8988,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/01867.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1867,"record_offset":9056,"parent_record_offset":9032,"ancestor_record_offset":8988,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/01867.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1867,"record_offset":9080,"parent_record_offset":9056,"ancestor_record_offset":8988,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/01867.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":1867,"record_offset":9104,"parent_record_offset":9080,"ancestor_record_offset":8988,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/01867.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$0::program_outcome
// 记录形式: compiled_procedure; instances=1
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":261,"record_offset":7964,"rva":4066576,"va":5372775696}
// 编译实例 RVAs: 0x3e0d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":261,"record_offset":8092,"parent_record_offset":7964,"ancestor_record_offset":7964,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$0::tick_message
// 记录形式: compiled_procedure; instances=1
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":261,"record_offset":8120,"rva":4066608,"va":5372775728}
// 编译实例 RVAs: 0x3e0d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::Controls::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":261,"record_offset":8236,"rva":4066640,"va":5372775760}
// 编译实例 RVAs: 0x3e0d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400056b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4045279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372775760,"raw_hex":"e82146c2ff","size":5,"soft_fail":false,"target_va":5368731312,"va":5372776586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140005a60
//     owner=unresolved_namespace; count=1
//     target_aliases=118 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4044954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372775760,"raw_hex":"e86647c2ff","size":5,"soft_fail":false,"target_va":5368732256,"va":5372777205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-461853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372775760,"raw_hex":"e8e3f3f8ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5372776056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403785b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-429186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372775760,"raw_hex":"e87e73f9ff","size":5,"soft_fail":false,"target_va":5372347824,"va":5372777005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372775760,"raw_hex":"e82599ffff","size":5,"soft_fail":false,"target_va":5372749712,"va":5372776038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2395052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372775760,"raw_hex":"e8ac8b2400","size":5,"soft_fail":false,"target_va":5375171056,"va":5372775999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tg_package_integrity::PackageVerificationError> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372775760,"raw_hex":"e866f0ffff","size":5,"soft_fail":false,"target_va":5372773280,"va":5372777269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::contains<ref$<str$> >
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-2573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372775760,"raw_hex":"e8f3f5ffff","size":5,"soft_fail":false,"target_va":5372773584,"va":5372776152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4629597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372775760,"raw_hex":"e85da44600","size":5,"soft_fail":false,"target_va":5377405616,"va":5372776014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::controls::Controls::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5086624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372775760,"raw_hex":"e8a09d4d00","size":5,"soft_fail":false,"target_va":5377862528,"va":5372775899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":261,"record_offset":8376,"parent_record_offset":8236,"ancestor_record_offset":8236,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":261,"record_offset":9612,"parent_record_offset":8236,"ancestor_record_offset":8236,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":9804,"parent_record_offset":8236,"ancestor_record_offset":8236,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":261,"record_offset":9584,"parent_record_offset":8236,"ancestor_record_offset":8236,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":261,"record_offset":9312,"parent_record_offset":8236,"ancestor_record_offset":8236,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":261,"record_offset":9396,"parent_record_offset":8236,"ancestor_record_offset":8236,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":8236,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":8852,"parent_record_offset":8796,"ancestor_record_offset":8236,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":8236,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":8796,"parent_record_offset":8768,"ancestor_record_offset":8236,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":8636,"parent_record_offset":8436,"ancestor_record_offset":8236,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":261,"record_offset":9360,"parent_record_offset":9336,"ancestor_record_offset":8236,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":261,"record_offset":9516,"parent_record_offset":9492,"ancestor_record_offset":8236,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":261,"record_offset":9464,"parent_record_offset":9428,"ancestor_record_offset":8236,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":9240,"parent_record_offset":8408,"ancestor_record_offset":8236,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":261,"record_offset":9336,"parent_record_offset":9312,"ancestor_record_offset":8236,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":261,"record_offset":9492,"parent_record_offset":9428,"ancestor_record_offset":8236,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":261,"record_offset":9428,"parent_record_offset":9396,"ancestor_record_offset":8236,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":8408,"parent_record_offset":8376,"ancestor_record_offset":8236,"inlinee":14456,"source_line":187,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":9552,"parent_record_offset":9396,"ancestor_record_offset":8236,"inlinee":14457,"source_line":193,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":9764,"parent_record_offset":9672,"ancestor_record_offset":8236,"inlinee":14459,"source_line":197,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":9640,"parent_record_offset":9612,"ancestor_record_offset":8236,"inlinee":14458,"source_line":208,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":261,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":8236,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":261,"record_offset":8588,"parent_record_offset":8564,"ancestor_record_offset":8236,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":261,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":8236,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":261,"record_offset":8436,"parent_record_offset":8408,"ancestor_record_offset":8236,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":261,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":8236,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":261,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":8236,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":261,"record_offset":8468,"parent_record_offset":8436,"ancestor_record_offset":8236,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$1::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3878
// 首条位置: {"module":261,"inlinee":14456}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":9240,"parent_record_offset":8408,"ancestor_record_offset":8408,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":261,"record_offset":8436,"parent_record_offset":8408,"ancestor_record_offset":8408,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":8408,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":8852,"parent_record_offset":8796,"ancestor_record_offset":8408,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":8408,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":8796,"parent_record_offset":8768,"ancestor_record_offset":8408,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":8636,"parent_record_offset":8436,"ancestor_record_offset":8408,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":261,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":8408,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":261,"record_offset":8588,"parent_record_offset":8564,"ancestor_record_offset":8408,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":261,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":8408,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":261,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":8408,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":261,"record_offset":8492,"parent_record_offset":8468,"ancestor_record_offset":8408,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":261,"record_offset":8468,"parent_record_offset":8436,"ancestor_record_offset":8408,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$1::new::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 193
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3879
// 首条位置: {"module":261,"inlinee":14457}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$1::new::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x387B
// 首条位置: {"module":261,"inlinee":14459}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$1::new::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x387A
// 首条位置: {"module":261,"inlinee":14458}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::text
// 记录形式: inline_instance, inlinee_declaration; instances=41
// 源码行: 231
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3887
// 首条位置: {"module":261,"inlinee":14471}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":14332,"parent_record_offset":14296,"ancestor_record_offset":14296,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":13224,"parent_record_offset":13192,"ancestor_record_offset":13192,"inlinee":5386,"source_line":2659,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":13308,"parent_record_offset":13192,"ancestor_record_offset":13192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":261,"record_offset":13280,"parent_record_offset":13192,"ancestor_record_offset":13192,"inlinee":14473,"source_line":41,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":13464,"parent_record_offset":13440,"ancestor_record_offset":13192,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":13440,"parent_record_offset":13384,"ancestor_record_offset":13192,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":261,"record_offset":13248,"parent_record_offset":13224,"ancestor_record_offset":13192,"inlinee":5388,"source_line":88,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":13412,"parent_record_offset":13384,"ancestor_record_offset":13192,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":13384,"parent_record_offset":13356,"ancestor_record_offset":13192,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":14388,"parent_record_offset":14332,"ancestor_record_offset":14296,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$21::fmt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":17060,"parent_record_offset":17036,"ancestor_record_offset":16892,"inlinee":4475,"source_line":2366,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":261,"record_offset":14416,"parent_record_offset":14332,"ancestor_record_offset":14296,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":261,"record_offset":14444,"parent_record_offset":14332,"ancestor_record_offset":14296,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=112
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":13332,"parent_record_offset":13308,"ancestor_record_offset":13192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":14472,"parent_record_offset":14332,"ancestor_record_offset":14296,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$2::update::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3885
// 首条位置: {"module":261,"inlinee":14469}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$2::view::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 351
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1B5
// 首条位置: {"module":719,"inlinee":45493}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":719,"record_offset":3180,"parent_record_offset":3148,"ancestor_record_offset":3148,"inlinee":14471,"source_line":231,"archive":"_pdb/A/modules/00719.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":719,"record_offset":3536,"parent_record_offset":3512,"ancestor_record_offset":3148,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00719.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":719,"record_offset":3512,"parent_record_offset":3456,"ancestor_record_offset":3148,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00719.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":719,"record_offset":3484,"parent_record_offset":3456,"ancestor_record_offset":3148,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00719.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":719,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":3148,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00719.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":719,"record_offset":3260,"parent_record_offset":3212,"ancestor_record_offset":3148,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00719.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":719,"record_offset":3212,"parent_record_offset":3180,"ancestor_record_offset":3148,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00719.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":719,"record_offset":3288,"parent_record_offset":3212,"ancestor_record_offset":3148,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00719.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":719,"record_offset":3316,"parent_record_offset":3212,"ancestor_record_offset":3148,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/00719.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":719,"record_offset":3380,"parent_record_offset":3212,"ancestor_record_offset":3148,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00719.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":719,"record_offset":3344,"parent_record_offset":3212,"ancestor_record_offset":3148,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00719.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::copy_to_clipboard
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 409
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x387F
// 首条位置: {"module":261,"inlinee":14463}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arboard::Clipboard::set_text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arboard","version":"3.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arboard-3.4.0/src/lib.rs
//     evidence={"module":261,"record_offset":11476,"parent_record_offset":11440,"ancestor_record_offset":11440,"inlinee":14465,"source_line":93,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":11712,"parent_record_offset":11440,"ancestor_record_offset":11440,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":11748,"parent_record_offset":11440,"ancestor_record_offset":11440,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":11440,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":11636,"parent_record_offset":11612,"ancestor_record_offset":11440,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":11612,"parent_record_offset":11588,"ancestor_record_offset":11440,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] arboard::Set::text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arboard","version":"3.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arboard-3.4.0/src/lib.rs
//     evidence={"module":261,"record_offset":11512,"parent_record_offset":11476,"ancestor_record_offset":11440,"inlinee":14467,"source_line":203,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":11540,"parent_record_offset":11476,"ancestor_record_offset":11440,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::wide_button
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 415
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3897
// 首条位置: {"module":261,"inlinee":14487}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":261,"record_offset":17880,"parent_record_offset":17836,"ancestor_record_offset":17836,"inlinee":14471,"source_line":231,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":18984,"parent_record_offset":17836,"ancestor_record_offset":17836,"inlinee":14494,"source_line":127,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::width
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":18956,"parent_record_offset":17836,"ancestor_record_offset":17836,"inlinee":14493,"source_line":92,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":261,"record_offset":18300,"parent_record_offset":17836,"ancestor_record_offset":17836,"inlinee":14488,"source_line":116,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":18584,"parent_record_offset":18560,"ancestor_record_offset":17836,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":18608,"parent_record_offset":18584,"ancestor_record_offset":17836,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":18244,"parent_record_offset":18220,"ancestor_record_offset":17836,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":18528,"parent_record_offset":18492,"ancestor_record_offset":17836,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":18560,"parent_record_offset":18528,"ancestor_record_offset":17836,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":261,"record_offset":18220,"parent_record_offset":18164,"ancestor_record_offset":17836,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":261,"record_offset":18492,"parent_record_offset":18456,"ancestor_record_offset":17836,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":18192,"parent_record_offset":18164,"ancestor_record_offset":17836,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":261,"record_offset":18164,"parent_record_offset":18136,"ancestor_record_offset":17836,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":17968,"parent_record_offset":17916,"ancestor_record_offset":17836,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":261,"record_offset":17916,"parent_record_offset":17880,"ancestor_record_offset":17836,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":261,"record_offset":18384,"parent_record_offset":18336,"ancestor_record_offset":17836,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":261,"record_offset":17996,"parent_record_offset":17916,"ancestor_record_offset":17836,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":261,"record_offset":18024,"parent_record_offset":17916,"ancestor_record_offset":17836,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=88
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":18088,"parent_record_offset":17916,"ancestor_record_offset":17836,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":261,"record_offset":18636,"parent_record_offset":18608,"ancestor_record_offset":17836,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":261,"record_offset":18052,"parent_record_offset":17916,"ancestor_record_offset":17836,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":261,"record_offset":18456,"parent_record_offset":18420,"ancestor_record_offset":17836,"inlinee":14476,"source_line":32,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::length::Length::fluid
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/length.rs
//     evidence={"module":261,"record_offset":18920,"parent_record_offset":18336,"ancestor_record_offset":17836,"inlinee":14492,"source_line":51,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":261,"record_offset":18420,"parent_record_offset":18384,"ancestor_record_offset":17836,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::new
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":261,"record_offset":18336,"parent_record_offset":18300,"ancestor_record_offset":17836,"inlinee":14490,"source_line":74,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$3::main_view::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 460
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB177
// 首条位置: {"module":717,"inlinee":45431}
// 编译实例 RVAs: 0x8bf560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$1,tuple$<bool> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 489
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":725,"record_offset":4784,"rva":9199792,"va":5377908912}
// 编译实例 RVAs: 0x8c60b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":725,"record_offset":4976,"parent_record_offset":4784,"ancestor_record_offset":4784,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00725.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$3::main_view::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":725,"record_offset":4996,"parent_record_offset":4976,"ancestor_record_offset":4784,"inlinee":45624,"source_line":489,"archive":"_pdb/A/modules/00725.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$3::main_view::closure$1
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 489
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB238
// 首条位置: {"module":725,"inlinee":45624}
// 编译实例 RVAs: 0x8c60b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$4::active
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 702
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38DD
// 首条位置: {"module":261,"record_offset":78592,"rva":4123184,"va":5372832304}
// 编译实例 RVAs: 0x3eea30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5077723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372832304,"raw_hex":"e8db7a4d00","size":5,"soft_fail":false,"target_va":5377910048,"va":5372832320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$4::hovered
// 记录形式: compiled_procedure; instances=1
// 源码行: 716
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":261,"record_offset":78704,"rva":4123344,"va":5372832464}
// 编译实例 RVAs: 0x3eead0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5077563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372832464,"raw_hex":"e83b7a4d00","size":5,"soft_fail":false,"target_va":5377910048,"va":5372832480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$4::pressed
// 记录形式: compiled_procedure; instances=1
// 源码行: 730
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":261,"record_offset":78816,"rva":4123504,"va":5372832624}
// 编译实例 RVAs: 0x3eeb70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5077403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372832624,"raw_hex":"e89b794d00","size":5,"soft_fail":false,"target_va":5377910048,"va":5372832640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$5::appearance
// 记录形式: compiled_procedure; instances=1
// 源码行: 750
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":261,"record_offset":78928,"rva":4123664,"va":5372832784}
// 编译实例 RVAs: 0x3eec10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] iced_style::container::Appearance::with_border
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/style/src/container.rs
//     evidence={"module":261,"record_offset":79080,"parent_record_offset":78928,"ancestor_record_offset":78928,"inlinee":14539,"source_line":20,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_direct] iced_style::container::impl$4::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/style/src/container.rs
//     evidence={"module":261,"record_offset":79056,"parent_record_offset":78928,"ancestor_record_offset":78928,"inlinee":14537,"source_line":5,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$57::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":261,"record_offset":79152,"parent_record_offset":79132,"ancestor_record_offset":78928,"inlinee":11220,"source_line":439,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::border::impl$12::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/border.rs
//     evidence={"module":261,"record_offset":79108,"parent_record_offset":79080,"ancestor_record_offset":78928,"inlinee":14541,"source_line":5,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::border::impl$18::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/border.rs
//     evidence={"module":261,"record_offset":79132,"parent_record_offset":79108,"ancestor_record_offset":78928,"inlinee":14543,"source_line":29,"archive":"_pdb/A/modules/00261.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$6::active
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 761
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38E3
// 首条位置: {"module":261,"record_offset":79196,"rva":4123808,"va":5372832928}
// 编译实例 RVAs: 0x3eeca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5077099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372832928,"raw_hex":"e86b784d00","size":5,"soft_fail":false,"target_va":5377910048,"va":5372832944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$6::hovered
// 记录形式: compiled_procedure; instances=1
// 源码行: 769
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":261,"record_offset":79308,"rva":4123952,"va":5372833072}
// 编译实例 RVAs: 0x3eed30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5076955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372833072,"raw_hex":"e8db774d00","size":5,"soft_fail":false,"target_va":5377910048,"va":5372833088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] crash_reporter_gui::ButtonStyle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<crash_reporter_gui::ButtonStyle>", "constructor": "core::ptr::drop_in_place", "arguments": ["crash_reporter_gui::ButtonStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::disabled<crash_reporter_gui::ButtonStyle>", "constructor": "iced_style::button::StyleSheet::disabled", "arguments": ["crash_reporter_gui::ButtonStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] crash_reporter_gui::Controls
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$1", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$0", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> > >", "constructor": "core::iter::adapters::cloned::Cloned", "arguments": ["core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<iced_core::event::Event>,core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<iced_core::event::Event>", "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>", "constructor": "tg_iced::gui_runner::run_gui_with_options::closure_env$1", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "tuple$<enum2$<winit::event::Event<tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls>", "constructor": "tg_iced::gui_runner::run_gui_with_options", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::new<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::State::new", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::update<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::State::update", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::EventLoop::pump_events", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<crash_reporter_gui::Controls>", "constructor": "core::ptr::drop_in_place", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::State", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::program::state::State<crash_reporter_gui::Controls> >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::program::state::State<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] crash_reporter_gui::CurrentState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<crash_reporter_gui::CurrentState>", "constructor": "enum2$", "arguments": ["crash_reporter_gui::CurrentState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<crash_reporter_gui::CurrentState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<crash_reporter_gui::CurrentState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] crash_reporter_gui::HyperlinkStyle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::pressed<crash_reporter_gui::HyperlinkStyle>", "constructor": "iced_style::button::StyleSheet::pressed", "arguments": ["crash_reporter_gui::HyperlinkStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::disabled<crash_reporter_gui::HyperlinkStyle>", "constructor": "iced_style::button::StyleSheet::disabled", "arguments": ["crash_reporter_gui::HyperlinkStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] crash_reporter_gui::Message
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 230
// [PDB 泛型关系，非字段关系] {"type": "enum2$<crash_reporter_gui::Message>", "constructor": "enum2$", "arguments": ["crash_reporter_gui::Message"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::into_cache<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::into_cache", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::Element", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::build<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_runtime::user_interface::UserInterface::build", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::update<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::update", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_runtime::command::action::Action", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >", "constructor": "enum2$", "arguments": ["iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::impl$0::update::closure_env$2", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::event::Status,core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::event::Status", "core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::event::Status,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::event::Status", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::draw::recurse::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::draw::recurse::closure_env$1", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_graphics::renderer::impl$1::with_layer<iced_tiny_skia::backend::Backend,iced_runtime::overlay::nested::impl$0::draw::recurse::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_graphics::renderer::impl$1::with_layer", "arguments": ["iced_tiny_skia::backend::Backend", "iced_runtime::overlay::nested::impl$0::draw::recurse::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<crash_reporter_gui::Message> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Item,enum2$<crash_reporter_gui::Message> >", "constructor": "assoc$", "arguments": ["Item", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_core::stream::Stream<assoc$<Item,enum2$<crash_reporter_gui::Message> > >", "constructor": "futures_core::stream::Stream", "arguments": ["assoc$<Item,enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<crash_reporter_gui::Message> > >,core::marker::Send>", "constructor": "dyn$", "arguments": ["futures_core::stream::Stream<assoc$<Item,enum2$<crash_reporter_gui::Message> > >", "core::marker::Send"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<crash_reporter_gui::Message> > >,core::marker::Send>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<crash_reporter_gui::Message> > >,core::marker::Send>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<crash_reporter_gui::Message> > >,core::marker::Send>,alloc::alloc::Global> >", "constructor": "core::pin::Pin", "arguments": ["alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<crash_reporter_gui::Message> > >,core::marker::Send>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<crash_reporter_gui::Message> > >,core::marker::Send>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<crash_reporter_gui::Message> > >,core::marker::Send>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::Nested<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::Nested", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::overlay::nested::Nested<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::overlay::nested::Nested<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::user_interface::UserInterface<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::user_interface::UserInterface<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_runtime::command::Command", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::command::Command<enum2$<crash_reporter_gui::Message> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::command::Command<enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::draw::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::draw::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::layout::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::layout::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::is_over::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::is_over::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::on_event::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::on_event::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::Row", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::Button", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::Column", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Output,enum2$<crash_reporter_gui::Message> >", "constructor": "assoc$", "arguments": ["Output", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > >", "constructor": "core::ops::function::Fn", "arguments": ["tuple$<enum2$<tg_integrity_report::controls::Message> >", "assoc$<Output,enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "constructor": "dyn$", "arguments": ["core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::Checkbox", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::Group<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::Group", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::group::Group<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::group::Group<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::Container<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::Container", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::container::Container<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::container::Container<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::Scrollable<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::Scrollable", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::scrollable::Scrollable<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::scrollable::Scrollable<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::Map", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "core::array::iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::iter::IntoIter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::iter::IntoIter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>", "constructor": "core::array::iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::iter::IntoIter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::iter::IntoIter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::Map", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >", "constructor": "ref$", "arguments": ["dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<crash_reporter_gui::Message> >", "constructor": "core::option::Option", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<crash_reporter_gui::Message> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<crash_reporter_gui::Message> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<enum2$<crash_reporter_gui::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$1::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$1::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$1::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$1::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::mouse_interaction<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::mouse_interaction", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::tag<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::tag", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::diff<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::diff", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::draw<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::draw", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::size<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::size", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::state<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::state", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::layout<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::layout", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::operate", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::overlay<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::overlay", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::children<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::children", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::on_event<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::on_event", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::size_hint<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::impl$4::size_hint", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::scrollable<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::scrollable", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::text_input<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::text_input", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::custom<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::custom", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::container<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::container", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::container::closure$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::container::closure$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::focusable<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::focusable", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>,tuple$<enum2$<tg_integrity_report::controls::Message> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "tuple$<enum2$<tg_integrity_report::controls::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::container::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::widget::operation::Operation", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > >", "constructor": "dyn$", "arguments": ["iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > >", "constructor": "ref_mut$", "arguments": ["dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >", "constructor": "tuple$", "arguments": ["ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_core::element::impl$4::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_core::element::impl$4::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::MapOperation", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::mouse_interaction<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::mouse_interaction", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::mouse_interaction<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::mouse_interaction", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::state<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::state", "arguments": ["iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::state<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::state", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::operate<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::operate", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::operate<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::operate", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::overlay<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::overlay", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::overlay<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::overlay", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation::finish<iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >,enum2$<tg_integrity_report::controls::Message> >", "constructor": "iced_core::widget::operation::Operation::finish", "arguments": ["iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >", "enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::tag<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::tag", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::state<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::state", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::push<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::column::Column::push", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::diff<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::diff", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::overlay<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::overlay", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::column::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::column::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::column::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::column::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::helpers::row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,alloc::vec::Vec<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "iced_widget::helpers::row", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "alloc::vec::Vec<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::helpers::column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,alloc::vec::Vec<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "iced_widget::helpers::column", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "alloc::vec::Vec<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::notify_on_scroll<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_widget::scrollable::notify_on_scroll", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::tag<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::tag", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::diff<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::diff", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::state<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::state", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate::closure$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate::closure$2", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::overlay<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::overlay", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate::closure_env$2", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::scrollable::impl$5::operate::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::scrollable::impl$5::operate::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::scrollable::impl$5::operate::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::scrollable::impl$5::operate::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::scrollable::Scrollable<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::scrollable::Scrollable<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row::with_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::row::Row::with_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row::with_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "iced_widget::row::Row::with_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::diff<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::diff", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate::closure$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate::closure$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::overlay<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::overlay", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "tuple$", "arguments": ["iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnMut::call_mut<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),tuple$<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::ops::function::FnMut::call_mut", "arguments": ["iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "tuple$<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::row::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::row::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::row::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::row::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::tag<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::tag", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::state<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::state", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$49::call<tuple$<enum2$<tg_integrity_report::controls::Message> >,dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$49::call", "arguments": ["tuple$<enum2$<tg_integrity_report::controls::Message> >", "dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$48::call_mut<tuple$<enum2$<tg_integrity_report::controls::Message> >,dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$48::call_mut", "arguments": ["tuple$<enum2$<tg_integrity_report::controls::Message> >", "dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::shell::Shell::merge<enum2$<crash_reporter_gui::Message>,enum2$<tg_integrity_report::controls::Message>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "constructor": "iced_core::shell::Shell::merge", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<tg_integrity_report::controls::Message>", "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::shell::Shell::merge<enum2$<crash_reporter_gui::Message>,enum2$<tg_integrity_report::controls::Message>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "iced_core::shell::Shell::merge", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<tg_integrity_report::controls::Message>", "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$1::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$1::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::tag<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::tag", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::diff<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::diff", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::state<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::state", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate::closure$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate::closure$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::overlay<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::overlay", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::button::impl$1::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::button::impl$1::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::button::impl$1::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::button::impl$1::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::push<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec::push", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::tag<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::tag", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::diff<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::diff", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::state<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::state", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate::closure$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate::closure$1", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::overlay<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::overlay", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate::closure_env$1", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::container::impl$1::operate::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::container::impl$1::operate::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::container::impl$1::operate::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::container::impl$1::operate::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::container::Container<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::container::Container<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::mouse_interaction<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::impl$2::mouse_interaction", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::draw<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::impl$2::draw", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::layout<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::impl$2::layout", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::is_over<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::impl$2::is_over", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::operate<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::impl$2::operate", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::overlay<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::impl$2::overlay", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::on_event<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::impl$2::on_event", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::operate::impl$0::scrollable<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::overlay::element::impl$2::operate::impl$0::scrollable", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::operate::impl$0::text_input<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::overlay::element::impl$2::operate::impl$0::text_input", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::operate::impl$0::custom<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::overlay::element::impl$2::operate::impl$0::custom", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::operate::impl$0::container<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::overlay::element::impl$2::operate::impl$0::container", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::operate::impl$0::container::closure$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::overlay::element::impl$2::operate::impl$0::container::closure$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::operate::impl$0::focusable<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::overlay::element::impl$2::operate::impl$0::focusable", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::overlay::element::impl$2::operate::impl$0::container::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_core::overlay::element::impl$2::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_core::overlay::element::impl$2::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::impl$2::operate::MapOperation<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::overlay::element::impl$2::operate::MapOperation", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::element::impl$2::operate::MapOperation<enum2$<crash_reporter_gui::Message> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::element::impl$2::operate::MapOperation<enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation::finish<iced_core::overlay::element::impl$2::operate::MapOperation<enum2$<crash_reporter_gui::Message> >,enum2$<tg_integrity_report::controls::Message> >", "constructor": "iced_core::widget::operation::Operation::finish", "arguments": ["iced_core::overlay::element::impl$2::operate::MapOperation<enum2$<crash_reporter_gui::Message> >", "enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$1", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::Tree::diff_children_custom<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::widget::tree::Tree::diff_children_custom", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::draw::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure$1<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure$1", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure_env$0", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_graphics::renderer::impl$1::with_layer<iced_tiny_skia::backend::Backend,iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_graphics::renderer::impl$1::with_layer", "arguments": ["iced_tiny_skia::backend::Backend", "iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure_env$2", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_graphics::renderer::impl$1::with_layer<iced_tiny_skia::backend::Backend,iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_graphics::renderer::impl$1::with_layer", "arguments": ["iced_tiny_skia::backend::Backend", "iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::layout::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::layout::closure_env$0", "arguments": ["iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::layout::closure_env$1", "arguments": ["iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::positioned<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::positioned", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::layout::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::layout::padded::closure_env$0", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::positioned<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::positioned", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::layout::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::layout::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::layout::closure_env$1", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::next_to_each_other<iced_widget::checkbox::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::checkbox::impl$1::layout::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::layout::next_to_each_other", "arguments": ["iced_widget::checkbox::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::checkbox::impl$1::layout::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::layout::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::layout::closure_env$0", "arguments": ["iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::contained<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::contained", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::Iter", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<iced_core::widget::tree::Tree,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["iced_core::widget::tree::Tree", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::Element", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::slice::iter::IterMut<iced_core::widget::tree::Tree>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::from_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "ref$", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::widget::tree::Tree,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::widget::tree::Tree", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::is_over<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::is_over", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] crash_reporter_gui::ReportParams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<crash_reporter_gui::ReportParams>", "constructor": "core::ptr::drop_in_place", "arguments": ["crash_reporter_gui::ReportParams"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$1,tuple$<bool> >
// 信息 ID: 2f234380c83389eb2982c265
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["bool"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["crash_reporter_gui::impl$3::main_view::closure_env$1", "tuple$<bool>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::Controls::main_view
// 信息 ID: f65ccbab1ed214a69eb1ad03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_reporter_gui::impl$2::view @ 0x3e1e0b -> 0x3e3d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::Controls::new
// 信息 ID: f6b70599ab741fc26366f77c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6> @ 0xd755d -> 0x3e0d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::copy_to_clipboard
// 信息 ID: c39a05ea1888404f694b78ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::impl$2::update; module=261, inline_record=11440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$0::program_outcome
// 信息 ID: 5760760ed7f47b6f2bf937c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls> @ 0x30cde4 -> 0x3e0d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$0::tick_message
// 信息 ID: 373bc13ecfa388fd0753e432
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls> @ 0x30cd3c -> 0x3e0d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$11::clone
// 信息 ID: 4571a155288e1ca8e2e2307d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_widget::button::impl$1::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >; module=1867, inline_record=8988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$0
// 信息 ID: 198746636b0db2b49e42c46d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=261, inline_record=8408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$1
// 信息 ID: 020e6df161e8af06ed2b6919
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=261, inline_record=9552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$2
// 信息 ID: d5c8251d774a127c2ac659bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=261, inline_record=9764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$3
// 信息 ID: 59b87832a0914a103e2b2101
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=261, inline_record=9640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::update
// 信息 ID: c7838ca3589920663cecd12f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::command::Command::batch<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > > @ 0x3673 -> 0x3e1350 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::update::closure$0
// 信息 ID: 7fc1839ca1edb772eaa994bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$2::update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::impl$2::update; module=261, inline_record=11812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::view
// 信息 ID: 2999bbabcb83a4a87cc21669
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::program::state::State::new<crash_reporter_gui::Controls> @ 0x322be1 -> 0x3e1db0 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_runtime::program::state::State::update<crash_reporter_gui::Controls> @ 0x322e5c -> 0x3e1db0 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_runtime::program::state::State::update<crash_reporter_gui::Controls> @ 0x3231bb -> 0x3e1db0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::view::closure$0
// 信息 ID: dbb76faca61e5b1fe1f2c5cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4df6506252c83c8495a5ff87: iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] b9123650d11dc9236a011191: iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$2::view", "captures": "unknown"}
// [闭包环境类型引用] crash_reporter_gui::impl$2::view::closure_env$0 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; module=719, inline_record=3148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$3::main_view::closure$0
// 信息 ID: 4d7962776b6b14f5cba33ab4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$3::main_view", "captures": "unknown"}
// [闭包环境类型引用] crash_reporter_gui::impl$3::main_view::closure_env$0 <- core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$0,tuple$<enum2$<tg_integrity_report::controls::Message> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] crash_reporter_gui::impl$3::main_view::closure_env$0 <- core::ptr::drop_in_place<crash_reporter_gui::impl$3::main_view::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$0,tuple$<enum2$<tg_integrity_report::controls::Message> > >; module=717, inline_record=9180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$3::main_view::closure$1
// 信息 ID: 7d86b592f5cc0ce3b265ef24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "crash_reporter_gui::impl$3::main_view", "captures": "unknown"}
// [闭包环境类型引用] crash_reporter_gui::impl$3::main_view::closure_env$1 <- core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$1,tuple$<bool> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] crash_reporter_gui::impl$3::main_view::closure_env$1 <- core::ptr::drop_in_place<crash_reporter_gui::impl$3::main_view::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$1,tuple$<bool> >; module=725, inline_record=4996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$4::active
// 信息 ID: e3606221153b04a13bd99d5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_style::button::StyleSheet::disabled<crash_reporter_gui::ButtonStyle>; module=716, inline_record=800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$4::hovered
// 信息 ID: b05629cc14863ce754a1f315
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$4::pressed
// 信息 ID: 9efb9de55ce8c9ab191b7213
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$5::appearance
// 信息 ID: 4f7654d7aee93fb75b4fab37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$6::active
// 信息 ID: 1d46caf3cc8a3f0dd098ecbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_style::button::StyleSheet::pressed<crash_reporter_gui::HyperlinkStyle>; module=716, inline_record=416; 不是独立调用边。
// [PDB 内联宿主] iced_style::button::StyleSheet::disabled<crash_reporter_gui::HyperlinkStyle>; module=716, inline_record=608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$6::hovered
// 信息 ID: d5f2e4740b38556959b75f87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6>
// 信息 ID: b93749a512d7e40203b9d115
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] crash_reporter_gui::run_crash_reporter_gui -> ["tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x75b1 -> 0xd6b90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::text
// 信息 ID: 12513dfa32605cfe948d814f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4df6506252c83c8495a5ff87: iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] b9123650d11dc9236a011191: iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 0 条；内联宿主 37 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=261, inline_record=13192; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=261, inline_record=14296; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=261, inline_record=15976; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=261, inline_record=16892; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=261, inline_record=17880; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=261, inline_record=19932; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=261, inline_record=23116; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=261, inline_record=24256; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::wide_button
// 信息 ID: 83e31ac53d64f21fa36cdde6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=261, inline_record=17836; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=261, inline_record=19888; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=261, inline_record=38836; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=261, inline_record=40744; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=261, inline_record=43088; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=261, inline_record=44296; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=261, inline_record=46828; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=261, inline_record=59380; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

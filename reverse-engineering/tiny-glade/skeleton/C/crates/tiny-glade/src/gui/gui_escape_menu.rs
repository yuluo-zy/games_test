// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/gui/gui_escape_menu.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e9a5ad21d503181cf6b973e2757e738d7c922bd81f57ac126a279d22dfd1dcec
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_escape_menu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 63, 84, 246
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_state; // namespace candidate
//   package identities: [{"name":"bevy_state","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
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
// 首条位置: {"module":84,"record_offset":9324,"rva":3209344,"va":5371918464}
// 编译实例 RVAs: 0x30f880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t3759944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918464,"raw_hex":"e8485f3900","size":5,"soft_fail":false,"target_va":5375678704,"va":5371918755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140607d00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3113960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918464,"raw_hex":"e8e8832f00","size":5,"soft_fail":false,"target_va":5375032576,"va":5371918611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e7a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4320934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918464,"raw_hex":"e8a6ee4100","size":5,"soft_fail":false,"target_va":5376239520,"va":5371918581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12876302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918464,"raw_hex":"e80e7ac400","size":5,"soft_fail":false,"target_va":5384794864,"va":5371918557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::LabelView::content_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4266548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918464,"raw_hex":"e8341a4100","size":5,"soft_fail":false,"target_va":5376185152,"va":5371918599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":84,"record_offset":9480,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":9700,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":84,"record_offset":9560,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":9324,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":9900,"parent_record_offset":9876,"ancestor_record_offset":9324,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":9924,"parent_record_offset":9900,"ancestor_record_offset":9324,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":9852,"parent_record_offset":9796,"ancestor_record_offset":9324,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":84,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":9324,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":9824,"parent_record_offset":9796,"ancestor_record_offset":9324,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":9796,"parent_record_offset":9772,"ancestor_record_offset":9324,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":9772,"parent_record_offset":9748,"ancestor_record_offset":9324,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":84,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":9324,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":9324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":5380,"rva":6753728,"va":5375462848}
// 编译实例 RVAs: 0x670dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140314930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3527441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8ef2ccaff","size":5,"soft_fail":false,"target_va":5371939120,"va":5375466556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140314ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3523863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8e93acaff","size":5,"soft_fail":false,"target_va":5371940512,"va":5375464370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403152a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3524760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e86837caff","size":5,"soft_fail":false,"target_va":5371941536,"va":5375466291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140315760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3523785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8373bcaff","size":5,"soft_fail":false,"target_va":5371942752,"va":5375466532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140315f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3520945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e84f46caff","size":5,"soft_fail":false,"target_va":5371944832,"va":5375465772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403161f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3519989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e80b4acaff","size":5,"soft_fail":false,"target_va":5371945456,"va":5375465440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140317430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3515349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e82b5ccaff","size":5,"soft_fail":false,"target_va":5371950128,"va":5375465472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140317c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3511165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8836ccaff","size":5,"soft_fail":false,"target_va":5371952224,"va":5375463384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140318290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3511998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e84269caff","size":5,"soft_fail":false,"target_va":5371953808,"va":5375465801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403189f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3510812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8e46dcaff","size":5,"soft_fail":false,"target_va":5371955696,"va":5375466503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140318e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3508583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e89976caff","size":5,"soft_fail":false,"target_va":5371956800,"va":5375465378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140319070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3508663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e84976caff","size":5,"soft_fail":false,"target_va":5371957360,"va":5375466018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403192a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3506129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e82f80caff","size":5,"soft_fail":false,"target_va":5371957920,"va":5375464044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403194e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3507010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8be7ccaff","size":5,"soft_fail":false,"target_va":5371958496,"va":5375465501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140319700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3507545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8a77acaff","size":5,"soft_fail":false,"target_va":5371959040,"va":5375466580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140319b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3506176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e80080caff","size":5,"soft_fail":false,"target_va":5371960096,"va":5375466267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140725200
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t738127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e84f430b00","size":5,"soft_fail":false,"target_va":5376201216,"va":5375463084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e990
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t776797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e85dda0b00","size":5,"soft_fail":false,"target_va":5376240016,"va":5375463214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2662841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8b9a12800","size":5,"soft_fail":false,"target_va":5378126304,"va":5375463458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b4d240
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5094861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8cdbd4d00","size":5,"soft_fail":false,"target_va":5380559424,"va":5375464558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6143905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8a1bf5d00","size":5,"soft_fail":false,"target_va":5381607664,"va":5375463754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40885714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8d2dd6f02","size":5,"soft_fail":false,"target_va":5416350384,"va":5375464665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40887029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8f5e26f02","size":5,"soft_fail":false,"target_va":5416350720,"va":5375463686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40887542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8f6e46f02","size":5,"soft_fail":false,"target_va":5416351056,"va":5375463509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40888250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e8bae76f02","size":5,"soft_fail":false,"target_va":5416351392,"va":5375463137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2941249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e841e12c00","size":5,"soft_fail":false,"target_va":5378404688,"va":5375463434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6144032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e820c05d00","size":5,"soft_fail":false,"target_va":5381607616,"va":5375463579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::history_process::HistoryCmd::interrupt
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5967515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e89b0e5b00","size":5,"soft_fail":false,"target_va":5381430992,"va":5375463472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t743252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462848,"raw_hex":"e854570b00","size":5,"soft_fail":false,"target_va":5376206368,"va":5375463111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":246,"record_offset":5536,"parent_record_offset":5380,"ancestor_record_offset":5380,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":246,"record_offset":5968,"parent_record_offset":5380,"ancestor_record_offset":5380,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":246,"record_offset":6332,"parent_record_offset":5380,"ancestor_record_offset":5380,"inlinee":11211,"source_line":90,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::impl$2::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_state","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_state-0.17.3/src/state/resources.rs
//     evidence={"module":246,"record_offset":5884,"parent_record_offset":5380,"ancestor_record_offset":5380,"inlinee":15895,"source_line":81,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":246,"record_offset":10288,"parent_record_offset":5380,"ancestor_record_offset":5380,"inlinee":10268,"source_line":477,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::wide_safe_frame
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":246,"record_offset":5596,"parent_record_offset":5380,"ancestor_record_offset":5380,"inlinee":10270,"source_line":483,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":5752,"parent_record_offset":5720,"ancestor_record_offset":5380,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":5800,"parent_record_offset":5776,"ancestor_record_offset":5380,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":5776,"parent_record_offset":5752,"ancestor_record_offset":5380,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":5720,"parent_record_offset":5684,"ancestor_record_offset":5380,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":5660,"parent_record_offset":5620,"ancestor_record_offset":5380,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":5684,"parent_record_offset":5660,"ancestor_record_offset":5380,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":246,"record_offset":5564,"parent_record_offset":5536,"ancestor_record_offset":5380,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":246,"record_offset":5992,"parent_record_offset":5968,"ancestor_record_offset":5380,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":246,"record_offset":6440,"parent_record_offset":6416,"ancestor_record_offset":5380,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":246,"record_offset":6416,"parent_record_offset":6332,"ancestor_record_offset":5380,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":246,"record_offset":6056,"parent_record_offset":5992,"ancestor_record_offset":5380,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":246,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":5380,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":5840,"parent_record_offset":5684,"ancestor_record_offset":5380,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] country_core::app_loading_state::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/app_loading_state.rs
//     evidence={"module":246,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":5380,"inlinee":9069,"source_line":1,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":246,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":5380,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":84,"record_offset":13404,"rva":3210800,"va":5371919920}
// 编译实例 RVAs: 0x30fe30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fc50
//     owner=unresolved_namespace; count=3
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4259208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371919920,"raw_hex":"e888fd4000","size":5,"soft_fail":false,"target_va":5376179280,"va":5371920067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9687344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371919920,"raw_hex":"e830d19300","size":5,"soft_fail":false,"target_va":5381607664,"va":5371920315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44430459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371919920,"raw_hex":"e87bf4a502","size":5,"soft_fail":false,"target_va":5416350720,"va":5371920256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44430933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371919920,"raw_hex":"e855f6a502","size":5,"soft_fail":false,"target_va":5416351056,"va":5371920118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::missing_mods::MissingMods::has_issues
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7328081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371919920,"raw_hex":"e851d16f00","size":5,"soft_fail":false,"target_va":5379248064,"va":5371919978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4313040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371919920,"raw_hex":"e8d0cf4100","size":5,"soft_fail":false,"target_va":5376233120,"va":5371920075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::PillButtonView::inactive_opacity
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4261744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371919920,"raw_hex":"e870074100","size":5,"soft_fail":false,"target_va":5376181808,"va":5371920059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::back_button
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4259843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371919920,"raw_hex":"e803004100","size":5,"soft_fail":false,"target_va":5376179888,"va":5371920040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":14328,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":84,"record_offset":13560,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":84,"record_offset":13924,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":11211,"source_line":90,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":13788,"parent_record_offset":13756,"ancestor_record_offset":13404,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":13836,"parent_record_offset":13812,"ancestor_record_offset":13404,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":13404,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":13756,"parent_record_offset":13716,"ancestor_record_offset":13404,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":13684,"parent_record_offset":13648,"ancestor_record_offset":13404,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":13716,"parent_record_offset":13684,"ancestor_record_offset":13404,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":13620,"parent_record_offset":13584,"ancestor_record_offset":13404,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":13584,"parent_record_offset":13560,"ancestor_record_offset":13404,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":84,"record_offset":14032,"parent_record_offset":14008,"ancestor_record_offset":13404,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":84,"record_offset":14008,"parent_record_offset":13924,"ancestor_record_offset":13404,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":84,"record_offset":13648,"parent_record_offset":13584,"ancestor_record_offset":13404,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":13876,"parent_record_offset":13716,"ancestor_record_offset":13404,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C14
// 首条位置: {"module":84,"inlinee":11284}
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
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C05
// 首条位置: {"module":84,"inlinee":11269}
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
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C10
// 首条位置: {"module":84,"inlinee":11280}
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
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C15
// 首条位置: {"module":84,"inlinee":11285}
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
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C02
// 首条位置: {"module":84,"inlinee":11266}
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
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C11
// 首条位置: {"module":84,"inlinee":11281}
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
// 源码行: 343
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":84,"record_offset":10556,"rva":3209792,"va":5371918912}
// 编译实例 RVAs: 0x30fa40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <str>::to_uppercase
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t12871157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e8f565c400","size":5,"soft_fail":false,"target_va":5384790144,"va":5371918982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3759133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e81d5c3900","size":5,"soft_fail":false,"target_va":5375678688,"va":5371919550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t3759588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e8e45d3900","size":5,"soft_fail":false,"target_va":5375678704,"va":5371919111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3197348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e85c36cfff","size":5,"soft_fail":false,"target_va":5368722192,"va":5371919535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3910035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e893a93b00","size":5,"soft_fail":false,"target_va":5375829056,"va":5371919016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071eff0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4257043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e813f54000","size":5,"soft_fail":false,"target_va":5376176112,"va":5371919064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fc50
//     owner=unresolved_namespace; count=3
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4260201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e869014100","size":5,"soft_fail":false,"target_va":5376179280,"va":5371919074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4260618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e80a034100","size":5,"soft_fail":false,"target_va":5376179648,"va":5371919025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407204d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4262153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e809094100","size":5,"soft_fail":false,"target_va":5376181456,"va":5371919298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4317853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e89de24100","size":5,"soft_fail":false,"target_va":5376236864,"va":5371919006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9688448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e880d59300","size":5,"soft_fail":false,"target_va":5381607664,"va":5371919211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44431900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e81cfaa502","size":5,"soft_fail":false,"target_va":5416351056,"va":5371919151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t44701050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e87a15aa02","size":5,"soft_fail":false,"target_va":5416620769,"va":5371919714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11215748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e88423ab00","size":5,"soft_fail":false,"target_va":5383134720,"va":5371918967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::default_focus
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4314017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e8a1d34100","size":5,"soft_fail":false,"target_va":5376233056,"va":5371919034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4314032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918912,"raw_hex":"e8b0d34100","size":5,"soft_fail":false,"target_va":5376233120,"va":5371919083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":11428,"parent_record_offset":10556,"ancestor_record_offset":10556,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":84,"record_offset":11064,"parent_record_offset":10556,"ancestor_record_offset":10556,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":10780,"parent_record_offset":10556,"ancestor_record_offset":10556,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gui::impl$30::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":84,"record_offset":12176,"parent_record_offset":10556,"ancestor_record_offset":10556,"inlinee":11208,"source_line":272,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::with_node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":84,"record_offset":10720,"parent_record_offset":10556,"ancestor_record_offset":10556,"inlinee":9899,"source_line":254,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":12328,"parent_record_offset":12304,"ancestor_record_offset":10556,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":12352,"parent_record_offset":12328,"ancestor_record_offset":10556,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":10956,"parent_record_offset":10932,"ancestor_record_offset":10556,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":10980,"parent_record_offset":10956,"ancestor_record_offset":10556,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":12376,"parent_record_offset":12352,"ancestor_record_offset":10556,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":11004,"parent_record_offset":10980,"ancestor_record_offset":10556,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":12304,"parent_record_offset":12272,"ancestor_record_offset":10556,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":10932,"parent_record_offset":10876,"ancestor_record_offset":10556,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":84,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":10556,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":84,"record_offset":12244,"parent_record_offset":12204,"ancestor_record_offset":10556,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$6::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":84,"record_offset":12484,"parent_record_offset":12204,"ancestor_record_offset":10556,"inlinee":7479,"source_line":1212,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":11292,"parent_record_offset":11260,"ancestor_record_offset":10556,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":10904,"parent_record_offset":10876,"ancestor_record_offset":10556,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":10876,"parent_record_offset":10852,"ancestor_record_offset":10556,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":11340,"parent_record_offset":11316,"ancestor_record_offset":10556,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":11316,"parent_record_offset":11292,"ancestor_record_offset":10556,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":10852,"parent_record_offset":10828,"ancestor_record_offset":10556,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$62::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":84,"record_offset":12204,"parent_record_offset":12176,"ancestor_record_offset":10556,"inlinee":7477,"source_line":3726,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":11260,"parent_record_offset":11220,"ancestor_record_offset":10556,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":11188,"parent_record_offset":11152,"ancestor_record_offset":10556,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":11220,"parent_record_offset":11188,"ancestor_record_offset":10556,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":11124,"parent_record_offset":11088,"ancestor_record_offset":10556,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":11088,"parent_record_offset":11064,"ancestor_record_offset":10556,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":84,"record_offset":11152,"parent_record_offset":11088,"ancestor_record_offset":10556,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":84,"record_offset":12508,"parent_record_offset":12484,"ancestor_record_offset":10556,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":10804,"parent_record_offset":10780,"ancestor_record_offset":10556,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":11380,"parent_record_offset":11220,"ancestor_record_offset":10556,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$29::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":84,"record_offset":12452,"parent_record_offset":12428,"ancestor_record_offset":10556,"inlinee":11210,"source_line":231,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":84,"record_offset":12428,"parent_record_offset":12204,"ancestor_record_offset":10556,"inlinee":8489,"source_line":436,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_escape_menu.rs
//     evidence={"module":84,"record_offset":10748,"parent_record_offset":10720,"ancestor_record_offset":10556,"inlinee":9880,"source_line":346,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 346
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2698
// 首条位置: {"module":63,"inlinee":9880}
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
// 源码行: 375
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":84,"record_offset":8996,"rva":3208976,"va":5371918096}
// 编译实例 RVAs: 0x30f710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140223f50
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-964609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918096,"raw_hex":"e8ff47f1ff","size":5,"soft_fail":false,"target_va":5370953552,"va":5371918156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fc50
//     owner=unresolved_namespace; count=3
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4261111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918096,"raw_hex":"e8f7044100","size":5,"soft_fail":false,"target_va":5376179280,"va":5371918164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407204d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4263167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918096,"raw_hex":"e8ff0c4100","size":5,"soft_fail":false,"target_va":5376181456,"va":5371918284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9689447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918096,"raw_hex":"e867d99300","size":5,"soft_fail":false,"target_va":5381607664,"va":5371918212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4314943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371918096,"raw_hex":"e83fd74100","size":5,"soft_fail":false,"target_va":5376233120,"va":5371918172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":9144,"parent_record_offset":8996,"ancestor_record_offset":8996,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":9168,"parent_record_offset":9144,"ancestor_record_offset":8996,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$10
// 记录形式: compiled_procedure; instances=1
// 源码行: 393
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":84,"record_offset":5900,"rva":3207840,"va":5371916960}
// 编译实例 RVAs: 0x30f2a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <str>::to_uppercase
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t12873116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371916960,"raw_hex":"e89c6dc400","size":5,"soft_fail":false,"target_va":5384790144,"va":5371917023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3761563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371916960,"raw_hex":"e89b653900","size":5,"soft_fail":false,"target_va":5375678704,"va":5371917136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140607d00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3115497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371916960,"raw_hex":"e8e9892f00","size":5,"soft_fail":false,"target_va":5375032576,"va":5371917074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fc50
//     owner=unresolved_namespace; count=2
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4262175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371916960,"raw_hex":"e81f094100","size":5,"soft_fail":false,"target_va":5376179280,"va":5371917100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4319812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371916960,"raw_hex":"e844ea4100","size":5,"soft_fail":false,"target_va":5376236864,"va":5371917047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9690490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371916960,"raw_hex":"e87add9300","size":5,"soft_fail":false,"target_va":5381607664,"va":5371917169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11217707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371916960,"raw_hex":"e82b2bab00","size":5,"soft_fail":false,"target_va":5383134720,"va":5371917008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4316006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371916960,"raw_hex":"e866db4100","size":5,"soft_fail":false,"target_va":5376233120,"va":5371917109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4263626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371916960,"raw_hex":"e8ca0e4100","size":5,"soft_fail":false,"target_va":5376180688,"va":5371917057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":6372,"parent_record_offset":5900,"ancestor_record_offset":5900,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":6088,"parent_record_offset":5900,"ancestor_record_offset":5900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":84,"record_offset":6056,"parent_record_offset":5900,"ancestor_record_offset":5900,"inlinee":9900,"source_line":388,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":6264,"parent_record_offset":6240,"ancestor_record_offset":5900,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":6288,"parent_record_offset":6264,"ancestor_record_offset":5900,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":6312,"parent_record_offset":6288,"ancestor_record_offset":5900,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":6240,"parent_record_offset":6184,"ancestor_record_offset":5900,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":6212,"parent_record_offset":6184,"ancestor_record_offset":5900,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":5900,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":5900,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":6400,"parent_record_offset":6372,"ancestor_record_offset":5900,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":5900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 421
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C12
// 首条位置: {"module":84,"inlinee":11282}
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
// 源码行: 430
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C06
// 首条位置: {"module":84,"inlinee":11270}
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
// 源码行: 449
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C0E
// 首条位置: {"module":84,"inlinee":11278}
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
// 源码行: 458
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":84,"record_offset":7380,"rva":3208400,"va":5371917520}
// 编译实例 RVAs: 0x30f4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <str>::to_uppercase
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t12872551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e8676bc400","size":5,"soft_fail":false,"target_va":5384790144,"va":5371917588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3760991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e85f633900","size":5,"soft_fail":false,"target_va":5375678704,"va":5371917708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fc50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4261464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e858064100","size":5,"soft_fail":false,"target_va":5376179280,"va":5371917811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4261984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e860084100","size":5,"soft_fail":false,"target_va":5376179648,"va":5371917659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407204d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4263819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e88b0f4100","size":5,"soft_fail":false,"target_va":5376181456,"va":5371917632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4319244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e80ce84100","size":5,"soft_fail":false,"target_va":5376236864,"va":5371917615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c4d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9689917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e83ddb9300","size":5,"soft_fail":false,"target_va":5381607664,"va":5371917742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44435083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e88b06a602","size":5,"soft_fail":false,"target_va":5416353040,"va":5371917952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11217142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e8f628ab00","size":5,"soft_fail":false,"target_va":5383134720,"va":5371917573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::default_focus
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4315383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e8f7d84100","size":5,"soft_fail":false,"target_va":5376233056,"va":5371917668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4315438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e82ed94100","size":5,"soft_fail":false,"target_va":5376233120,"va":5371917677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4263242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371917520,"raw_hex":"e84a0d4100","size":5,"soft_fail":false,"target_va":5376180896,"va":5371917649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":7820,"parent_record_offset":7380,"ancestor_record_offset":7380,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":84,"record_offset":8164,"parent_record_offset":7380,"ancestor_record_offset":7380,"inlinee":11204,"source_line":840,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":7536,"parent_record_offset":7380,"ancestor_record_offset":7380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":7380,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":7380,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":7380,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":84,"record_offset":7688,"parent_record_offset":7632,"ancestor_record_offset":7380,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":8280,"parent_record_offset":8252,"ancestor_record_offset":7380,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":8360,"parent_record_offset":8336,"ancestor_record_offset":7380,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":7660,"parent_record_offset":7632,"ancestor_record_offset":7380,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":7632,"parent_record_offset":7608,"ancestor_record_offset":7380,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":8336,"parent_record_offset":8308,"ancestor_record_offset":7380,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":8308,"parent_record_offset":8280,"ancestor_record_offset":7380,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":84,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":7380,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":84,"record_offset":8252,"parent_record_offset":8220,"ancestor_record_offset":7380,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":84,"record_offset":7848,"parent_record_offset":7820,"ancestor_record_offset":7380,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":84,"record_offset":8192,"parent_record_offset":8164,"ancestor_record_offset":7380,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":84,"record_offset":8220,"parent_record_offset":8192,"ancestor_record_offset":7380,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":84,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":7380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00084.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$15
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C13
// 首条位置: {"module":84,"inlinee":11283}
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
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":4632,"rva":6753248,"va":5375462368}
// 编译实例 RVAs: 0x670be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <str>::to_uppercase
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9327634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e812548e00","size":5,"soft_fail":false,"target_va":5384790144,"va":5375462505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t215960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e8984b0300","size":5,"soft_fail":false,"target_va":5375678704,"va":5375462739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140223a20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4510254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e8d22dbbff","size":5,"soft_fail":false,"target_va":5370952224,"va":5375462473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140607d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-430041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e82770f9ff","size":5,"soft_fail":false,"target_va":5375032576,"va":5375462612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140646170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-175056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e83054fdff","size":5,"soft_fail":false,"target_va":5375287664,"va":5375462715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14071fc50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t716595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e833ef0a00","size":5,"soft_fail":false,"target_va":5376179280,"va":5375462680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t774273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e881d00b00","size":5,"soft_fail":false,"target_va":5376236864,"va":5375462586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7672225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e8a1117500","size":5,"soft_fail":false,"target_va":5383134720,"va":5375462490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::Node::used
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t770426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e87ac10b00","size":5,"soft_fail":false,"target_va":5376233120,"va":5375462689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::align
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t718093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e80df50a00","size":5,"soft_fail":false,"target_va":5376180768,"va":5375462670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t718087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e807f50a00","size":5,"soft_fail":false,"target_va":5376180688,"va":5375462596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t719070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e8def80a00","size":5,"soft_fail":false,"target_va":5376181728,"va":5375462653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7819003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e8fb4e7700","size":5,"soft_fail":false,"target_va":5383281568,"va":5375462560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::linearstep
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7811060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375462368,"raw_hex":"e8f42f7700","size":5,"soft_fail":false,"target_va":5383273600,"va":5375462535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":4808,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":246,"record_offset":4772,"parent_record_offset":4632,"ancestor_record_offset":4632,"inlinee":9900,"source_line":388,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":4632,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":4632,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":5032,"parent_record_offset":5008,"ancestor_record_offset":4632,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":4960,"parent_record_offset":4904,"ancestor_record_offset":4632,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":4932,"parent_record_offset":4904,"ancestor_record_offset":4632,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":4632,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":4632,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":4632,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::open_community_server
// 记录形式: compiled_procedure; instances=1
// 源码行: 523
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":11956,"rva":6757488,"va":5375466608}
// 编译实例 RVAs: 0x671c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t211784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375466608,"raw_hex":"e8483b0300","size":5,"soft_fail":false,"target_va":5375678688,"va":5375466899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6744697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375466608,"raw_hex":"e8871599ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375466884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140645f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-180261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375466608,"raw_hex":"e8db3ffdff","size":5,"soft_fail":false,"target_va":5375287072,"va":5375467328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41153428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375466608,"raw_hex":"e894f37302","size":5,"soft_fail":false,"target_va":5416620794,"va":5375467361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] obfstr::xref::inner<9123129150649708094>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375466608,"raw_hex":"e807220000","size":5,"soft_fail":false,"target_va":5375475392,"va":5375466676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":246,"record_offset":12312,"parent_record_offset":11956,"ancestor_record_offset":11956,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] obfstr::bytes::deobfuscate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/obfstr-0.4.3/src/bytes.rs
//     evidence={"module":246,"record_offset":12160,"parent_record_offset":11956,"ancestor_record_offset":11956,"inlinee":17748,"source_line":171,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] obfstr::xref::xref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/obfstr-0.4.3/src/xref.rs
//     evidence={"module":246,"record_offset":12104,"parent_record_offset":11956,"ancestor_record_offset":11956,"inlinee":17746,"source_line":78,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":12712,"parent_record_offset":12688,"ancestor_record_offset":11956,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":12736,"parent_record_offset":12712,"ancestor_record_offset":11956,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":12760,"parent_record_offset":12736,"ancestor_record_offset":11956,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":12688,"parent_record_offset":12660,"ancestor_record_offset":11956,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":12600,"parent_record_offset":12572,"ancestor_record_offset":11956,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":12660,"parent_record_offset":12628,"ancestor_record_offset":11956,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":12628,"parent_record_offset":12600,"ancestor_record_offset":11956,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":246,"record_offset":12448,"parent_record_offset":12392,"ancestor_record_offset":11956,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":246,"record_offset":12488,"parent_record_offset":12448,"ancestor_record_offset":11956,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":246,"record_offset":12528,"parent_record_offset":12488,"ancestor_record_offset":11956,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":246,"record_offset":12392,"parent_record_offset":12352,"ancestor_record_offset":11956,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":246,"record_offset":12352,"parent_record_offset":12312,"ancestor_record_offset":11956,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":12572,"parent_record_offset":12528,"ancestor_record_offset":11956,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::black_box
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":246,"record_offset":12128,"parent_record_offset":12104,"ancestor_record_offset":11956,"inlinee":17105,"source_line":490,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":246,"record_offset":12816,"parent_record_offset":12528,"ancestor_record_offset":11956,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":12848,"parent_record_offset":12816,"ancestor_record_offset":11956,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":12224,"parent_record_offset":12160,"ancestor_record_offset":11956,"inlinee":17749,"source_line":2080,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::gui::gui_escape_menu::open_newsletter
// 记录形式: compiled_procedure; instances=1
// 源码行: 528
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":3640,"rva":6751520,"va":5375460640}
// 编译实例 RVAs: 0x670520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t217513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375460640,"raw_hex":"e8a9510300","size":5,"soft_fail":false,"target_va":5375678688,"va":5375461170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6738968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375460640,"raw_hex":"e8e82b99ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375461155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140645f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_escape_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-175246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375460640,"raw_hex":"e87253fdff","size":5,"soft_fail":false,"target_va":5375287072,"va":5375462313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41158440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375460640,"raw_hex":"e828077402","size":5,"soft_fail":false,"target_va":5416620794,"va":5375462349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] obfstr::xref::inner<7773327620952731212>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375460640,"raw_hex":"e871390000","size":5,"soft_fail":false,"target_va":5375475424,"va":5375460714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":246,"record_offset":3996,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] obfstr::bytes::deobfuscate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/obfstr-0.4.3/src/bytes.rs
//     evidence={"module":246,"record_offset":3840,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":17748,"source_line":171,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] obfstr::xref::xref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"obfstr","version":"0.4.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/obfstr-0.4.3/src/xref.rs
//     evidence={"module":246,"record_offset":3784,"parent_record_offset":3640,"ancestor_record_offset":3640,"inlinee":17746,"source_line":78,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":4420,"parent_record_offset":4396,"ancestor_record_offset":3640,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":3640,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":3640,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":4396,"parent_record_offset":4368,"ancestor_record_offset":3640,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":4308,"parent_record_offset":4280,"ancestor_record_offset":3640,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":4368,"parent_record_offset":4336,"ancestor_record_offset":3640,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":4336,"parent_record_offset":4308,"ancestor_record_offset":3640,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":246,"record_offset":4144,"parent_record_offset":4084,"ancestor_record_offset":3640,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":246,"record_offset":4188,"parent_record_offset":4144,"ancestor_record_offset":3640,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":246,"record_offset":4232,"parent_record_offset":4188,"ancestor_record_offset":3640,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":246,"record_offset":4084,"parent_record_offset":4040,"ancestor_record_offset":3640,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":246,"record_offset":4040,"parent_record_offset":3996,"ancestor_record_offset":3640,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":4280,"parent_record_offset":4232,"ancestor_record_offset":3640,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::black_box
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":246,"record_offset":3808,"parent_record_offset":3784,"ancestor_record_offset":3640,"inlinee":17105,"source_line":490,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":246,"record_offset":4524,"parent_record_offset":4232,"ancestor_record_offset":3640,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":4560,"parent_record_offset":4524,"ancestor_record_offset":3640,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":3908,"parent_record_offset":3840,"ancestor_record_offset":3640,"inlinee":17749,"source_line":2080,"archive":"_pdb/C/modules/00246.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >", "tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "country_core::systems::steam::SteamOverlayOpenResources", "bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>", "bevy_ecs::system::commands::Commands", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu
// 信息 ID: e7ca3af878cadf799179e3e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ee2c21026a36ca55840aa149: void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)> > @ 0x28a1ba -> 0x670dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PhotomodeScreen> >,tuple$<bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::message::message_writer::MessageWriter<country_core::game_profile::SaveGameProfileCmd>,bevy_ecs::system::commands::Commands,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::change_detection::ResMut<country_core::systems::history::replay_mode::RestoreAppTo> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>)> @ 0x60972b -> 0x670dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$0
// 信息 ID: e1904c6e761947610477958b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$0,ref$<str$> > @ 0x317e3c -> 0x30f880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$1
// 信息 ID: c5f5880b183077dc1ab112cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$1,ref$<str$> > @ 0x31507c -> 0x30fe30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$10
// 信息 ID: dea11d404a29e22be84c4d48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$10 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$10,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$10,ref$<str$> > @ 0x3198dc -> 0x30f2a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$11
// 信息 ID: 83aae885071c42e0aaec6834
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 11, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11,ref$<str$> >; module=84, inline_record=66816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$12
// 信息 ID: e71e9464e8f4cfadc486493f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 12, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12,ref$<str$> >; module=84, inline_record=51356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$13
// 信息 ID: 4c23a2d00795745c6c5f0b85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 13, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$13 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$13,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$13,ref$<str$> >; module=84, inline_record=58448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$14
// 信息 ID: d66a4e48665a1261badbbcfa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 14, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$16,ref$<str$> > @ 0x3196d3 -> 0x30f4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$15
// 信息 ID: f6d2d15874ab6e869b94bd4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 15, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$15 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$15,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$15,ref$<str$> >; module=84, inline_record=67532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$2
// 信息 ID: d85801cb6379a9697b7e0c72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$2,ref$<str$> >; module=84, inline_record=68248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$3
// 信息 ID: 944812395b4c300765d26abc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3,ref$<str$> >; module=84, inline_record=50624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$4
// 信息 ID: cba190c5c697850d98af634b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4,ref$<str$> >; module=84, inline_record=63332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$5
// 信息 ID: 3dac30095e65110bfbbfee1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5,ref$<str$> >; module=84, inline_record=71032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$6
// 信息 ID: f1808b2ad2718020b7a6006d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6,ref$<str$> >; module=84, inline_record=46448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$7
// 信息 ID: 3eab4be8ec41b35da9af6448
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$7 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$7,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$7,ref$<str$> >; module=84, inline_record=65416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8
// 信息 ID: 90df5be1ba6105cb99e8cf41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$8 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$8,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$8,ref$<str$> > @ 0x315953 -> 0x30fa40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8::closure$0
// 信息 ID: 91eb9d74830e1acc8f8625d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$8; module=84, inline_record=10748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure$9
// 信息 ID: 8138269b26e8022f0b84467e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "tiny_glade::gui::gui_escape_menu::do_gui_escape_menu", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$9 <- toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$9,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$9,ref$<str$> > @ 0x314b0c -> 0x30f710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::open_community_server
// 信息 ID: ae1b22c7e98a009e5ff3a451
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$6,ref$<str$> > @ 0x3154ee -> 0x671c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$3,ref$<str$> > @ 0x3161ce -> 0x671c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$12,ref$<str$> > @ 0x31643e -> 0x671c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::open_newsletter
// 信息 ID: 171e92d45c917d101be90830
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$5,ref$<str$> > @ 0x319d6e -> 0x670520 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::gui::gui_escape_menu::wishlist_button
// 信息 ID: 52b469a74cc03d09e2d2a13e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$13,ref$<str$> > @ 0x317614 -> 0x670be0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$4,ref$<str$> > @ 0x318474 -> 0x670be0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/toolbar_systems.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b257bd9f346e098215ce662c97d13c785db38182518e735c92aefb6b889b9d01
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: toolbar_systems.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3911, 3945
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// use taffy; // namespace candidate
//   package identities: [{"name":"taffy","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.13"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3911,"record_offset":10320,"rva":36980064,"va":5405689184}
// 编译实例 RVAs: 0x2344560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142375b40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t201913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405689184,"raw_hex":"e8b9140300","size":5,"soft_fail":false,"target_va":5405891392,"va":5405689474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142375c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t201878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405689184,"raw_hex":"e896140300","size":5,"soft_fail":false,"target_va":5405891600,"va":5405689717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142375ce0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t202349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405689184,"raw_hex":"e86d160300","size":5,"soft_fail":false,"target_va":5405891808,"va":5405689454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142375eb0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t202705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405689184,"raw_hex":"e8d1170300","size":5,"soft_fail":false,"target_va":5405892272,"va":5405689562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142375f80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t202933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405689184,"raw_hex":"e8b5180300","size":5,"soft_fail":false,"target_va":5405892480,"va":5405689542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142376050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t203246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405689184,"raw_hex":"e8ee190300","size":5,"soft_fail":false,"target_va":5405892688,"va":5405689437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142376230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t203638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405689184,"raw_hex":"e8761b0300","size":5,"soft_fail":false,"target_va":5405893168,"va":5405689525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142376300
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t203733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405689184,"raw_hex":"e8d51b0300","size":5,"soft_fail":false,"target_va":5405893376,"va":5405689638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":10648,"parent_record_offset":10320,"ancestor_record_offset":10320,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3911,"record_offset":10472,"parent_record_offset":10320,"ancestor_record_offset":10320,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3911,"record_offset":12264,"parent_record_offset":10320,"ancestor_record_offset":10320,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":10732,"parent_record_offset":10320,"ancestor_record_offset":10320,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::roots_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3911,"record_offset":10704,"parent_record_offset":10320,"ancestor_record_offset":10320,"inlinee":122669,"source_line":76,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::style_mut
//     owner=private_source_ownership_candidate; count=20
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3911,"record_offset":10864,"parent_record_offset":10320,"ancestor_record_offset":10320,"inlinee":100419,"source_line":64,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::contextual_menus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":3911,"record_offset":12152,"parent_record_offset":10320,"ancestor_record_offset":10320,"inlinee":100124,"source_line":281,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":10672,"parent_record_offset":10648,"ancestor_record_offset":10320,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3911,"record_offset":10528,"parent_record_offset":10496,"ancestor_record_offset":10320,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":3911,"record_offset":10552,"parent_record_offset":10528,"ancestor_record_offset":10320,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3911,"record_offset":12376,"parent_record_offset":12288,"ancestor_record_offset":10320,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3911,"record_offset":10496,"parent_record_offset":10472,"ancestor_record_offset":10320,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3911,"record_offset":12424,"parent_record_offset":12400,"ancestor_record_offset":10320,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3911,"record_offset":10576,"parent_record_offset":10552,"ancestor_record_offset":10320,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3911,"record_offset":10772,"parent_record_offset":10732,"ancestor_record_offset":10320,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3911,"record_offset":10600,"parent_record_offset":10576,"ancestor_record_offset":10320,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":12288,"parent_record_offset":12264,"ancestor_record_offset":10320,"inlinee":11874,"source_line":313,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":12316,"parent_record_offset":12288,"ancestor_record_offset":10320,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::resouces_and_components::toolbar_layout::impl$1::contextual_menus::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":3911,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":10320,"inlinee":100421,"source_line":284,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::undo_redo_alignment
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3911,"record_offset":12756,"rva":36980896,"va":5405690016}
// 编译实例 RVAs: 0x23448a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142375c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t201437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690016,"raw_hex":"e8dd120300","size":5,"soft_fail":false,"target_va":5405891600,"va":5405690158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142375ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t201564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690016,"raw_hex":"e85c130300","size":5,"soft_fail":false,"target_va":5405891808,"va":5405690239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142375f80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t202299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690016,"raw_hex":"e83b160300","size":5,"soft_fail":false,"target_va":5405892480,"va":5405690176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142376050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t202588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690016,"raw_hex":"e85c170300","size":5,"soft_fail":false,"target_va":5405892688,"va":5405690095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142376230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t202879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690016,"raw_hex":"e87f180300","size":5,"soft_fail":false,"target_va":5405893168,"va":5405690284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142376300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t203258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690016,"raw_hex":"e8fa190300","size":5,"soft_fail":false,"target_va":5405893376,"va":5405690113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142375b40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t201243","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5405690016,"raw_hex":"e91b120300","size":5,"soft_fail":false,"target_va":5405891392,"va":5405690144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142375eb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t202060","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5405690016,"raw_hex":"e94c150300","size":5,"soft_fail":false,"target_va":5405892272,"va":5405690207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":12892,"parent_record_offset":12756,"ancestor_record_offset":12756,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::style_mut
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3911,"record_offset":12948,"parent_record_offset":12756,"ancestor_record_offset":12756,"inlinee":100419,"source_line":64,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":12916,"parent_record_offset":12892,"ancestor_record_offset":12756,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::undo_redo_auto_hide
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3911,"record_offset":13764,"rva":36981232,"va":5405690352}
// 编译实例 RVAs: 0x23449f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-31334452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690352,"raw_hex":"e8ccdf21fe","size":5,"soft_fail":false,"target_va":5374356528,"va":5405690975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::pressed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27283801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690352,"raw_hex":"e8a7ae5ffe","size":5,"soft_fail":false,"target_va":5378407024,"va":5405690820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27283322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690352,"raw_hex":"e886b05ffe","size":5,"soft_fail":false,"target_va":5378407392,"va":5405690709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27242073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690352,"raw_hex":"e8a75160fe","size":5,"soft_fail":false,"target_va":5378448400,"va":5405690468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23784961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690352,"raw_hex":"e8ff1195fe","size":5,"soft_fail":false,"target_va":5381906160,"va":5405691116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17844846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690352,"raw_hex":"e892b5effe","size":5,"soft_fail":false,"target_va":5387845952,"va":5405690793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17845162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405690352,"raw_hex":"e856b4effe","size":5,"soft_fail":false,"target_va":5387845616,"va":5405690773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":13912,"parent_record_offset":13764,"ancestor_record_offset":13764,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3911,"record_offset":14344,"parent_record_offset":13764,"ancestor_record_offset":13764,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3911,"record_offset":14312,"parent_record_offset":13764,"ancestor_record_offset":13764,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$40::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3911,"record_offset":13972,"parent_record_offset":13764,"ancestor_record_offset":13764,"inlinee":19989,"source_line":1333,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::root_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3911,"record_offset":15044,"parent_record_offset":13764,"ancestor_record_offset":13764,"inlinee":100100,"source_line":80,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3911,"record_offset":14532,"parent_record_offset":14500,"ancestor_record_offset":13764,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3911,"record_offset":14580,"parent_record_offset":14556,"ancestor_record_offset":13764,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3911,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":13764,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3911,"record_offset":14500,"parent_record_offset":14464,"ancestor_record_offset":13764,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3911,"record_offset":14240,"parent_record_offset":14212,"ancestor_record_offset":13764,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":13940,"parent_record_offset":13912,"ancestor_record_offset":13764,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":14368,"parent_record_offset":14344,"ancestor_record_offset":13764,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3911,"record_offset":14188,"parent_record_offset":14124,"ancestor_record_offset":13764,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3911,"record_offset":14212,"parent_record_offset":14188,"ancestor_record_offset":13764,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3911,"record_offset":14000,"parent_record_offset":13972,"ancestor_record_offset":13764,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3911,"record_offset":15212,"parent_record_offset":15188,"ancestor_record_offset":13764,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3911,"record_offset":15136,"parent_record_offset":15100,"ancestor_record_offset":13764,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3911,"record_offset":14268,"parent_record_offset":14240,"ancestor_record_offset":13764,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":15188,"parent_record_offset":15164,"ancestor_record_offset":13764,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":15068,"parent_record_offset":15044,"ancestor_record_offset":13764,"inlinee":9926,"source_line":296,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":15100,"parent_record_offset":15068,"ancestor_record_offset":13764,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":15164,"parent_record_offset":15100,"ancestor_record_offset":13764,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/lib.rs
//     evidence={"module":3911,"record_offset":15284,"parent_record_offset":15252,"ancestor_record_offset":13764,"inlinee":61955,"source_line":448,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::impl$7::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/lib.rs
//     evidence={"module":3911,"record_offset":15312,"parent_record_offset":15284,"ancestor_record_offset":13764,"inlinee":17393,"source_line":260,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::resouces_and_components::taffy_layout::impl$1::root_mut::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3911,"record_offset":15252,"parent_record_offset":15068,"ancestor_record_offset":13764,"inlinee":100093,"source_line":81,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_auto_hide
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3911,"record_offset":2408,"rva":36977168,"va":5405686288}
// 编译实例 RVAs: 0x2343a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-31332068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405686288,"raw_hex":"e81ce921fe","size":5,"soft_fail":false,"target_va":5374356528,"va":5405688591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::pressed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27281401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405686288,"raw_hex":"e807b85ffe","size":5,"soft_fail":false,"target_va":5378407024,"va":5405688420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27281009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405686288,"raw_hex":"e88fb95ffe","size":5,"soft_fail":false,"target_va":5378407392,"va":5405688396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27238532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405686288,"raw_hex":"e87c5f60fe","size":5,"soft_fail":false,"target_va":5378448400,"va":5405686927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23782709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405686288,"raw_hex":"e8cb1a95fe","size":5,"soft_fail":false,"target_va":5381906160,"va":5405688864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17842432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405686288,"raw_hex":"e800bfeffe","size":5,"soft_fail":false,"target_va":5387845952,"va":5405688379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17842743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405686288,"raw_hex":"e8c9bdeffe","size":5,"soft_fail":false,"target_va":5387845616,"va":5405688354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":2572,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":9100,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3911,"record_offset":9196,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3911,"record_offset":3276,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3911,"record_offset":2932,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3911,"record_offset":6432,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3911,"record_offset":2692,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3911,"record_offset":9164,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3911,"record_offset":6348,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":122668,"source_line":69,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::root_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3911,"record_offset":9900,"parent_record_offset":2408,"ancestor_record_offset":2408,"inlinee":100100,"source_line":80,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3911,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":2408,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3911,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":2408,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3911,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":2408,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3911,"record_offset":9356,"parent_record_offset":9316,"ancestor_record_offset":2408,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3911,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":2408,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3911,"record_offset":3060,"parent_record_offset":3028,"ancestor_record_offset":2408,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3911,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":2408,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3911,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":2408,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3911,"record_offset":9316,"parent_record_offset":9280,"ancestor_record_offset":2408,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3911,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":2408,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3911,"record_offset":5844,"parent_record_offset":5816,"ancestor_record_offset":2408,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3911,"record_offset":6088,"parent_record_offset":3324,"ancestor_record_offset":2408,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3911,"record_offset":5044,"parent_record_offset":3324,"ancestor_record_offset":2408,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3911,"record_offset":5128,"parent_record_offset":3324,"ancestor_record_offset":2408,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3911,"record_offset":6284,"parent_record_offset":3324,"ancestor_record_offset":2408,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3911,"record_offset":4960,"parent_record_offset":3324,"ancestor_record_offset":2408,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":2600,"parent_record_offset":2572,"ancestor_record_offset":2408,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":2408,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3911,"record_offset":9128,"parent_record_offset":9100,"ancestor_record_offset":2408,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3911,"record_offset":9280,"parent_record_offset":9220,"ancestor_record_offset":2408,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":4716,"parent_record_offset":4692,"ancestor_record_offset":2408,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":4364,"parent_record_offset":4336,"ancestor_record_offset":2408,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":5244,"parent_record_offset":5220,"ancestor_record_offset":2408,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":3764,"parent_record_offset":3736,"ancestor_record_offset":2408,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":4692,"parent_record_offset":4668,"ancestor_record_offset":2408,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":4740,"parent_record_offset":4716,"ancestor_record_offset":2408,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":2408,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":2408,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":8036,"parent_record_offset":8012,"ancestor_record_offset":2408,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":2408,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":5760,"parent_record_offset":5220,"ancestor_record_offset":2408,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":5816,"parent_record_offset":5784,"ancestor_record_offset":2408,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":4336,"parent_record_offset":3736,"ancestor_record_offset":2408,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":4400,"parent_record_offset":4364,"ancestor_record_offset":2408,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":8012,"parent_record_offset":6808,"ancestor_record_offset":2408,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3911,"record_offset":4668,"parent_record_offset":3324,"ancestor_record_offset":2408,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3911,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":2408,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3911,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":2408,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3911,"record_offset":3324,"parent_record_offset":3276,"ancestor_record_offset":2408,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3911,"record_offset":7240,"parent_record_offset":6808,"ancestor_record_offset":2408,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3911,"record_offset":8180,"parent_record_offset":6808,"ancestor_record_offset":2408,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3911,"record_offset":5220,"parent_record_offset":3324,"ancestor_record_offset":2408,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3911,"record_offset":3736,"parent_record_offset":3324,"ancestor_record_offset":2408,"inlinee":11676,"source_line":195,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3911,"record_offset":4288,"parent_record_offset":4264,"ancestor_record_offset":2408,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3911,"record_offset":4484,"parent_record_offset":4456,"ancestor_record_offset":2408,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3911,"record_offset":3976,"parent_record_offset":3952,"ancestor_record_offset":2408,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3911,"record_offset":4456,"parent_record_offset":4428,"ancestor_record_offset":2408,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3911,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":2408,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3911,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":2408,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3911,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":2408,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3911,"record_offset":3788,"parent_record_offset":3764,"ancestor_record_offset":2408,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3911,"record_offset":4428,"parent_record_offset":4400,"ancestor_record_offset":2408,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3911,"record_offset":3652,"parent_record_offset":3324,"ancestor_record_offset":2408,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3911,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":2408,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3911,"record_offset":3568,"parent_record_offset":3324,"ancestor_record_offset":2408,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3911,"record_offset":7692,"parent_record_offset":7288,"ancestor_record_offset":2408,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3911,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":2408,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3911,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":2408,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3911,"record_offset":7716,"parent_record_offset":7692,"ancestor_record_offset":2408,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3911,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":2408,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3911,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":2408,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3911,"record_offset":2812,"parent_record_offset":2720,"ancestor_record_offset":2408,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":3911,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":2408,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3911,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":2408,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3911,"record_offset":4060,"parent_record_offset":4000,"ancestor_record_offset":2408,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3911,"record_offset":2756,"parent_record_offset":2720,"ancestor_record_offset":2408,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3911,"record_offset":2720,"parent_record_offset":2692,"ancestor_record_offset":2408,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3911,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":2408,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3911,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":2408,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3911,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":2408,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3911,"record_offset":4124,"parent_record_offset":3952,"ancestor_record_offset":2408,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3911,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":2408,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3911,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":2408,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3911,"record_offset":3528,"parent_record_offset":3504,"ancestor_record_offset":2408,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3911,"record_offset":3452,"parent_record_offset":3416,"ancestor_record_offset":2408,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3911,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":2408,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3911,"record_offset":9476,"parent_record_offset":9316,"ancestor_record_offset":2408,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3911,"record_offset":3592,"parent_record_offset":3568,"ancestor_record_offset":2408,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3911,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":2408,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3911,"record_offset":3616,"parent_record_offset":3592,"ancestor_record_offset":2408,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3911,"record_offset":7560,"parent_record_offset":7528,"ancestor_record_offset":2408,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3911,"record_offset":3200,"parent_record_offset":3028,"ancestor_record_offset":2408,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":2408,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":10044,"parent_record_offset":10020,"ancestor_record_offset":2408,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":3416,"parent_record_offset":3324,"ancestor_record_offset":2408,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":3480,"parent_record_offset":3416,"ancestor_record_offset":2408,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":9924,"parent_record_offset":9900,"ancestor_record_offset":2408,"inlinee":9926,"source_line":296,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":9956,"parent_record_offset":9924,"ancestor_record_offset":2408,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":10020,"parent_record_offset":9956,"ancestor_record_offset":2408,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3911,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":2408,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::impl$19::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/lib.rs
//     evidence={"module":3911,"record_offset":10136,"parent_record_offset":10108,"ancestor_record_offset":2408,"inlinee":61955,"source_line":448,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::impl$7::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/lib.rs
//     evidence={"module":3911,"record_offset":10160,"parent_record_offset":10136,"ancestor_record_offset":2408,"inlinee":17393,"source_line":260,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::resouces_and_components::taffy_layout::impl$1::root_mut::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3911,"record_offset":10108,"parent_record_offset":9924,"ancestor_record_offset":2408,"inlinee":100093,"source_line":81,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 192
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E070
// 首条位置: {"module":3945,"inlinee":122992}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E072
// 首条位置: {"module":3945,"inlinee":122994}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$10
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E06F
// 首条位置: {"module":3945,"inlinee":122991}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E071
// 首条位置: {"module":3945,"inlinee":122993}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$12
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E075
// 首条位置: {"module":3945,"inlinee":122997}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_alignment::closure$13
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E078
// 首条位置: {"module":3945,"inlinee":123000}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::undo_redo_alignment::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 299
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E077
// 首条位置: {"module":3945,"inlinee":122999}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::undo_redo_alignment::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 302
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E074
// 首条位置: {"module":3945,"inlinee":122996}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::toolbar_systems::toolbar_steal_mouse_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 420
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3911,"record_offset":15504,"rva":36982256,"va":5405691376}
// 编译实例 RVAs: 0x2344df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30612482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8fee32cfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405693981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140895470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27984608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e820fd54fe","size":5,"soft_fail":false,"target_va":5377709168,"va":5405693771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409231f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: toolbar_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27402569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8b7de5dfe","size":5,"soft_fail":false,"target_va":5378290160,"va":5405692724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22852318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8224da3fe","size":5,"soft_fail":false,"target_va":5382840208,"va":5405692521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5848525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8cd3d5900","size":5,"soft_fail":false,"target_va":5411541584,"va":5405693054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5849759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e89f425900","size":5,"soft_fail":false,"target_va":5411542880,"va":5405693116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8cce5ffff","size":5,"soft_fail":false,"target_va":5405684800,"va":5405691503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5851775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e87f4a5900","size":5,"soft_fail":false,"target_va":5411543360,"va":5405691580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26854231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8a93c66fe","size":5,"soft_fail":false,"target_va":5378838448,"va":5405692674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26854614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e82a3b66fe","size":5,"soft_fail":false,"target_va":5378838032,"va":5405692641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::no_history_recording
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26854376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8183c66fe","size":5,"soft_fail":false,"target_va":5378838288,"va":5405692659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26854556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8643b66fe","size":5,"soft_fail":false,"target_va":5378838144,"va":5405692695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24306525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8a31c8dfe","size":5,"soft_fail":false,"target_va":5381385536,"va":5405692056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24301336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8e8308dfe","size":5,"soft_fail":false,"target_va":5381390640,"va":5405691971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24306335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8611d8dfe","size":5,"soft_fail":false,"target_va":5381386480,"va":5405692810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24306772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8ac1b8dfe","size":5,"soft_fail":false,"target_va":5381384704,"va":5405691471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5901874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8320e5a00","size":5,"soft_fail":false,"target_va":5411594832,"va":5405692953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20404515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8dda6c8fe","size":5,"soft_fail":false,"target_va":5385288736,"va":5405693246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5844057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e8592c5900","size":5,"soft_fail":false,"target_va":5411537104,"va":5405693042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::layout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"taffy","version":"0.3.13","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20305137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405691376,"raw_hex":"e80f2bcafe","size":5,"soft_fail":false,"target_va":5385387264,"va":5405692396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3911,"record_offset":18996,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3911,"record_offset":18116,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":3911,"record_offset":19080,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3911,"record_offset":18464,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3911,"record_offset":19332,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3911,"record_offset":18956,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3911,"record_offset":19220,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$40::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3911,"record_offset":19108,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":19989,"source_line":1333,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3911,"record_offset":19276,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3911,"record_offset":19164,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3911,"record_offset":17132,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3911,"record_offset":15696,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3911,"record_offset":16156,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3911,"record_offset":18928,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":122656,"source_line":85,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::toolbar_layout::ToolbarLayout::contextual_menus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":3911,"record_offset":18352,"parent_record_offset":15504,"ancestor_record_offset":15504,"inlinee":100124,"source_line":281,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3911,"record_offset":21732,"parent_record_offset":21708,"ancestor_record_offset":15504,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3911,"record_offset":21708,"parent_record_offset":21652,"ancestor_record_offset":15504,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3911,"record_offset":19044,"parent_record_offset":19020,"ancestor_record_offset":15504,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3911,"record_offset":21680,"parent_record_offset":21652,"ancestor_record_offset":15504,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3911,"record_offset":21652,"parent_record_offset":21628,"ancestor_record_offset":15504,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3911,"record_offset":21628,"parent_record_offset":21604,"ancestor_record_offset":15504,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":3911,"record_offset":18232,"parent_record_offset":18140,"ancestor_record_offset":15504,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3911,"record_offset":17468,"parent_record_offset":17440,"ancestor_record_offset":15504,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3911,"record_offset":17520,"parent_record_offset":17496,"ancestor_record_offset":15504,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3911,"record_offset":17496,"parent_record_offset":17468,"ancestor_record_offset":15504,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3911,"record_offset":17416,"parent_record_offset":17388,"ancestor_record_offset":15504,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3911,"record_offset":17440,"parent_record_offset":17416,"ancestor_record_offset":15504,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3911,"record_offset":17612,"parent_record_offset":17440,"ancestor_record_offset":15504,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3911,"record_offset":17724,"parent_record_offset":17700,"ancestor_record_offset":15504,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3911,"record_offset":18488,"parent_record_offset":18464,"ancestor_record_offset":15504,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3911,"record_offset":18568,"parent_record_offset":18544,"ancestor_record_offset":15504,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3911,"record_offset":18620,"parent_record_offset":18568,"ancestor_record_offset":15504,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":3911,"record_offset":18256,"parent_record_offset":18232,"ancestor_record_offset":15504,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3911,"record_offset":17544,"parent_record_offset":17520,"ancestor_record_offset":15504,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3911,"record_offset":18200,"parent_record_offset":18176,"ancestor_record_offset":15504,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3911,"record_offset":15856,"parent_record_offset":15824,"ancestor_record_offset":15504,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3911,"record_offset":19132,"parent_record_offset":19108,"ancestor_record_offset":15504,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3911,"record_offset":19188,"parent_record_offset":19164,"ancestor_record_offset":15504,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3911,"record_offset":19300,"parent_record_offset":19276,"ancestor_record_offset":15504,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3911,"record_offset":17284,"parent_record_offset":17240,"ancestor_record_offset":15504,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3911,"record_offset":18824,"parent_record_offset":18644,"ancestor_record_offset":15504,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3911,"record_offset":18176,"parent_record_offset":18140,"ancestor_record_offset":15504,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":3911,"record_offset":18140,"parent_record_offset":18116,"ancestor_record_offset":15504,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3911,"record_offset":18512,"parent_record_offset":18488,"ancestor_record_offset":15504,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3911,"record_offset":18872,"parent_record_offset":18848,"ancestor_record_offset":15504,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3911,"record_offset":19020,"parent_record_offset":18996,"ancestor_record_offset":15504,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3911,"record_offset":18544,"parent_record_offset":18464,"ancestor_record_offset":15504,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3911,"record_offset":16104,"parent_record_offset":16076,"ancestor_record_offset":15504,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3911,"record_offset":18072,"parent_record_offset":17240,"ancestor_record_offset":15504,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3911,"record_offset":18280,"parent_record_offset":18256,"ancestor_record_offset":15504,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3911,"record_offset":17676,"parent_record_offset":17388,"ancestor_record_offset":15504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3911,"record_offset":18424,"parent_record_offset":18400,"ancestor_record_offset":15504,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3911,"record_offset":18784,"parent_record_offset":18760,"ancestor_record_offset":15504,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3911,"record_offset":16568,"parent_record_offset":16544,"ancestor_record_offset":15504,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3911,"record_offset":16544,"parent_record_offset":16520,"ancestor_record_offset":15504,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3911,"record_offset":16752,"parent_record_offset":16724,"ancestor_record_offset":15504,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3911,"record_offset":18304,"parent_record_offset":18280,"ancestor_record_offset":15504,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3911,"record_offset":17568,"parent_record_offset":17544,"ancestor_record_offset":15504,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3911,"record_offset":18376,"parent_record_offset":18352,"ancestor_record_offset":15504,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3911,"record_offset":18400,"parent_record_offset":18376,"ancestor_record_offset":15504,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":16496,"parent_record_offset":16472,"ancestor_record_offset":15504,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":18760,"parent_record_offset":18736,"ancestor_record_offset":15504,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":16520,"parent_record_offset":16496,"ancestor_record_offset":15504,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":16824,"parent_record_offset":16796,"ancestor_record_offset":15504,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":15504,"inlinee":11874,"source_line":313,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":18672,"parent_record_offset":18644,"ancestor_record_offset":15504,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":16472,"parent_record_offset":16356,"ancestor_record_offset":15504,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3911,"record_offset":18736,"parent_record_offset":18672,"ancestor_record_offset":15504,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3911,"record_offset":15824,"parent_record_offset":15796,"ancestor_record_offset":15504,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3911,"record_offset":16184,"parent_record_offset":16156,"ancestor_record_offset":15504,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3911,"record_offset":16324,"parent_record_offset":16292,"ancestor_record_offset":15504,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3911,"record_offset":16292,"parent_record_offset":16256,"ancestor_record_offset":15504,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3911,"record_offset":16232,"parent_record_offset":16208,"ancestor_record_offset":15504,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3911,"record_offset":16256,"parent_record_offset":16232,"ancestor_record_offset":15504,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3911,"record_offset":16208,"parent_record_offset":16184,"ancestor_record_offset":15504,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3911,"record_offset":16796,"parent_record_offset":16256,"ancestor_record_offset":15504,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3911,"record_offset":15796,"parent_record_offset":15760,"ancestor_record_offset":15504,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3911,"record_offset":15760,"parent_record_offset":15732,"ancestor_record_offset":15504,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3911,"record_offset":15956,"parent_record_offset":15924,"ancestor_record_offset":15504,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3911,"record_offset":16052,"parent_record_offset":15956,"ancestor_record_offset":15504,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3911,"record_offset":15924,"parent_record_offset":15900,"ancestor_record_offset":15504,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3911,"record_offset":15732,"parent_record_offset":15696,"ancestor_record_offset":15504,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3911,"record_offset":16076,"parent_record_offset":16052,"ancestor_record_offset":15504,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3911,"record_offset":16356,"parent_record_offset":16324,"ancestor_record_offset":15504,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3911,"record_offset":16640,"parent_record_offset":16356,"ancestor_record_offset":15504,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3911,"record_offset":16612,"parent_record_offset":16356,"ancestor_record_offset":15504,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3911,"record_offset":16416,"parent_record_offset":16356,"ancestor_record_offset":15504,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3911,"record_offset":17168,"parent_record_offset":17132,"ancestor_record_offset":15504,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3911,"record_offset":19396,"parent_record_offset":19364,"ancestor_record_offset":15504,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3911,"record_offset":19912,"parent_record_offset":19652,"ancestor_record_offset":15504,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3911,"record_offset":17312,"parent_record_offset":17284,"ancestor_record_offset":15504,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3911,"record_offset":17388,"parent_record_offset":17240,"ancestor_record_offset":15504,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3911,"record_offset":17648,"parent_record_offset":17388,"ancestor_record_offset":15504,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3911,"record_offset":17340,"parent_record_offset":17312,"ancestor_record_offset":15504,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3911,"record_offset":17240,"parent_record_offset":17204,"ancestor_record_offset":15504,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3911,"record_offset":17204,"parent_record_offset":17168,"ancestor_record_offset":15504,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03911.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::resouces_and_components::toolbar_layout::impl$1::contextual_menus::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs
//     evidence={"module":3911,"record_offset":18848,"parent_record_offset":18824,"ancestor_record_offset":15504,"inlinee":100421,"source_line":284,"archive":"_pdb/B/modules/03911.jsonl.gz"}
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
// 完整结构及原始类型记录: toolbar_systems.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: toolbar_systems.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: toolbar_systems.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment
// 信息 ID: df4ba7da9781c0f89313d076
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 01f77029bb05d937ca08973d: void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>),void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)> > > @ 0x1e0a225 -> 0x2344560 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>),void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)> > @ 0x1eb1b1a -> 0x2344560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$0
// 信息 ID: 680f8f11fcbe8a4dca412d68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::toolbar_alignment::closure_env$0 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$0>; module=3945, inline_record=3616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$10
// 信息 ID: ae1c9731c6bad06940972b27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::toolbar_alignment::closure_env$10 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$10>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$10>; module=3945, inline_record=3304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$11
// 信息 ID: bc5eb72b8de0676ad3c1d39a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 11, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::toolbar_alignment::closure_env$11 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$11>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$11>; module=3945, inline_record=3928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$12
// 信息 ID: f8b9e2ed6ffafc7982ea27c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 12, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::toolbar_alignment::closure_env$12 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$12>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$12>; module=3945, inline_record=5200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$13
// 信息 ID: bb9f79774576bd7ddd0a3dda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 13, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::toolbar_alignment::closure_env$13 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$13>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$13>; module=3945, inline_record=6172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_alignment::closure$2
// 信息 ID: 8a533b612d351500b08ca5ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::toolbar_systems::toolbar_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::toolbar_alignment::closure_env$2 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env$2>; module=3945, inline_record=4576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_auto_hide
// 信息 ID: 9d9fccf7e7aa93acf653b9cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 53473affac35d724a387867e: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)> > > @ 0x1e0c5ac -> 0x2343a10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::system::query::Query<tuple$<ref$<toolbar::resouces_and_components::ToolbarButtonCooldown>,ref$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::DebugToolbarActivationZone> >,tuple$<> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)> > @ 0x1eb1651 -> 0x2343a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::toolbar_steal_mouse_input
// 信息 ID: 96d840ff1f9367973b124621
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c1a6140d9f59c5a7a0844ab7: void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>),void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)> > > @ 0x1e1979b -> 0x2344df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>),void (*)(bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<toolbar::toolbar_update::ToolbarSizeMult>)> > @ 0x1eb1f79 -> 0x2344df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::undo_redo_alignment
// 信息 ID: 3a700f724bae2f86f4c3a25b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a3f62fabb5a5af9af95c6ea1: void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>),void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>)> > > @ 0x1e18c79 -> 0x23448a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>),void (*)(bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>)> > @ 0x1eb0d5e -> 0x23448a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::undo_redo_alignment::closure$0
// 信息 ID: 0a88f1b15720335f09a350b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::toolbar_systems::undo_redo_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::undo_redo_alignment::closure_env$0 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$0>; module=3945, inline_record=5860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::undo_redo_alignment::closure$1
// 信息 ID: 989c8844a5f24df2dfa8f5e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::toolbar_systems::undo_redo_alignment", "captures": "unknown"}
// [闭包环境类型引用] toolbar::toolbar_systems::undo_redo_alignment::closure_env$1 <- toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::resouces_and_components::taffy_utils::taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env$1>; module=3945, inline_record=4888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::toolbar_systems::undo_redo_auto_hide
// 信息 ID: ec6ce25ed6bbd58ec1a2d06f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ff494df04d196c3c0ac22285: void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> > > @ 0x1e10e25 -> 0x23449f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >),void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::undo_redo_layout::UndoRedoLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)> > @ 0x1eb0993 -> 0x23449f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

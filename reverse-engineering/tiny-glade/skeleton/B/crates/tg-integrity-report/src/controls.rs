// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tg-integrity-report/src/controls.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f177fe81f8252ade8e61c3debe16b9ebecd9f6f84fad564232759e9042a42659
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: controls.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 268, 882, 2199, 2204, 2206, 2213
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use iced_core; // namespace candidate
//   package identities: [{"name":"iced_core","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_runtime; // namespace candidate
//   package identities: [{"name":"iced_runtime","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_widget; // namespace candidate
//   package identities: [{"name":"iced_widget","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_iced; // namespace candidate
//   package identities: [{"name":"tg-iced","source":"local_or_path","version":"0.1.0"}]
// use tg_integrity_report; // namespace candidate
//   package identities: [{"name":"tg-integrity-report","source":"local_or_path","version":"0.1.0"}]
// use webbrowser; // namespace candidate
//   package identities: [{"name":"webbrowser","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.10"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::Controls::verification_status
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":882,"record_offset":1644,"rva":14397648,"va":5383106768}
// 编译实例 RVAs: 0xdbb0d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-260819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e82d05fcff","size":5,"soft_fail":false,"target_va":5382847888,"va":5383108702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c04b0
//     owner=unresolved_namespace; count=13
//     target_aliases=122 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13619551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e8a12e30ff","size":5,"soft_fail":false,"target_va":5369496752,"va":5383116298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cfe90
//     owner=unresolved_namespace; count=15
//     target_aliases=40 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8312943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e8912781ff","size":5,"soft_fail":false,"target_va":5374803600,"va":5383116538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=34
//     target_aliases=1 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8025405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e8c38a85ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5383106888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=34
//     target_aliases=1 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8027377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e80f8385ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5383108876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419d8b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12702316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e86cd2c100","size":5,"soft_fail":false,"target_va":5395811216,"va":5383108895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419dafe0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12709886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e8feefc100","size":5,"soft_fail":false,"target_va":5395820512,"va":5383110621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419dba60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12709186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e842edc100","size":5,"soft_fail":false,"target_va":5395823200,"va":5383114009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28739629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e82d88b601","size":5,"soft_fail":false,"target_va":5411847168,"va":5383107534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=26
//     evidence={"assembly":"\tcallq\t28430776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e8b8d1b101","size":5,"soft_fail":false,"target_va":5411540992,"va":5383110211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-268452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e85ce7fbff","size":5,"soft_fail":false,"target_va":5382840208,"va":5383108655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-8757729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e81f5e7aff","size":5,"soft_fail":false,"target_va":5374354992,"va":5383112716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t28425556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e854bdb101","size":5,"soft_fail":false,"target_va":5411540963,"va":5383115402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28427070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e83ec3b101","size":5,"soft_fail":false,"target_va":5411542296,"va":5383115221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$tg_integrity_report..controls..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12687053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e8cd96c100","size":5,"soft_fail":false,"target_va":5395805536,"va":5383118478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::pattern::StrSearcher::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-202940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e844e7fcff","size":5,"soft_fail":false,"target_va":5382904128,"va":5383107063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28428005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e8e5c6b101","size":5,"soft_fail":false,"target_va":5411543360,"va":5383115350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t12713772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e82cffc100","size":5,"soft_fail":false,"target_va":5395826352,"va":5383112575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t12714689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e8c102c200","size":5,"soft_fail":false,"target_va":5395829600,"va":5383114906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::row::Row$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t12707318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e8f6e5c100","size":5,"soft_fail":false,"target_va":5395817136,"va":5383109813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2219164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106768,"raw_hex":"e86423deff","size":5,"soft_fail":false,"target_va":5380887680,"va":5383106839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5383106768,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5383109165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":882,"record_offset":10776,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":882,"record_offset":2788,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":882,"record_offset":12164,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":882,"record_offset":12136,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":882,"record_offset":20800,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":882,"record_offset":10208,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":882,"record_offset":10860,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":18664,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":882,"record_offset":12192,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":882,"record_offset":19088,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":16849,"source_line":279,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":27132,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":16870,"source_line":70,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":19860,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":16860,"source_line":333,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":882,"record_offset":12276,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":16852,"source_line":68,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":882,"record_offset":20160,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":16892,"source_line":322,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":882,"record_offset":19612,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":16903,"source_line":95,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":882,"record_offset":1852,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":55281,"source_line":7,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] tg_integrity_report::controls::Controls::advice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-integrity-report/src/controls.rs
//     evidence={"module":882,"record_offset":3260,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":55282,"source_line":86,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":1644,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":1644,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":6620,"parent_record_offset":6596,"ancestor_record_offset":1644,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":6276,"parent_record_offset":6240,"ancestor_record_offset":1644,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":2592,"parent_record_offset":2408,"ancestor_record_offset":1644,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":1644,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":882,"record_offset":6888,"parent_record_offset":6864,"ancestor_record_offset":1644,"inlinee":6411,"source_line":87,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":882,"record_offset":6240,"parent_record_offset":6204,"ancestor_record_offset":1644,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":882,"record_offset":7880,"parent_record_offset":7852,"ancestor_record_offset":1644,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":882,"record_offset":6692,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":882,"record_offset":6740,"parent_record_offset":6716,"ancestor_record_offset":1644,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":20876,"parent_record_offset":20844,"ancestor_record_offset":1644,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":2332,"parent_record_offset":2296,"ancestor_record_offset":1644,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":7104,"parent_record_offset":7076,"ancestor_record_offset":1644,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":1644,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":20924,"parent_record_offset":20900,"ancestor_record_offset":1644,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":20900,"parent_record_offset":20876,"ancestor_record_offset":1644,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":2408,"parent_record_offset":2368,"ancestor_record_offset":1644,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":2368,"parent_record_offset":2332,"ancestor_record_offset":1644,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":1644,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":2452,"parent_record_offset":2408,"ancestor_record_offset":1644,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":2248,"parent_record_offset":2204,"ancestor_record_offset":1644,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":2204,"parent_record_offset":2160,"ancestor_record_offset":1644,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":2116,"parent_record_offset":2072,"ancestor_record_offset":1644,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":2160,"parent_record_offset":2116,"ancestor_record_offset":1644,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":2072,"parent_record_offset":2028,"ancestor_record_offset":1644,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":882,"record_offset":2028,"parent_record_offset":1984,"ancestor_record_offset":1644,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$109::spec_to_string
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":1940,"parent_record_offset":1896,"ancestor_record_offset":1644,"inlinee":8861,"source_line":2877,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":1896,"parent_record_offset":1852,"ancestor_record_offset":1644,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$42::spec_to_string
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":1644,"inlinee":6409,"source_line":2912,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":1984,"parent_record_offset":1940,"ancestor_record_offset":1644,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":882,"record_offset":20844,"parent_record_offset":20800,"ancestor_record_offset":1644,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":882,"record_offset":2296,"parent_record_offset":2248,"ancestor_record_offset":1644,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":7912,"parent_record_offset":7880,"ancestor_record_offset":1644,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":1644,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1644,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":882,"record_offset":4588,"parent_record_offset":4116,"ancestor_record_offset":1644,"inlinee":5002,"source_line":633,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":882,"record_offset":4612,"parent_record_offset":4588,"ancestor_record_offset":1644,"inlinee":5004,"source_line":1765,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":882,"record_offset":3848,"parent_record_offset":3816,"ancestor_record_offset":1644,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":882,"record_offset":9864,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":882,"record_offset":6776,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":882,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":1644,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":882,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":1644,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":882,"record_offset":3940,"parent_record_offset":3620,"ancestor_record_offset":1644,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":4044,"parent_record_offset":3988,"ancestor_record_offset":1644,"inlinee":12495,"source_line":209,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":3876,"parent_record_offset":3816,"ancestor_record_offset":1644,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":3816,"parent_record_offset":3788,"ancestor_record_offset":1644,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":1644,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":3788,"parent_record_offset":3620,"ancestor_record_offset":1644,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":1644,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":882,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":1644,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":882,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":1644,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":882,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":1644,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":882,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":1644,"inlinee":12496,"source_line":750,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":882,"record_offset":4316,"parent_record_offset":4284,"ancestor_record_offset":1644,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":882,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":1644,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":882,"record_offset":5284,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":882,"record_offset":5240,"parent_record_offset":5212,"ancestor_record_offset":1644,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":882,"record_offset":2692,"parent_record_offset":2248,"ancestor_record_offset":1644,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=123
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":6428,"parent_record_offset":6240,"ancestor_record_offset":1644,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":882,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":1644,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":882,"record_offset":4436,"parent_record_offset":4408,"ancestor_record_offset":1644,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":1644,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":20964,"parent_record_offset":20800,"ancestor_record_offset":1644,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":882,"record_offset":3708,"parent_record_offset":3620,"ancestor_record_offset":1644,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":882,"record_offset":3744,"parent_record_offset":3708,"ancestor_record_offset":1644,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":882,"record_offset":12216,"parent_record_offset":12192,"ancestor_record_offset":1644,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":882,"record_offset":4408,"parent_record_offset":4356,"ancestor_record_offset":1644,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":882,"record_offset":5164,"parent_record_offset":3404,"ancestor_record_offset":1644,"inlinee":8776,"source_line":662,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":882,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":1644,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::split_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":882,"record_offset":3404,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":55030,"source_line":1925,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::TwoWaySearcher::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":882,"record_offset":3592,"parent_record_offset":3488,"ancestor_record_offset":1644,"inlinee":16918,"source_line":1457,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":882,"record_offset":3460,"parent_record_offset":3404,"ancestor_record_offset":1644,"inlinee":4997,"source_line":975,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":882,"record_offset":4116,"parent_record_offset":3488,"ancestor_record_offset":1644,"inlinee":16825,"source_line":1115,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":882,"record_offset":3488,"parent_record_offset":3404,"ancestor_record_offset":1644,"inlinee":16919,"source_line":1167,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":882,"record_offset":4260,"parent_record_offset":4228,"ancestor_record_offset":1644,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":882,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":1644,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":882,"record_offset":4228,"parent_record_offset":4184,"ancestor_record_offset":1644,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":882,"record_offset":4184,"parent_record_offset":4116,"ancestor_record_offset":1644,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":882,"record_offset":5212,"parent_record_offset":5188,"ancestor_record_offset":1644,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":882,"record_offset":4356,"parent_record_offset":4184,"ancestor_record_offset":1644,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":882,"record_offset":4496,"parent_record_offset":4356,"ancestor_record_offset":1644,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":882,"record_offset":4468,"parent_record_offset":4356,"ancestor_record_offset":1644,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=27
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":882,"record_offset":6204,"parent_record_offset":6168,"ancestor_record_offset":1644,"inlinee":16850,"source_line":32,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":882,"record_offset":6924,"parent_record_offset":6804,"ancestor_record_offset":1644,"inlinee":16847,"source_line":41,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":882,"record_offset":6952,"parent_record_offset":6804,"ancestor_record_offset":1644,"inlinee":16862,"source_line":113,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":882,"record_offset":6168,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":16849,"source_line":279,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":882,"record_offset":7468,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":16867,"source_line":112,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":882,"record_offset":8184,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":16907,"source_line":104,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":882,"record_offset":7784,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":16866,"source_line":127,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":882,"record_offset":8212,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":16869,"source_line":307,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":12380,"parent_record_offset":12300,"ancestor_record_offset":1644,"inlinee":16857,"source_line":145,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":12348,"parent_record_offset":12324,"ancestor_record_offset":1644,"inlinee":16856,"source_line":50,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":1644,"inlinee":16855,"source_line":32,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":12300,"parent_record_offset":12276,"ancestor_record_offset":1644,"inlinee":16854,"source_line":37,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":20496,"parent_record_offset":20460,"ancestor_record_offset":1644,"inlinee":16860,"source_line":333,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::helpers::button
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":882,"record_offset":7440,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":16863,"source_line":116,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":882,"record_offset":9900,"parent_record_offset":9864,"ancestor_record_offset":1644,"inlinee":16892,"source_line":322,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::text
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":882,"record_offset":5312,"parent_record_offset":3260,"ancestor_record_offset":1644,"inlinee":55281,"source_line":7,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::impl$3::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":882,"record_offset":29312,"rva":14411616,"va":5383120736}
// 编译实例 RVAs: 0xdbe760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8039396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383120736,"raw_hex":"e81c5485ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5383120895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::controls::Controls::remove_unrecognized_package_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383120736,"raw_hex":"e877c6ffff","size":5,"soft_fail":false,"target_va":5383106256,"va":5383120980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] webbrowser::open
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"webbrowser","version":"0.8.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2356451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383120736,"raw_hex":"e81d0bdcff","size":5,"soft_fail":false,"target_va":5380764368,"va":5383120814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5383120736,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5383120872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":29484,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":882,"record_offset":29456,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::none
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":882,"record_offset":30312,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":16844,"source_line":30,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::window::close
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/window.rs
//     evidence={"module":882,"record_offset":30340,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":16835,"source_line":50,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":29932,"parent_record_offset":29908,"ancestor_record_offset":29312,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":29908,"parent_record_offset":29824,"ancestor_record_offset":29312,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":882,"record_offset":29824,"parent_record_offset":29796,"ancestor_record_offset":29312,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":30548,"parent_record_offset":30520,"ancestor_record_offset":29312,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":30520,"parent_record_offset":30496,"ancestor_record_offset":29312,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":30496,"parent_record_offset":30472,"ancestor_record_offset":29312,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":29852,"parent_record_offset":29824,"ancestor_record_offset":29312,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":882,"record_offset":29876,"parent_record_offset":29852,"ancestor_record_offset":29312,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":29512,"parent_record_offset":29484,"ancestor_record_offset":29312,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":882,"record_offset":29624,"parent_record_offset":29596,"ancestor_record_offset":29312,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":882,"record_offset":29672,"parent_record_offset":29648,"ancestor_record_offset":29312,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":882,"record_offset":29648,"parent_record_offset":29624,"ancestor_record_offset":29312,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::single
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":882,"record_offset":30364,"parent_record_offset":30340,"ancestor_record_offset":29312,"inlinee":16836,"source_line":35,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":882,"record_offset":29596,"parent_record_offset":29568,"ancestor_record_offset":29312,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":882,"record_offset":29568,"parent_record_offset":29540,"ancestor_record_offset":29312,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::impl$9::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16A38
// 首条位置: {"module":2204,"inlinee":92728}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::Controls::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4235
// 首条位置: {"module":268,"inlinee":16949}
// 编译实例 RVAs: 0xdbaea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::Controls::remove_unrecognized_package_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":882,"record_offset":388,"rva":14397136,"va":5383106256}
// 编译实例 RVAs: 0xdbaed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=5
//     target_aliases=18 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8134741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106256,"raw_hex":"e8abdf83ff","size":5,"soft_fail":false,"target_va":5374971760,"va":5383106496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8024879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106256,"raw_hex":"e8d18c85ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5383106378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419d3370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12682215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106256,"raw_hex":"e8e783c100","size":5,"soft_fail":false,"target_va":5395788656,"va":5383106436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::unlink
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-334186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383106256,"raw_hex":"e896e6faff","size":5,"soft_fail":false,"target_va":5382772112,"va":5383106293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-8024906","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5383106256,"raw_hex":"e9b68c85ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5383106405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5383106256,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5383106355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":616,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":882,"record_offset":588,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":882,"record_offset":560,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] tg_integrity_report::controls::Controls::normalize_verification_result
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-integrity-report/src/controls.rs
//     evidence={"module":882,"record_offset":1444,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":55279,"source_line":60,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":388,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":1040,"parent_record_offset":956,"ancestor_record_offset":388,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":882,"record_offset":956,"parent_record_offset":928,"ancestor_record_offset":388,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":882,"record_offset":1488,"parent_record_offset":1444,"ancestor_record_offset":388,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":984,"parent_record_offset":956,"ancestor_record_offset":388,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":882,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":388,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":388,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":882,"record_offset":756,"parent_record_offset":728,"ancestor_record_offset":388,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":882,"record_offset":804,"parent_record_offset":780,"ancestor_record_offset":388,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":882,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":388,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":882,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":388,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":882,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":388,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::impl$1::remove_unrecognized_package_file::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16A2C
// 首条位置: {"module":2199,"inlinee":92716}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2199,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":6476,"inlinee":10917,"source_line":1925,"archive":"_pdb/B/modules/02199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$88::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2199,"record_offset":6528,"parent_record_offset":6500,"ancestor_record_offset":6476,"inlinee":42244,"source_line":2537,"archive":"_pdb/B/modules/02199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2199,"record_offset":6556,"parent_record_offset":6528,"ancestor_record_offset":6476,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/02199.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2199,"record_offset":6612,"parent_record_offset":6584,"ancestor_record_offset":6476,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2199,"record_offset":6640,"parent_record_offset":6612,"ancestor_record_offset":6476,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02199.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2199,"record_offset":6668,"parent_record_offset":6640,"ancestor_record_offset":6476,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02199.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2199,"record_offset":6584,"parent_record_offset":6556,"ancestor_record_offset":6476,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::Controls::normalize_verification_result
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD7EF
// 首条位置: {"module":882,"inlinee":55279}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":882,"record_offset":1488,"parent_record_offset":1444,"ancestor_record_offset":1444,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":1572,"parent_record_offset":1444,"ancestor_record_offset":1444,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":1600,"parent_record_offset":1572,"ancestor_record_offset":1444,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::Controls::advice
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD7F2
// 首条位置: {"module":882,"inlinee":55282}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":882,"record_offset":6692,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":882,"record_offset":9864,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":882,"record_offset":6776,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":882,"record_offset":5284,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::split_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":882,"record_offset":3404,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":55030,"source_line":1925,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":882,"record_offset":6168,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":16849,"source_line":279,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":882,"record_offset":7468,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":16867,"source_line":112,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":882,"record_offset":8184,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":16907,"source_line":104,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":882,"record_offset":7784,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":16866,"source_line":127,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":882,"record_offset":8212,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":16869,"source_line":307,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":882,"record_offset":7440,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":16863,"source_line":116,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":882,"record_offset":5312,"parent_record_offset":3260,"ancestor_record_offset":3260,"inlinee":55281,"source_line":7,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":3260,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":3260,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":6620,"parent_record_offset":6596,"ancestor_record_offset":3260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":6276,"parent_record_offset":6240,"ancestor_record_offset":3260,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":5972,"parent_record_offset":5788,"ancestor_record_offset":3260,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":3260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":882,"record_offset":6888,"parent_record_offset":6864,"ancestor_record_offset":3260,"inlinee":6411,"source_line":87,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":882,"record_offset":6240,"parent_record_offset":6204,"ancestor_record_offset":3260,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":882,"record_offset":7880,"parent_record_offset":7852,"ancestor_record_offset":3260,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":882,"record_offset":6740,"parent_record_offset":6716,"ancestor_record_offset":3260,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":5712,"parent_record_offset":5676,"ancestor_record_offset":3260,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":7104,"parent_record_offset":7076,"ancestor_record_offset":3260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":3260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":5788,"parent_record_offset":5748,"ancestor_record_offset":3260,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":5748,"parent_record_offset":5712,"ancestor_record_offset":3260,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":3260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":5832,"parent_record_offset":5788,"ancestor_record_offset":3260,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":5636,"parent_record_offset":5600,"ancestor_record_offset":3260,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":5600,"parent_record_offset":5564,"ancestor_record_offset":3260,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":5528,"parent_record_offset":5492,"ancestor_record_offset":3260,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":5564,"parent_record_offset":5528,"ancestor_record_offset":3260,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":5492,"parent_record_offset":5456,"ancestor_record_offset":3260,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":882,"record_offset":5456,"parent_record_offset":5420,"ancestor_record_offset":3260,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$109::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":5384,"parent_record_offset":5348,"ancestor_record_offset":3260,"inlinee":8861,"source_line":2877,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":5348,"parent_record_offset":5312,"ancestor_record_offset":3260,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$42::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":3260,"inlinee":6409,"source_line":2912,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":5420,"parent_record_offset":5384,"ancestor_record_offset":3260,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":882,"record_offset":5676,"parent_record_offset":5636,"ancestor_record_offset":3260,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":7912,"parent_record_offset":7880,"ancestor_record_offset":3260,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":5856,"parent_record_offset":5832,"ancestor_record_offset":3260,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":5880,"parent_record_offset":5856,"ancestor_record_offset":3260,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":882,"record_offset":4588,"parent_record_offset":4116,"ancestor_record_offset":3260,"inlinee":5002,"source_line":633,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":882,"record_offset":4612,"parent_record_offset":4588,"ancestor_record_offset":3260,"inlinee":5004,"source_line":1765,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":882,"record_offset":3848,"parent_record_offset":3816,"ancestor_record_offset":3260,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":882,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":3260,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":882,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":3260,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":882,"record_offset":3940,"parent_record_offset":3620,"ancestor_record_offset":3260,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":4044,"parent_record_offset":3988,"ancestor_record_offset":3260,"inlinee":12495,"source_line":209,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":3876,"parent_record_offset":3816,"ancestor_record_offset":3260,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":3816,"parent_record_offset":3788,"ancestor_record_offset":3260,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":3260,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":3788,"parent_record_offset":3620,"ancestor_record_offset":3260,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":882,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":3260,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":882,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":3260,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":882,"record_offset":5904,"parent_record_offset":5880,"ancestor_record_offset":3260,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":882,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":3260,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":882,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":3260,"inlinee":12496,"source_line":750,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":882,"record_offset":4316,"parent_record_offset":4284,"ancestor_record_offset":3260,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":882,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":3260,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":882,"record_offset":5240,"parent_record_offset":5212,"ancestor_record_offset":3260,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":882,"record_offset":6072,"parent_record_offset":5636,"ancestor_record_offset":3260,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":6428,"parent_record_offset":6240,"ancestor_record_offset":3260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":882,"record_offset":4436,"parent_record_offset":4408,"ancestor_record_offset":3260,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":6384,"parent_record_offset":6356,"ancestor_record_offset":3260,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":882,"record_offset":3708,"parent_record_offset":3620,"ancestor_record_offset":3260,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":882,"record_offset":3744,"parent_record_offset":3708,"ancestor_record_offset":3260,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":882,"record_offset":4408,"parent_record_offset":4356,"ancestor_record_offset":3260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":882,"record_offset":5164,"parent_record_offset":3404,"ancestor_record_offset":3260,"inlinee":8776,"source_line":662,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":882,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":3260,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::TwoWaySearcher::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":882,"record_offset":3592,"parent_record_offset":3488,"ancestor_record_offset":3260,"inlinee":16918,"source_line":1457,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":882,"record_offset":3460,"parent_record_offset":3404,"ancestor_record_offset":3260,"inlinee":4997,"source_line":975,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":882,"record_offset":4116,"parent_record_offset":3488,"ancestor_record_offset":3260,"inlinee":16825,"source_line":1115,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":882,"record_offset":3488,"parent_record_offset":3404,"ancestor_record_offset":3260,"inlinee":16919,"source_line":1167,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":882,"record_offset":4260,"parent_record_offset":4228,"ancestor_record_offset":3260,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":882,"record_offset":5188,"parent_record_offset":5164,"ancestor_record_offset":3260,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":882,"record_offset":4228,"parent_record_offset":4184,"ancestor_record_offset":3260,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":882,"record_offset":4184,"parent_record_offset":4116,"ancestor_record_offset":3260,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":882,"record_offset":5212,"parent_record_offset":5188,"ancestor_record_offset":3260,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":882,"record_offset":4356,"parent_record_offset":4184,"ancestor_record_offset":3260,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":882,"record_offset":4496,"parent_record_offset":4356,"ancestor_record_offset":3260,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":882,"record_offset":4468,"parent_record_offset":4356,"ancestor_record_offset":3260,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":882,"record_offset":6204,"parent_record_offset":6168,"ancestor_record_offset":3260,"inlinee":16850,"source_line":32,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":882,"record_offset":6924,"parent_record_offset":6804,"ancestor_record_offset":3260,"inlinee":16847,"source_line":41,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":882,"record_offset":6952,"parent_record_offset":6804,"ancestor_record_offset":3260,"inlinee":16862,"source_line":113,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":882,"record_offset":9900,"parent_record_offset":9864,"ancestor_record_offset":3260,"inlinee":16892,"source_line":322,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::impl$2::verification_status::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16A3E
// 首条位置: {"module":2206,"inlinee":92734}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":2206,"record_offset":2172,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":2206,"record_offset":1672,"parent_record_offset":1644,"ancestor_record_offset":1644,"inlinee":55281,"source_line":7,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":1644,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":2396,"parent_record_offset":2372,"ancestor_record_offset":1644,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1644,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":2316,"parent_record_offset":2280,"ancestor_record_offset":1644,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":2348,"parent_record_offset":2316,"ancestor_record_offset":1644,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":2084,"parent_record_offset":2028,"ancestor_record_offset":1644,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2206,"record_offset":2280,"parent_record_offset":2244,"ancestor_record_offset":1644,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2206,"record_offset":2056,"parent_record_offset":2028,"ancestor_record_offset":1644,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2206,"record_offset":2028,"parent_record_offset":2004,"ancestor_record_offset":1644,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2206,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":1644,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":1784,"parent_record_offset":1736,"ancestor_record_offset":1644,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$23::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":1644,"inlinee":5709,"source_line":2578,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":1704,"parent_record_offset":1672,"ancestor_record_offset":1644,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":1736,"parent_record_offset":1704,"ancestor_record_offset":1644,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":2206,"record_offset":1812,"parent_record_offset":1736,"ancestor_record_offset":1644,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$76::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":2206,"record_offset":1840,"parent_record_offset":1736,"ancestor_record_offset":1644,"inlinee":6437,"source_line":2652,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2206,"record_offset":1932,"parent_record_offset":1736,"ancestor_record_offset":1644,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2206,"record_offset":2424,"parent_record_offset":2396,"ancestor_record_offset":1644,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2206,"record_offset":1896,"parent_record_offset":1736,"ancestor_record_offset":1644,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":2206,"record_offset":2244,"parent_record_offset":2208,"ancestor_record_offset":1644,"inlinee":16850,"source_line":32,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":2206,"record_offset":2208,"parent_record_offset":2172,"ancestor_record_offset":1644,"inlinee":16849,"source_line":279,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::impl$2::verification_status::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16A3F
// 首条位置: {"module":2206,"inlinee":92735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":2206,"record_offset":11060,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":2206,"record_offset":10508,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":16849,"source_line":279,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":2206,"record_offset":8760,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":16867,"source_line":112,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":2206,"record_offset":9076,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":16866,"source_line":127,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":2206,"record_offset":9476,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":16869,"source_line":307,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":2206,"record_offset":8732,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":16863,"source_line":116,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::align_items
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":2206,"record_offset":11032,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":16889,"source_line":91,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":2206,"record_offset":9748,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":16903,"source_line":95,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":2206,"record_offset":8100,"parent_record_offset":8008,"ancestor_record_offset":8008,"inlinee":55281,"source_line":7,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":8008,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":8008,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":9012,"parent_record_offset":8988,"ancestor_record_offset":8008,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":9584,"parent_record_offset":9548,"ancestor_record_offset":8008,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":8508,"parent_record_offset":8480,"ancestor_record_offset":8008,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2206,"record_offset":8988,"parent_record_offset":8932,"ancestor_record_offset":8008,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2206,"record_offset":9548,"parent_record_offset":9512,"ancestor_record_offset":8008,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2206,"record_offset":9172,"parent_record_offset":9144,"ancestor_record_offset":8008,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2206,"record_offset":8420,"parent_record_offset":8392,"ancestor_record_offset":8008,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2206,"record_offset":8960,"parent_record_offset":8932,"ancestor_record_offset":8008,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2206,"record_offset":8932,"parent_record_offset":8908,"ancestor_record_offset":8008,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2206,"record_offset":8480,"parent_record_offset":8448,"ancestor_record_offset":8008,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2206,"record_offset":8448,"parent_record_offset":8420,"ancestor_record_offset":8008,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2206,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":8008,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2206,"record_offset":8356,"parent_record_offset":8328,"ancestor_record_offset":8008,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2206,"record_offset":8328,"parent_record_offset":8300,"ancestor_record_offset":8008,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2206,"record_offset":8272,"parent_record_offset":8244,"ancestor_record_offset":8008,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2206,"record_offset":8300,"parent_record_offset":8272,"ancestor_record_offset":8008,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2206,"record_offset":8244,"parent_record_offset":8216,"ancestor_record_offset":8008,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":2206,"record_offset":8216,"parent_record_offset":8188,"ancestor_record_offset":8008,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":10120,"parent_record_offset":10068,"ancestor_record_offset":8008,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$109::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":8160,"parent_record_offset":8132,"ancestor_record_offset":8008,"inlinee":8861,"source_line":2877,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$23::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":10200,"parent_record_offset":10176,"ancestor_record_offset":8008,"inlinee":5709,"source_line":2578,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":8132,"parent_record_offset":8100,"ancestor_record_offset":8008,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":10068,"parent_record_offset":10032,"ancestor_record_offset":8008,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2206,"record_offset":8188,"parent_record_offset":8160,"ancestor_record_offset":8008,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2206,"record_offset":8392,"parent_record_offset":8356,"ancestor_record_offset":8008,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2206,"record_offset":9204,"parent_record_offset":9172,"ancestor_record_offset":8008,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":2206,"record_offset":10148,"parent_record_offset":10068,"ancestor_record_offset":8008,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$76::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":2206,"record_offset":10176,"parent_record_offset":10068,"ancestor_record_offset":8008,"inlinee":6437,"source_line":2652,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2206,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":8008,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2206,"record_offset":9228,"parent_record_offset":9204,"ancestor_record_offset":8008,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2206,"record_offset":8608,"parent_record_offset":8356,"ancestor_record_offset":8008,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2206,"record_offset":8788,"parent_record_offset":8760,"ancestor_record_offset":8008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2206,"record_offset":9692,"parent_record_offset":9664,"ancestor_record_offset":8008,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2206,"record_offset":10232,"parent_record_offset":10068,"ancestor_record_offset":8008,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":2206,"record_offset":9512,"parent_record_offset":9476,"ancestor_record_offset":8008,"inlinee":16850,"source_line":32,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":2206,"record_offset":8700,"parent_record_offset":8100,"ancestor_record_offset":8008,"inlinee":16862,"source_line":113,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":2206,"record_offset":11096,"parent_record_offset":11060,"ancestor_record_offset":8008,"inlinee":16892,"source_line":322,"archive":"_pdb/B/modules/02206.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::impl$3::view
// 记录形式: compiled_procedure; instances=1
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":882,"record_offset":30884,"rva":14412064,"va":5383121184}
// 编译实例 RVAs: 0xdbe920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$core..iter..adapters..chain..Chain$LT$A$C$B$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12708838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e8e6ebc100","size":5,"soft_fail":false,"target_va":5395830304,"va":5383121461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c04b0
//     owner=unresolved_namespace; count=3
//     target_aliases=122 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13627126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e80a1130ff","size":5,"soft_fail":false,"target_va":5369496752,"va":5383123873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cfe90
//     owner=unresolved_namespace; count=3
//     target_aliases=40 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8320006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e8fa0b81ff","size":5,"soft_fail":false,"target_va":5374803600,"va":5383123601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8040111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e8515185ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5383121594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8040616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e8584f85ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5383122115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db97c0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e8f3a2ffff","size":5,"soft_fail":false,"target_va":5383100352,"va":5383124168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db9920
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e8e6a4ffff","size":5,"soft_fail":false,"target_va":5383100704,"va":5383124021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dc0e20
//     owner=unresolved_namespace; count=1
//     target_aliases=21 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e8111d0000","size":5,"soft_fail":false,"target_va":5383130656,"va":5383123210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ded0e0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t189055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e87fe20200","size":5,"soft_fail":false,"target_va":5383311584,"va":5383122524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419d8b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12689256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e8689fc100","size":5,"soft_fail":false,"target_va":5395811216,"va":5383121955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28725378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e88250b601","size":5,"soft_fail":false,"target_va":5411847168,"va":5383121785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t28417549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e80d9eb101","size":5,"soft_fail":false,"target_va":5411540992,"va":5383123438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28417490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e8d29db101","size":5,"soft_fail":false,"target_va":5411540963,"va":5383123468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ops::function::FnMut::call_mut::h5f0645c4c1005adc (.llvm.1967343670944735959)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12697600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e800c0c100","size":5,"soft_fail":false,"target_va":5395820048,"va":5383122443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$tg_integrity_report..controls..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12681862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e88682c100","size":5,"soft_fail":false,"target_va":5395805536,"va":5383123669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t12703602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e872d7c100","size":5,"soft_fail":false,"target_va":5395826352,"va":5383122745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2234030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e852e9ddff","size":5,"soft_fail":false,"target_va":5380887680,"va":5383121705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::controls::Controls::verification_status
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383121184,"raw_hex":"e880c7ffff","size":5,"soft_fail":false,"target_va":5383106768,"va":5383121227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":882,"record_offset":31328,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::horizontal_alignment
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":882,"record_offset":32476,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":17067,"source_line":95,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":882,"record_offset":32564,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16867,"source_line":112,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::width
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":882,"record_offset":32532,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16865,"source_line":92,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":882,"record_offset":32848,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16869,"source_line":307,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::align_items
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":31300,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16858,"source_line":100,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":33756,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16908,"source_line":76,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":31268,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16870,"source_line":70,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::width
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":33724,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":55283,"source_line":82,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_x
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":882,"record_offset":34572,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16878,"source_line":121,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_y
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":882,"record_offset":34600,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16879,"source_line":127,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":882,"record_offset":32504,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16863,"source_line":116,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":882,"record_offset":31084,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16852,"source_line":68,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":882,"record_offset":34168,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16873,"source_line":57,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":882,"record_offset":33396,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16890,"source_line":67,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::with_children
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":882,"record_offset":33120,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":17068,"source_line":35,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":882,"record_offset":33424,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16892,"source_line":322,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::scrollable::Scrollable::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/scrollable.rs
//     evidence={"module":882,"record_offset":33784,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":16872,"source_line":51,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::text
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":882,"record_offset":31672,"parent_record_offset":30884,"ancestor_record_offset":30884,"inlinee":55281,"source_line":7,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":31532,"parent_record_offset":31508,"ancestor_record_offset":30884,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":31556,"parent_record_offset":31532,"ancestor_record_offset":30884,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":32784,"parent_record_offset":32760,"ancestor_record_offset":30884,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":31476,"parent_record_offset":31436,"ancestor_record_offset":30884,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":31508,"parent_record_offset":31476,"ancestor_record_offset":30884,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":882,"record_offset":32760,"parent_record_offset":32736,"ancestor_record_offset":30884,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":882,"record_offset":31436,"parent_record_offset":31400,"ancestor_record_offset":30884,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":32032,"parent_record_offset":32000,"ancestor_record_offset":30884,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":32736,"parent_record_offset":32712,"ancestor_record_offset":30884,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":32100,"parent_record_offset":32064,"ancestor_record_offset":30884,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":32064,"parent_record_offset":32032,"ancestor_record_offset":30884,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":32712,"parent_record_offset":32688,"ancestor_record_offset":30884,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":882,"record_offset":32140,"parent_record_offset":32100,"ancestor_record_offset":30884,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":31960,"parent_record_offset":31928,"ancestor_record_offset":30884,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":31928,"parent_record_offset":31896,"ancestor_record_offset":30884,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":31864,"parent_record_offset":31832,"ancestor_record_offset":30884,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":31896,"parent_record_offset":31864,"ancestor_record_offset":30884,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":882,"record_offset":31832,"parent_record_offset":31800,"ancestor_record_offset":30884,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":882,"record_offset":31800,"parent_record_offset":31768,"ancestor_record_offset":30884,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$109::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":31736,"parent_record_offset":31704,"ancestor_record_offset":30884,"inlinee":8861,"source_line":2877,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":31704,"parent_record_offset":31672,"ancestor_record_offset":30884,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":882,"record_offset":31768,"parent_record_offset":31736,"ancestor_record_offset":30884,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":882,"record_offset":32000,"parent_record_offset":31960,"ancestor_record_offset":30884,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":32164,"parent_record_offset":32140,"ancestor_record_offset":30884,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":882,"record_offset":32188,"parent_record_offset":32164,"ancestor_record_offset":30884,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":882,"record_offset":33184,"parent_record_offset":33152,"ancestor_record_offset":30884,"inlinee":4555,"source_line":56,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":882,"record_offset":33212,"parent_record_offset":33152,"ancestor_record_offset":30884,"inlinee":9468,"source_line":258,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":882,"record_offset":33316,"parent_record_offset":33292,"ancestor_record_offset":30884,"inlinee":9474,"source_line":263,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":882,"record_offset":33824,"parent_record_offset":33784,"ancestor_record_offset":30884,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":882,"record_offset":32404,"parent_record_offset":32380,"ancestor_record_offset":30884,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":882,"record_offset":32236,"parent_record_offset":32212,"ancestor_record_offset":30884,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":882,"record_offset":33244,"parent_record_offset":33212,"ancestor_record_offset":30884,"inlinee":9470,"source_line":40,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":882,"record_offset":31224,"parent_record_offset":31196,"ancestor_record_offset":30884,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":882,"record_offset":33268,"parent_record_offset":33244,"ancestor_record_offset":30884,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":882,"record_offset":32212,"parent_record_offset":32188,"ancestor_record_offset":30884,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\try_trait.rs
//     evidence={"module":882,"record_offset":33292,"parent_record_offset":33268,"ancestor_record_offset":30884,"inlinee":9472,"source_line":395,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\try_trait.rs
//     evidence={"module":882,"record_offset":33348,"parent_record_offset":33268,"ancestor_record_offset":30884,"inlinee":13491,"source_line":406,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":882,"record_offset":32380,"parent_record_offset":31960,"ancestor_record_offset":30884,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":31628,"parent_record_offset":31436,"ancestor_record_offset":30884,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":882,"record_offset":31584,"parent_record_offset":31556,"ancestor_record_offset":30884,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":882,"record_offset":31400,"parent_record_offset":31364,"ancestor_record_offset":30884,"inlinee":16850,"source_line":32,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":31196,"parent_record_offset":31112,"ancestor_record_offset":30884,"inlinee":16857,"source_line":145,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":31164,"parent_record_offset":31140,"ancestor_record_offset":30884,"inlinee":16856,"source_line":50,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":31140,"parent_record_offset":31112,"ancestor_record_offset":30884,"inlinee":16855,"source_line":32,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":31112,"parent_record_offset":31084,"ancestor_record_offset":30884,"inlinee":16854,"source_line":37,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":882,"record_offset":31364,"parent_record_offset":31328,"ancestor_record_offset":30884,"inlinee":16860,"source_line":333,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":882,"record_offset":34204,"parent_record_offset":34168,"ancestor_record_offset":30884,"inlinee":16875,"source_line":50,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":882,"record_offset":34664,"parent_record_offset":34628,"ancestor_record_offset":30884,"inlinee":16881,"source_line":313,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::row::Row::extend
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":882,"record_offset":33152,"parent_record_offset":33120,"ancestor_record_offset":30884,"inlinee":17069,"source_line":136,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::scrollable::impl$6::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/scrollable.rs
//     evidence={"module":882,"record_offset":34288,"parent_record_offset":34252,"ancestor_record_offset":30884,"inlinee":16877,"source_line":420,"archive":"_pdb/B/modules/00882.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::impl$4::active
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 299
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD7FC
// 首条位置: {"module":882,"record_offset":34948,"rva":6481696,"va":5375190816}
// 编译实例 RVAs: 0x62e720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7978907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375190816,"raw_hex":"e89bbf7900","size":5,"soft_fail":false,"target_va":5383169744,"va":5375190832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::impl$4::hovered
// 记录形式: compiled_procedure; instances=1
// 源码行: 307
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":882,"record_offset":35072,"rva":6481840,"va":5375190960}
// 编译实例 RVAs: 0x62e7b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7978763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375190960,"raw_hex":"e80bbf7900","size":5,"soft_fail":false,"target_va":5383169744,"va":5375190976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::impl$5::active
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD7FF
// 首条位置: {"module":882,"record_offset":35196,"rva":14415152,"va":5383124272}
// 编译实例 RVAs: 0xdbf530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t45447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383124272,"raw_hex":"e887b10000","size":5,"soft_fail":false,"target_va":5383169744,"va":5383124292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28722837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383124272,"raw_hex":"e89546b601","size":5,"soft_fail":false,"target_va":5411847168,"va":5383124326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_integrity_report::controls::impl$5::hovered
// 记录形式: compiled_procedure; instances=1
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":882,"record_offset":35320,"rva":14415312,"va":5383124432}
// 编译实例 RVAs: 0xdbf5d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t45267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383124432,"raw_hex":"e8d3b00000","size":5,"soft_fail":false,"target_va":5383169744,"va":5383124472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28722657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383124432,"raw_hex":"e8e145b601","size":5,"soft_fail":false,"target_va":5411847168,"va":5383124506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: controls.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_integrity_report::controls::Controls
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_integrity_report::controls::Controls>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_integrity_report::controls::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>", "constructor": "tg_iced::gui_runner::run_gui", "arguments": ["tg_integrity_report::controls::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$0<tg_integrity_report::controls::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$0", "arguments": ["tg_integrity_report::controls::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_integrity_report::controls::Controls> >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::program::state::impl$0::update::closure_env$0<tg_integrity_report::controls::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_integrity_report::controls::Controls> > >", "constructor": "core::iter::adapters::cloned::Cloned", "arguments": ["core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_integrity_report::controls::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<iced_core::event::Event>,core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_integrity_report::controls::Controls> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<iced_core::event::Event>", "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_integrity_report::controls::Controls> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State<tg_integrity_report::controls::Controls>", "constructor": "iced_runtime::program::state::State", "arguments": ["tg_integrity_report::controls::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::program::state::State<tg_integrity_report::controls::Controls> >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::program::state::State<tg_integrity_report::controls::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$1", "arguments": ["tg_integrity_report::controls::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::new<tg_integrity_report::controls::Controls>", "constructor": "iced_runtime::program::state::State::new", "arguments": ["tg_integrity_report::controls::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::update<tg_integrity_report::controls::Controls>", "constructor": "iced_runtime::program::state::State::update", "arguments": ["tg_integrity_report::controls::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls>", "constructor": "tg_iced::gui_runner::run_gui_with_options::closure_env$1", "arguments": ["tg_integrity_report::controls::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::EventLoop::pump_events", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<tg_integrity_report::controls::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >", "tuple$<enum2$<winit::event::Event<tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_integrity_report::controls::DeleteFileButton
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::pressed<tg_integrity_report::controls::DeleteFileButton>", "constructor": "iced_style::button::StyleSheet::pressed", "arguments": ["tg_integrity_report::controls::DeleteFileButton"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::disabled<tg_integrity_report::controls::DeleteFileButton>", "constructor": "iced_style::button::StyleSheet::disabled", "arguments": ["tg_integrity_report::controls::DeleteFileButton"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_integrity_report::controls::HyperlinkStyle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::pressed<tg_integrity_report::controls::HyperlinkStyle>", "constructor": "iced_style::button::StyleSheet::pressed", "arguments": ["tg_integrity_report::controls::HyperlinkStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::disabled<tg_integrity_report::controls::HyperlinkStyle>", "constructor": "iced_style::button::StyleSheet::disabled", "arguments": ["tg_integrity_report::controls::HyperlinkStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_integrity_report::controls::Message
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 247
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_integrity_report::controls::Message>", "constructor": "enum2$", "arguments": ["tg_integrity_report::controls::Message"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<tg_integrity_report::controls::Message> >", "constructor": "tuple$", "arguments": ["enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > >", "constructor": "core::ops::function::Fn", "arguments": ["tuple$<enum2$<tg_integrity_report::controls::Message> >", "assoc$<Output,enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "constructor": "dyn$", "arguments": ["core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::Column", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::Map", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::Element", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::option::IntoIter", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::Map", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::sources::once::Once<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::sources::once::Once", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::sources::once::Once<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::sources::once::Once<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::size<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::size", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::scrollable<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::scrollable", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::text_input<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::text_input", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::custom<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::custom", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::container<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::container", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::container::closure$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::container::closure$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::focusable<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::focusable", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$0,tuple$<enum2$<tg_integrity_report::controls::Message> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["crash_reporter_gui::impl$3::main_view::closure_env$0", "tuple$<enum2$<tg_integrity_report::controls::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>,tuple$<enum2$<tg_integrity_report::controls::Message> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "tuple$<enum2$<tg_integrity_report::controls::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::container::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_core::element::impl$4::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_core::element::impl$4::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation::finish<iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >,enum2$<tg_integrity_report::controls::Message> >", "constructor": "iced_core::widget::operation::Operation::finish", "arguments": ["iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >", "enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::push<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::column::Column::push", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::mouse_interaction<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::mouse_interaction", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::diff<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::diff", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::draw<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::draw", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::layout<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::layout", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::overlay<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::overlay", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::children<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::children", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::on_event<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::on_event", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::impl$1::fold<core::iter::sources::once::Once<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::chain::impl$1::fold", "arguments": ["core::iter::sources::once::Once<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::Iter", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$1", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<iced_core::widget::tree::Tree,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["iced_core::widget::tree::Tree", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >", "constructor": "ref$", "arguments": ["dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::Element", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::slice::iter::IterMut<iced_core::widget::tree::Tree>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::from_children::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "ref$", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::widget::tree::Tree,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::widget::tree::Tree", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::layout::node::Node,core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::layout::node::Node", "core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$30::call<tuple$<enum2$<tg_integrity_report::controls::Message> >,dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$30::call", "arguments": ["tuple$<enum2$<tg_integrity_report::controls::Message> >", "dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$29::call_mut<tuple$<enum2$<tg_integrity_report::controls::Message> >,dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$29::call_mut", "arguments": ["tuple$<enum2$<tg_integrity_report::controls::Message> >", "dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> >", "constructor": "iced_runtime::command::action::Action", "arguments": ["enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >", "constructor": "enum2$", "arguments": ["iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::impl$0::update::closure_env$2", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::event::Status,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::event::Status", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Item,enum2$<tg_integrity_report::controls::Message> >", "constructor": "assoc$", "arguments": ["Item", "enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_core::stream::Stream<assoc$<Item,enum2$<tg_integrity_report::controls::Message> > >", "constructor": "futures_core::stream::Stream", "arguments": ["assoc$<Item,enum2$<tg_integrity_report::controls::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_integrity_report::controls::Message> > >,core::marker::Send>", "constructor": "dyn$", "arguments": ["futures_core::stream::Stream<assoc$<Item,enum2$<tg_integrity_report::controls::Message> > >", "core::marker::Send"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_integrity_report::controls::Message> > >,core::marker::Send>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_integrity_report::controls::Message> > >,core::marker::Send>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_integrity_report::controls::Message> > >,core::marker::Send>,alloc::alloc::Global> >", "constructor": "core::pin::Pin", "arguments": ["alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_integrity_report::controls::Message> > >,core::marker::Send>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_integrity_report::controls::Message> > >,core::marker::Send>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,enum2$<tg_integrity_report::controls::Message> > >,core::marker::Send>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::Row", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::Button", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::button::Button<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::button::Button<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::Group<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::Group", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::group::Group<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::group::Group<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::Container<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::Container", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::container::Container<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::container::Container<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::Scrollable<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::Scrollable", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::scrollable::Scrollable<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::scrollable::Scrollable<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::Nested<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::Nested", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::overlay::nested::Nested<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::overlay::nested::Nested<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::user_interface::UserInterface<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::user_interface::UserInterface<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Output,enum2$<tg_integrity_report::controls::Message> >", "constructor": "assoc$", "arguments": ["Output", "enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_integrity_report::controls::Message> > >", "constructor": "core::ops::function::Fn", "arguments": ["tuple$<iced_widget::scrollable::Viewport>", "assoc$<Output,enum2$<tg_integrity_report::controls::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_integrity_report::controls::Message> > > >", "constructor": "dyn$", "arguments": ["core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_integrity_report::controls::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_integrity_report::controls::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_integrity_report::controls::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_integrity_report::controls::Message> > > >,alloc::alloc::Global> >", "constructor": "core::option::Option", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_integrity_report::controls::Message> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_integrity_report::controls::Message> > > >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_integrity_report::controls::Message> > > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_integrity_report::controls::Message> > > >,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<tg_integrity_report::controls::Message> > > >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,1>", "constructor": "core::array::iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,1> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "core::array::iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>", "constructor": "core::array::iter::IntoIter", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::iter::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::option::Option", "arguments": ["core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::sources::once::Once<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::sources::once::Once<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command<enum2$<tg_integrity_report::controls::Message> >", "constructor": "iced_runtime::command::Command", "arguments": ["enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::command::Command<enum2$<tg_integrity_report::controls::Message> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::command::Command<enum2$<tg_integrity_report::controls::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$1::draw<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$1::draw", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$1::size<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$1::size", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::size<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::size", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::size<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::size", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::size<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::size", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::size<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::size", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::size<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::size", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::mouse_interaction<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::mouse_interaction", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::mouse_interaction<iced_widget::space::Space,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::mouse_interaction", "arguments": ["iced_widget::space::Space", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_widget::space::Space,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_widget::space::Space", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::state<iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::state", "arguments": ["iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::state<iced_widget::space::Space,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::state", "arguments": ["iced_widget::space::Space", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::operate<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::operate", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::operate<iced_widget::space::Space,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::operate", "arguments": ["iced_widget::space::Space", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::overlay<iced_widget::space::Space,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::overlay", "arguments": ["iced_widget::space::Space", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::overlay<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::overlay", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_widget::space::Space,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_widget::space::Space", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button::new<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::button::Button::new", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::mouse_interaction<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::mouse_interaction", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::tag<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::tag", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::diff<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::diff", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::draw<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::draw", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::state<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::state", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::layout<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::layout", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate::closure$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate::closure$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::overlay<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::overlay", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::children<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::children", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::on_event<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::on_event", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> >", "constructor": "iced_core::widget::operation::Operation", "arguments": ["enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > >", "constructor": "dyn$", "arguments": ["iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > >", "constructor": "ref_mut$", "arguments": ["dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > >", "constructor": "tuple$", "arguments": ["ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::button::impl$1::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::button::impl$1::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::push<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec::push", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::button::Button<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::button::Button<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row::with_children<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::row::Row::with_children", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row::with_children<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "iced_widget::row::Row::with_children", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::mouse_interaction<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::mouse_interaction", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::diff<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::diff", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::draw<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::draw", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::layout<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::layout", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate::closure$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate::closure$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::overlay<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::overlay", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::children<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::children", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::on_event<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::on_event", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "tuple$", "arguments": ["iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnMut::call_mut<iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),tuple$<iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::ops::function::FnMut::call_mut", "arguments": ["iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "tuple$<iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::row::impl$2::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::row::impl$2::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::row::Row<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::slice::iter::Iter<alloc::string::String>,tg_integrity_report::controls::impl$2::verification_status::closure_env$0,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::slice::iter::Iter<alloc::string::String>", "tg_integrity_report::controls::impl$2::verification_status::closure_env$0", "iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$20::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::vec::impl$20::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::event::Status,core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$20::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::event::Status", "core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$20::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::slice::iter::Iter<alloc::string::String>,tg_integrity_report::controls::impl$2::verification_status::closure_env$2,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::slice::iter::Iter<alloc::string::String>", "tg_integrity_report::controls::impl$2::verification_status::closure_env$2", "iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::column::impl$2::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::column::impl$2::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::helpers::column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,alloc::vec::Vec<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "iced_widget::helpers::column", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "alloc::vec::Vec<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::impl$1::fold<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >,core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::chain::impl$1::fold", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "core::option::IntoIter<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::notify_on_scroll<enum2$<tg_integrity_report::controls::Message> >", "constructor": "iced_widget::scrollable::notify_on_scroll", "arguments": ["enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::mouse_interaction<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::mouse_interaction", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::tag<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::tag", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::diff<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::diff", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::draw<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::draw", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::state<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::state", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::layout<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::layout", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate::closure$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate::closure$2", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::overlay<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::overlay", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::children<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::children", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::on_event<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::on_event", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate::closure_env$2", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::scrollable::impl$5::operate::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::scrollable::impl$5::operate::closure_env$2<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::scrollable::Scrollable<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::scrollable::Scrollable<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::shell::Shell::merge<enum2$<crash_reporter_gui::Message>,enum2$<tg_integrity_report::controls::Message>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "constructor": "iced_core::shell::Shell::merge", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<tg_integrity_report::controls::Message>", "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::shell::Shell::merge<enum2$<crash_reporter_gui::Message>,enum2$<tg_integrity_report::controls::Message>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "iced_core::shell::Shell::merge", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<tg_integrity_report::controls::Message>", "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation::finish<iced_core::overlay::element::impl$2::operate::MapOperation<enum2$<crash_reporter_gui::Message> >,enum2$<tg_integrity_report::controls::Message> >", "constructor": "iced_core::widget::operation::Operation::finish", "arguments": ["iced_core::overlay::element::impl$2::operate::MapOperation<enum2$<crash_reporter_gui::Message> >", "enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::Tree::diff_children_custom<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::widget::tree::Tree::diff_children_custom", "arguments": ["iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<tg_integrity_report::controls::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$1::layout<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$1::layout", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::space::Space,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::space::Space", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_widget::space::Space,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_widget::space::Space", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::space::Space,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::space::Space", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::mouse_interaction<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::mouse_interaction", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::tag<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::tag", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::diff<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::diff", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::draw<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::draw", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::state<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::state", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::layout<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::layout", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate::closure$1<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate::closure$1", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::overlay<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::overlay", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::children<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::children", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::on_event<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::on_event", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate::closure_env$1<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate::closure_env$1", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::container::impl$1::operate::closure_env$1<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::container::impl$1::operate::closure_env$1<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::container::Container<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::container::Container<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::tag<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::tag", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::draw<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::draw", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::state<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::state", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::layout<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::layout", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::draw::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure$1<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure$1", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure_env$0", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_graphics::renderer::impl$1::with_layer<iced_tiny_skia::backend::Backend,iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_graphics::renderer::impl$1::with_layer", "arguments": ["iced_tiny_skia::backend::Backend", "iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure_env$2", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_graphics::renderer::impl$1::with_layer<iced_tiny_skia::backend::Backend,iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_graphics::renderer::impl$1::with_layer", "arguments": ["iced_tiny_skia::backend::Backend", "iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::layout::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::layout::padded::closure_env$0", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::positioned<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::positioned", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::layout::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::layout::closure_env$0", "arguments": ["iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::layout::closure_env$1", "arguments": ["iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::positioned<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::positioned", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::layout::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::layout::closure_env$0", "arguments": ["iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::contained<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::contained", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::flex::resolve<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::layout::flex::resolve", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::mouse_interaction<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::mouse_interaction", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::draw<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::draw", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::is_over<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::is_over", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate::closure$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate::closure$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::into_cache<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::into_cache", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::draw<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::draw", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::build<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_runtime::user_interface::UserInterface::build", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::update<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::update", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::on_event<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::on_event", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<tg_integrity_report::controls::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::draw::recurse<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::draw::recurse", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::layout::recurse<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::layout::recurse", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::is_over::recurse<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::is_over::recurse", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::on_event::recurse<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::on_event::recurse", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: controls.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: controls.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)
// 候选参数类型: ["ref$<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::Controls::advice
// 信息 ID: 739f1dc1f26342b78588d851
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_integrity_report::controls::Controls::verification_status; module=882, inline_record=3260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::Controls::new
// 信息 ID: d0465832b549a93ed73bc621
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_reporter_gui::Controls::new @ 0x61f6f2 -> 0xdbaea0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_integrity_report::run_integrity_report_gui; module=268, inline_record=400; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::Controls::normalize_verification_result
// 信息 ID: e29b16ccd294b97d812bbc7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_integrity_report::controls::Controls::remove_unrecognized_package_file; module=882, inline_record=1444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::Controls::remove_unrecognized_package_file
// 信息 ID: f61c04fc51e560ab3b58fbda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_integrity_report::controls::impl$3::update @ 0xdbe854 -> 0xdbaed0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::Controls::verification_status
// 信息 ID: 28f73bc218cab8e680d6b308
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_reporter_gui::Controls::main_view @ 0x623a55 -> 0xdbb0d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_integrity_report::controls::impl$3::view @ 0xdbe94b -> 0xdbb0d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::impl$1::remove_unrecognized_package_file::closure$0
// 信息 ID: 7b8f98ffb3251cebe2d7c1d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_integrity_report::controls::impl$1::remove_unrecognized_package_file", "captures": "unknown"}
// [闭包环境类型引用] tg_integrity_report::controls::impl$1::remove_unrecognized_package_file::closure_env$0 <- alloc::vec::Vec::retain<alloc::string::String,alloc::alloc::Global,tg_integrity_report::controls::impl$1::remove_unrecognized_package_file::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::string::String,alloc::alloc::Global,tg_integrity_report::controls::impl$1::remove_unrecognized_package_file::closure_env$0>; module=2199, inline_record=6476; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::string::String,alloc::alloc::Global,tg_integrity_report::controls::impl$1::remove_unrecognized_package_file::closure_env$0>; module=2199, inline_record=7044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::impl$2::verification_status::closure$0
// 信息 ID: 95c9df273eba3f48850a283e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 18d8741a28d27deaef6be6a5: iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_integrity_report::controls::impl$2::verification_status", "captures": "unknown"}
// [闭包环境类型引用] tg_integrity_report::controls::impl$2::verification_status::closure_env$0 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::slice::iter::Iter<alloc::string::String>,tg_integrity_report::controls::impl$2::verification_status::closure_env$0,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::slice::iter::Iter<alloc::string::String>,tg_integrity_report::controls::impl$2::verification_status::closure_env$0,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; module=2206, inline_record=1644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::impl$2::verification_status::closure$2
// 信息 ID: 50b380f19c648dd04ef16130
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 18d8741a28d27deaef6be6a5: iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_integrity_report::controls::impl$2::verification_status", "captures": "unknown"}
// [闭包环境类型引用] tg_integrity_report::controls::impl$2::verification_status::closure_env$2 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::slice::iter::Iter<alloc::string::String>,tg_integrity_report::controls::impl$2::verification_status::closure_env$2,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::slice::iter::Iter<alloc::string::String>,tg_integrity_report::controls::impl$2::verification_status::closure_env$2,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; module=2206, inline_record=8008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::impl$3::update
// 信息 ID: cc7ffd4290134a688145f519
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::command::Command::batch<enum2$<tg_integrity_report::controls::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_integrity_report::controls::Controls> > > @ 0x19ed7ae -> 0xdbe760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::impl$3::view
// 信息 ID: 2a789532f126410dc66ca1b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::program::state::State::new<tg_integrity_report::controls::Controls> @ 0x19eb891 -> 0xdbe920 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_runtime::program::state::State::update<tg_integrity_report::controls::Controls> @ 0x19ebad4 -> 0xdbe920 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_runtime::program::state::State::update<tg_integrity_report::controls::Controls> @ 0x19ebe09 -> 0xdbe920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::impl$4::active
// 信息 ID: 8006c19730c59c2b7ab065e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_style::button::StyleSheet::pressed<tg_integrity_report::controls::HyperlinkStyle>; module=2213, inline_record=440; 不是独立调用边。
// [PDB 内联宿主] iced_style::button::StyleSheet::disabled<tg_integrity_report::controls::HyperlinkStyle>; module=2213, inline_record=848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::impl$4::hovered
// 信息 ID: f140ff64456051f92e682bdf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::impl$5::active
// 信息 ID: 261e7f87ba8d49e3a42b7a3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_style::button::StyleSheet::pressed<tg_integrity_report::controls::DeleteFileButton>; module=2213, inline_record=644; 不是独立调用边。
// [PDB 内联宿主] iced_style::button::StyleSheet::disabled<tg_integrity_report::controls::DeleteFileButton>; module=2213, inline_record=1060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::impl$5::hovered
// 信息 ID: 3355a961da96273d5d620271
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_integrity_report::controls::impl$9::clone
// 信息 ID: 2d99360a3f84a64ebcff1036
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_widget::button::impl$1::on_event<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >; module=2204, inline_record=11120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

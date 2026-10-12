// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/crash-reporter-gui/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 2696f8cf12a52dc812d008a2c58977bdce887781fd0eb6a29c44e35e8bee1a24
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 137, 155, 211, 265, 888, 889, 890, 891, 896
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use arboard; // namespace candidate
//   package identities: [{"name":"arboard","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.6.0"}]
// use core; // namespace candidate
//   package identities: []
// use crash_reporter_gui; // namespace candidate
//   package identities: [{"name":"crash-reporter-gui","source":"local_or_path","version":"0.1.0"}]
// use iced_core; // namespace candidate
//   package identities: [{"name":"iced_core","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_runtime; // namespace candidate
//   package identities: [{"name":"iced_runtime","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_style; // namespace candidate
//   package identities: [{"name":"iced_style","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_widget; // namespace candidate
//   package identities: [{"name":"iced_widget","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use open; // namespace candidate
//   package identities: [{"name":"open","source":"registry+https://github.com/rust-lang/crates.io-index","version":"5.0.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// use tg_integrity_report; // namespace candidate
//   package identities: [{"name":"tg-integrity-report","source":"local_or_path","version":"0.1.0"}]
// use webbrowser; // namespace candidate
//   package identities: [{"name":"webbrowser","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.10"}]
// use winit; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.9.0"},{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::Controls::main_view
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":265,"record_offset":31512,"rva":6435696,"va":5375144816}
// 编译实例 RVAs: 0x623370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t7702657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e881887500","size":5,"soft_fail":false,"target_va":5382847888,"va":5375145226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$core..iter..adapters..chain..Chain$LT$A$C$B$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8000411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e89b137a00","size":5,"soft_fail":false,"target_va":5383161840,"va":5375161424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c04b0
//     owner=unresolved_namespace; count=53
//     target_aliases=122 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5666134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8aa8aa9ff","size":5,"soft_fail":false,"target_va":5369496752,"va":5375162881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cfe90
//     owner=unresolved_namespace; count=21
//     target_aliases=40 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-372440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e82851faff","size":5,"soft_fail":false,"target_va":5374803600,"va":5375176035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=86
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-63816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8b806ffff","size":5,"soft_fail":false,"target_va":5375081488,"va":5375145299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=70
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-63778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8de06ffff","size":5,"soft_fail":false,"target_va":5375081504,"va":5375145277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db9650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7924365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e88dea7800","size":5,"soft_fail":false,"target_va":5383099984,"va":5375175614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db97c0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7923848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e888e87800","size":5,"soft_fail":false,"target_va":5383100352,"va":5375176499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dc0e20
//     owner=unresolved_namespace; count=1
//     target_aliases=21 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7968704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8c0977900","size":5,"soft_fail":false,"target_va":5383130656,"va":5375161947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dc1830
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7981777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8d1ca7900","size":5,"soft_fail":false,"target_va":5383133232,"va":5375151450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=29
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36701578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e88a053002","size":5,"soft_fail":false,"target_va":5411847168,"va":5375145585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=55
//     evidence={"assembly":"\tcallq\t36369074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8b2f22a02","size":5,"soft_fail":false,"target_va":5411540992,"va":5375171913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t7695018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8aa6a7500","size":5,"soft_fail":false,"target_va":5382840208,"va":5375145185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=37
//     evidence={"assembly":"\tcallq\t-790478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e832f0f3ff","size":5,"soft_fail":false,"target_va":5374354992,"va":5375145465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36177990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e846082802","size":5,"soft_fail":false,"target_va":5411327968,"va":5375149973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=31
//     evidence={"assembly":"\tcallq\t36368915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e813f22a02","size":5,"soft_fail":false,"target_va":5411540963,"va":5375172043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_core..element..Map$LT$tg_integrity_report..controls..Message$C$crash_reporter_gui..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7925731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8e3ef7800","size":5,"soft_fail":false,"target_va":5383101232,"va":5375175496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$crash_reporter_gui..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=32
//     evidence={"assembly":"\tcallq\t7921800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e888e07800","size":5,"soft_fail":false,"target_va":5383099568,"va":5375177763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::pattern::StrSearcher::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7754447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8cf527600","size":5,"soft_fail":false,"target_va":5382904128,"va":5375149676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e84dadffff","size":5,"soft_fail":false,"target_va":5375128640,"va":5375149806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_reporter_gui::Controls::more_info_button
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t31254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8167a0000","size":5,"soft_fail":false,"target_va":5375189120,"va":5375157861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t7984192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e840d47900","size":5,"soft_fail":false,"target_va":5383139120,"va":5375154923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t7983660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e82cd27900","size":5,"soft_fail":false,"target_va":5383144368,"va":5375160703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::row
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t7979172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8a4c07900","size":5,"soft_fail":false,"target_va":5383143760,"va":5375164583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::row::Row$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t8001210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8ba167a00","size":5,"soft_fail":false,"target_va":5383155696,"va":5375154481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=26
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5742763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e8aba05700","size":5,"soft_fail":false,"target_va":5380887680,"va":5375144912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::controls::Controls::verification_status
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7960182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375144816,"raw_hex":"e876767900","size":5,"soft_fail":false,"target_va":5383106768,"va":5375146581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375144816,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375151633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":38340,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":105252,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":265,"record_offset":31848,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":81340,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":265,"record_offset":53636,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16882,"source_line":271,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":31764,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":33172,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":265,"record_offset":32588,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":265,"record_offset":31932,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":56392,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=22
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":31960,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16845,"source_line":283,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::wide_button
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":59572,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16861,"source_line":493,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_core::element::Element::map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":265,"record_offset":81424,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16893,"source_line":212,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":84244,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16849,"source_line":279,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":61672,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16867,"source_line":112,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":96272,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16907,"source_line":104,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":65348,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16866,"source_line":127,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":65768,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16869,"source_line":307,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::checkbox::Checkbox::on_toggle
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/checkbox.rs
//     evidence={"module":265,"record_offset":82676,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16899,"source_line":103,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::align_items
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":103128,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16858,"source_line":100,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":104824,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16908,"source_line":76,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":84784,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16870,"source_line":70,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":84816,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16860,"source_line":333,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":265,"record_offset":87256,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16906,"source_line":133,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":265,"record_offset":64812,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16863,"source_line":116,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::checkbox
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":265,"record_offset":81920,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16896,"source_line":159,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":265,"record_offset":85572,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16852,"source_line":68,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":265,"record_offset":86736,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16873,"source_line":57,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::align_items
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":265,"record_offset":66320,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16889,"source_line":91,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":265,"record_offset":66352,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16890,"source_line":67,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":265,"record_offset":66380,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16892,"source_line":322,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":265,"record_offset":86444,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":16903,"source_line":95,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":265,"record_offset":60924,"parent_record_offset":31512,"ancestor_record_offset":31512,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=86
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":32792,"parent_record_offset":32768,"ancestor_record_offset":31512,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=86
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":32816,"parent_record_offset":32792,"ancestor_record_offset":31512,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=59
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":32284,"parent_record_offset":32260,"ancestor_record_offset":31512,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=53
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":32736,"parent_record_offset":32696,"ancestor_record_offset":31512,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=86
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":32768,"parent_record_offset":32736,"ancestor_record_offset":31512,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=59
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":32260,"parent_record_offset":32204,"ancestor_record_offset":31512,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":265,"record_offset":32044,"parent_record_offset":32020,"ancestor_record_offset":31512,"inlinee":6411,"source_line":87,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=53
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":32696,"parent_record_offset":32660,"ancestor_record_offset":31512,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":60596,"parent_record_offset":60568,"ancestor_record_offset":31512,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":265,"record_offset":31896,"parent_record_offset":31872,"ancestor_record_offset":31512,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":33248,"parent_record_offset":33216,"ancestor_record_offset":31512,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":54620,"parent_record_offset":54592,"ancestor_record_offset":31512,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":33772,"parent_record_offset":33736,"ancestor_record_offset":31512,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":54704,"parent_record_offset":54680,"ancestor_record_offset":31512,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":32232,"parent_record_offset":32204,"ancestor_record_offset":31512,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":32204,"parent_record_offset":32180,"ancestor_record_offset":31512,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":54680,"parent_record_offset":54648,"ancestor_record_offset":31512,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":33296,"parent_record_offset":33272,"ancestor_record_offset":31512,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":33272,"parent_record_offset":33248,"ancestor_record_offset":31512,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":54648,"parent_record_offset":54620,"ancestor_record_offset":31512,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":33848,"parent_record_offset":33808,"ancestor_record_offset":31512,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":33808,"parent_record_offset":33772,"ancestor_record_offset":31512,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":32180,"parent_record_offset":32156,"ancestor_record_offset":31512,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":33892,"parent_record_offset":33848,"ancestor_record_offset":31512,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":33692,"parent_record_offset":33656,"ancestor_record_offset":31512,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":81364,"parent_record_offset":81340,"ancestor_record_offset":31512,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":33656,"parent_record_offset":33620,"ancestor_record_offset":31512,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":33584,"parent_record_offset":33548,"ancestor_record_offset":31512,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":33620,"parent_record_offset":33584,"ancestor_record_offset":31512,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":33548,"parent_record_offset":33512,"ancestor_record_offset":31512,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":265,"record_offset":33512,"parent_record_offset":33476,"ancestor_record_offset":31512,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":54308,"parent_record_offset":53636,"ancestor_record_offset":31512,"inlinee":6418,"source_line":1126,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":53716,"parent_record_offset":53636,"ancestor_record_offset":31512,"inlinee":9147,"source_line":489,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$109::spec_to_string
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":33440,"parent_record_offset":33404,"ancestor_record_offset":31512,"inlinee":8861,"source_line":2877,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":31996,"parent_record_offset":31960,"ancestor_record_offset":31512,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$42::spec_to_string
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":32020,"parent_record_offset":31996,"ancestor_record_offset":31512,"inlinee":6409,"source_line":2912,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":33476,"parent_record_offset":33440,"ancestor_record_offset":31512,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":54548,"parent_record_offset":54372,"ancestor_record_offset":31512,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":33216,"parent_record_offset":33172,"ancestor_record_offset":31512,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":61116,"parent_record_offset":61076,"ancestor_record_offset":31512,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":54340,"parent_record_offset":54308,"ancestor_record_offset":31512,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":81388,"parent_record_offset":81364,"ancestor_record_offset":31512,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":54888,"parent_record_offset":54548,"ancestor_record_offset":31512,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":54592,"parent_record_offset":54548,"ancestor_record_offset":31512,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":53748,"parent_record_offset":53716,"ancestor_record_offset":31512,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":33736,"parent_record_offset":33692,"ancestor_record_offset":31512,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":61044,"parent_record_offset":61016,"ancestor_record_offset":31512,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":61076,"parent_record_offset":61044,"ancestor_record_offset":31512,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":265,"record_offset":54372,"parent_record_offset":54340,"ancestor_record_offset":31512,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":60628,"parent_record_offset":60596,"ancestor_record_offset":31512,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":33916,"parent_record_offset":33892,"ancestor_record_offset":31512,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":33940,"parent_record_offset":33916,"ancestor_record_offset":31512,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":265,"record_offset":81984,"parent_record_offset":81948,"ancestor_record_offset":31512,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":265,"record_offset":34156,"parent_record_offset":34132,"ancestor_record_offset":31512,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":265,"record_offset":33988,"parent_record_offset":33964,"ancestor_record_offset":31512,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":265,"record_offset":85716,"parent_record_offset":85684,"ancestor_record_offset":31512,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":265,"record_offset":60652,"parent_record_offset":60628,"ancestor_record_offset":31512,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":265,"record_offset":33964,"parent_record_offset":33940,"ancestor_record_offset":31512,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":265,"record_offset":54268,"parent_record_offset":54240,"ancestor_record_offset":31512,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":265,"record_offset":31872,"parent_record_offset":31848,"ancestor_record_offset":31512,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":265,"record_offset":34132,"parent_record_offset":33692,"ancestor_record_offset":31512,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":265,"record_offset":54504,"parent_record_offset":54480,"ancestor_record_offset":31512,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=294
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":32108,"parent_record_offset":31960,"ancestor_record_offset":31512,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":54756,"parent_record_offset":54548,"ancestor_record_offset":31512,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":54480,"parent_record_offset":54456,"ancestor_record_offset":31512,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":265,"record_offset":54456,"parent_record_offset":54432,"ancestor_record_offset":31512,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":32844,"parent_record_offset":32816,"ancestor_record_offset":31512,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":33336,"parent_record_offset":33172,"ancestor_record_offset":31512,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":265,"record_offset":54408,"parent_record_offset":54372,"ancestor_record_offset":31512,"inlinee":6415,"source_line":137,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::make_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":265,"record_offset":54432,"parent_record_offset":54408,"ancestor_record_offset":31512,"inlinee":6416,"source_line":88,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::match_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":265,"record_offset":54160,"parent_record_offset":53636,"ancestor_record_offset":31512,"inlinee":16883,"source_line":2061,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::MatchIndicesInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":265,"record_offset":54240,"parent_record_offset":54216,"ancestor_record_offset":31512,"inlinee":16887,"source_line":1062,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$91::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":265,"record_offset":54216,"parent_record_offset":53636,"ancestor_record_offset":31512,"inlinee":16885,"source_line":514,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":265,"record_offset":54184,"parent_record_offset":54160,"ancestor_record_offset":31512,"inlinee":4997,"source_line":975,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":59616,"parent_record_offset":59572,"ancestor_record_offset":31512,"inlinee":16845,"source_line":283,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=53
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":265,"record_offset":32660,"parent_record_offset":32624,"ancestor_record_offset":31512,"inlinee":16850,"source_line":32,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Map::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":265,"record_offset":81460,"parent_record_offset":81424,"ancestor_record_offset":31512,"inlinee":16895,"source_line":270,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::length::Length::fluid
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/length.rs
//     evidence={"module":265,"record_offset":87184,"parent_record_offset":86776,"ancestor_record_offset":31512,"inlinee":16905,"source_line":51,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":32080,"parent_record_offset":31960,"ancestor_record_offset":31512,"inlinee":16847,"source_line":41,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":60404,"parent_record_offset":59616,"ancestor_record_offset":31512,"inlinee":16862,"source_line":113,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=19
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":32624,"parent_record_offset":32588,"ancestor_record_offset":31512,"inlinee":16849,"source_line":279,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":60496,"parent_record_offset":59572,"ancestor_record_offset":31512,"inlinee":16866,"source_line":127,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::width
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":60464,"parent_record_offset":59572,"ancestor_record_offset":31512,"inlinee":16865,"source_line":92,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":71356,"parent_record_offset":71320,"ancestor_record_offset":31512,"inlinee":16869,"source_line":307,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::checkbox::Checkbox::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/checkbox.rs
//     evidence={"module":265,"record_offset":81948,"parent_record_offset":81920,"ancestor_record_offset":31512,"inlinee":16898,"source_line":75,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::checkbox::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/checkbox.rs
//     evidence={"module":265,"record_offset":82740,"parent_record_offset":82704,"ancestor_record_offset":31512,"inlinee":16901,"source_line":372,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":85684,"parent_record_offset":85600,"ancestor_record_offset":31512,"inlinee":16857,"source_line":145,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":85652,"parent_record_offset":85628,"ancestor_record_offset":31512,"inlinee":16856,"source_line":50,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":85628,"parent_record_offset":85600,"ancestor_record_offset":31512,"inlinee":16855,"source_line":32,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":85600,"parent_record_offset":85572,"ancestor_record_offset":31512,"inlinee":16854,"source_line":37,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":66996,"parent_record_offset":66960,"ancestor_record_offset":31512,"inlinee":16860,"source_line":333,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":265,"record_offset":86776,"parent_record_offset":86736,"ancestor_record_offset":31512,"inlinee":16875,"source_line":50,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":265,"record_offset":87324,"parent_record_offset":87288,"ancestor_record_offset":31512,"inlinee":16881,"source_line":313,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::helpers::button
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":265,"record_offset":60436,"parent_record_offset":59572,"ancestor_record_offset":31512,"inlinee":16863,"source_line":116,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":265,"record_offset":85184,"parent_record_offset":85148,"ancestor_record_offset":31512,"inlinee":16892,"source_line":322,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":265,"record_offset":83392,"parent_record_offset":83364,"ancestor_record_offset":31512,"inlinee":16903,"source_line":95,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":265,"record_offset":60956,"parent_record_offset":60924,"ancestor_record_offset":31512,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":265,"record_offset":60988,"parent_record_offset":60956,"ancestor_record_offset":31512,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":265,"record_offset":61016,"parent_record_offset":60988,"ancestor_record_offset":31512,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00265.jsonl.gz"}
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
// 首条位置: {"module":265,"record_offset":18836,"rva":6427584,"va":5375136704}
// 编译实例 RVAs: 0x6213c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7710610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e892a77500","size":5,"soft_fail":false,"target_va":5382847888,"va":5375137273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7760422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8266a7600","size":5,"soft_fail":false,"target_va":5382899440,"va":5375139013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c04b0
//     owner=unresolved_namespace; count=7
//     target_aliases=122 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5646232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e868d8a9ff","size":5,"soft_fail":false,"target_va":5369496752,"va":5375142979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cfe90
//     owner=unresolved_namespace; count=4
//     target_aliases=40 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-339519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8c1d1faff","size":5,"soft_fail":false,"target_va":5374803600,"va":5375143114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-55857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8cf25ffff","size":5,"soft_fail":false,"target_va":5375081488,"va":5375137340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-55819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8f525ffff","size":5,"soft_fail":false,"target_va":5375081504,"va":5375137318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db97c0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7957304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8386b7900","size":5,"soft_fail":false,"target_va":5383100352,"va":5375143043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db9920
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7957787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e81b6d7900","size":5,"soft_fail":false,"target_va":5383100704,"va":5375142912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dc0e20
//     owner=unresolved_namespace; count=1
//     target_aliases=21 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7988529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e831e57900","size":5,"soft_fail":false,"target_va":5383130656,"va":5375142122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dc10c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7993287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8c7f77900","size":5,"soft_fail":false,"target_va":5383131328,"va":5375138036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dc1830
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7993778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8b2f97900","size":5,"soft_fail":false,"target_va":5383133232,"va":5375139449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36708784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8b0213002","size":5,"soft_fail":false,"target_va":5411847168,"va":5375138379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t36398639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e82f662b02","size":5,"soft_fail":false,"target_va":5411540992,"va":5375142348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7702962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8b2897500","size":5,"soft_fail":false,"target_va":5382840208,"va":5375137241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-782519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8490ff4ff","size":5,"soft_fail":false,"target_va":5374354992,"va":5375137506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36398567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8e7652b02","size":5,"soft_fail":false,"target_va":5411540963,"va":5375142391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36399614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8fe692b02","size":5,"soft_fail":false,"target_va":5411542064,"va":5375142445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$crash_reporter_gui..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t7955965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8fd657900","size":5,"soft_fail":false,"target_va":5383099568,"va":5375143598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36400172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e82c6c2b02","size":5,"soft_fail":false,"target_va":5411542880,"va":5375142703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_reporter_gui::Controls::main_view
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8501f0000","size":5,"soft_fail":false,"target_va":5375144816,"va":5375136795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t7999203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8e30e7a00","size":5,"soft_fail":false,"target_va":5383139536,"va":5375140328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t8002642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8521c7a00","size":5,"soft_fail":false,"target_va":5383144368,"va":5375141721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5750707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8b3bf5700","size":5,"soft_fail":false,"target_va":5380887680,"va":5375136968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7611585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8c1247400","size":5,"soft_fail":false,"target_va":5382748432,"va":5375136842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7610794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375136704,"raw_hex":"e8aa217400","size":5,"soft_fail":false,"target_va":5382747616,"va":5375136817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375136704,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375139655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":265,"record_offset":19136,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":19108,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":20416,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":265,"record_offset":19852,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":265,"record_offset":19220,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":265,"record_offset":19024,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":265,"record_offset":19076,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":11237,"source_line":853,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":19248,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16845,"source_line":283,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::wide_button
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":25316,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16861,"source_line":493,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":23756,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16849,"source_line":279,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":26600,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16867,"source_line":112,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":27080,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16869,"source_line":307,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::align_items
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":21676,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16858,"source_line":100,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":27352,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16870,"source_line":70,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_x
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":265,"record_offset":31116,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16878,"source_line":121,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_y
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":265,"record_offset":31144,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16879,"source_line":127,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":265,"record_offset":21508,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16852,"source_line":68,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":265,"record_offset":30704,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16873,"source_line":57,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::scrollable::Scrollable::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/scrollable.rs
//     evidence={"module":265,"record_offset":30676,"parent_record_offset":18836,"ancestor_record_offset":18836,"inlinee":16872,"source_line":51,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":20052,"parent_record_offset":20028,"ancestor_record_offset":18836,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":20076,"parent_record_offset":20052,"ancestor_record_offset":18836,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":19568,"parent_record_offset":19544,"ancestor_record_offset":18836,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":19996,"parent_record_offset":19960,"ancestor_record_offset":18836,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":20028,"parent_record_offset":19996,"ancestor_record_offset":18836,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":19544,"parent_record_offset":19488,"ancestor_record_offset":18836,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":265,"record_offset":19328,"parent_record_offset":19304,"ancestor_record_offset":18836,"inlinee":6411,"source_line":87,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":19960,"parent_record_offset":19924,"ancestor_record_offset":18836,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":26292,"parent_record_offset":26264,"ancestor_record_offset":18836,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":265,"record_offset":19184,"parent_record_offset":19160,"ancestor_record_offset":18836,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":20492,"parent_record_offset":20460,"ancestor_record_offset":18836,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":21052,"parent_record_offset":21016,"ancestor_record_offset":18836,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":19516,"parent_record_offset":19488,"ancestor_record_offset":18836,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":19488,"parent_record_offset":19464,"ancestor_record_offset":18836,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":20540,"parent_record_offset":20516,"ancestor_record_offset":18836,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":20516,"parent_record_offset":20492,"ancestor_record_offset":18836,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":21128,"parent_record_offset":21088,"ancestor_record_offset":18836,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":21088,"parent_record_offset":21052,"ancestor_record_offset":18836,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":19464,"parent_record_offset":19440,"ancestor_record_offset":18836,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":21172,"parent_record_offset":21128,"ancestor_record_offset":18836,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":20972,"parent_record_offset":20932,"ancestor_record_offset":18836,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":20932,"parent_record_offset":20892,"ancestor_record_offset":18836,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":20852,"parent_record_offset":20812,"ancestor_record_offset":18836,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":20892,"parent_record_offset":20852,"ancestor_record_offset":18836,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":20812,"parent_record_offset":20772,"ancestor_record_offset":18836,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":265,"record_offset":20772,"parent_record_offset":20732,"ancestor_record_offset":18836,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":24404,"parent_record_offset":24352,"ancestor_record_offset":18836,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$109::spec_to_string
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":20692,"parent_record_offset":20652,"ancestor_record_offset":18836,"inlinee":8861,"source_line":2877,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$23::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":24484,"parent_record_offset":24460,"ancestor_record_offset":18836,"inlinee":5709,"source_line":2578,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":19280,"parent_record_offset":19248,"ancestor_record_offset":18836,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":24352,"parent_record_offset":24316,"ancestor_record_offset":18836,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$42::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":19304,"parent_record_offset":19280,"ancestor_record_offset":18836,"inlinee":6409,"source_line":2912,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":20732,"parent_record_offset":20692,"ancestor_record_offset":18836,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":20460,"parent_record_offset":20416,"ancestor_record_offset":18836,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":21016,"parent_record_offset":20972,"ancestor_record_offset":18836,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":26324,"parent_record_offset":26292,"ancestor_record_offset":18836,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":21196,"parent_record_offset":21172,"ancestor_record_offset":18836,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":21220,"parent_record_offset":21196,"ancestor_record_offset":18836,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":265,"record_offset":30792,"parent_record_offset":30740,"ancestor_record_offset":18836,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":265,"record_offset":24432,"parent_record_offset":24352,"ancestor_record_offset":18836,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$76::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":265,"record_offset":24460,"parent_record_offset":24352,"ancestor_record_offset":18836,"inlinee":6437,"source_line":2652,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":265,"record_offset":21436,"parent_record_offset":21412,"ancestor_record_offset":18836,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":265,"record_offset":21268,"parent_record_offset":21244,"ancestor_record_offset":18836,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":265,"record_offset":21636,"parent_record_offset":21612,"ancestor_record_offset":18836,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":265,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":18836,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":265,"record_offset":21244,"parent_record_offset":21220,"ancestor_record_offset":18836,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":265,"record_offset":19048,"parent_record_offset":19024,"ancestor_record_offset":18836,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":265,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":18836,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":265,"record_offset":21412,"parent_record_offset":20972,"ancestor_record_offset":18836,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":19392,"parent_record_offset":19248,"ancestor_record_offset":18836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":20104,"parent_record_offset":20076,"ancestor_record_offset":18836,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":20580,"parent_record_offset":20416,"ancestor_record_offset":18836,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":265,"record_offset":24516,"parent_record_offset":24352,"ancestor_record_offset":18836,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":25356,"parent_record_offset":25316,"ancestor_record_offset":18836,"inlinee":16845,"source_line":283,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":265,"record_offset":19924,"parent_record_offset":19888,"ancestor_record_offset":18836,"inlinee":16850,"source_line":32,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":19364,"parent_record_offset":19248,"ancestor_record_offset":18836,"inlinee":16847,"source_line":41,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":26104,"parent_record_offset":25356,"ancestor_record_offset":18836,"inlinee":16862,"source_line":113,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":19888,"parent_record_offset":19852,"ancestor_record_offset":18836,"inlinee":16849,"source_line":279,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":26196,"parent_record_offset":25316,"ancestor_record_offset":18836,"inlinee":16866,"source_line":127,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::width
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":26164,"parent_record_offset":25316,"ancestor_record_offset":18836,"inlinee":16865,"source_line":92,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":29800,"parent_record_offset":29764,"ancestor_record_offset":18836,"inlinee":16869,"source_line":307,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":21612,"parent_record_offset":21532,"ancestor_record_offset":18836,"inlinee":16857,"source_line":145,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":21580,"parent_record_offset":21556,"ancestor_record_offset":18836,"inlinee":16856,"source_line":50,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":21556,"parent_record_offset":21532,"ancestor_record_offset":18836,"inlinee":16855,"source_line":32,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":21532,"parent_record_offset":21508,"ancestor_record_offset":18836,"inlinee":16854,"source_line":37,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":265,"record_offset":21744,"parent_record_offset":21708,"ancestor_record_offset":18836,"inlinee":16860,"source_line":333,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":265,"record_offset":30740,"parent_record_offset":30704,"ancestor_record_offset":18836,"inlinee":16875,"source_line":50,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":265,"record_offset":31204,"parent_record_offset":31172,"ancestor_record_offset":18836,"inlinee":16881,"source_line":313,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::helpers::button
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":265,"record_offset":26136,"parent_record_offset":25316,"ancestor_record_offset":18836,"inlinee":16863,"source_line":116,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::scrollable::impl$6::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/scrollable.rs
//     evidence={"module":265,"record_offset":30828,"parent_record_offset":30792,"ancestor_record_offset":18836,"inlinee":16877,"source_line":420,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::str::pattern::TwoWaySearcher::next<enum2$<core::str::pattern::MatchOnly> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":265,"record_offset":7652,"rva":6419008,"va":5375128128}
// 编译实例 RVAs: 0x61f240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36413698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375128128,"raw_hex":"e802a12b02","size":5,"soft_fail":false,"target_va":5411542296,"va":5375128593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":265,"record_offset":8040,"parent_record_offset":7652,"ancestor_record_offset":7652,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":265,"record_offset":7892,"parent_record_offset":7652,"ancestor_record_offset":7652,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":265,"record_offset":7828,"parent_record_offset":7652,"ancestor_record_offset":7652,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::str::pattern::impl$36::matching
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":265,"record_offset":8212,"parent_record_offset":7652,"ancestor_record_offset":7652,"inlinee":5012,"source_line":1736,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":265,"record_offset":7948,"parent_record_offset":7920,"ancestor_record_offset":7652,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":265,"record_offset":8144,"parent_record_offset":8088,"ancestor_record_offset":7652,"inlinee":12495,"source_line":209,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":265,"record_offset":7976,"parent_record_offset":7920,"ancestor_record_offset":7652,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":265,"record_offset":7920,"parent_record_offset":7892,"ancestor_record_offset":7652,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":265,"record_offset":8088,"parent_record_offset":8064,"ancestor_record_offset":7652,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":265,"record_offset":8064,"parent_record_offset":8040,"ancestor_record_offset":7652,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":265,"record_offset":8000,"parent_record_offset":7976,"ancestor_record_offset":7652,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":265,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":7652,"inlinee":12496,"source_line":750,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":265,"record_offset":7856,"parent_record_offset":7828,"ancestor_record_offset":7652,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::run_crash_reporter_gui<tiny_glade::mock_crash_reporter::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":137,"record_offset":50176,"rva":3372848,"va":5372081968}
// 编译实例 RVAs: 0x337730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t3537771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e86bfb3500","size":5,"soft_fail":false,"target_va":5375621152,"va":5372083376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3538264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e858fd3500","size":5,"soft_fail":false,"target_va":5375621104,"va":5372082835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3537994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e84afc3500","size":5,"soft_fail":false,"target_va":5375620560,"va":5372082561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e7c70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2426029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e853fbdaff","size":5,"soft_fail":false,"target_va":5369658480,"va":5372084504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dc880
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2770139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8db442a00","size":5,"soft_fail":false,"target_va":5374855296,"va":5372085152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=3
//     target_aliases=92 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2815170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8c2f42a00","size":5,"soft_fail":false,"target_va":5374900736,"va":5372085561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2814866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e892f32a00","size":5,"soft_fail":false,"target_va":5374901408,"va":5372086537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405eb1b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2828360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e848282b00","size":5,"soft_fail":false,"target_va":5374914992,"va":5372086627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ebc10
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2831803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8bb352b00","size":5,"soft_fail":false,"target_va":5374917648,"va":5372085840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405eeac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2843947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e82b652b00","size":5,"soft_fail":false,"target_va":5374929600,"va":5372085648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ef060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2845553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8716b2b00","size":5,"soft_fail":false,"target_va":5374931040,"va":5372085482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f5d60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2873552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8d0d82b00","size":5,"soft_fail":false,"target_va":5374958944,"va":5372085387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f97d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2889341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e87d162c00","size":5,"soft_fail":false,"target_va":5374973904,"va":5372084558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2999115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e84bc32d00","size":5,"soft_fail":false,"target_va":5375081488,"va":5372082368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2999215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8afc32d00","size":5,"soft_fail":false,"target_va":5375081504,"va":5372082284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39764577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e861c25e02","size":5,"soft_fail":false,"target_va":5411847168,"va":5372082586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39455893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8950c5a02","size":5,"soft_fail":false,"target_va":5411540992,"va":5372085094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39456044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e82c0d5a02","size":5,"soft_fail":false,"target_va":5411540963,"va":5372084914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10570480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8f04aa100","size":5,"soft_fail":false,"target_va":5382655024,"va":5372084539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8dd210000","size":5,"soft_fail":false,"target_va":5372094144,"va":5372085470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39457995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8cb145a02","size":5,"soft_fail":false,"target_va":5411542880,"va":5372084880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_reporter_gui::Controls::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3044939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e84b762e00","size":5,"soft_fail":false,"target_va":5375129152,"va":5372084208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8805008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8905a8600","size":5,"soft_fail":false,"target_va":5380887680,"va":5372082667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39453871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8af045a02","size":5,"soft_fail":false,"target_va":5411538608,"va":5372084732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39454277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e845065a02","size":5,"soft_fail":false,"target_va":5411538960,"va":5372084678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2118689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e821542000","size":5,"soft_fail":false,"target_va":5374200800,"va":5372082106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2123760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8f0672000","size":5,"soft_fail":false,"target_va":5374207712,"va":5372083947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3538862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8aeff3500","size":5,"soft_fail":false,"target_va":5375621904,"va":5372083037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::primary_monitor
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3538862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8aeff3500","size":5,"soft_fail":false,"target_va":5375621872,"va":5372083005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::is_minimized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3530391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e897de3500","size":5,"soft_fail":false,"target_va":5375614240,"va":5372083844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3561613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e88d583600","size":5,"soft_fail":false,"target_va":5375645312,"va":5372083694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::force_window_active
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3568210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e852723600","size":5,"soft_fail":false,"target_va":5375652112,"va":5372083897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::WindowBuilder::build<tuple$<> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t11298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081968,"raw_hex":"e8222c0000","size":5,"soft_fail":false,"target_va":5372094928,"va":5372083625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5372081968,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5372082255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":137,"record_offset":56796,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":137,"record_offset":53208,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":137,"record_offset":53440,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5973,"source_line":1859,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":137,"record_offset":57068,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":55328,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":51096,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::impl$18::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":54740,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5979,"source_line":302,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":50428,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":10937,"source_line":200,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":53012,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5948,"source_line":329,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":55272,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5996,"source_line":305,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":137,"record_offset":53496,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5958,"source_line":125,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":137,"record_offset":53068,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5969,"source_line":118,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::focus_window
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":55968,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":12713,"source_line":1261,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::request_user_attention
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":56676,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":12720,"source_line":1289,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":55400,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":8430,"source_line":568,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":51196,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5917,"source_line":205,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":52760,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5956,"source_line":222,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":54940,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5980,"source_line":274,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":51800,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5922,"source_line":307,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":52984,"parent_record_offset":50176,"ancestor_record_offset":50176,"inlinee":5925,"source_line":340,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":51632,"parent_record_offset":51608,"ancestor_record_offset":50176,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":51656,"parent_record_offset":51632,"ancestor_record_offset":50176,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":50896,"parent_record_offset":50872,"ancestor_record_offset":50176,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":56872,"parent_record_offset":56832,"ancestor_record_offset":50176,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":51608,"parent_record_offset":51580,"ancestor_record_offset":50176,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":50872,"parent_record_offset":50788,"ancestor_record_offset":50176,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":137,"record_offset":56832,"parent_record_offset":56796,"ancestor_record_offset":50176,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":137,"record_offset":50788,"parent_record_offset":50760,"ancestor_record_offset":50176,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":51520,"parent_record_offset":51492,"ancestor_record_offset":50176,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":52652,"parent_record_offset":52624,"ancestor_record_offset":50176,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":52624,"parent_record_offset":52600,"ancestor_record_offset":50176,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":51580,"parent_record_offset":51548,"ancestor_record_offset":50176,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":51548,"parent_record_offset":51520,"ancestor_record_offset":50176,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":52600,"parent_record_offset":52576,"ancestor_record_offset":50176,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":52200,"parent_record_offset":52164,"ancestor_record_offset":50176,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":51456,"parent_record_offset":51428,"ancestor_record_offset":50176,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":51428,"parent_record_offset":51400,"ancestor_record_offset":50176,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":51372,"parent_record_offset":51344,"ancestor_record_offset":50176,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":51400,"parent_record_offset":51372,"ancestor_record_offset":50176,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":51344,"parent_record_offset":51316,"ancestor_record_offset":50176,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":137,"record_offset":51316,"parent_record_offset":51280,"ancestor_record_offset":50176,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":137,"record_offset":51872,"parent_record_offset":51844,"ancestor_record_offset":50176,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":137,"record_offset":51492,"parent_record_offset":51456,"ancestor_record_offset":50176,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":137,"record_offset":50816,"parent_record_offset":50788,"ancestor_record_offset":50176,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":137,"record_offset":52224,"parent_record_offset":52200,"ancestor_record_offset":50176,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":137,"record_offset":52248,"parent_record_offset":52224,"ancestor_record_offset":50176,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":137,"record_offset":51844,"parent_record_offset":51800,"ancestor_record_offset":50176,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$103::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":137,"record_offset":54584,"parent_record_offset":54560,"ancestor_record_offset":50176,"inlinee":5977,"source_line":77,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$108::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":137,"record_offset":53312,"parent_record_offset":53288,"ancestor_record_offset":50176,"inlinee":5955,"source_line":77,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":137,"record_offset":51732,"parent_record_offset":51708,"ancestor_record_offset":50176,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":137,"record_offset":52296,"parent_record_offset":52272,"ancestor_record_offset":50176,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":137,"record_offset":50840,"parent_record_offset":50816,"ancestor_record_offset":50176,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":137,"record_offset":52272,"parent_record_offset":52248,"ancestor_record_offset":50176,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":137,"record_offset":51708,"parent_record_offset":51456,"ancestor_record_offset":50176,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":137,"record_offset":50656,"parent_record_offset":50428,"ancestor_record_offset":50176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":137,"record_offset":53784,"parent_record_offset":53756,"ancestor_record_offset":50176,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":137,"record_offset":53832,"parent_record_offset":53808,"ancestor_record_offset":50176,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":137,"record_offset":53808,"parent_record_offset":53784,"ancestor_record_offset":50176,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":137,"record_offset":56988,"parent_record_offset":56960,"ancestor_record_offset":50176,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":53572,"parent_record_offset":53528,"ancestor_record_offset":50176,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":53156,"parent_record_offset":53108,"ancestor_record_offset":50176,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":53608,"parent_record_offset":53528,"ancestor_record_offset":50176,"inlinee":5961,"source_line":1439,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":50616,"parent_record_offset":50528,"ancestor_record_offset":50176,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":55792,"parent_record_offset":55604,"ancestor_record_offset":50176,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":50560,"parent_record_offset":50528,"ancestor_record_offset":50176,"inlinee":10942,"source_line":683,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":55884,"parent_record_offset":55852,"ancestor_record_offset":50176,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":55724,"parent_record_offset":55692,"ancestor_record_offset":50176,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":55748,"parent_record_offset":55724,"ancestor_record_offset":50176,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":55816,"parent_record_offset":55792,"ancestor_record_offset":50176,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":50584,"parent_record_offset":50560,"ancestor_record_offset":50176,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":137,"record_offset":53756,"parent_record_offset":53724,"ancestor_record_offset":50176,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":137,"record_offset":53724,"parent_record_offset":53696,"ancestor_record_offset":50176,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":137,"record_offset":55692,"parent_record_offset":55664,"ancestor_record_offset":50176,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":137,"record_offset":55664,"parent_record_offset":55636,"ancestor_record_offset":50176,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":137,"record_offset":55604,"parent_record_offset":55564,"ancestor_record_offset":50176,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":137,"record_offset":55564,"parent_record_offset":55532,"ancestor_record_offset":50176,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":137,"record_offset":55852,"parent_record_offset":55564,"ancestor_record_offset":50176,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":137,"record_offset":55636,"parent_record_offset":55604,"ancestor_record_offset":50176,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":55024,"parent_record_offset":55000,"ancestor_record_offset":50176,"inlinee":5984,"source_line":281,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalSize::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":52844,"parent_record_offset":52820,"ancestor_record_offset":50176,"inlinee":6845,"source_line":439,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=13
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":52880,"parent_record_offset":52844,"ancestor_record_offset":50176,"inlinee":5953,"source_line":109,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$19::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":54512,"parent_record_offset":54488,"ancestor_record_offset":50176,"inlinee":5975,"source_line":308,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":53236,"parent_record_offset":53208,"ancestor_record_offset":50176,"inlinee":5972,"source_line":466,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$33::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":52820,"parent_record_offset":52796,"ancestor_record_offset":50176,"inlinee":6843,"source_line":531,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":55000,"parent_record_offset":54976,"ancestor_record_offset":50176,"inlinee":5982,"source_line":573,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":50528,"parent_record_offset":50428,"ancestor_record_offset":50176,"inlinee":10941,"source_line":120,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":50472,"parent_record_offset":50428,"ancestor_record_offset":50176,"inlinee":10939,"source_line":69,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":50496,"parent_record_offset":50472,"ancestor_record_offset":50176,"inlinee":10940,"source_line":80,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoop::window_target
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":137,"record_offset":55296,"parent_record_offset":55272,"ancestor_record_offset":50176,"inlinee":5998,"source_line":210,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":137,"record_offset":53036,"parent_record_offset":53012,"ancestor_record_offset":50176,"inlinee":5949,"source_line":494,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":137,"record_offset":53528,"parent_record_offset":53496,"ancestor_record_offset":50176,"inlinee":5960,"source_line":211,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":137,"record_offset":53108,"parent_record_offset":53068,"ancestor_record_offset":50176,"inlinee":5970,"source_line":185,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::focus_window
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":56072,"parent_record_offset":56040,"ancestor_record_offset":50176,"inlinee":12716,"source_line":1149,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":56568,"parent_record_offset":56072,"ancestor_record_offset":50176,"inlinee":6012,"source_line":348,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":56004,"parent_record_offset":55968,"ancestor_record_offset":50176,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":55432,"parent_record_offset":55400,"ancestor_record_offset":50176,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_user_attention
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":56748,"parent_record_offset":56724,"ancestor_record_offset":50176,"inlinee":12723,"source_line":1080,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":55496,"parent_record_offset":55464,"ancestor_record_offset":50176,"inlinee":6848,"source_line":456,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::WindowFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":137,"record_offset":56596,"parent_record_offset":56072,"ancestor_record_offset":50176,"inlinee":12718,"source_line":703,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::WindowState::window_flags
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window_state.rs
//     evidence={"module":137,"record_offset":56120,"parent_record_offset":56072,"ancestor_record_offset":50176,"inlinee":12717,"source_line":180,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":137,"record_offset":56624,"parent_record_offset":56596,"ancestor_record_offset":50176,"inlinee":12719,"source_line":703,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":55464,"parent_record_offset":55432,"ancestor_record_offset":50176,"inlinee":8432,"source_line":569,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::focus_window::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":56040,"parent_record_offset":56004,"ancestor_record_offset":50176,"inlinee":12715,"source_line":1262,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::request_user_attention::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":56724,"parent_record_offset":56700,"ancestor_record_offset":50176,"inlinee":12722,"source_line":1291,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$30::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":51232,"parent_record_offset":51196,"ancestor_record_offset":50176,"inlinee":5919,"source_line":116,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":51280,"parent_record_offset":51232,"ancestor_record_offset":50176,"inlinee":5921,"source_line":162,"archive":"_pdb/B/modules/00137.jsonl.gz"}
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
// 首条位置: {"module":137,"record_offset":57496,"rva":3378096,"va":5372087216}
// 编译实例 RVAs: 0x338bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t3532521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8e9e63500","size":5,"soft_fail":false,"target_va":5375621152,"va":5372088626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3533022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8dee83500","size":5,"soft_fail":false,"target_va":5375621104,"va":5372088077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3532746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8cae73500","size":5,"soft_fail":false,"target_va":5375620560,"va":5372087809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e7c70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2431400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e858e6daff","size":5,"soft_fail":false,"target_va":5369658480,"va":5372089875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14033a860
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e86c0e0000","size":5,"soft_fail":false,"target_va":5372094560,"va":5372090863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dc880
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2764708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8a42f2a00","size":5,"soft_fail":false,"target_va":5374855296,"va":5372090583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=3
//     target_aliases=92 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2809729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e881df2a00","size":5,"soft_fail":false,"target_va":5374900736,"va":5372091002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2809314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8e2dd2a00","size":5,"soft_fail":false,"target_va":5374901408,"va":5372092089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405eb1b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2822811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e89b122b00","size":5,"soft_fail":false,"target_va":5374914992,"va":5372092176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ebc10
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2826324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e854202b00","size":5,"soft_fail":false,"target_va":5374917648,"va":5372091319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405eeac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2838484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8d44f2b00","size":5,"soft_fail":false,"target_va":5374929600,"va":5372091111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ef060
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2840103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e827562b00","size":5,"soft_fail":false,"target_va":5374931040,"va":5372090932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f5d60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2868115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e893c32b00","size":5,"soft_fail":false,"target_va":5374958944,"va":5372090824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f97d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2883956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e874012c00","size":5,"soft_fail":false,"target_va":5374973904,"va":5372089943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2993870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8ceae2d00","size":5,"soft_fail":false,"target_va":5375081488,"va":5372087613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2993979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e83baf2d00","size":5,"soft_fail":false,"target_va":5375081504,"va":5372087520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39759332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8e4ad5e02","size":5,"soft_fail":false,"target_va":5411847168,"va":5372087831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39450485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e875f75902","size":5,"soft_fail":false,"target_va":5411540992,"va":5372090502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39450624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e800f85902","size":5,"soft_fail":false,"target_va":5411540963,"va":5372090334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10565102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8ee35a100","size":5,"soft_fail":false,"target_va":5382655024,"va":5372089917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8930c0000","size":5,"soft_fail":false,"target_va":5372094144,"va":5372090920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39452582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8a6ff5902","size":5,"soft_fail":false,"target_va":5411542880,"va":5372090293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_reporter_gui::Controls::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3039565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e84d612e00","size":5,"soft_fail":false,"target_va":5375129152,"va":5372089582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8799748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e804468600","size":5,"soft_fail":false,"target_va":5380887680,"va":5372087927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39448459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e88bef5902","size":5,"soft_fail":false,"target_va":5411538608,"va":5372090144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39448879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e82ff15902","size":5,"soft_fail":false,"target_va":5411538960,"va":5372090076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2113447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8a73f2000","size":5,"soft_fail":false,"target_va":5374200800,"va":5372087348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2118473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e849532000","size":5,"soft_fail":false,"target_va":5374207712,"va":5372089234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3533606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e826eb3500","size":5,"soft_fail":false,"target_va":5375621904,"va":5372088293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::primary_monitor
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3533610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e82aeb3500","size":5,"soft_fail":false,"target_va":5375621872,"va":5372088257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::is_minimized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3525118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8fec93500","size":5,"soft_fail":false,"target_va":5375614240,"va":5372089117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3556354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e802443600","size":5,"soft_fail":false,"target_va":5375645312,"va":5372088953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::force_window_active
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3562930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e8b25d3600","size":5,"soft_fail":false,"target_va":5375652112,"va":5372089177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::WindowBuilder::build<tuple$<> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t6046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372087216,"raw_hex":"e89e170000","size":5,"soft_fail":false,"target_va":5372094928,"va":5372088877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5372087216,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5372087491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":137,"record_offset":64164,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":137,"record_offset":64356,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":137,"record_offset":60564,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":137,"record_offset":60808,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5973,"source_line":1859,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":137,"record_offset":64628,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":62696,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":58444,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::impl$18::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":62108,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5979,"source_line":302,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":57776,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":10937,"source_line":200,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":60360,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5948,"source_line":329,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":62640,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5996,"source_line":305,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":137,"record_offset":60864,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5958,"source_line":125,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":137,"record_offset":60416,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5969,"source_line":118,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::focus_window
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":63336,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":12713,"source_line":1261,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::request_user_attention
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":64044,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":12720,"source_line":1289,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":62768,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":8430,"source_line":568,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":58544,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5917,"source_line":205,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":60108,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5956,"source_line":222,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":62308,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5980,"source_line":274,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":59148,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5922,"source_line":307,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":60332,"parent_record_offset":57496,"ancestor_record_offset":57496,"inlinee":5925,"source_line":340,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":58980,"parent_record_offset":58956,"ancestor_record_offset":57496,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":59004,"parent_record_offset":58980,"ancestor_record_offset":57496,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":58244,"parent_record_offset":58220,"ancestor_record_offset":57496,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":64200,"parent_record_offset":64164,"ancestor_record_offset":57496,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":58956,"parent_record_offset":58928,"ancestor_record_offset":57496,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":137,"record_offset":58220,"parent_record_offset":58136,"ancestor_record_offset":57496,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":137,"record_offset":64392,"parent_record_offset":64356,"ancestor_record_offset":57496,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":137,"record_offset":58136,"parent_record_offset":58108,"ancestor_record_offset":57496,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":58868,"parent_record_offset":58840,"ancestor_record_offset":57496,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":60000,"parent_record_offset":59972,"ancestor_record_offset":57496,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":59972,"parent_record_offset":59948,"ancestor_record_offset":57496,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":58928,"parent_record_offset":58896,"ancestor_record_offset":57496,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":58896,"parent_record_offset":58868,"ancestor_record_offset":57496,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":59948,"parent_record_offset":59924,"ancestor_record_offset":57496,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":137,"record_offset":59548,"parent_record_offset":59512,"ancestor_record_offset":57496,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":58804,"parent_record_offset":58776,"ancestor_record_offset":57496,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":58776,"parent_record_offset":58748,"ancestor_record_offset":57496,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":58720,"parent_record_offset":58692,"ancestor_record_offset":57496,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":58748,"parent_record_offset":58720,"ancestor_record_offset":57496,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":137,"record_offset":58692,"parent_record_offset":58664,"ancestor_record_offset":57496,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":137,"record_offset":58664,"parent_record_offset":58628,"ancestor_record_offset":57496,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":137,"record_offset":59220,"parent_record_offset":59192,"ancestor_record_offset":57496,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":137,"record_offset":58840,"parent_record_offset":58804,"ancestor_record_offset":57496,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":137,"record_offset":58164,"parent_record_offset":58136,"ancestor_record_offset":57496,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":137,"record_offset":59572,"parent_record_offset":59548,"ancestor_record_offset":57496,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":137,"record_offset":59596,"parent_record_offset":59572,"ancestor_record_offset":57496,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":137,"record_offset":59192,"parent_record_offset":59148,"ancestor_record_offset":57496,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$103::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":137,"record_offset":61952,"parent_record_offset":61928,"ancestor_record_offset":57496,"inlinee":5977,"source_line":77,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$108::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":137,"record_offset":60680,"parent_record_offset":60656,"ancestor_record_offset":57496,"inlinee":5955,"source_line":77,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":137,"record_offset":59080,"parent_record_offset":59056,"ancestor_record_offset":57496,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":137,"record_offset":59644,"parent_record_offset":59620,"ancestor_record_offset":57496,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":137,"record_offset":58188,"parent_record_offset":58164,"ancestor_record_offset":57496,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":137,"record_offset":59620,"parent_record_offset":59596,"ancestor_record_offset":57496,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":137,"record_offset":59056,"parent_record_offset":58804,"ancestor_record_offset":57496,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":137,"record_offset":58004,"parent_record_offset":57776,"ancestor_record_offset":57496,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":137,"record_offset":61152,"parent_record_offset":61124,"ancestor_record_offset":57496,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":137,"record_offset":61200,"parent_record_offset":61176,"ancestor_record_offset":57496,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":137,"record_offset":61176,"parent_record_offset":61152,"ancestor_record_offset":57496,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":137,"record_offset":64308,"parent_record_offset":64280,"ancestor_record_offset":57496,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":60940,"parent_record_offset":60896,"ancestor_record_offset":57496,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":60508,"parent_record_offset":60460,"ancestor_record_offset":57496,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":60976,"parent_record_offset":60896,"ancestor_record_offset":57496,"inlinee":5961,"source_line":1439,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":137,"record_offset":57964,"parent_record_offset":57876,"ancestor_record_offset":57496,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":63160,"parent_record_offset":62972,"ancestor_record_offset":57496,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":57908,"parent_record_offset":57876,"ancestor_record_offset":57496,"inlinee":10942,"source_line":683,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":63252,"parent_record_offset":63220,"ancestor_record_offset":57496,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":63092,"parent_record_offset":63060,"ancestor_record_offset":57496,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":63116,"parent_record_offset":63092,"ancestor_record_offset":57496,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":63184,"parent_record_offset":63160,"ancestor_record_offset":57496,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":137,"record_offset":57932,"parent_record_offset":57908,"ancestor_record_offset":57496,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":137,"record_offset":61124,"parent_record_offset":61092,"ancestor_record_offset":57496,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":137,"record_offset":61092,"parent_record_offset":61064,"ancestor_record_offset":57496,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":137,"record_offset":63060,"parent_record_offset":63032,"ancestor_record_offset":57496,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":137,"record_offset":63032,"parent_record_offset":63004,"ancestor_record_offset":57496,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":137,"record_offset":62972,"parent_record_offset":62932,"ancestor_record_offset":57496,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":137,"record_offset":62932,"parent_record_offset":62900,"ancestor_record_offset":57496,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":137,"record_offset":63220,"parent_record_offset":62932,"ancestor_record_offset":57496,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":137,"record_offset":63004,"parent_record_offset":62972,"ancestor_record_offset":57496,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":62392,"parent_record_offset":62368,"ancestor_record_offset":57496,"inlinee":5984,"source_line":281,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalSize::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":60192,"parent_record_offset":60168,"ancestor_record_offset":57496,"inlinee":6845,"source_line":439,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=13
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":60228,"parent_record_offset":60192,"ancestor_record_offset":57496,"inlinee":5953,"source_line":109,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$19::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":61880,"parent_record_offset":61856,"ancestor_record_offset":57496,"inlinee":5975,"source_line":308,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":60596,"parent_record_offset":60564,"ancestor_record_offset":57496,"inlinee":5972,"source_line":466,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$33::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":60168,"parent_record_offset":60144,"ancestor_record_offset":57496,"inlinee":6843,"source_line":531,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":137,"record_offset":62368,"parent_record_offset":62344,"ancestor_record_offset":57496,"inlinee":5982,"source_line":573,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":57876,"parent_record_offset":57776,"ancestor_record_offset":57496,"inlinee":10941,"source_line":120,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":57820,"parent_record_offset":57776,"ancestor_record_offset":57496,"inlinee":10939,"source_line":69,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":137,"record_offset":57844,"parent_record_offset":57820,"ancestor_record_offset":57496,"inlinee":10940,"source_line":80,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoop::window_target
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":137,"record_offset":62664,"parent_record_offset":62640,"ancestor_record_offset":57496,"inlinee":5998,"source_line":210,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":137,"record_offset":60384,"parent_record_offset":60360,"ancestor_record_offset":57496,"inlinee":5949,"source_line":494,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":137,"record_offset":60896,"parent_record_offset":60864,"ancestor_record_offset":57496,"inlinee":5960,"source_line":211,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":137,"record_offset":60460,"parent_record_offset":60416,"ancestor_record_offset":57496,"inlinee":5970,"source_line":185,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::focus_window
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":63440,"parent_record_offset":63408,"ancestor_record_offset":57496,"inlinee":12716,"source_line":1149,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":63936,"parent_record_offset":63440,"ancestor_record_offset":57496,"inlinee":6012,"source_line":348,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":63372,"parent_record_offset":63336,"ancestor_record_offset":57496,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":62800,"parent_record_offset":62768,"ancestor_record_offset":57496,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_user_attention
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":64116,"parent_record_offset":64092,"ancestor_record_offset":57496,"inlinee":12723,"source_line":1080,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":137,"record_offset":62864,"parent_record_offset":62832,"ancestor_record_offset":57496,"inlinee":6848,"source_line":456,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::WindowFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":137,"record_offset":63964,"parent_record_offset":63440,"ancestor_record_offset":57496,"inlinee":12718,"source_line":703,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::WindowState::window_flags
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window_state.rs
//     evidence={"module":137,"record_offset":63488,"parent_record_offset":63440,"ancestor_record_offset":57496,"inlinee":12717,"source_line":180,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":137,"record_offset":63992,"parent_record_offset":63964,"ancestor_record_offset":57496,"inlinee":12719,"source_line":703,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":62832,"parent_record_offset":62800,"ancestor_record_offset":57496,"inlinee":8432,"source_line":569,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::focus_window::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":63408,"parent_record_offset":63372,"ancestor_record_offset":57496,"inlinee":12715,"source_line":1262,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::request_user_attention::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":64092,"parent_record_offset":64068,"ancestor_record_offset":57496,"inlinee":12722,"source_line":1291,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$30::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":58580,"parent_record_offset":58544,"ancestor_record_offset":57496,"inlinee":5919,"source_line":116,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":137,"record_offset":58628,"parent_record_offset":58580,"ancestor_record_offset":57496,"inlinee":5921,"source_line":162,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31BC
// 首条位置: {"module":211,"inlinee":12732}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":137,"record_offset":49056,"rva":3372432,"va":5372081552}
// 编译实例 RVAs: 0x337590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t39399195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081552,"raw_hex":"e81b2f5902","size":5,"soft_fail":false,"target_va":5411481152,"va":5372081952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39765255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372081552,"raw_hex":"e807c55e02","size":5,"soft_fail":false,"target_va":5411847152,"va":5372081892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":137,"record_offset":49232,"parent_record_offset":49056,"ancestor_record_offset":49056,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":137,"record_offset":49260,"parent_record_offset":49232,"ancestor_record_offset":49056,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":137,"record_offset":49288,"parent_record_offset":49260,"ancestor_record_offset":49056,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":137,"record_offset":49316,"parent_record_offset":49288,"ancestor_record_offset":49056,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00137.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x33F1
// 首条位置: {"module":155,"inlinee":13297}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":155,"record_offset":8272,"parent_record_offset":8248,"ancestor_record_offset":8248,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":8308,"parent_record_offset":8272,"ancestor_record_offset":8248,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":8372,"parent_record_offset":8272,"ancestor_record_offset":8248,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":8344,"parent_record_offset":8272,"ancestor_record_offset":8248,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31B8
// 首条位置: {"module":155,"inlinee":12728}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":74272,"parent_record_offset":74216,"ancestor_record_offset":74216,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":74244,"parent_record_offset":74216,"ancestor_record_offset":74216,"inlinee":5583,"source_line":1999,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$11::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD89C
// 首条位置: {"module":891,"inlinee":55452}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":891,"record_offset":11052,"parent_record_offset":10980,"ancestor_record_offset":10980,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":891,"record_offset":11076,"parent_record_offset":11052,"ancestor_record_offset":10980,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":891,"record_offset":11100,"parent_record_offset":11076,"ancestor_record_offset":10980,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00891.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":891,"record_offset":11124,"parent_record_offset":11100,"ancestor_record_offset":10980,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00891.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$0::program_outcome
// 记录形式: compiled_procedure; instances=1
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":265,"record_offset":9208,"rva":6419984,"va":5375129104}
// 编译实例 RVAs: 0x61f610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$0::tick_message
// 记录形式: compiled_procedure; instances=1
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":265,"record_offset":9328,"rva":6420000,"va":5375129120}
// 编译实例 RVAs: 0x61f620
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
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":265,"record_offset":9444,"rva":6420032,"va":5375129152}
// 编译实例 RVAs: 0x61f640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14002e900
//     owner=unresolved_namespace; count=1
//     target_aliases=130 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6232724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375129152,"raw_hex":"e86ce5a0ff","size":5,"soft_fail":false,"target_va":5368899840,"va":5375132559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ef060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-201730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375129152,"raw_hex":"e8feebfcff","size":5,"soft_fail":false,"target_va":5374931040,"va":5375132765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375129152,"raw_hex":"e88744ffff","size":5,"soft_fail":false,"target_va":5375081504,"va":5375129492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7698270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375129152,"raw_hex":"e85e777500","size":5,"soft_fail":false,"target_va":5382827728,"va":5375129453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_integrity_report..controls..Controls$GT$
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t7972380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375129152,"raw_hex":"e81ca67900","size":5,"soft_fail":false,"target_va":5383103120,"va":5375130735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tg_package_integrity::PackageVerificationError> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375129152,"raw_hex":"e89defffff","size":5,"soft_fail":false,"target_va":5375126640,"va":5375130830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::contains<ref$<str$> >
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t-2669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375129152,"raw_hex":"e893f5ffff","size":5,"soft_fail":false,"target_va":5375127024,"va":5375129688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7575999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375129152,"raw_hex":"e8bf997300","size":5,"soft_fail":false,"target_va":5382705472,"va":5375129468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::controls::Controls::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7976873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375129152,"raw_hex":"e8a9b77900","size":5,"soft_fail":false,"target_va":5383106208,"va":5375129330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375129152,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375129571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":265,"record_offset":12164,"parent_record_offset":9444,"ancestor_record_offset":9444,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":265,"record_offset":9584,"parent_record_offset":9444,"ancestor_record_offset":9444,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":265,"record_offset":11360,"parent_record_offset":9444,"ancestor_record_offset":9444,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":11884,"parent_record_offset":9444,"ancestor_record_offset":9444,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":265,"record_offset":11388,"parent_record_offset":9444,"ancestor_record_offset":9444,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":265,"record_offset":11472,"parent_record_offset":9444,"ancestor_record_offset":9444,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":10096,"parent_record_offset":10072,"ancestor_record_offset":9444,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":10072,"parent_record_offset":10016,"ancestor_record_offset":9444,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":10856,"parent_record_offset":10828,"ancestor_record_offset":9444,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":10044,"parent_record_offset":10016,"ancestor_record_offset":9444,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":10016,"parent_record_offset":9992,"ancestor_record_offset":9444,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":9992,"parent_record_offset":9968,"ancestor_record_offset":9444,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":10888,"parent_record_offset":10856,"ancestor_record_offset":9444,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":265,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":9444,"inlinee":15001,"source_line":78,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::try_fold::check::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":265,"record_offset":12396,"parent_record_offset":12212,"ancestor_record_offset":9444,"inlinee":8144,"source_line":87,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":265,"record_offset":12424,"parent_record_offset":12396,"ancestor_record_offset":9444,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":265,"record_offset":12212,"parent_record_offset":12188,"ancestor_record_offset":9444,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":265,"record_offset":10912,"parent_record_offset":10888,"ancestor_record_offset":9444,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":9848,"parent_record_offset":9648,"ancestor_record_offset":9444,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":11436,"parent_record_offset":11412,"ancestor_record_offset":9444,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":10656,"parent_record_offset":10628,"ancestor_record_offset":9444,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":10704,"parent_record_offset":10680,"ancestor_record_offset":9444,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":10680,"parent_record_offset":10656,"ancestor_record_offset":9444,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":265,"record_offset":11812,"parent_record_offset":11788,"ancestor_record_offset":9444,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":265,"record_offset":11552,"parent_record_offset":11512,"ancestor_record_offset":9444,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":265,"record_offset":10476,"parent_record_offset":9620,"ancestor_record_offset":9444,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":265,"record_offset":11412,"parent_record_offset":11388,"ancestor_record_offset":9444,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":265,"record_offset":11788,"parent_record_offset":11764,"ancestor_record_offset":9444,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":265,"record_offset":11512,"parent_record_offset":11472,"ancestor_record_offset":9444,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":265,"record_offset":11764,"parent_record_offset":11700,"ancestor_record_offset":9444,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":9620,"parent_record_offset":9584,"ancestor_record_offset":9444,"inlinee":16827,"source_line":196,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":11596,"parent_record_offset":11472,"ancestor_record_offset":9444,"inlinee":16828,"source_line":203,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":11852,"parent_record_offset":11668,"ancestor_record_offset":9444,"inlinee":16829,"source_line":220,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":12448,"parent_record_offset":12424,"ancestor_record_offset":9444,"inlinee":16830,"source_line":229,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":12780,"parent_record_offset":12756,"ancestor_record_offset":9444,"inlinee":16831,"source_line":236,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":13012,"parent_record_offset":12988,"ancestor_record_offset":9444,"inlinee":16832,"source_line":243,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":13244,"parent_record_offset":13060,"ancestor_record_offset":9444,"inlinee":16833,"source_line":254,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":265,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":9444,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":265,"record_offset":9800,"parent_record_offset":9776,"ancestor_record_offset":9444,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":265,"record_offset":9776,"parent_record_offset":9752,"ancestor_record_offset":9444,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":265,"record_offset":9648,"parent_record_offset":9620,"ancestor_record_offset":9444,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":265,"record_offset":10628,"parent_record_offset":10600,"ancestor_record_offset":9444,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":265,"record_offset":10600,"parent_record_offset":10572,"ancestor_record_offset":9444,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":265,"record_offset":9728,"parent_record_offset":9704,"ancestor_record_offset":9444,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":265,"record_offset":9704,"parent_record_offset":9680,"ancestor_record_offset":9444,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":265,"record_offset":9680,"parent_record_offset":9648,"ancestor_record_offset":9444,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$1::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41BB
// 首条位置: {"module":265,"inlinee":16827}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":265,"record_offset":10476,"parent_record_offset":9620,"ancestor_record_offset":9620,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":265,"record_offset":9648,"parent_record_offset":9620,"ancestor_record_offset":9620,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":10096,"parent_record_offset":10072,"ancestor_record_offset":9620,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":10072,"parent_record_offset":10016,"ancestor_record_offset":9620,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":10856,"parent_record_offset":10828,"ancestor_record_offset":9620,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":10044,"parent_record_offset":10016,"ancestor_record_offset":9620,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":10016,"parent_record_offset":9992,"ancestor_record_offset":9620,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":9992,"parent_record_offset":9968,"ancestor_record_offset":9620,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":10888,"parent_record_offset":10856,"ancestor_record_offset":9620,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":265,"record_offset":10912,"parent_record_offset":10888,"ancestor_record_offset":9620,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":9848,"parent_record_offset":9648,"ancestor_record_offset":9620,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":10656,"parent_record_offset":10628,"ancestor_record_offset":9620,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":10704,"parent_record_offset":10680,"ancestor_record_offset":9620,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":10680,"parent_record_offset":10656,"ancestor_record_offset":9620,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":265,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":9620,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":265,"record_offset":9800,"parent_record_offset":9776,"ancestor_record_offset":9620,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":265,"record_offset":9776,"parent_record_offset":9752,"ancestor_record_offset":9620,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":265,"record_offset":10628,"parent_record_offset":10600,"ancestor_record_offset":9620,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":265,"record_offset":10600,"parent_record_offset":10572,"ancestor_record_offset":9620,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":265,"record_offset":9728,"parent_record_offset":9704,"ancestor_record_offset":9620,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":265,"record_offset":9704,"parent_record_offset":9680,"ancestor_record_offset":9620,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":265,"record_offset":9680,"parent_record_offset":9648,"ancestor_record_offset":9620,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$1::new::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41BC
// 首条位置: {"module":265,"inlinee":16828}
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
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41BD
// 首条位置: {"module":265,"inlinee":16829}
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
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41BE
// 首条位置: {"module":265,"inlinee":16830}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$1::new::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 236
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41BF
// 首条位置: {"module":265,"inlinee":16831}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$1::new::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41C0
// 首条位置: {"module":265,"inlinee":16832}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$1::new::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 254
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41C1
// 首条位置: {"module":265,"inlinee":16833}
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
// 记录形式: inline_instance, inlinee_declaration; instances=43
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41CD
// 首条位置: {"module":265,"inlinee":16845}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":19280,"parent_record_offset":19248,"ancestor_record_offset":19248,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":19392,"parent_record_offset":19248,"ancestor_record_offset":19248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=7
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":19364,"parent_record_offset":19248,"ancestor_record_offset":19248,"inlinee":16847,"source_line":41,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=14
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":26104,"parent_record_offset":25356,"ancestor_record_offset":25356,"inlinee":16862,"source_line":113,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":21336,"parent_record_offset":21312,"ancestor_record_offset":20612,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":21360,"parent_record_offset":21336,"ancestor_record_offset":20612,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":19568,"parent_record_offset":19544,"ancestor_record_offset":19248,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":21312,"parent_record_offset":21128,"ancestor_record_offset":20612,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":19544,"parent_record_offset":19488,"ancestor_record_offset":19248,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":265,"record_offset":19328,"parent_record_offset":19304,"ancestor_record_offset":19248,"inlinee":6411,"source_line":87,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":21052,"parent_record_offset":21016,"ancestor_record_offset":20612,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":19516,"parent_record_offset":19488,"ancestor_record_offset":19248,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":19488,"parent_record_offset":19464,"ancestor_record_offset":19248,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":21128,"parent_record_offset":21088,"ancestor_record_offset":20612,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":21088,"parent_record_offset":21052,"ancestor_record_offset":20612,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":19464,"parent_record_offset":19440,"ancestor_record_offset":19248,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":21172,"parent_record_offset":21128,"ancestor_record_offset":20612,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":20972,"parent_record_offset":20932,"ancestor_record_offset":20612,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":20932,"parent_record_offset":20892,"ancestor_record_offset":20612,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":20852,"parent_record_offset":20812,"ancestor_record_offset":20612,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":20892,"parent_record_offset":20852,"ancestor_record_offset":20612,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":20812,"parent_record_offset":20772,"ancestor_record_offset":20612,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":265,"record_offset":20772,"parent_record_offset":20732,"ancestor_record_offset":20612,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":24404,"parent_record_offset":24352,"ancestor_record_offset":24280,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$109::spec_to_string
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":20692,"parent_record_offset":20652,"ancestor_record_offset":20612,"inlinee":8861,"source_line":2877,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$23::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":24484,"parent_record_offset":24460,"ancestor_record_offset":24280,"inlinee":5709,"source_line":2578,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":24352,"parent_record_offset":24316,"ancestor_record_offset":24280,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$42::spec_to_string
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":19304,"parent_record_offset":19280,"ancestor_record_offset":19248,"inlinee":6409,"source_line":2912,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":20732,"parent_record_offset":20692,"ancestor_record_offset":20612,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":21016,"parent_record_offset":20972,"ancestor_record_offset":20612,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":21196,"parent_record_offset":21172,"ancestor_record_offset":20612,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":21220,"parent_record_offset":21196,"ancestor_record_offset":20612,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":265,"record_offset":24432,"parent_record_offset":24352,"ancestor_record_offset":24280,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$76::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":265,"record_offset":24460,"parent_record_offset":24352,"ancestor_record_offset":24280,"inlinee":6437,"source_line":2652,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":265,"record_offset":21436,"parent_record_offset":21412,"ancestor_record_offset":20612,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":265,"record_offset":21268,"parent_record_offset":21244,"ancestor_record_offset":20612,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":265,"record_offset":21244,"parent_record_offset":21220,"ancestor_record_offset":20612,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":265,"record_offset":21412,"parent_record_offset":20972,"ancestor_record_offset":20612,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":19416,"parent_record_offset":19392,"ancestor_record_offset":19248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":265,"record_offset":24516,"parent_record_offset":24352,"ancestor_record_offset":24280,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$2::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 311
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":265,"record_offset":13904,"rva":6423664,"va":5375132784}
// 编译实例 RVAs: 0x620470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t7510332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e83c997200","size":5,"soft_fail":false,"target_va":5382644528,"va":5375134191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c04b0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5639540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e88cf2a9ff","size":5,"soft_fail":false,"target_va":5369496752,"va":5375136287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-235380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e88c68fcff","size":5,"soft_fail":false,"target_va":5374900736,"va":5375136111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-51954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e80e35ffff","size":5,"soft_fail":false,"target_va":5375081504,"va":5375133453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t381985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e821d40500","size":5,"soft_fail":false,"target_va":5375516032,"va":5375134042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36413233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e8319f2b02","size":5,"soft_fail":false,"target_va":5411548112,"va":5375134874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7519791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e82fbe7200","size":5,"soft_fail":false,"target_va":5382655024,"va":5375135228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arboard::Clipboard::set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arboard","version":"3.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t58056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e8c8e20000","size":5,"soft_fail":false,"target_va":5375191904,"va":5375133843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arboard::platform::windows::Set::text
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arboard","version":"3.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7993460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e874f87900","size":5,"soft_fail":false,"target_va":5383127392,"va":5375133927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36406733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e8cd852b02","size":5,"soft_fail":false,"target_va":5411542064,"va":5375135326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_integrity_report..controls..Controls$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7970120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e8489d7900","size":5,"soft_fail":false,"target_va":5383103120,"va":5375132995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<crash_reporter_gui::CurrentState> >
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-8639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e841deffff","size":5,"soft_fail":false,"target_va":5375126144,"va":5375134778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36407462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e8a6882b02","size":5,"soft_fail":false,"target_va":5411542880,"va":5375135413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] open::that
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"open","version":"5.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7990887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e867ee7900","size":5,"soft_fail":false,"target_va":5383124608,"va":5375133716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::JoinHandle$LT$T$GT$::join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7993723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e87bf97900","size":5,"soft_fail":false,"target_va":5383128432,"va":5375134704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7613641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e8c92c7400","size":5,"soft_fail":false,"target_va":5382747616,"va":5375133970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::controls::Controls::remove_unrecognized_package_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7973114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e8faa87900","size":5,"soft_fail":false,"target_va":5383106256,"va":5375133137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] webbrowser::open
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"webbrowser","version":"0.8.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5631182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375132784,"raw_hex":"e8ceec5500","size":5,"soft_fail":false,"target_va":5380764368,"va":5375133181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375132784,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375133254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$28::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":15152,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":7605,"source_line":1975,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":265,"record_offset":16672,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":265,"record_offset":15036,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":14124,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":265,"record_offset":16524,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":265,"record_offset":18004,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::copy_to_clipboard
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":16128,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":16837,"source_line":472,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::none
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":265,"record_offset":18436,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":16844,"source_line":30,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::window::close
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/window.rs
//     evidence={"module":265,"record_offset":14068,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":16835,"source_line":50,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":265,"record_offset":16704,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":265,"record_offset":16588,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] std::thread::JoinHandle::is_finished
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":265,"record_offset":17740,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":11423,"source_line":1901,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":14384,"parent_record_offset":14352,"ancestor_record_offset":13904,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":14352,"parent_record_offset":14292,"ancestor_record_offset":13904,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":14776,"parent_record_offset":14748,"ancestor_record_offset":13904,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":14324,"parent_record_offset":14292,"ancestor_record_offset":13904,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":14292,"parent_record_offset":14264,"ancestor_record_offset":13904,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":14264,"parent_record_offset":14236,"ancestor_record_offset":13904,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$54::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":16224,"parent_record_offset":16200,"ancestor_record_offset":13904,"inlinee":16841,"source_line":3088,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":265,"record_offset":17788,"parent_record_offset":17764,"ancestor_record_offset":13904,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::strong_count
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":265,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":13904,"inlinee":11385,"source_line":1799,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] arboard::Set::text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arboard","version":"3.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arboard-3.6.0/src/lib.rs
//     evidence={"module":265,"record_offset":16172,"parent_record_offset":16128,"ancestor_record_offset":13904,"inlinee":16839,"source_line":216,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":14804,"parent_record_offset":14776,"ancestor_record_offset":13904,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":265,"record_offset":16200,"parent_record_offset":16172,"ancestor_record_offset":13904,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":265,"record_offset":15060,"parent_record_offset":15036,"ancestor_record_offset":13904,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":265,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":13904,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":265,"record_offset":16760,"parent_record_offset":16728,"ancestor_record_offset":13904,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":265,"record_offset":16788,"parent_record_offset":16728,"ancestor_record_offset":13904,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":14148,"parent_record_offset":14124,"ancestor_record_offset":13904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":14576,"parent_record_offset":14552,"ancestor_record_offset":13904,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":14624,"parent_record_offset":14600,"ancestor_record_offset":13904,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":265,"record_offset":14600,"parent_record_offset":14576,"ancestor_record_offset":13904,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":265,"record_offset":17820,"parent_record_offset":17788,"ancestor_record_offset":13904,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":15088,"parent_record_offset":15060,"ancestor_record_offset":13904,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":15116,"parent_record_offset":15060,"ancestor_record_offset":13904,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":265,"record_offset":16260,"parent_record_offset":16128,"ancestor_record_offset":13904,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":265,"record_offset":16296,"parent_record_offset":16128,"ancestor_record_offset":13904,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":265,"record_offset":16612,"parent_record_offset":16588,"ancestor_record_offset":13904,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":265,"record_offset":16636,"parent_record_offset":16612,"ancestor_record_offset":13904,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$2::update::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":16556,"parent_record_offset":16524,"ancestor_record_offset":13904,"inlinee":16843,"source_line":338,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::single
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":265,"record_offset":14092,"parent_record_offset":14068,"ancestor_record_offset":13904,"inlinee":16836,"source_line":35,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":265,"record_offset":16816,"parent_record_offset":16728,"ancestor_record_offset":13904,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":265,"record_offset":16728,"parent_record_offset":16704,"ancestor_record_offset":13904,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":265,"record_offset":16840,"parent_record_offset":16816,"ancestor_record_offset":13904,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":265,"record_offset":14552,"parent_record_offset":14524,"ancestor_record_offset":13904,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":265,"record_offset":14524,"parent_record_offset":14496,"ancestor_record_offset":13904,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$2::update::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 338
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41CB
// 首条位置: {"module":265,"inlinee":16843}
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
// 源码行: 414
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD87E
// 首条位置: {"module":890,"inlinee":55422}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":890,"record_offset":3300,"parent_record_offset":3264,"ancestor_record_offset":3264,"inlinee":16845,"source_line":283,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":890,"record_offset":4124,"parent_record_offset":3264,"ancestor_record_offset":3264,"inlinee":16862,"source_line":113,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":890,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":3264,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":890,"record_offset":3976,"parent_record_offset":3952,"ancestor_record_offset":3264,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":890,"record_offset":3928,"parent_record_offset":3752,"ancestor_record_offset":3264,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":890,"record_offset":3692,"parent_record_offset":3664,"ancestor_record_offset":3264,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":890,"record_offset":3752,"parent_record_offset":3720,"ancestor_record_offset":3264,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":890,"record_offset":3720,"parent_record_offset":3692,"ancestor_record_offset":3264,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":890,"record_offset":3788,"parent_record_offset":3752,"ancestor_record_offset":3264,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":890,"record_offset":3624,"parent_record_offset":3588,"ancestor_record_offset":3264,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":890,"record_offset":3588,"parent_record_offset":3552,"ancestor_record_offset":3264,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":890,"record_offset":3516,"parent_record_offset":3480,"ancestor_record_offset":3264,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":890,"record_offset":3552,"parent_record_offset":3516,"ancestor_record_offset":3264,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":890,"record_offset":3480,"parent_record_offset":3444,"ancestor_record_offset":3264,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":890,"record_offset":3444,"parent_record_offset":3408,"ancestor_record_offset":3264,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$109::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":890,"record_offset":3372,"parent_record_offset":3336,"ancestor_record_offset":3264,"inlinee":8861,"source_line":2877,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":890,"record_offset":3336,"parent_record_offset":3300,"ancestor_record_offset":3264,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":890,"record_offset":3408,"parent_record_offset":3372,"ancestor_record_offset":3264,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":890,"record_offset":3664,"parent_record_offset":3624,"ancestor_record_offset":3264,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":890,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":3264,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":890,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":3264,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":890,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":3264,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":890,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":3264,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":890,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":3264,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":890,"record_offset":4028,"parent_record_offset":3624,"ancestor_record_offset":3264,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00890.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::copy_to_clipboard
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 472
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41C5
// 首条位置: {"module":265,"inlinee":16837}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arboard::Set::text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arboard","version":"3.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arboard-3.6.0/src/lib.rs
//     evidence={"module":265,"record_offset":16172,"parent_record_offset":16128,"ancestor_record_offset":16128,"inlinee":16839,"source_line":216,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":16324,"parent_record_offset":16128,"ancestor_record_offset":16128,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":265,"record_offset":16260,"parent_record_offset":16128,"ancestor_record_offset":16128,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":265,"record_offset":16296,"parent_record_offset":16128,"ancestor_record_offset":16128,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":16128,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":16444,"parent_record_offset":16420,"ancestor_record_offset":16128,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":16420,"parent_record_offset":16396,"ancestor_record_offset":16128,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":16396,"parent_record_offset":16372,"ancestor_record_offset":16128,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$54::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":16224,"parent_record_offset":16200,"ancestor_record_offset":16128,"inlinee":16841,"source_line":3088,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":265,"record_offset":16200,"parent_record_offset":16172,"ancestor_record_offset":16128,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":16348,"parent_record_offset":16324,"ancestor_record_offset":16128,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::wide_button
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 493
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x41DD
// 首条位置: {"module":265,"inlinee":16861}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":25356,"parent_record_offset":25316,"ancestor_record_offset":25316,"inlinee":16845,"source_line":283,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":26196,"parent_record_offset":25316,"ancestor_record_offset":25316,"inlinee":16866,"source_line":127,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::width
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":26164,"parent_record_offset":25316,"ancestor_record_offset":25316,"inlinee":16865,"source_line":92,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":265,"record_offset":26136,"parent_record_offset":25316,"ancestor_record_offset":25316,"inlinee":16863,"source_line":116,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":25936,"parent_record_offset":25912,"ancestor_record_offset":25316,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":25960,"parent_record_offset":25936,"ancestor_record_offset":25316,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":26404,"parent_record_offset":26380,"ancestor_record_offset":25316,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":25912,"parent_record_offset":25736,"ancestor_record_offset":25316,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":26380,"parent_record_offset":26292,"ancestor_record_offset":25316,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":26292,"parent_record_offset":26264,"ancestor_record_offset":25316,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":25676,"parent_record_offset":25648,"ancestor_record_offset":25316,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":25736,"parent_record_offset":25704,"ancestor_record_offset":25316,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":25704,"parent_record_offset":25676,"ancestor_record_offset":25316,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":25772,"parent_record_offset":25736,"ancestor_record_offset":25316,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":25612,"parent_record_offset":25584,"ancestor_record_offset":25316,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":25584,"parent_record_offset":25556,"ancestor_record_offset":25316,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":25528,"parent_record_offset":25500,"ancestor_record_offset":25316,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":25556,"parent_record_offset":25528,"ancestor_record_offset":25316,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":265,"record_offset":25500,"parent_record_offset":25472,"ancestor_record_offset":25316,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":265,"record_offset":25472,"parent_record_offset":25444,"ancestor_record_offset":25316,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$109::spec_to_string
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":25416,"parent_record_offset":25388,"ancestor_record_offset":25316,"inlinee":8861,"source_line":2877,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":25388,"parent_record_offset":25356,"ancestor_record_offset":25316,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":25444,"parent_record_offset":25416,"ancestor_record_offset":25316,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":265,"record_offset":25648,"parent_record_offset":25612,"ancestor_record_offset":25316,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":26324,"parent_record_offset":26292,"ancestor_record_offset":25316,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":25796,"parent_record_offset":25772,"ancestor_record_offset":25316,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":25820,"parent_record_offset":25796,"ancestor_record_offset":25316,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":265,"record_offset":26036,"parent_record_offset":26012,"ancestor_record_offset":25316,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":265,"record_offset":25868,"parent_record_offset":25844,"ancestor_record_offset":25316,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":265,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":25316,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":265,"record_offset":25844,"parent_record_offset":25820,"ancestor_record_offset":25316,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":265,"record_offset":26012,"parent_record_offset":25612,"ancestor_record_offset":25316,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":26236,"parent_record_offset":26196,"ancestor_record_offset":25316,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":26104,"parent_record_offset":25356,"ancestor_record_offset":25316,"inlinee":16862,"source_line":113,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$3::main_view::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 538
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD849
// 首条位置: {"module":889,"inlinee":55369}
// 编译实例 RVAs: 0xdc1060
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
// 源码行: 572
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":896,"record_offset":5056,"rva":14452688,"va":5383161808}
// 编译实例 RVAs: 0xdc87d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":896,"record_offset":5248,"parent_record_offset":5056,"ancestor_record_offset":5056,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00896.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$3::main_view::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":896,"record_offset":5268,"parent_record_offset":5248,"ancestor_record_offset":5056,"inlinee":55571,"source_line":572,"archive":"_pdb/B/modules/00896.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$3::main_view::closure$1
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 572
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD913
// 首条位置: {"module":896,"inlinee":55571}
// 编译实例 RVAs: 0xdc87d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::Controls::more_info_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 819
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":265,"record_offset":105788,"rva":6480000,"va":5375189120}
// 编译实例 RVAs: 0x62e080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7658617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375189120,"raw_hex":"e879dc7400","size":5,"soft_fail":false,"target_va":5382847888,"va":5375189266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-108373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375189120,"raw_hex":"e8ab58feff","size":5,"soft_fail":false,"target_va":5375081488,"va":5375189856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-107912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375189120,"raw_hex":"e8785afeff","size":5,"soft_fail":false,"target_va":5375081504,"va":5375189411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dc1830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7943803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375189120,"raw_hex":"e87b367900","size":5,"soft_fail":false,"target_va":5383133232,"va":5375189424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36351037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375189120,"raw_hex":"e83dac2a02","size":5,"soft_fail":false,"target_va":5411540992,"va":5375189950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7650949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375189120,"raw_hex":"e885be7400","size":5,"soft_fail":false,"target_va":5382840208,"va":5375189254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$crash_reporter_gui..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7909441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375189120,"raw_hex":"e841b07800","size":5,"soft_fail":false,"target_va":5383099568,"va":5375190122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5698507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375189120,"raw_hex":"e8cbf35600","size":5,"soft_fail":false,"target_va":5380887680,"va":5375189168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375189120,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375189682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":265,"record_offset":105944,"parent_record_offset":105788,"ancestor_record_offset":105788,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":265,"record_offset":107628,"parent_record_offset":105788,"ancestor_record_offset":105788,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":265,"record_offset":106024,"parent_record_offset":105788,"ancestor_record_offset":105788,"inlinee":16845,"source_line":283,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":106692,"parent_record_offset":105788,"ancestor_record_offset":105788,"inlinee":16867,"source_line":112,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":107600,"parent_record_offset":105788,"ancestor_record_offset":105788,"inlinee":16907,"source_line":104,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":107200,"parent_record_offset":105788,"ancestor_record_offset":105788,"inlinee":16866,"source_line":127,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":265,"record_offset":106664,"parent_record_offset":105788,"ancestor_record_offset":105788,"inlinee":16863,"source_line":116,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":107820,"parent_record_offset":107796,"ancestor_record_offset":105788,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":107844,"parent_record_offset":107820,"ancestor_record_offset":105788,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":106380,"parent_record_offset":106356,"ancestor_record_offset":105788,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":107764,"parent_record_offset":107728,"ancestor_record_offset":105788,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":107796,"parent_record_offset":107764,"ancestor_record_offset":105788,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":265,"record_offset":106356,"parent_record_offset":106300,"ancestor_record_offset":105788,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$1::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":265,"record_offset":106112,"parent_record_offset":106088,"ancestor_record_offset":105788,"inlinee":6411,"source_line":87,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":107728,"parent_record_offset":107696,"ancestor_record_offset":105788,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":265,"record_offset":107296,"parent_record_offset":107268,"ancestor_record_offset":105788,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":265,"record_offset":105992,"parent_record_offset":105968,"ancestor_record_offset":105788,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":106328,"parent_record_offset":106300,"ancestor_record_offset":105788,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":106300,"parent_record_offset":106276,"ancestor_record_offset":105788,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":265,"record_offset":106276,"parent_record_offset":106252,"ancestor_record_offset":105788,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":106064,"parent_record_offset":106024,"ancestor_record_offset":105788,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$42::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":265,"record_offset":106088,"parent_record_offset":106064,"ancestor_record_offset":105788,"inlinee":6409,"source_line":2912,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":265,"record_offset":107328,"parent_record_offset":107296,"ancestor_record_offset":105788,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":265,"record_offset":107352,"parent_record_offset":107328,"ancestor_record_offset":105788,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":265,"record_offset":105968,"parent_record_offset":105944,"ancestor_record_offset":105788,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":106204,"parent_record_offset":106024,"ancestor_record_offset":105788,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":265,"record_offset":107872,"parent_record_offset":107844,"ancestor_record_offset":105788,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":265,"record_offset":107696,"parent_record_offset":107660,"ancestor_record_offset":105788,"inlinee":16850,"source_line":32,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":106148,"parent_record_offset":106024,"ancestor_record_offset":105788,"inlinee":16847,"source_line":41,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":265,"record_offset":106176,"parent_record_offset":106024,"ancestor_record_offset":105788,"inlinee":16862,"source_line":113,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":265,"record_offset":107660,"parent_record_offset":107628,"ancestor_record_offset":105788,"inlinee":16869,"source_line":307,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$4::active
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 833
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4223
// 首条位置: {"module":265,"record_offset":107936,"rva":6481072,"va":5375190192}
// 编译实例 RVAs: 0x62e4b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7979531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375190192,"raw_hex":"e80bc27900","size":5,"soft_fail":false,"target_va":5383169744,"va":5375190208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$4::hovered
// 记录形式: compiled_procedure; instances=1
// 源码行: 847
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":265,"record_offset":108048,"rva":6481232,"va":5375190352}
// 编译实例 RVAs: 0x62e550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7979371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375190352,"raw_hex":"e86bc17900","size":5,"soft_fail":false,"target_va":5383169744,"va":5375190368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$4::pressed
// 记录形式: compiled_procedure; instances=1
// 源码行: 861
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":265,"record_offset":108160,"rva":6481392,"va":5375190512}
// 编译实例 RVAs: 0x62e5f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7979211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375190512,"raw_hex":"e8cbc07900","size":5,"soft_fail":false,"target_va":5383169744,"va":5375190528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$5::appearance
// 记录形式: compiled_procedure; instances=1
// 源码行: 881
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":265,"record_offset":108272,"rva":6481552,"va":5375190672}
// 编译实例 RVAs: 0x62e690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] iced_style::container::Appearance::with_border
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/container.rs
//     evidence={"module":265,"record_offset":108424,"parent_record_offset":108272,"ancestor_record_offset":108272,"inlinee":16912,"source_line":20,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_direct] iced_style::container::impl$4::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/container.rs
//     evidence={"module":265,"record_offset":108400,"parent_record_offset":108272,"ancestor_record_offset":108272,"inlinee":16910,"source_line":5,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$57::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":265,"record_offset":108492,"parent_record_offset":108472,"ancestor_record_offset":108272,"inlinee":15721,"source_line":455,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::border::impl$12::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/border.rs
//     evidence={"module":265,"record_offset":108452,"parent_record_offset":108424,"ancestor_record_offset":108272,"inlinee":16914,"source_line":5,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::border::impl$18::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/border.rs
//     evidence={"module":265,"record_offset":108472,"parent_record_offset":108452,"ancestor_record_offset":108272,"inlinee":16916,"source_line":29,"archive":"_pdb/B/modules/00265.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$6::active
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 892
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4229
// 首条位置: {"module":265,"record_offset":108536,"rva":6481696,"va":5375190816}
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
// 函数符号: crash_reporter_gui::impl$6::hovered
// 记录形式: compiled_procedure; instances=1
// 源码行: 900
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":265,"record_offset":108648,"rva":6481840,"va":5375190960}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "iced_style::button::StyleSheet::disabled<crash_reporter_gui::ButtonStyle>", "constructor": "iced_style::button::StyleSheet::disabled", "arguments": ["crash_reporter_gui::ButtonStyle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] crash_reporter_gui::Controls
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls>", "constructor": "tg_iced::gui_runner::run_gui_with_options", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::new<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::State::new", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::update<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::State::update", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$1", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$0", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> > >", "constructor": "core::iter::adapters::cloned::Cloned", "arguments": ["core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<iced_core::event::Event>,core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<iced_core::event::Event>", "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>", "constructor": "tg_iced::gui_runner::run_gui_with_options::closure_env$1", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::EventLoop::pump_events", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "tuple$<enum2$<winit::event::Event<tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<crash_reporter_gui::Controls>", "constructor": "core::ptr::drop_in_place", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::State", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::program::state::State<crash_reporter_gui::Controls> >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::program::state::State<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] crash_reporter_gui::CrashAdvice
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "enum2$<crash_reporter_gui::CrashAdvice>", "constructor": "enum2$", "arguments": ["crash_reporter_gui::CrashAdvice"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<enum2$<crash_reporter_gui::CrashAdvice> >", "constructor": "core::marker::PhantomData", "arguments": ["enum2$<crash_reporter_gui::CrashAdvice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::from_str_seed<core::marker::PhantomData<enum2$<crash_reporter_gui::CrashAdvice> >,enum2$<crash_reporter_gui::CrashAdvice> >", "constructor": "ron::options::Options::from_str_seed", "arguments": ["core::marker::PhantomData<enum2$<crash_reporter_gui::CrashAdvice> >", "enum2$<crash_reporter_gui::CrashAdvice>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 216
// [PDB 泛型关系，非字段关系] {"type": "enum2$<crash_reporter_gui::Message>", "constructor": "enum2$", "arguments": ["crash_reporter_gui::Message"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::into_cache<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::into_cache", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::Element", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::build<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_runtime::user_interface::UserInterface::build", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::update<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::update", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_runtime::command::action::Action", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >", "constructor": "enum2$", "arguments": ["iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<crash_reporter_gui::Message> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::draw::recurse::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::draw::recurse::closure_env$1", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_graphics::renderer::impl$1::with_layer<iced_tiny_skia::backend::Backend,iced_runtime::overlay::nested::impl$0::draw::recurse::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_graphics::renderer::impl$1::with_layer", "arguments": ["iced_tiny_skia::backend::Backend", "iced_runtime::overlay::nested::impl$0::draw::recurse::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::impl$0::update::closure_env$2", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::event::Status,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::event::Status", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::draw::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::draw::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::layout::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::layout::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::is_over::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::is_over::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::on_event::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::on_event::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$20::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::vec::impl$20::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::event::Status,core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$20::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::event::Status", "core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$20::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<crash_reporter_gui::Message> > >", "constructor": "core::ops::function::Fn", "arguments": ["tuple$<iced_widget::scrollable::Viewport>", "assoc$<Output,enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "constructor": "dyn$", "arguments": ["core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<crash_reporter_gui::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >", "constructor": "core::option::Option", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<iced_widget::scrollable::Viewport>,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::mouse_interaction<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::mouse_interaction", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::mouse_interaction<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::mouse_interaction", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::state<iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::state", "arguments": ["iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::state<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::state", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::operate<iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::operate", "arguments": ["iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::operate<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::operate", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::overlay<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::overlay", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::overlay<iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::overlay", "arguments": ["iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::MapOperation", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation::finish<iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >,enum2$<tg_integrity_report::controls::Message> >", "constructor": "iced_core::widget::operation::Operation::finish", "arguments": ["iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >", "enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button::new<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::button::Button::new", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::push<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec::push", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::push<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::column::Column::push", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::diff<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::diff", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::overlay<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::overlay", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::column::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::column::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::scrollable::Scrollable<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::scrollable::Scrollable<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row::with_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::row::Row::with_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row::push<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::row::Row::push", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::diff<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::diff", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate::closure$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate::closure$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::overlay<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::overlay", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::row::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::row::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::tag<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::tag", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::state<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::state", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >", "constructor": "ref$", "arguments": ["dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::Element", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::slice::iter::IterMut<iced_core::widget::tree::Tree>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::from_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$30::call<tuple$<enum2$<tg_integrity_report::controls::Message> >,dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$30::call", "arguments": ["tuple$<enum2$<tg_integrity_report::controls::Message> >", "dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$29::call_mut<tuple$<enum2$<tg_integrity_report::controls::Message> >,dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$29::call_mut", "arguments": ["tuple$<enum2$<tg_integrity_report::controls::Message> >", "dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::shell::Shell::merge<enum2$<crash_reporter_gui::Message>,enum2$<tg_integrity_report::controls::Message>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "constructor": "iced_core::shell::Shell::merge", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<tg_integrity_report::controls::Message>", "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::shell::Shell::merge<enum2$<crash_reporter_gui::Message>,enum2$<tg_integrity_report::controls::Message>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "iced_core::shell::Shell::merge", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<tg_integrity_report::controls::Message>", "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$1::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$1::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation::finish<iced_core::overlay::element::impl$2::operate::MapOperation<enum2$<crash_reporter_gui::Message> >,enum2$<tg_integrity_report::controls::Message> >", "constructor": "iced_core::widget::operation::Operation::finish", "arguments": ["iced_core::overlay::element::impl$2::operate::MapOperation<enum2$<crash_reporter_gui::Message> >", "enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::tag<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::tag", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::state<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::state", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$1", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::Tree::diff_children_custom<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::widget::tree::Tree::diff_children_custom", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::draw::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure$1<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure$1", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure_env$2", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_graphics::renderer::impl$1::with_layer<iced_tiny_skia::backend::Backend,iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_graphics::renderer::impl$1::with_layer", "arguments": ["iced_tiny_skia::backend::Backend", "iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure_env$0", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_graphics::renderer::impl$1::with_layer<iced_tiny_skia::backend::Backend,iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_graphics::renderer::impl$1::with_layer", "arguments": ["iced_tiny_skia::backend::Backend", "iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::layout::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::layout::padded::closure_env$0", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::positioned<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::positioned", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::layout::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::layout::closure_env$0", "arguments": ["iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::layout::closure_env$1", "arguments": ["iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::positioned<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::positioned", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::layout::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::layout::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::layout::closure_env$1", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::next_to_each_other<iced_widget::checkbox::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::checkbox::impl$1::layout::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::layout::next_to_each_other", "arguments": ["iced_widget::checkbox::impl$1::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::checkbox::impl$1::layout::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::layout::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::layout::closure_env$0", "arguments": ["iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::contained<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::contained", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::flex::resolve<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::layout::flex::resolve", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_core::overlay::group::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::system::system_param::Local<country_core::systems::save::load_completion_checker::LoadingCompletionState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::system::system_param::Local<country_core::systems::save::load_completion_checker::LoadingCompletionState>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::save::load_completion_checker::LoadingCompletionState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$1,tuple$<bool> >
// 信息 ID: 9deafc88d6cd108956bd9992
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["bool"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["crash_reporter_gui::impl$3::main_view::closure_env$1", "tuple$<bool>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::str::pattern::TwoWaySearcher::next<enum2$<core::str::pattern::MatchOnly> >
// 信息 ID: 19b3b3e3b265309086146281
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["core::str::pattern::MatchOnly"]; 字段布局 = unknown。
// [类型关系] core::str::pattern::TwoWaySearcher::next -> ["enum2$<core::str::pattern::MatchOnly>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::str::pattern::impl$33::next_match @ 0x61f5b4 -> 0x61f240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::Controls::main_view
// 信息 ID: d9ac10c13b04f9a9f0fc696c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_reporter_gui::impl$2::view @ 0x62141b -> 0x623370 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::Controls::more_info_button
// 信息 ID: 808b62ac16dc2ce697daed3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::Controls::new
// 信息 ID: 4984f65e049624537d6b322b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_reporter_gui::run_crash_reporter_gui<tiny_glade::mock_crash_reporter::closure_env$1> @ 0x337ff0 -> 0x61f640 (direct_call); 同地址别名未裁定。
// [静态直接引用] crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6> @ 0x3394ee -> 0x61f640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::_::impl$0::deserialize
// 信息 ID: 8eb5da159184bf187369130e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::options::Options::from_str_seed<core::marker::PhantomData<enum2$<crash_reporter_gui::CrashAdvice> >,enum2$<crash_reporter_gui::CrashAdvice> >; module=211, inline_record=58456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 9a6f56e874054066ee38a4bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] crash_reporter_gui::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<crash_reporter_gui::_::impl$0::deserialize::__Field> > @ 0x37f146 -> 0x337590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: efcecda6afe4ab565fb5c316
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<crash_reporter_gui::_::impl$0::deserialize::__Field> >; module=155, inline_record=8248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 3a96287123059a1d37c84f6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<crash_reporter_gui::_::impl$0::deserialize::__Visitor>; module=155, inline_record=74216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::copy_to_clipboard
// 信息 ID: f8f2bfce1bda5188d4270ab4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::impl$2::update; module=265, inline_record=16128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$0::program_outcome
// 信息 ID: 6ac3f725509ac71a718c728a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls> @ 0xe8a99 -> 0x61f610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$0::tick_message
// 信息 ID: 484e7e3fec9164d3e552e9f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls> @ 0xe89f9 -> 0x61f620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$11::clone
// 信息 ID: f2aba2f8de00abd83b0ecd52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_widget::button::impl$1::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >; module=891, inline_record=10980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$0
// 信息 ID: 311a76ab88ef443cdcbdb120
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=265, inline_record=9620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$1
// 信息 ID: fd0a3fb258163c6f65fa49f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=265, inline_record=11596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$2
// 信息 ID: fb1cadca60a2182b1a63eca4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=265, inline_record=11852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$3
// 信息 ID: 66d11d3a0b8aa03d602d58b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=265, inline_record=12448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$4
// 信息 ID: e318e086657bab053c1fc344
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=265, inline_record=12780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$5
// 信息 ID: 2800ad27307efac70d15faed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=265, inline_record=13012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$6
// 信息 ID: 7d8c42bb6bf0f8c1a638efe6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=265, inline_record=13244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::update
// 信息 ID: 6ea4ff906070fc84d7c92b90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::command::Command::batch<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > > @ 0xf3e7f -> 0x620470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::update::closure$0
// 信息 ID: 59d577728facde1a4e5a8b8d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$2::update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::impl$2::update; module=265, inline_record=16556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::view
// 信息 ID: 8aee6052377d3b4d4b3946a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::program::state::State::new<crash_reporter_gui::Controls> @ 0xeb5b1 -> 0x6213c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_runtime::program::state::State::update<crash_reporter_gui::Controls> @ 0xeb7ff -> 0x6213c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_runtime::program::state::State::update<crash_reporter_gui::Controls> @ 0xebb47 -> 0x6213c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::view::closure$0
// 信息 ID: e7a326d6345d52c6cd6532db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4df6506252c83c8495a5ff87: iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] b9123650d11dc9236a011191: iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$2::view", "captures": "unknown"}
// [闭包环境类型引用] crash_reporter_gui::impl$2::view::closure_env$0 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; module=890, inline_record=3264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$3::main_view::closure$0
// 信息 ID: 5d76b70d2c3f89bc42351718
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$3::main_view", "captures": "unknown"}
// [闭包环境类型引用] crash_reporter_gui::impl$3::main_view::closure_env$0 <- core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$0,tuple$<enum2$<tg_integrity_report::controls::Message> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$0,tuple$<enum2$<tg_integrity_report::controls::Message> > >; module=889, inline_record=8612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$3::main_view::closure$1
// 信息 ID: d120b9e3bf80b4c7920a33c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "crash_reporter_gui::impl$3::main_view", "captures": "unknown"}
// [闭包环境类型引用] crash_reporter_gui::impl$3::main_view::closure_env$1 <- core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$1,tuple$<bool> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$1,tuple$<bool> >; module=896, inline_record=5268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$4::active
// 信息 ID: a45592d3cfe747300eecde87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_style::button::StyleSheet::disabled<crash_reporter_gui::ButtonStyle>; module=888, inline_record=616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$4::hovered
// 信息 ID: 83492b1cb6cbee4307e48853
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$4::pressed
// 信息 ID: 83cec8039bd395c1669e6147
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$5::appearance
// 信息 ID: 65b0919b07a81b0c39a95ddc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$6::active
// 信息 ID: 1eb7d57fef75c208105ae4c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_style::button::StyleSheet::pressed<crash_reporter_gui::HyperlinkStyle>; module=888, inline_record=424; 不是独立调用边。
// [PDB 内联宿主] iced_style::button::StyleSheet::disabled<crash_reporter_gui::HyperlinkStyle>; module=888, inline_record=812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$6::hovered
// 信息 ID: 37ae8255c17a1ddbcfc353fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::run_crash_reporter_gui<tiny_glade::mock_crash_reporter::closure_env$1>
// 信息 ID: 5d08f86bda9764dd19bce699
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] crash_reporter_gui::run_crash_reporter_gui -> ["tiny_glade::mock_crash_reporter::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::mock_crash_reporter @ 0x3df3b7 -> 0x337730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6>
// 信息 ID: 83f3a9715cf3679adc548ce7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] crash_reporter_gui::run_crash_reporter_gui -> ["tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x3b08ec -> 0x338bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::text
// 信息 ID: c4e0225d61eb24eff6f881bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4df6506252c83c8495a5ff87: iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] b9123650d11dc9236a011191: iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 0 条；内联宿主 39 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=265, inline_record=19248; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=265, inline_record=20612; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=265, inline_record=22848; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=265, inline_record=24280; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=265, inline_record=25356; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=265, inline_record=28012; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=265, inline_record=31960; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=265, inline_record=33368; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::wide_button
// 信息 ID: 7cc6ebf011747a38ff110dbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=265, inline_record=25316; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::impl$2::view; module=265, inline_record=27972; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=265, inline_record=59572; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=265, inline_record=62184; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=265, inline_record=67528; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=265, inline_record=68920; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=265, inline_record=76104; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=265, inline_record=78296; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/crash-reporter-gui/src/lib.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a1fd0858166b00f3ca8bbd290bde2582f921306c22f6608a2ee28b85a6e5d1ed
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 64, 211, 240, 263, 927, 928, 929, 936, 939, 2362, 3720
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
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
//   package identities: [{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use std; // namespace candidate
//   package identities: []
// use tg_integrity_report; // namespace candidate
//   package identities: [{"name":"tg-integrity-report","source":"local_or_path","version":"0.1.0"}]
// use webbrowser; // namespace candidate
//   package identities: [{"name":"webbrowser","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.10"}]
// use winit; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.11.0"},{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::Controls::main_view
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":263,"record_offset":7776,"rva":6997776,"va":5375706896}
// 编译实例 RVAs: 0x6ac710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::str::pattern::StrSearcher>::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9108766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e81efd8a00","size":5,"soft_fail":false,"target_va":5384824608,"va":5375715837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=79
//     evidence={"assembly":"\tcallq\t-28483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8bd90ffff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375707166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=67
//     evidence={"assembly":"\tcallq\t-29751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8c98bffff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375708450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=79
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6984960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8006b95ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375707147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007e380
//     owner=unresolved_namespace; count=19
//     target_aliases=39 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6518178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e85e8a9cff","size":5,"soft_fail":false,"target_va":5369226112,"va":5375744285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=50
//     target_aliases=66 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6419913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8370a9eff","size":5,"soft_fail":false,"target_va":5369308864,"va":5375728772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140db7490
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7372365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e84d7e7000","size":5,"soft_fail":false,"target_va":5383091344,"va":5375718974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa9bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9414452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e834a78f00","size":5,"soft_fail":false,"target_va":5385132976,"va":5375718519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa9c90
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9420331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e82bbe8f00","size":5,"soft_fail":false,"target_va":5385133200,"va":5375712864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa9e00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9414613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8d5a78f00","size":5,"soft_fail":false,"target_va":5385133568,"va":5375718950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa9f30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9414770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e872a88f00","size":5,"soft_fail":false,"target_va":5385133872,"va":5375719097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fab4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9411498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8aa9b8f00","size":5,"soft_fail":false,"target_va":5385139408,"va":5375727905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fab520
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9420348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e83cbe8f00","size":5,"soft_fail":false,"target_va":5385139488,"va":5375719135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fab5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9411845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8059d8f00","size":5,"soft_fail":false,"target_va":5385139664,"va":5375727814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fab670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9411622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8269c8f00","size":5,"soft_fail":false,"target_va":5385139824,"va":5375728197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fabd60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9419069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e83db98f00","size":5,"soft_fail":false,"target_va":5385141600,"va":5375722526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9397662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e89e658f00","size":5,"soft_fail":false,"target_va":5385143120,"va":5375745453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=36
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40641606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e846246c02","size":5,"soft_fail":false,"target_va":5416351056,"va":5375709445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40622836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8f4da6b02","size":5,"soft_fail":false,"target_va":5416353040,"va":5375730199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=31
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41349863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8e7f27602","size":5,"soft_fail":false,"target_va":5417057056,"va":5375707188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=46
//     evidence={"assembly":"\tcallq\t40887460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8a4e46f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5375733304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t9086265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e839a58a00","size":5,"soft_fail":false,"target_va":5384794864,"va":5375708594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=33
//     evidence={"assembly":"\tcallq\t40913658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8fa4a7002","size":5,"soft_fail":false,"target_va":5416620794,"va":5375707131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$crash_reporter_gui..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=28
//     evidence={"assembly":"\tcallq\t9405699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e803858f00","size":5,"soft_fail":false,"target_va":5385142704,"va":5375737000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::contains<ref$<str$> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e815b70000","size":5,"soft_fail":false,"target_va":5375754448,"va":5375707574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::iter::SplitInternal::next_back<array$<char,2> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t48390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e806bd0000","size":5,"soft_fail":false,"target_va":5375755488,"va":5375707093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-27741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8a393ffff","size":5,"soft_fail":false,"target_va":5375702528,"va":5375730264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_reporter_gui::Controls::more_info_button
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e850c6ffff","size":5,"soft_fail":false,"target_va":5375702992,"va":5375717755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9448460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e80c2c9000","size":5,"soft_fail":false,"target_va":5385173056,"va":5375724591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::container::Container$LT$Message$C$Theme$C$Renderer$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9455768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e898489000","size":5,"soft_fail":false,"target_va":5385183760,"va":5375727987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::container::Container$LT$Message$C$Theme$C$Renderer$GT$::style
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9455914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e82a499000","size":5,"soft_fail":false,"target_va":5385184096,"va":5375728177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9448463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e80f2c9000","size":5,"soft_fail":false,"target_va":5385172352,"va":5375723884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::row
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9448002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8422a9000","size":5,"soft_fail":false,"target_va":5385171536,"va":5375723529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::row::Row$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9452705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8a13c9000","size":5,"soft_fail":false,"target_va":5385180048,"va":5375727338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=26
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7426218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e8aa507100","size":5,"soft_fail":false,"target_va":5383134720,"va":5375708497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::controls::Controls::verification_status
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9436969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375706896,"raw_hex":"e829ff8f00","size":5,"soft_fail":false,"target_va":5385148272,"va":5375711298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375706896,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375713050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":58728,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":25516,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":263,"record_offset":13596,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":263,"record_offset":34204,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":16752,"source_line":268,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":8040,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":17252,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":263,"record_offset":16584,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":263,"record_offset":13680,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":263,"record_offset":69332,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":8135,"source_line":507,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":8068,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":5727,"source_line":1390,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":12752,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":12896,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":63688,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::external_crashing_module
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":8124,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18726,"source_line":124,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=21
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":13708,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18706,"source_line":316,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::wide_button
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":23172,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18742,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":263,"record_offset":67868,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18741,"source_line":279,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":32788,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18712,"source_line":112,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":73872,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18714,"source_line":104,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":73212,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18713,"source_line":127,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":77784,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18716,"source_line":307,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::checkbox::Checkbox::on_toggle
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/checkbox.rs
//     evidence={"module":263,"record_offset":66076,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18760,"source_line":103,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::align_items
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":61536,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18754,"source_line":100,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":63252,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18759,"source_line":76,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":61504,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18753,"source_line":70,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":61568,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18756,"source_line":333,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":263,"record_offset":69532,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18767,"source_line":79,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":263,"record_offset":47880,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18710,"source_line":116,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::checkbox
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":263,"record_offset":59040,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18745,"source_line":159,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":263,"record_offset":69500,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18765,"source_line":57,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::align_items
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":263,"record_offset":78368,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18768,"source_line":91,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":263,"record_offset":60860,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18749,"source_line":67,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":263,"record_offset":78432,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18751,"source_line":322,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::Space::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":263,"record_offset":69468,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18764,"source_line":20,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":263,"record_offset":62272,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":18758,"source_line":95,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":263,"record_offset":44496,"parent_record_offset":7776,"ancestor_record_offset":7776,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=79
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":8992,"parent_record_offset":8968,"ancestor_record_offset":7776,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=79
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":9016,"parent_record_offset":8992,"ancestor_record_offset":7776,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=58
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":7776,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=58
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":12244,"parent_record_offset":12220,"ancestor_record_offset":7776,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=79
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":9040,"parent_record_offset":9016,"ancestor_record_offset":7776,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=58
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":7776,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=79
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":8968,"parent_record_offset":8816,"ancestor_record_offset":7776,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=58
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":12196,"parent_record_offset":12132,"ancestor_record_offset":7776,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":16752,"parent_record_offset":16704,"ancestor_record_offset":7776,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":16704,"parent_record_offset":16664,"ancestor_record_offset":7776,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":24084,"parent_record_offset":24056,"ancestor_record_offset":7776,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":263,"record_offset":12804,"parent_record_offset":12780,"ancestor_record_offset":7776,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":263,"record_offset":12852,"parent_record_offset":12828,"ancestor_record_offset":7776,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":17360,"parent_record_offset":17328,"ancestor_record_offset":7776,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":34940,"parent_record_offset":34908,"ancestor_record_offset":7776,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":8760,"parent_record_offset":8736,"ancestor_record_offset":7776,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":35032,"parent_record_offset":35004,"ancestor_record_offset":7776,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":12164,"parent_record_offset":12132,"ancestor_record_offset":7776,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=57
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":12132,"parent_record_offset":12104,"ancestor_record_offset":7776,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":35004,"parent_record_offset":34972,"ancestor_record_offset":7776,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":17408,"parent_record_offset":17384,"ancestor_record_offset":7776,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":17384,"parent_record_offset":17360,"ancestor_record_offset":7776,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":34972,"parent_record_offset":34940,"ancestor_record_offset":7776,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":8816,"parent_record_offset":8784,"ancestor_record_offset":7776,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":7776,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=57
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":12104,"parent_record_offset":12076,"ancestor_record_offset":7776,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":8856,"parent_record_offset":8816,"ancestor_record_offset":7776,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":58752,"parent_record_offset":58728,"ancestor_record_offset":7776,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":7776,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":8684,"parent_record_offset":8660,"ancestor_record_offset":7776,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":7776,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":7776,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=31
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":263,"record_offset":8612,"parent_record_offset":8584,"ancestor_record_offset":7776,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$5::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":263,"record_offset":8584,"parent_record_offset":8124,"ancestor_record_offset":7776,"inlinee":18731,"source_line":591,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":34764,"parent_record_offset":34204,"ancestor_record_offset":7776,"inlinee":6765,"source_line":1101,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":34288,"parent_record_offset":34204,"ancestor_record_offset":7776,"inlinee":6749,"source_line":484,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$114::spec_to_string
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":13800,"parent_record_offset":13764,"ancestor_record_offset":7776,"inlinee":10264,"source_line":3043,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":17556,"parent_record_offset":17520,"ancestor_record_offset":7776,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":13764,"parent_record_offset":13708,"ancestor_record_offset":7776,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":13836,"parent_record_offset":13800,"ancestor_record_offset":7776,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":34860,"parent_record_offset":34828,"ancestor_record_offset":7776,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":17328,"parent_record_offset":17284,"ancestor_record_offset":7776,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":44664,"parent_record_offset":44632,"ancestor_record_offset":7776,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":34796,"parent_record_offset":34764,"ancestor_record_offset":7776,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":58776,"parent_record_offset":58752,"ancestor_record_offset":7776,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":35088,"parent_record_offset":34860,"ancestor_record_offset":7776,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":17284,"parent_record_offset":17252,"ancestor_record_offset":7776,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":34908,"parent_record_offset":34860,"ancestor_record_offset":7776,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":34316,"parent_record_offset":34288,"ancestor_record_offset":7776,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":8736,"parent_record_offset":8708,"ancestor_record_offset":7776,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":44608,"parent_record_offset":44580,"ancestor_record_offset":7776,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":44632,"parent_record_offset":44608,"ancestor_record_offset":7776,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":263,"record_offset":34828,"parent_record_offset":34796,"ancestor_record_offset":7776,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":263,"record_offset":44580,"parent_record_offset":44552,"ancestor_record_offset":7776,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":24116,"parent_record_offset":24084,"ancestor_record_offset":7776,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":8880,"parent_record_offset":8856,"ancestor_record_offset":7776,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":263,"record_offset":9440,"parent_record_offset":9416,"ancestor_record_offset":7776,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":263,"record_offset":59100,"parent_record_offset":59068,"ancestor_record_offset":7776,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":263,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":7776,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":263,"record_offset":69364,"parent_record_offset":69332,"ancestor_record_offset":7776,"inlinee":8137,"source_line":36,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":24180,"parent_record_offset":24144,"ancestor_record_offset":7776,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":24144,"parent_record_offset":24116,"ancestor_record_offset":7776,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":263,"record_offset":8904,"parent_record_offset":8880,"ancestor_record_offset":7776,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_uppercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":263,"record_offset":9312,"parent_record_offset":9280,"ancestor_record_offset":7776,"inlinee":18736,"source_line":816,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::make_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":263,"record_offset":9248,"parent_record_offset":9196,"ancestor_record_offset":7776,"inlinee":18734,"source_line":746,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":263,"record_offset":9280,"parent_record_offset":9248,"ancestor_record_offset":7776,"inlinee":18735,"source_line":668,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":8092,"parent_record_offset":8068,"ancestor_record_offset":7776,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":35236,"parent_record_offset":35204,"ancestor_record_offset":7776,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":12828,"parent_record_offset":12804,"ancestor_record_offset":7776,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":8556,"parent_record_offset":8124,"ancestor_record_offset":7776,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":263,"record_offset":9096,"parent_record_offset":8708,"ancestor_record_offset":7776,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":263,"record_offset":35496,"parent_record_offset":35468,"ancestor_record_offset":7776,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":7776,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=280
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":12020,"parent_record_offset":8124,"ancestor_record_offset":7776,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":263,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":7776,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":263,"record_offset":35468,"parent_record_offset":35440,"ancestor_record_offset":7776,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":263,"record_offset":35440,"parent_record_offset":35412,"ancestor_record_offset":7776,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":17448,"parent_record_offset":17284,"ancestor_record_offset":7776,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::make_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":263,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":7776,"inlinee":18733,"source_line":195,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":263,"record_offset":9464,"parent_record_offset":9440,"ancestor_record_offset":7776,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":263,"record_offset":9488,"parent_record_offset":9464,"ancestor_record_offset":7776,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":263,"record_offset":9416,"parent_record_offset":9392,"ancestor_record_offset":7776,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":263,"record_offset":8420,"parent_record_offset":8396,"ancestor_record_offset":7776,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":263,"record_offset":9928,"parent_record_offset":9900,"ancestor_record_offset":7776,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":263,"record_offset":35384,"parent_record_offset":35348,"ancestor_record_offset":7776,"inlinee":7436,"source_line":137,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::make_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":263,"record_offset":35412,"parent_record_offset":35384,"ancestor_record_offset":7776,"inlinee":7437,"source_line":89,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":263,"record_offset":8444,"parent_record_offset":8420,"ancestor_record_offset":7776,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":7776,"inlinee":8292,"source_line":1123,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":8396,"parent_record_offset":8372,"ancestor_record_offset":7776,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":9368,"parent_record_offset":8124,"ancestor_record_offset":7776,"inlinee":8326,"source_line":1430,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::make_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":9172,"parent_record_offset":8584,"ancestor_record_offset":7776,"inlinee":18732,"source_line":2894,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::match_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":34708,"parent_record_offset":34204,"ancestor_record_offset":7776,"inlinee":16753,"source_line":2107,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rsplit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":8276,"parent_record_offset":8124,"ancestor_record_offset":7776,"inlinee":18727,"source_line":1751,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::split
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":7776,"inlinee":18728,"source_line":1655,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":9872,"parent_record_offset":8124,"ancestor_record_offset":7776,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::MatchIndicesInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":263,"record_offset":35204,"parent_record_offset":35176,"ancestor_record_offset":7776,"inlinee":16758,"source_line":1059,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$64::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":263,"record_offset":8528,"parent_record_offset":8124,"ancestor_record_offset":7776,"inlinee":18730,"source_line":550,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$91::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":263,"record_offset":35176,"parent_record_offset":34204,"ancestor_record_offset":7776,"inlinee":16756,"source_line":511,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$13::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":263,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":7776,"inlinee":16754,"source_line":736,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":263,"record_offset":34732,"parent_record_offset":34708,"ancestor_record_offset":7776,"inlinee":9296,"source_line":975,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":263,"record_offset":9900,"parent_record_offset":9872,"ancestor_record_offset":7776,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":263,"record_offset":9392,"parent_record_offset":9368,"ancestor_record_offset":7776,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$9::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":263,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":7776,"inlinee":8291,"source_line":677,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":263,"record_offset":10772,"parent_record_offset":8124,"ancestor_record_offset":7776,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$3::main_view::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":12780,"parent_record_offset":12752,"ancestor_record_offset":7776,"inlinee":18738,"source_line":551,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$3::main_view::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":12928,"parent_record_offset":12896,"ancestor_record_offset":7776,"inlinee":18739,"source_line":552,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":23224,"parent_record_offset":23172,"ancestor_record_offset":7776,"inlinee":18706,"source_line":316,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=44
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":263,"record_offset":16664,"parent_record_offset":16624,"ancestor_record_offset":7776,"inlinee":18717,"source_line":32,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::horizontal_alignment
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":263,"record_offset":23896,"parent_record_offset":23172,"ancestor_record_offset":7776,"inlinee":18743,"source_line":95,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":263,"record_offset":47848,"parent_record_offset":47208,"ancestor_record_offset":7776,"inlinee":18708,"source_line":113,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=18
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":263,"record_offset":16624,"parent_record_offset":16584,"ancestor_record_offset":7776,"inlinee":18741,"source_line":279,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=7
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":23984,"parent_record_offset":23172,"ancestor_record_offset":7776,"inlinee":18713,"source_line":127,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::width
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":23952,"parent_record_offset":23172,"ancestor_record_offset":7776,"inlinee":18744,"source_line":92,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":45736,"parent_record_offset":45700,"ancestor_record_offset":7776,"inlinee":18716,"source_line":307,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::checkbox::Checkbox::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/checkbox.rs
//     evidence={"module":263,"record_offset":59068,"parent_record_offset":59040,"ancestor_record_offset":7776,"inlinee":18747,"source_line":75,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::checkbox::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/checkbox.rs
//     evidence={"module":263,"record_offset":66144,"parent_record_offset":66108,"ancestor_record_offset":7776,"inlinee":18762,"source_line":372,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":63356,"parent_record_offset":63316,"ancestor_record_offset":7776,"inlinee":18756,"source_line":333,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::helpers::button
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":263,"record_offset":23924,"parent_record_offset":23172,"ancestor_record_offset":7776,"inlinee":18710,"source_line":116,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/row.rs
//     evidence={"module":263,"record_offset":60928,"parent_record_offset":60892,"ancestor_record_offset":7776,"inlinee":18751,"source_line":322,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::space::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/space.rs
//     evidence={"module":263,"record_offset":66924,"parent_record_offset":66892,"ancestor_record_offset":7776,"inlinee":18758,"source_line":95,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":263,"record_offset":44524,"parent_record_offset":44496,"ancestor_record_offset":7776,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":263,"record_offset":44552,"parent_record_offset":44524,"ancestor_record_offset":7776,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00263.jsonl.gz"}
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
// 首条位置: {"module":263,"record_offset":109296,"rva":7054848,"va":5375763968}
// 编译实例 RVAs: 0x6ba600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8883180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e8ec8b8700","size":5,"soft_fail":false,"target_va":5384648032,"va":5375764847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t8844364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e84cf48600","size":5,"soft_fail":false,"target_va":5384609408,"va":5375765039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-85575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e8b9b1feff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375764274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6425827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e81df39dff","size":5,"soft_fail":false,"target_va":5369338848,"va":5375764670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140436220
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2641780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e88cb0d7ff","size":5,"soft_fail":false,"target_va":5373125152,"va":5375766927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t40861348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e8a47e6f02","size":5,"soft_fail":false,"target_va":5416626928,"va":5375765575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8858189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e84d2a8700","size":5,"soft_fail":false,"target_va":5384624080,"va":5375765886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arboard::Clipboard::set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arboard","version":"3.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e8020d0000","size":5,"soft_fail":false,"target_va":5375768064,"va":5375764729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arboard::platform::windows::Set::text
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arboard","version":"3.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9426344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e8a8d58f00","size":5,"soft_fail":false,"target_va":5385191152,"va":5375764803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40856219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e89b6a6f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5375765984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_integrity_report..controls..Controls$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9381477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e865268f00","size":5,"soft_fail":false,"target_va":5385145616,"va":5375764134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<crash_reporter_gui::CurrentState> >
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-11531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e8f5d2ffff","size":5,"soft_fail":false,"target_va":5375753952,"va":5375765478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40855557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e805686f02","size":5,"soft_fail":false,"target_va":5416621632,"va":5375766070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] open::that
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"open","version":"5.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9423634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e812cb8f00","size":5,"soft_fail":false,"target_va":5385188288,"va":5375764649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::lifecycle::JoinInner$LT$T$GT$::join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9426450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e812d68f00","size":5,"soft_fail":false,"target_va":5385191856,"va":5375765401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::controls::Controls::remove_unrecognized_package_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9397852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e85c668f00","size":5,"soft_fail":false,"target_va":5385162096,"va":5375764239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] webbrowser::open
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"webbrowser","version":"0.8.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6220227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375763968,"raw_hex":"e8c3e95e00","size":5,"soft_fail":false,"target_va":5381984528,"va":5375764296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5375763968,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5375764446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$31::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":109956,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":12245,"source_line":2239,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":263,"record_offset":111080,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":109896,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":109520,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":263,"record_offset":110932,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":263,"record_offset":111764,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::copy_to_clipboard
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":110540,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":18815,"source_line":505,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::none
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":263,"record_offset":112156,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":18820,"source_line":30,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::window::close
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/window.rs
//     evidence={"module":263,"record_offset":109464,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":18813,"source_line":50,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":263,"record_offset":111136,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":263,"record_offset":110996,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] std::thread::join_handle::JoinHandle::is_finished
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\join_handle.rs
//     evidence={"module":263,"record_offset":111500,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":10776,"source_line":164,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] std::thread::join_handle::JoinHandle::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\join_handle.rs
//     evidence={"module":263,"record_offset":111736,"parent_record_offset":109296,"ancestor_record_offset":109296,"inlinee":12195,"source_line":149,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":109752,"parent_record_offset":109728,"ancestor_record_offset":109296,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":109776,"parent_record_offset":109752,"ancestor_record_offset":109296,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":109804,"parent_record_offset":109776,"ancestor_record_offset":109296,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":109728,"parent_record_offset":109672,"ancestor_record_offset":109296,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":109984,"parent_record_offset":109956,"ancestor_record_offset":109296,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":109700,"parent_record_offset":109672,"ancestor_record_offset":109296,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":109672,"parent_record_offset":109648,"ancestor_record_offset":109296,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":109648,"parent_record_offset":109624,"ancestor_record_offset":109296,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":263,"record_offset":111572,"parent_record_offset":111548,"ancestor_record_offset":109296,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::strong_count
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":263,"record_offset":111548,"parent_record_offset":111524,"ancestor_record_offset":109296,"inlinee":10779,"source_line":2003,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] arboard::Set::text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arboard","version":"3.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arboard-3.6.0/src/lib.rs
//     evidence={"module":263,"record_offset":110584,"parent_record_offset":110540,"ancestor_record_offset":109296,"inlinee":18817,"source_line":216,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":110016,"parent_record_offset":109984,"ancestor_record_offset":109296,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":110080,"parent_record_offset":110044,"ancestor_record_offset":109296,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":109920,"parent_record_offset":109896,"ancestor_record_offset":109296,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":110044,"parent_record_offset":110016,"ancestor_record_offset":109296,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":111104,"parent_record_offset":111080,"ancestor_record_offset":109296,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":109544,"parent_record_offset":109520,"ancestor_record_offset":109296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":263,"record_offset":111604,"parent_record_offset":111572,"ancestor_record_offset":109296,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":263,"record_offset":110612,"parent_record_offset":110540,"ancestor_record_offset":109296,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":263,"record_offset":110648,"parent_record_offset":110540,"ancestor_record_offset":109296,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":263,"record_offset":111020,"parent_record_offset":110996,"ancestor_record_offset":109296,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":263,"record_offset":111044,"parent_record_offset":111020,"ancestor_record_offset":109296,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$2::update::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":110964,"parent_record_offset":110932,"ancestor_record_offset":109296,"inlinee":18819,"source_line":371,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::single
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":263,"record_offset":109488,"parent_record_offset":109464,"ancestor_record_offset":109296,"inlinee":18814,"source_line":35,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":263,"record_offset":111184,"parent_record_offset":111160,"ancestor_record_offset":109296,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":263,"record_offset":111160,"parent_record_offset":111136,"ancestor_record_offset":109296,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":263,"record_offset":111208,"parent_record_offset":111184,"ancestor_record_offset":109296,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::lifecycle::JoinInner::is_finished
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\lifecycle.rs
//     evidence={"module":263,"record_offset":111524,"parent_record_offset":111500,"ancestor_record_offset":109296,"inlinee":10778,"source_line":232,"archive":"_pdb/C/modules/00263.jsonl.gz"}
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
// 首条位置: {"module":263,"record_offset":95832,"rva":7047232,"va":5375756352}
// 编译实例 RVAs: 0x6b8840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9036275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8f3e18900","size":5,"soft_fail":false,"target_va":5384795984,"va":5375759704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8891702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e836ad8700","size":5,"soft_fail":false,"target_va":5384648448,"va":5375756741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8891311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8afab8700","size":5,"soft_fail":false,"target_va":5384648032,"va":5375756716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <str as core::fmt::Display>::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9112205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e88d0a8b00","size":5,"soft_fail":false,"target_va":5384871088,"va":5375758878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t-78437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e89bcdfeff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375757120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=13
//     evidence={"assembly":"\tcallq\t-79435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8b5c9feff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375758134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=15
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7034918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8daa794ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375757105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007e380
//     owner=unresolved_namespace; count=4
//     target_aliases=39 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6536159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e821449cff","size":5,"soft_fail":false,"target_va":5369226112,"va":5375762266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400926c0
//     owner=unresolved_namespace; count=6
//     target_aliases=66 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6453653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e86b869dff","size":5,"soft_fail":false,"target_va":5369308864,"va":5375762512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa9c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9373869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8ad088f00","size":5,"soft_fail":false,"target_va":5385133200,"va":5375759326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9381288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8a8258f00","size":5,"soft_fail":false,"target_va":5385143488,"va":5375762195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fb6c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9429403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e89be18f00","size":5,"soft_fail":false,"target_va":5385186448,"va":5375757040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40593816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e898696b02","size":5,"soft_fail":false,"target_va":5416351056,"va":5375757235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41299508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8342e7602","size":5,"soft_fail":false,"target_va":5417057056,"va":5375757543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t40859007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e87f756f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5375761757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9038224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e890e98900","size":5,"soft_fail":false,"target_va":5384794864,"va":5375756635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t40864096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e860896f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375756693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40860381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8dd7a6f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5375761822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$crash_reporter_gui..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t9379949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e86d208f00","size":5,"soft_fail":false,"target_va":5385142704,"va":5375762750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40859609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8d9776f02","size":5,"soft_fail":false,"target_va":5416621632,"va":5375762018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_reporter_gui::Controls::main_view
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-49564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8643effff","size":5,"soft_fail":false,"target_va":5375706896,"va":5375756455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9412905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e829a18f00","size":5,"soft_fail":false,"target_va":5385173056,"va":5375760146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::container::Container$LT$Message$C$Theme$C$Renderer$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9425627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8dbd28f00","size":5,"soft_fail":false,"target_va":5385183392,"va":5375757760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::helpers::column
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t9410800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8f0988f00","size":5,"soft_fail":false,"target_va":5385172352,"va":5375761547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7378175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375756352,"raw_hex":"e8ff947000","size":5,"soft_fail":false,"target_va":5383134720,"va":5375756540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375756352,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375759518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":263,"record_offset":96048,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":96020,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":98224,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":263,"record_offset":97876,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":263,"record_offset":96132,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":97584,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_secs_f32
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":263,"record_offset":97644,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":7360,"source_line":885,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":96160,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18706,"source_line":316,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::wide_button
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":103480,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18742,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":263,"record_offset":101736,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18741,"source_line":279,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":104892,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18712,"source_line":112,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":105372,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18716,"source_line":307,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::align_items
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":97844,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18754,"source_line":100,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":105652,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18753,"source_line":70,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_x
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":263,"record_offset":99800,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18809,"source_line":121,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::width
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":263,"record_offset":99768,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18808,"source_line":85,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::column
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":263,"record_offset":97676,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18801,"source_line":68,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":263,"record_offset":99740,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18765,"source_line":57,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::scrollable::Scrollable::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/scrollable.rs
//     evidence={"module":263,"record_offset":99712,"parent_record_offset":95832,"ancestor_record_offset":95832,"inlinee":18807,"source_line":51,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":96796,"parent_record_offset":96772,"ancestor_record_offset":95832,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":96820,"parent_record_offset":96796,"ancestor_record_offset":95832,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":97188,"parent_record_offset":97164,"ancestor_record_offset":95832,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":97212,"parent_record_offset":97188,"ancestor_record_offset":95832,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":96844,"parent_record_offset":96820,"ancestor_record_offset":95832,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":97236,"parent_record_offset":97212,"ancestor_record_offset":95832,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":96772,"parent_record_offset":96612,"ancestor_record_offset":95832,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":97164,"parent_record_offset":97100,"ancestor_record_offset":95832,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":98028,"parent_record_offset":97984,"ancestor_record_offset":95832,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":97984,"parent_record_offset":97948,"ancestor_record_offset":95832,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":104380,"parent_record_offset":104352,"ancestor_record_offset":95832,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":263,"record_offset":96096,"parent_record_offset":96072,"ancestor_record_offset":95832,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":98324,"parent_record_offset":98292,"ancestor_record_offset":95832,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":96536,"parent_record_offset":96500,"ancestor_record_offset":95832,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":97132,"parent_record_offset":97100,"ancestor_record_offset":95832,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":97100,"parent_record_offset":97072,"ancestor_record_offset":95832,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":98372,"parent_record_offset":98348,"ancestor_record_offset":95832,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":98348,"parent_record_offset":98324,"ancestor_record_offset":95832,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":96612,"parent_record_offset":96572,"ancestor_record_offset":95832,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":96572,"parent_record_offset":96536,"ancestor_record_offset":95832,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":97072,"parent_record_offset":97044,"ancestor_record_offset":95832,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":96660,"parent_record_offset":96612,"ancestor_record_offset":95832,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":96388,"parent_record_offset":96352,"ancestor_record_offset":95832,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":96424,"parent_record_offset":96388,"ancestor_record_offset":95832,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":96460,"parent_record_offset":96424,"ancestor_record_offset":95832,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":96352,"parent_record_offset":96316,"ancestor_record_offset":95832,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":263,"record_offset":96316,"parent_record_offset":96280,"ancestor_record_offset":95832,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":102448,"parent_record_offset":102396,"ancestor_record_offset":95832,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$114::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":96244,"parent_record_offset":96208,"ancestor_record_offset":95832,"inlinee":10264,"source_line":3043,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":98536,"parent_record_offset":98508,"ancestor_record_offset":95832,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":102528,"parent_record_offset":102504,"ancestor_record_offset":95832,"inlinee":4606,"source_line":2711,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":96208,"parent_record_offset":96160,"ancestor_record_offset":95832,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":102396,"parent_record_offset":102360,"ancestor_record_offset":95832,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":96280,"parent_record_offset":96244,"ancestor_record_offset":95832,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":98292,"parent_record_offset":98252,"ancestor_record_offset":95832,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":98252,"parent_record_offset":98224,"ancestor_record_offset":95832,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":96500,"parent_record_offset":96460,"ancestor_record_offset":95832,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":104412,"parent_record_offset":104380,"ancestor_record_offset":95832,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":96684,"parent_record_offset":96660,"ancestor_record_offset":95832,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":263,"record_offset":102476,"parent_record_offset":102396,"ancestor_record_offset":95832,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$82::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":263,"record_offset":102504,"parent_record_offset":102396,"ancestor_record_offset":95832,"inlinee":6317,"source_line":2865,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":263,"record_offset":96732,"parent_record_offset":96708,"ancestor_record_offset":95832,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":263,"record_offset":97804,"parent_record_offset":97780,"ancestor_record_offset":95832,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":104476,"parent_record_offset":104440,"ancestor_record_offset":95832,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":104440,"parent_record_offset":104412,"ancestor_record_offset":95832,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":263,"record_offset":96708,"parent_record_offset":96684,"ancestor_record_offset":95832,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":263,"record_offset":97612,"parent_record_offset":97584,"ancestor_record_offset":95832,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":96072,"parent_record_offset":96048,"ancestor_record_offset":95832,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":263,"record_offset":96900,"parent_record_offset":96460,"ancestor_record_offset":95832,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":96924,"parent_record_offset":96900,"ancestor_record_offset":95832,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=58
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":96988,"parent_record_offset":96160,"ancestor_record_offset":95832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":98412,"parent_record_offset":98252,"ancestor_record_offset":95832,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":263,"record_offset":102560,"parent_record_offset":102396,"ancestor_record_offset":95832,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":103524,"parent_record_offset":103480,"ancestor_record_offset":95832,"inlinee":18706,"source_line":316,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":263,"record_offset":97948,"parent_record_offset":97912,"ancestor_record_offset":95832,"inlinee":18717,"source_line":32,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::horizontal_alignment
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":263,"record_offset":104196,"parent_record_offset":103480,"ancestor_record_offset":95832,"inlinee":18743,"source_line":95,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":263,"record_offset":99112,"parent_record_offset":99076,"ancestor_record_offset":95832,"inlinee":18741,"source_line":279,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":104284,"parent_record_offset":103480,"ancestor_record_offset":95832,"inlinee":18713,"source_line":127,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::width
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":104252,"parent_record_offset":103480,"ancestor_record_offset":95832,"inlinee":18744,"source_line":92,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":108296,"parent_record_offset":108260,"ancestor_record_offset":95832,"inlinee":18716,"source_line":307,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::extend
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":97780,"parent_record_offset":97700,"ancestor_record_offset":95832,"inlinee":18805,"source_line":145,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::from_vec
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":97748,"parent_record_offset":97724,"ancestor_record_offset":95832,"inlinee":18804,"source_line":50,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":97724,"parent_record_offset":97700,"ancestor_record_offset":95832,"inlinee":18803,"source_line":32,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::Column::with_children
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":97700,"parent_record_offset":97676,"ancestor_record_offset":95832,"inlinee":18802,"source_line":37,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/column.rs
//     evidence={"module":263,"record_offset":97912,"parent_record_offset":97876,"ancestor_record_offset":95832,"inlinee":18756,"source_line":333,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/container.rs
//     evidence={"module":263,"record_offset":99864,"parent_record_offset":99828,"ancestor_record_offset":95832,"inlinee":18811,"source_line":313,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::helpers::button
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":263,"record_offset":104224,"parent_record_offset":103480,"ancestor_record_offset":95832,"inlinee":18710,"source_line":116,"archive":"_pdb/C/modules/00263.jsonl.gz"}
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
// 首条位置: {"module":263,"record_offset":95004,"rva":7046672,"va":5375755792}
// 编译实例 RVAs: 0x6b8610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40865500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375755792,"raw_hex":"e8dc8e6f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5375756257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":263,"record_offset":95392,"parent_record_offset":95004,"ancestor_record_offset":95004,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":263,"record_offset":95244,"parent_record_offset":95004,"ancestor_record_offset":95004,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":263,"record_offset":95180,"parent_record_offset":95004,"ancestor_record_offset":95004,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::str::pattern::impl$36::matching
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":263,"record_offset":95564,"parent_record_offset":95004,"ancestor_record_offset":95004,"inlinee":9307,"source_line":1740,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":263,"record_offset":95300,"parent_record_offset":95272,"ancestor_record_offset":95004,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":263,"record_offset":95496,"parent_record_offset":95440,"ancestor_record_offset":95004,"inlinee":10953,"source_line":217,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":263,"record_offset":95328,"parent_record_offset":95272,"ancestor_record_offset":95004,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":263,"record_offset":95272,"parent_record_offset":95244,"ancestor_record_offset":95004,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":263,"record_offset":95440,"parent_record_offset":95416,"ancestor_record_offset":95004,"inlinee":9305,"source_line":813,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":263,"record_offset":95416,"parent_record_offset":95392,"ancestor_record_offset":95004,"inlinee":9304,"source_line":978,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":263,"record_offset":95352,"parent_record_offset":95328,"ancestor_record_offset":95004,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":263,"record_offset":95520,"parent_record_offset":95496,"ancestor_record_offset":95004,"inlinee":10954,"source_line":1011,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":263,"record_offset":95208,"parent_record_offset":95180,"ancestor_record_offset":95004,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::run_crash_reporter_gui<tiny_glade::mock_crash_reporter::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":211,"record_offset":10276,"rva":5869200,"va":5374578320}
// 编译实例 RVAs: 0x598e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t42035166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8de678102","size":5,"soft_fail":false,"target_va":5416616304,"va":5374581133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1707443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8b30d1a00","size":5,"soft_fail":false,"target_va":5376286320,"va":5374578872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1099999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8dfc81000","size":5,"soft_fail":false,"target_va":5375678688,"va":5374578684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1100108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e84cc91000","size":5,"soft_fail":false,"target_va":5375678704,"va":5374578591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5856482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e81ea3a6ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374578669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008c0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5298707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8ed25afff","size":5,"soft_fail":false,"target_va":5369282736,"va":5374581438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5240824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e80808b0ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374579667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a040
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5241996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e87403b0ff","size":5,"soft_fail":false,"target_va":5369339968,"va":5374581959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009da40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5227238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e81a3db0ff","size":5,"soft_fail":false,"target_va":5369354816,"va":5374582049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e470
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5224467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8ed47b0ff","size":5,"soft_fail":false,"target_va":5369357424,"va":5374581886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a28c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5206819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8dd8cb0ff","size":5,"soft_fail":false,"target_va":5369374912,"va":5374581726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5206494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8228eb0ff","size":5,"soft_fail":false,"target_va":5369376144,"va":5374582633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400abbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5170024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8981cb1ff","size":5,"soft_fail":false,"target_va":5369412560,"va":5374582579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b1e10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5143187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e86d85b1ff","size":5,"soft_fail":false,"target_va":5369437712,"va":5374580894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140530f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-428337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8cf76f9ff","size":5,"soft_fail":false,"target_va":5374152512,"va":5374580844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42478153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8492a8802","size":5,"soft_fail":false,"target_va":5417057056,"va":5374578898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42039803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8fb798102","size":5,"soft_fail":false,"target_va":5416620769,"va":5374580961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42041766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8a6818102","size":5,"soft_fail":false,"target_va":5416620794,"va":5374579023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10043201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8413f9900","size":5,"soft_fail":false,"target_va":5384624080,"va":5374580874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e80e1a0000","size":5,"soft_fail":false,"target_va":5374589296,"va":5374582621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42040603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e81b7d8102","size":5,"soft_fail":false,"target_va":5416621632,"va":5374581024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_reporter_gui::Controls::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1123597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e80d251100","size":5,"soft_fail":false,"target_va":5375704144,"va":5374580542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8555715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8c38c8200","size":5,"soft_fail":false,"target_va":5383134720,"va":5374579000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42038502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8e6748102","size":5,"soft_fail":false,"target_va":5416619696,"va":5374581189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1643346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8aeece6ff","size":5,"soft_fail":false,"target_va":5372935104,"va":5374578445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1637461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8ab03e7ff","size":5,"soft_fail":false,"target_va":5372942816,"va":5374580272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1740899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e863901a00","size":5,"soft_fail":false,"target_va":5376320352,"va":5374579448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::primary_monitor
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1740916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e874901a00","size":5,"soft_fail":false,"target_va":5376320336,"va":5374579415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::is_minimized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1739778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8028c1a00","size":5,"soft_fail":false,"target_va":5376319952,"va":5374580169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1728938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e8aa611a00","size":5,"soft_fail":false,"target_va":5376308960,"va":5374580017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::force_window_active
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1728334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e84e5f1a00","size":5,"soft_fail":false,"target_va":5376308560,"va":5374580221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::WindowBuilder::build<tuple$<> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t10546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374578320,"raw_hex":"e832290000","size":5,"soft_fail":false,"target_va":5374590496,"va":5374579945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5374578320,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5374578561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":211,"record_offset":16184,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":211,"record_offset":13288,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":211,"record_offset":13568,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":13037,"source_line":2344,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":211,"record_offset":16464,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":14716,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":11368,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::impl$18::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":14064,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":13621,"source_line":302,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":10528,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":13712,"source_line":200,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":13100,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":13614,"source_line":329,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":14660,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":13624,"source_line":305,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":211,"record_offset":13624,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":13617,"source_line":125,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":211,"record_offset":13156,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":13616,"source_line":118,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::focus_window
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":15356,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":16521,"source_line":1261,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::request_user_attention
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":16064,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":16525,"source_line":1289,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":14788,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":12848,"source_line":568,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":11468,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":13604,"source_line":205,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":13044,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":13610,"source_line":222,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":14316,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":13622,"source_line":274,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":12068,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":13609,"source_line":307,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":13072,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":15157,"source_line":340,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":11876,"parent_record_offset":11852,"ancestor_record_offset":10276,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":11900,"parent_record_offset":11876,"ancestor_record_offset":10276,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":11008,"parent_record_offset":10984,"ancestor_record_offset":10276,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":11032,"parent_record_offset":11008,"ancestor_record_offset":10276,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":11924,"parent_record_offset":11900,"ancestor_record_offset":10276,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":11056,"parent_record_offset":11032,"ancestor_record_offset":10276,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":11852,"parent_record_offset":11824,"ancestor_record_offset":10276,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":10984,"parent_record_offset":10852,"ancestor_record_offset":10276,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":211,"record_offset":16268,"parent_record_offset":16220,"ancestor_record_offset":10276,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":211,"record_offset":16220,"parent_record_offset":16184,"ancestor_record_offset":10276,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":211,"record_offset":10852,"parent_record_offset":10824,"ancestor_record_offset":10276,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":11764,"parent_record_offset":11736,"ancestor_record_offset":10276,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":12876,"parent_record_offset":12840,"ancestor_record_offset":10276,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":12840,"parent_record_offset":12812,"ancestor_record_offset":10276,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":11824,"parent_record_offset":11792,"ancestor_record_offset":10276,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":11792,"parent_record_offset":11764,"ancestor_record_offset":10276,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":12812,"parent_record_offset":12784,"ancestor_record_offset":10276,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":12404,"parent_record_offset":12364,"ancestor_record_offset":10276,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":211,"record_offset":11644,"parent_record_offset":11616,"ancestor_record_offset":10276,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":211,"record_offset":11672,"parent_record_offset":11644,"ancestor_record_offset":10276,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":211,"record_offset":11700,"parent_record_offset":11672,"ancestor_record_offset":10276,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":211,"record_offset":11616,"parent_record_offset":11588,"ancestor_record_offset":10276,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":211,"record_offset":11588,"parent_record_offset":11552,"ancestor_record_offset":10276,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":211,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":10276,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":211,"record_offset":11736,"parent_record_offset":11700,"ancestor_record_offset":10276,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":211,"record_offset":10884,"parent_record_offset":10852,"ancestor_record_offset":10276,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":211,"record_offset":12428,"parent_record_offset":12404,"ancestor_record_offset":10276,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":211,"record_offset":12112,"parent_record_offset":12068,"ancestor_record_offset":10276,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$106::from
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":211,"record_offset":13876,"parent_record_offset":13852,"ancestor_record_offset":10276,"inlinee":12892,"source_line":79,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":211,"record_offset":13384,"parent_record_offset":13360,"ancestor_record_offset":10276,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":211,"record_offset":12476,"parent_record_offset":12452,"ancestor_record_offset":10276,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":211,"record_offset":10948,"parent_record_offset":10912,"ancestor_record_offset":10276,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":211,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":10276,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":211,"record_offset":12452,"parent_record_offset":12428,"ancestor_record_offset":10276,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":211,"record_offset":11980,"parent_record_offset":11700,"ancestor_record_offset":10276,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":211,"record_offset":12004,"parent_record_offset":11980,"ancestor_record_offset":10276,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":211,"record_offset":10732,"parent_record_offset":10528,"ancestor_record_offset":10276,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":13680,"parent_record_offset":13648,"ancestor_record_offset":10276,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":13236,"parent_record_offset":13192,"ancestor_record_offset":10276,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":13712,"parent_record_offset":13648,"ancestor_record_offset":10276,"inlinee":8040,"source_line":1590,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":10700,"parent_record_offset":10616,"ancestor_record_offset":10276,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":15180,"parent_record_offset":14992,"ancestor_record_offset":10276,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":10644,"parent_record_offset":10616,"ancestor_record_offset":10276,"inlinee":4686,"source_line":788,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":15272,"parent_record_offset":15240,"ancestor_record_offset":10276,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":15112,"parent_record_offset":15080,"ancestor_record_offset":10276,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":15136,"parent_record_offset":15112,"ancestor_record_offset":10276,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":10276,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":10276,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":211,"record_offset":15080,"parent_record_offset":15052,"ancestor_record_offset":10276,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":211,"record_offset":15052,"parent_record_offset":15024,"ancestor_record_offset":10276,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":211,"record_offset":14992,"parent_record_offset":14952,"ancestor_record_offset":10276,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":211,"record_offset":14952,"parent_record_offset":14920,"ancestor_record_offset":10276,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":211,"record_offset":15240,"parent_record_offset":14952,"ancestor_record_offset":10276,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":211,"record_offset":15024,"parent_record_offset":14992,"ancestor_record_offset":10276,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":14392,"parent_record_offset":14368,"ancestor_record_offset":10276,"inlinee":12883,"source_line":281,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":13336,"parent_record_offset":13312,"ancestor_record_offset":10276,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$19::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":13800,"parent_record_offset":13776,"ancestor_record_offset":10276,"inlinee":13619,"source_line":308,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":13416,"parent_record_offset":13336,"ancestor_record_offset":10276,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":13312,"parent_record_offset":13288,"ancestor_record_offset":10276,"inlinee":10979,"source_line":466,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":14368,"parent_record_offset":14344,"ancestor_record_offset":10276,"inlinee":12881,"source_line":573,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$5::from_f64
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":13908,"parent_record_offset":13824,"ancestor_record_offset":10276,"inlinee":12885,"source_line":140,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":10616,"parent_record_offset":10528,"ancestor_record_offset":10276,"inlinee":13716,"source_line":120,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":10560,"parent_record_offset":10528,"ancestor_record_offset":10276,"inlinee":13714,"source_line":69,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":10584,"parent_record_offset":10560,"ancestor_record_offset":10276,"inlinee":13715,"source_line":80,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoop::window_target
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":211,"record_offset":14684,"parent_record_offset":14660,"ancestor_record_offset":10276,"inlinee":13625,"source_line":210,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":211,"record_offset":13124,"parent_record_offset":13100,"ancestor_record_offset":10276,"inlinee":13615,"source_line":494,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":211,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":10276,"inlinee":13354,"source_line":211,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":211,"record_offset":13192,"parent_record_offset":13156,"ancestor_record_offset":10276,"inlinee":13355,"source_line":185,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::focus_window
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":15460,"parent_record_offset":15428,"ancestor_record_offset":10276,"inlinee":16524,"source_line":1149,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":15956,"parent_record_offset":15460,"ancestor_record_offset":10276,"inlinee":12895,"source_line":348,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":15392,"parent_record_offset":15356,"ancestor_record_offset":10276,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":14820,"parent_record_offset":14788,"ancestor_record_offset":10276,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_user_attention
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":16136,"parent_record_offset":16112,"ancestor_record_offset":10276,"inlinee":16528,"source_line":1080,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":14884,"parent_record_offset":14852,"ancestor_record_offset":10276,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::WindowFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":211,"record_offset":15984,"parent_record_offset":15460,"ancestor_record_offset":10276,"inlinee":12956,"source_line":696,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::WindowState::window_flags
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window_state.rs
//     evidence={"module":211,"record_offset":15508,"parent_record_offset":15460,"ancestor_record_offset":10276,"inlinee":12916,"source_line":180,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":211,"record_offset":16012,"parent_record_offset":15984,"ancestor_record_offset":10276,"inlinee":12930,"source_line":696,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":14852,"parent_record_offset":14820,"ancestor_record_offset":10276,"inlinee":12850,"source_line":569,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::focus_window::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":15428,"parent_record_offset":15392,"ancestor_record_offset":10276,"inlinee":16523,"source_line":1262,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::request_user_attention::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":16112,"parent_record_offset":16088,"ancestor_record_offset":10276,"inlinee":16527,"source_line":1291,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$31::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":11504,"parent_record_offset":11468,"ancestor_record_offset":10276,"inlinee":13606,"source_line":116,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":11552,"parent_record_offset":11504,"ancestor_record_offset":10276,"inlinee":13608,"source_line":162,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7>
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":211,"record_offset":16948,"rva":5873712,"va":5374582832}
// 编译实例 RVAs: 0x59a030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t42030540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8cc558102","size":5,"soft_fail":false,"target_va":5416616304,"va":5374585759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1702952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e828fc1900","size":5,"soft_fail":false,"target_va":5376286320,"va":5374583363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1095508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e854b71000","size":5,"soft_fail":false,"target_va":5375678688,"va":5374583175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t1095617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8c1b71000","size":5,"soft_fail":false,"target_va":5375678704,"va":5374583082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5860973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e89391a6ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374583160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008c0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5303354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8c613afff","size":5,"soft_fail":false,"target_va":5369282736,"va":5374586085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5245277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8a3f6afff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374584120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a040
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5246924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e834f0afff","size":5,"soft_fail":false,"target_va":5369339968,"va":5374586887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009da40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5232163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8dd29b0ff","size":5,"soft_fail":false,"target_va":5369354816,"va":5374586974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e470
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5229328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8f034b0ff","size":5,"soft_fail":false,"target_va":5369357424,"va":5374586747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a28c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5211658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8f679b0ff","size":5,"soft_fail":false,"target_va":5369374912,"va":5374586565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2d90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5210261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e86b7fb0ff","size":5,"soft_fail":false,"target_va":5369376144,"va":5374586400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400abbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5173755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8050eb1ff","size":5,"soft_fail":false,"target_va":5369412560,"va":5374586310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b1e10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5147797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e86b73b1ff","size":5,"soft_fail":false,"target_va":5369437712,"va":5374585504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140530f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-432927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8e164f9ff","size":5,"soft_fail":false,"target_va":5374152512,"va":5374585434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140545030
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-351679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e841a2faff","size":5,"soft_fail":false,"target_va":5374234672,"va":5374586346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42473665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8c1188802","size":5,"soft_fail":false,"target_va":5417057056,"va":5374583386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42035196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8fc678102","size":5,"soft_fail":false,"target_va":5416620769,"va":5374585568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42037278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e81e708102","size":5,"soft_fail":false,"target_va":5416620794,"va":5374583511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10038598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8462d9900","size":5,"soft_fail":false,"target_va":5384624080,"va":5374585477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8570b0000","size":5,"soft_fail":false,"target_va":5374589296,"va":5374586388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42035978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e80a6b8102","size":5,"soft_fail":false,"target_va":5416621632,"va":5374585649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_reporter_gui::Controls::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1119007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e81f131100","size":5,"soft_fail":false,"target_va":5375704144,"va":5374585132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8551227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e83b7b8200","size":5,"soft_fail":false,"target_va":5383134720,"va":5374583488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42033862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8c6628102","size":5,"soft_fail":false,"target_va":5416619696,"va":5374585829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1647855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e811dbe6ff","size":5,"soft_fail":false,"target_va":5372935104,"va":5374582954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1641938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e82ef2e6ff","size":5,"soft_fail":false,"target_va":5372942816,"va":5374584749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1736436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8f47e1a00","size":5,"soft_fail":false,"target_va":5376320352,"va":5374583911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::primary_monitor
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1736457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8097f1a00","size":5,"soft_fail":false,"target_va":5376320336,"va":5374583874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::is_minimized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1735315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8937a1a00","size":5,"soft_fail":false,"target_va":5376319952,"va":5374584632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1724489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e849501a00","size":5,"soft_fail":false,"target_va":5376308960,"va":5374584466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::force_window_active
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1723864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8d84d1a00","size":5,"soft_fail":false,"target_va":5376308560,"va":5374584691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::WindowBuilder::build<tuple$<> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t6095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374582832,"raw_hex":"e8cf170000","size":5,"soft_fail":false,"target_va":5374590496,"va":5374584396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5374582832,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5374583052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":211,"record_offset":22880,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":211,"record_offset":23092,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":211,"record_offset":19984,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":211,"record_offset":20264,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":13037,"source_line":2344,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":211,"record_offset":23372,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":21412,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":18068,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::impl$18::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":20760,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":13621,"source_line":302,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":17228,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":13712,"source_line":200,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":19796,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":13614,"source_line":329,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":21356,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":13624,"source_line":305,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":211,"record_offset":20320,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":13617,"source_line":125,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":211,"record_offset":19852,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":13616,"source_line":118,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::focus_window
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":22052,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":16521,"source_line":1261,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::request_user_attention
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":22760,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":16525,"source_line":1289,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":21484,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":12848,"source_line":568,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":18168,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":13604,"source_line":205,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":19740,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":13610,"source_line":222,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":21012,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":13622,"source_line":274,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":18768,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":13609,"source_line":307,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":19768,"parent_record_offset":16948,"ancestor_record_offset":16948,"inlinee":15157,"source_line":340,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":18576,"parent_record_offset":18552,"ancestor_record_offset":16948,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":18600,"parent_record_offset":18576,"ancestor_record_offset":16948,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":17708,"parent_record_offset":17684,"ancestor_record_offset":16948,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":17732,"parent_record_offset":17708,"ancestor_record_offset":16948,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":18624,"parent_record_offset":18600,"ancestor_record_offset":16948,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":17756,"parent_record_offset":17732,"ancestor_record_offset":16948,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":18552,"parent_record_offset":18524,"ancestor_record_offset":16948,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":211,"record_offset":17684,"parent_record_offset":17552,"ancestor_record_offset":16948,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":211,"record_offset":22928,"parent_record_offset":22880,"ancestor_record_offset":16948,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":211,"record_offset":23128,"parent_record_offset":23092,"ancestor_record_offset":16948,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":211,"record_offset":17552,"parent_record_offset":17524,"ancestor_record_offset":16948,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":18464,"parent_record_offset":18436,"ancestor_record_offset":16948,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":19572,"parent_record_offset":19540,"ancestor_record_offset":16948,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":19540,"parent_record_offset":19512,"ancestor_record_offset":16948,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":18524,"parent_record_offset":18492,"ancestor_record_offset":16948,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":18492,"parent_record_offset":18464,"ancestor_record_offset":16948,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":19512,"parent_record_offset":19484,"ancestor_record_offset":16948,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":211,"record_offset":19104,"parent_record_offset":19064,"ancestor_record_offset":16948,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":211,"record_offset":18344,"parent_record_offset":18316,"ancestor_record_offset":16948,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":211,"record_offset":18372,"parent_record_offset":18344,"ancestor_record_offset":16948,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":211,"record_offset":18400,"parent_record_offset":18372,"ancestor_record_offset":16948,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":211,"record_offset":18316,"parent_record_offset":18288,"ancestor_record_offset":16948,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":211,"record_offset":18288,"parent_record_offset":18252,"ancestor_record_offset":16948,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":211,"record_offset":18836,"parent_record_offset":18812,"ancestor_record_offset":16948,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":211,"record_offset":18436,"parent_record_offset":18400,"ancestor_record_offset":16948,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":211,"record_offset":17584,"parent_record_offset":17552,"ancestor_record_offset":16948,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":211,"record_offset":19128,"parent_record_offset":19104,"ancestor_record_offset":16948,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":211,"record_offset":18812,"parent_record_offset":18768,"ancestor_record_offset":16948,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$106::from
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":211,"record_offset":20572,"parent_record_offset":20548,"ancestor_record_offset":16948,"inlinee":12892,"source_line":79,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":211,"record_offset":20080,"parent_record_offset":20056,"ancestor_record_offset":16948,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":211,"record_offset":19176,"parent_record_offset":19152,"ancestor_record_offset":16948,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":211,"record_offset":17648,"parent_record_offset":17612,"ancestor_record_offset":16948,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":211,"record_offset":17612,"parent_record_offset":17584,"ancestor_record_offset":16948,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":211,"record_offset":19152,"parent_record_offset":19128,"ancestor_record_offset":16948,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":211,"record_offset":18680,"parent_record_offset":18400,"ancestor_record_offset":16948,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":211,"record_offset":18704,"parent_record_offset":18680,"ancestor_record_offset":16948,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":211,"record_offset":17432,"parent_record_offset":17228,"ancestor_record_offset":16948,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":20376,"parent_record_offset":20344,"ancestor_record_offset":16948,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":19932,"parent_record_offset":19888,"ancestor_record_offset":16948,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":20408,"parent_record_offset":20344,"ancestor_record_offset":16948,"inlinee":8040,"source_line":1590,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":211,"record_offset":17400,"parent_record_offset":17316,"ancestor_record_offset":16948,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":21876,"parent_record_offset":21688,"ancestor_record_offset":16948,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":17344,"parent_record_offset":17316,"ancestor_record_offset":16948,"inlinee":4686,"source_line":788,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":21968,"parent_record_offset":21936,"ancestor_record_offset":16948,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":21808,"parent_record_offset":21776,"ancestor_record_offset":16948,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":21832,"parent_record_offset":21808,"ancestor_record_offset":16948,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":21900,"parent_record_offset":21876,"ancestor_record_offset":16948,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":211,"record_offset":17368,"parent_record_offset":17344,"ancestor_record_offset":16948,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":211,"record_offset":21776,"parent_record_offset":21748,"ancestor_record_offset":16948,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":211,"record_offset":21748,"parent_record_offset":21720,"ancestor_record_offset":16948,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":211,"record_offset":21688,"parent_record_offset":21648,"ancestor_record_offset":16948,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":211,"record_offset":21648,"parent_record_offset":21616,"ancestor_record_offset":16948,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":211,"record_offset":21936,"parent_record_offset":21648,"ancestor_record_offset":16948,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":211,"record_offset":21720,"parent_record_offset":21688,"ancestor_record_offset":16948,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":21088,"parent_record_offset":21064,"ancestor_record_offset":16948,"inlinee":12883,"source_line":281,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":20032,"parent_record_offset":20008,"ancestor_record_offset":16948,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$19::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":20496,"parent_record_offset":20472,"ancestor_record_offset":16948,"inlinee":13619,"source_line":308,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":20112,"parent_record_offset":20032,"ancestor_record_offset":16948,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":20008,"parent_record_offset":19984,"ancestor_record_offset":16948,"inlinee":10979,"source_line":466,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":21064,"parent_record_offset":21040,"ancestor_record_offset":16948,"inlinee":12881,"source_line":573,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$5::from_f64
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":211,"record_offset":20604,"parent_record_offset":20520,"ancestor_record_offset":16948,"inlinee":12885,"source_line":140,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":17316,"parent_record_offset":17228,"ancestor_record_offset":16948,"inlinee":13716,"source_line":120,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":17260,"parent_record_offset":17228,"ancestor_record_offset":16948,"inlinee":13714,"source_line":69,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":211,"record_offset":17284,"parent_record_offset":17260,"ancestor_record_offset":16948,"inlinee":13715,"source_line":80,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoop::window_target
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":211,"record_offset":21380,"parent_record_offset":21356,"ancestor_record_offset":16948,"inlinee":13625,"source_line":210,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":211,"record_offset":19820,"parent_record_offset":19796,"ancestor_record_offset":16948,"inlinee":13615,"source_line":494,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":211,"record_offset":20344,"parent_record_offset":20320,"ancestor_record_offset":16948,"inlinee":13354,"source_line":211,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":211,"record_offset":19888,"parent_record_offset":19852,"ancestor_record_offset":16948,"inlinee":13355,"source_line":185,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::focus_window
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":22156,"parent_record_offset":22124,"ancestor_record_offset":16948,"inlinee":16524,"source_line":1149,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":22652,"parent_record_offset":22156,"ancestor_record_offset":16948,"inlinee":12895,"source_line":348,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":22088,"parent_record_offset":22052,"ancestor_record_offset":16948,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":21516,"parent_record_offset":21484,"ancestor_record_offset":16948,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_user_attention
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":22832,"parent_record_offset":22808,"ancestor_record_offset":16948,"inlinee":16528,"source_line":1080,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":211,"record_offset":21580,"parent_record_offset":21548,"ancestor_record_offset":16948,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::WindowFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":211,"record_offset":22680,"parent_record_offset":22156,"ancestor_record_offset":16948,"inlinee":12956,"source_line":696,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::WindowState::window_flags
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window_state.rs
//     evidence={"module":211,"record_offset":22204,"parent_record_offset":22156,"ancestor_record_offset":16948,"inlinee":12916,"source_line":180,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window_state::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":211,"record_offset":22708,"parent_record_offset":22680,"ancestor_record_offset":16948,"inlinee":12930,"source_line":696,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":21548,"parent_record_offset":21516,"ancestor_record_offset":16948,"inlinee":12850,"source_line":569,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::focus_window::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":22124,"parent_record_offset":22088,"ancestor_record_offset":16948,"inlinee":16523,"source_line":1262,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::request_user_attention::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":22808,"parent_record_offset":22784,"ancestor_record_offset":16948,"inlinee":16527,"source_line":1291,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$31::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":18204,"parent_record_offset":18168,"ancestor_record_offset":16948,"inlinee":13606,"source_line":116,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":211,"record_offset":18252,"parent_record_offset":18204,"ancestor_record_offset":16948,"inlinee":13608,"source_line":162,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::external_crashing_module
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4926
// 首条位置: {"module":263,"inlinee":18726}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$5::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":263,"record_offset":8584,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":18731,"source_line":591,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":8556,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":12020,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":9368,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":8326,"source_line":1430,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::rsplit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":8276,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":18727,"source_line":1751,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":9872,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$64::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":263,"record_offset":8528,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":18730,"source_line":550,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":263,"record_offset":10772,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":8992,"parent_record_offset":8968,"ancestor_record_offset":8124,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":9016,"parent_record_offset":8992,"ancestor_record_offset":8124,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":8124,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":12244,"parent_record_offset":12220,"ancestor_record_offset":8124,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":9040,"parent_record_offset":9016,"ancestor_record_offset":8124,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":8124,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":8968,"parent_record_offset":8816,"ancestor_record_offset":8124,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":12196,"parent_record_offset":12132,"ancestor_record_offset":8124,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":8760,"parent_record_offset":8736,"ancestor_record_offset":8124,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":12164,"parent_record_offset":12132,"ancestor_record_offset":8124,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":12132,"parent_record_offset":12104,"ancestor_record_offset":8124,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":8816,"parent_record_offset":8784,"ancestor_record_offset":8124,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":8124,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":12104,"parent_record_offset":12076,"ancestor_record_offset":8124,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":8856,"parent_record_offset":8816,"ancestor_record_offset":8124,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":8124,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":8684,"parent_record_offset":8660,"ancestor_record_offset":8124,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":8124,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":8124,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":263,"record_offset":8612,"parent_record_offset":8584,"ancestor_record_offset":8124,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":8736,"parent_record_offset":8708,"ancestor_record_offset":8124,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":8880,"parent_record_offset":8856,"ancestor_record_offset":8124,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":263,"record_offset":9440,"parent_record_offset":9416,"ancestor_record_offset":8124,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":263,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":8124,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":263,"record_offset":8904,"parent_record_offset":8880,"ancestor_record_offset":8124,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_uppercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":263,"record_offset":9312,"parent_record_offset":9280,"ancestor_record_offset":8124,"inlinee":18736,"source_line":816,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::make_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":263,"record_offset":9248,"parent_record_offset":9196,"ancestor_record_offset":8124,"inlinee":18734,"source_line":746,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":263,"record_offset":9280,"parent_record_offset":9248,"ancestor_record_offset":8124,"inlinee":18735,"source_line":668,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":263,"record_offset":9096,"parent_record_offset":8708,"ancestor_record_offset":8124,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":8124,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":12048,"parent_record_offset":12020,"ancestor_record_offset":8124,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":263,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":8124,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::make_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":263,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":8124,"inlinee":18733,"source_line":195,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":263,"record_offset":9464,"parent_record_offset":9440,"ancestor_record_offset":8124,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":263,"record_offset":9488,"parent_record_offset":9464,"ancestor_record_offset":8124,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":263,"record_offset":9416,"parent_record_offset":9392,"ancestor_record_offset":8124,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":263,"record_offset":8420,"parent_record_offset":8396,"ancestor_record_offset":8124,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":263,"record_offset":9928,"parent_record_offset":9900,"ancestor_record_offset":8124,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":263,"record_offset":8444,"parent_record_offset":8420,"ancestor_record_offset":8124,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":8124,"inlinee":8292,"source_line":1123,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":8396,"parent_record_offset":8372,"ancestor_record_offset":8124,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::make_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":9172,"parent_record_offset":8584,"ancestor_record_offset":8124,"inlinee":18732,"source_line":2894,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::split
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":263,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":8124,"inlinee":18728,"source_line":1655,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$13::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":263,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":8124,"inlinee":16754,"source_line":736,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":263,"record_offset":9900,"parent_record_offset":9872,"ancestor_record_offset":8124,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":263,"record_offset":9392,"parent_record_offset":9368,"ancestor_record_offset":8124,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$9::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":263,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":8124,"inlinee":8291,"source_line":677,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x40A4
// 首条位置: {"module":240,"inlinee":16548}
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
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":211,"record_offset":9016,"rva":5868752,"va":5374577872}
// 编译实例 RVAs: 0x598cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t41977487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374577872,"raw_hex":"e88f868002","size":5,"soft_fail":false,"target_va":5416555792,"va":5374578300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":211,"record_offset":9192,"parent_record_offset":9016,"ancestor_record_offset":9016,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":211,"record_offset":9220,"parent_record_offset":9192,"ancestor_record_offset":9016,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":211,"record_offset":9248,"parent_record_offset":9220,"ancestor_record_offset":9016,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":211,"record_offset":9276,"parent_record_offset":9248,"ancestor_record_offset":9016,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00211.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x26CE
// 首条位置: {"module":64,"inlinee":9934}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":64,"record_offset":22364,"parent_record_offset":22340,"ancestor_record_offset":22340,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":64,"record_offset":22400,"parent_record_offset":22364,"ancestor_record_offset":22340,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":64,"record_offset":22464,"parent_record_offset":22364,"ancestor_record_offset":22340,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":64,"record_offset":22436,"parent_record_offset":22364,"ancestor_record_offset":22340,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2758
// 首条位置: {"module":64,"inlinee":10072}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":64,"record_offset":109128,"parent_record_offset":109076,"ancestor_record_offset":109076,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":64,"record_offset":109100,"parent_record_offset":109076,"ancestor_record_offset":109076,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/00064.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$11::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFA6F
// 首条位置: {"module":928,"inlinee":64111}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":928,"record_offset":12172,"parent_record_offset":12104,"ancestor_record_offset":12104,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00928.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":928,"record_offset":12244,"parent_record_offset":12220,"ancestor_record_offset":12104,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00928.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":928,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":12104,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00928.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":928,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":12104,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00928.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$0::program_outcome
// 记录形式: compiled_procedure; instances=1
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":263,"record_offset":95712,"rva":7047216,"va":5375756336}
// 编译实例 RVAs: 0x6b8830
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
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":263,"record_offset":95596,"rva":7047184,"va":5375756304}
// 编译实例 RVAs: 0x6b8810
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
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":263,"record_offset":4320,"rva":6995024,"va":5375704144}
// 编译实例 RVAs: 0x6abc50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-25686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375704144,"raw_hex":"e8aa9bffff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375704385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140083cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=131 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6457920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375704144,"raw_hex":"e8c0759dff","size":5,"soft_fail":false,"target_va":5369248944,"va":5375706859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6365577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375704144,"raw_hex":"e877de9eff","size":5,"soft_fail":false,"target_va":5369338848,"va":5375704420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6330737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375704144,"raw_hex":"e88f669fff","size":5,"soft_fail":false,"target_va":5369376144,"va":5375706876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_integrity_report..controls..Controls$GT$
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t9439634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375704144,"raw_hex":"e892099000","size":5,"soft_fail":false,"target_va":5385145616,"va":5375705977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::contains<ref$<str$> >
//     owner=rust_standard_library; count=20
//     evidence={"assembly":"\tcallq\t49960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375704144,"raw_hex":"e828c30000","size":5,"soft_fail":false,"target_va":5375754448,"va":5375704483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9046899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375704144,"raw_hex":"e8730b8a00","size":5,"soft_fail":false,"target_va":5384751264,"va":5375704360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::controls::Controls::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9458147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375704144,"raw_hex":"e8e3519000","size":5,"soft_fail":false,"target_va":5385162432,"va":5375704280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":263,"record_offset":5800,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":4456,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":6728,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":263,"record_offset":5364,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":263,"record_offset":5448,"parent_record_offset":4320,"ancestor_record_offset":4320,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4320,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":4824,"parent_record_offset":4800,"ancestor_record_offset":4320,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":4320,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":4776,"parent_record_offset":4720,"ancestor_record_offset":4320,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":4320,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":4320,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":4320,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":263,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":4320,"inlinee":7942,"source_line":78,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::try_fold::check::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":263,"record_offset":5972,"parent_record_offset":5848,"ancestor_record_offset":4320,"inlinee":6682,"source_line":87,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":263,"record_offset":6000,"parent_record_offset":5972,"ancestor_record_offset":4320,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":263,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":4320,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":4552,"parent_record_offset":4520,"ancestor_record_offset":4320,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":263,"record_offset":5412,"parent_record_offset":5388,"ancestor_record_offset":4320,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":263,"record_offset":5736,"parent_record_offset":5676,"ancestor_record_offset":4320,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":263,"record_offset":5524,"parent_record_offset":5488,"ancestor_record_offset":4320,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":263,"record_offset":5292,"parent_record_offset":4492,"ancestor_record_offset":4320,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":263,"record_offset":5388,"parent_record_offset":5364,"ancestor_record_offset":4320,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":263,"record_offset":5488,"parent_record_offset":5448,"ancestor_record_offset":4320,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":4492,"parent_record_offset":4456,"ancestor_record_offset":4320,"inlinee":18719,"source_line":227,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":5568,"parent_record_offset":5448,"ancestor_record_offset":4320,"inlinee":18720,"source_line":235,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":5768,"parent_record_offset":5644,"ancestor_record_offset":4320,"inlinee":18721,"source_line":253,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":4320,"inlinee":18722,"source_line":262,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":4320,"inlinee":18723,"source_line":269,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":4320,"inlinee":18724,"source_line":276,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$1::new::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":6696,"parent_record_offset":6572,"ancestor_record_offset":4320,"inlinee":18725,"source_line":287,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":263,"record_offset":4520,"parent_record_offset":4492,"ancestor_record_offset":4320,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$1::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x491F
// 首条位置: {"module":263,"inlinee":18719}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":263,"record_offset":5292,"parent_record_offset":4492,"ancestor_record_offset":4492,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":263,"record_offset":4520,"parent_record_offset":4492,"ancestor_record_offset":4492,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4492,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":4824,"parent_record_offset":4800,"ancestor_record_offset":4492,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":4492,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":4776,"parent_record_offset":4720,"ancestor_record_offset":4492,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":4492,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":4492,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":4492,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":4552,"parent_record_offset":4520,"ancestor_record_offset":4492,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$1::new::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4920
// 首条位置: {"module":263,"inlinee":18720}
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
// 源码行: 253
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4921
// 首条位置: {"module":263,"inlinee":18721}
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
// 源码行: 262
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4922
// 首条位置: {"module":263,"inlinee":18722}
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
// 源码行: 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4923
// 首条位置: {"module":263,"inlinee":18723}
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
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4924
// 首条位置: {"module":263,"inlinee":18724}
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
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4925
// 首条位置: {"module":263,"inlinee":18725}
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
// 记录形式: inline_instance, inlinee_declaration; instances=40
// 源码行: 316
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4912
// 首条位置: {"module":263,"inlinee":18706}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":1592,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":2260,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":263,"record_offset":2232,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":18708,"source_line":113,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":1552,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1552,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":1552,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":1552,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":1552,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":1552,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2016,"parent_record_offset":1868,"ancestor_record_offset":1552,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2412,"parent_record_offset":2356,"ancestor_record_offset":1552,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":1552,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":2384,"parent_record_offset":2356,"ancestor_record_offset":1552,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":1552,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":1868,"parent_record_offset":1836,"ancestor_record_offset":1552,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1552,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":1552,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":1904,"parent_record_offset":1868,"ancestor_record_offset":1552,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":1552,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":1552,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":1552,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1552,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":263,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":1552,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":102448,"parent_record_offset":102396,"ancestor_record_offset":102324,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$114::spec_to_string
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":1616,"parent_record_offset":1592,"ancestor_record_offset":1552,"inlinee":10264,"source_line":3043,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":17556,"parent_record_offset":17520,"ancestor_record_offset":17484,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":102528,"parent_record_offset":102504,"ancestor_record_offset":102324,"inlinee":4606,"source_line":2711,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":102396,"parent_record_offset":102360,"ancestor_record_offset":102324,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":1552,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":1552,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":1552,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":263,"record_offset":102476,"parent_record_offset":102396,"ancestor_record_offset":102324,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$82::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":263,"record_offset":102504,"parent_record_offset":102396,"ancestor_record_offset":102324,"inlinee":6317,"source_line":2865,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":263,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":1552,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":263,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":1552,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":263,"record_offset":2144,"parent_record_offset":1760,"ancestor_record_offset":1552,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1552,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":1552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":263,"record_offset":102560,"parent_record_offset":102396,"ancestor_record_offset":102324,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::text<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 316
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":927,"record_offset":2276,"rva":16423856,"va":5385132976}
// 编译实例 RVAs: 0xfa9bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9454353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385132976,"raw_hex":"e8efbc6fff","size":5,"soft_fail":false,"target_va":5375678688,"va":5385133036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16410830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385132976,"raw_hex":"e8329705ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5385133017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31923993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385132976,"raw_hex":"e8191fe701","size":5,"soft_fail":false,"target_va":5417057056,"va":5385133058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31487785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385132976,"raw_hex":"e82977e001","size":5,"soft_fail":false,"target_va":5416620794,"va":5385133004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":927,"record_offset":2404,"parent_record_offset":2276,"ancestor_record_offset":2276,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":927,"record_offset":3044,"parent_record_offset":2276,"ancestor_record_offset":2276,"inlinee":18708,"source_line":113,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":927,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2276,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":927,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2276,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":927,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2276,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":927,"record_offset":2828,"parent_record_offset":2680,"ancestor_record_offset":2276,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":927,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":2276,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":927,"record_offset":2680,"parent_record_offset":2648,"ancestor_record_offset":2276,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":927,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":2276,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":927,"record_offset":2716,"parent_record_offset":2680,"ancestor_record_offset":2276,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":927,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":2276,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":927,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":2276,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":927,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":2276,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":927,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":2276,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":927,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":2276,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":927,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":2276,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":927,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":2276,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":927,"record_offset":2600,"parent_record_offset":2572,"ancestor_record_offset":2276,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":927,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":2276,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":927,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":2276,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":927,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":2276,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":927,"record_offset":2956,"parent_record_offset":2572,"ancestor_record_offset":2276,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":927,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":2276,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00927.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$2::update::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 371
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4983
// 首条位置: {"module":263,"inlinee":18819}
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
// 源码行: 447
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFB3F
// 首条位置: {"module":939,"inlinee":64319}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":939,"record_offset":3280,"parent_record_offset":3248,"ancestor_record_offset":3248,"inlinee":18706,"source_line":316,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":939,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":3248,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":939,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":3248,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":939,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":3248,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":939,"record_offset":3832,"parent_record_offset":3676,"ancestor_record_offset":3248,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":939,"record_offset":3608,"parent_record_offset":3576,"ancestor_record_offset":3248,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":939,"record_offset":3676,"parent_record_offset":3640,"ancestor_record_offset":3248,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":939,"record_offset":3640,"parent_record_offset":3608,"ancestor_record_offset":3248,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":939,"record_offset":3720,"parent_record_offset":3676,"ancestor_record_offset":3248,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":939,"record_offset":3472,"parent_record_offset":3440,"ancestor_record_offset":3248,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":939,"record_offset":3504,"parent_record_offset":3472,"ancestor_record_offset":3248,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":939,"record_offset":3536,"parent_record_offset":3504,"ancestor_record_offset":3248,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":939,"record_offset":3440,"parent_record_offset":3408,"ancestor_record_offset":3248,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":939,"record_offset":3408,"parent_record_offset":3376,"ancestor_record_offset":3248,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":939,"record_offset":3344,"parent_record_offset":3312,"ancestor_record_offset":3248,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":939,"record_offset":3312,"parent_record_offset":3280,"ancestor_record_offset":3248,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":939,"record_offset":3376,"parent_record_offset":3344,"ancestor_record_offset":3248,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":939,"record_offset":3576,"parent_record_offset":3536,"ancestor_record_offset":3248,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":939,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":3248,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":939,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":3248,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":939,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":3248,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":939,"record_offset":3960,"parent_record_offset":3536,"ancestor_record_offset":3248,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":939,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3248,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00939.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::copy_to_clipboard
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 505
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x497F
// 首条位置: {"module":263,"inlinee":18815}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arboard::Set::text
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arboard","version":"3.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arboard-3.6.0/src/lib.rs
//     evidence={"module":263,"record_offset":110584,"parent_record_offset":110540,"ancestor_record_offset":110540,"inlinee":18817,"source_line":216,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":110676,"parent_record_offset":110540,"ancestor_record_offset":110540,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":263,"record_offset":110612,"parent_record_offset":110540,"ancestor_record_offset":110540,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":263,"record_offset":110648,"parent_record_offset":110540,"ancestor_record_offset":110540,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":110820,"parent_record_offset":110796,"ancestor_record_offset":110540,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":110844,"parent_record_offset":110820,"ancestor_record_offset":110540,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":110868,"parent_record_offset":110844,"ancestor_record_offset":110540,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":110796,"parent_record_offset":110772,"ancestor_record_offset":110540,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":110772,"parent_record_offset":110748,"ancestor_record_offset":110540,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":110748,"parent_record_offset":110724,"ancestor_record_offset":110540,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":110700,"parent_record_offset":110676,"ancestor_record_offset":110540,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::wide_button
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 526
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4936
// 首条位置: {"module":263,"inlinee":18742}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":23224,"parent_record_offset":23172,"ancestor_record_offset":23172,"inlinee":18706,"source_line":316,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::Text::horizontal_alignment
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":263,"record_offset":23896,"parent_record_offset":23172,"ancestor_record_offset":23172,"inlinee":18743,"source_line":95,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":23984,"parent_record_offset":23172,"ancestor_record_offset":23172,"inlinee":18713,"source_line":127,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::width
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":23952,"parent_record_offset":23172,"ancestor_record_offset":23172,"inlinee":18744,"source_line":92,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":263,"record_offset":23924,"parent_record_offset":23172,"ancestor_record_offset":23172,"inlinee":18710,"source_line":116,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":23700,"parent_record_offset":23676,"ancestor_record_offset":23172,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":23724,"parent_record_offset":23700,"ancestor_record_offset":23172,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":24240,"parent_record_offset":24216,"ancestor_record_offset":23172,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":24264,"parent_record_offset":24240,"ancestor_record_offset":23172,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":23748,"parent_record_offset":23724,"ancestor_record_offset":23172,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":24288,"parent_record_offset":24264,"ancestor_record_offset":23172,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":23676,"parent_record_offset":23524,"ancestor_record_offset":23172,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":24216,"parent_record_offset":24084,"ancestor_record_offset":23172,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":24084,"parent_record_offset":24056,"ancestor_record_offset":23172,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":23468,"parent_record_offset":23444,"ancestor_record_offset":23172,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":23524,"parent_record_offset":23492,"ancestor_record_offset":23172,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":23492,"parent_record_offset":23468,"ancestor_record_offset":23172,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":23564,"parent_record_offset":23524,"ancestor_record_offset":23172,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":23368,"parent_record_offset":23344,"ancestor_record_offset":23172,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":23392,"parent_record_offset":23368,"ancestor_record_offset":23172,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":23416,"parent_record_offset":23392,"ancestor_record_offset":23172,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":23344,"parent_record_offset":23320,"ancestor_record_offset":23172,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":263,"record_offset":23320,"parent_record_offset":23296,"ancestor_record_offset":23172,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$126::spec_to_string
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":23272,"parent_record_offset":23248,"ancestor_record_offset":23172,"inlinee":6120,"source_line":3043,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":23248,"parent_record_offset":23224,"ancestor_record_offset":23172,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":23296,"parent_record_offset":23272,"ancestor_record_offset":23172,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":23444,"parent_record_offset":23416,"ancestor_record_offset":23172,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":24116,"parent_record_offset":24084,"ancestor_record_offset":23172,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":23588,"parent_record_offset":23564,"ancestor_record_offset":23172,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":263,"record_offset":23636,"parent_record_offset":23612,"ancestor_record_offset":23172,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":24180,"parent_record_offset":24144,"ancestor_record_offset":23172,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":24144,"parent_record_offset":24116,"ancestor_record_offset":23172,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":263,"record_offset":23612,"parent_record_offset":23588,"ancestor_record_offset":23172,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":263,"record_offset":23804,"parent_record_offset":23416,"ancestor_record_offset":23172,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":23828,"parent_record_offset":23804,"ancestor_record_offset":23172,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":24028,"parent_record_offset":23984,"ancestor_record_offset":23172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$3::main_view::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 551
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4932
// 首条位置: {"module":263,"inlinee":18738}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":263,"record_offset":12804,"parent_record_offset":12780,"ancestor_record_offset":12780,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":263,"record_offset":12852,"parent_record_offset":12828,"ancestor_record_offset":12780,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":12828,"parent_record_offset":12804,"ancestor_record_offset":12780,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$3::main_view::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 552
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4933
// 首条位置: {"module":263,"inlinee":18739}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":263,"record_offset":12952,"parent_record_offset":12928,"ancestor_record_offset":12928,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":13352,"parent_record_offset":13328,"ancestor_record_offset":12928,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":13376,"parent_record_offset":13352,"ancestor_record_offset":12928,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":12928,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":13328,"parent_record_offset":13172,"ancestor_record_offset":12928,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":13108,"parent_record_offset":13080,"ancestor_record_offset":12928,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":13172,"parent_record_offset":13136,"ancestor_record_offset":12928,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":13136,"parent_record_offset":13108,"ancestor_record_offset":12928,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":13216,"parent_record_offset":13172,"ancestor_record_offset":12928,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":13000,"parent_record_offset":12976,"ancestor_record_offset":12928,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":13024,"parent_record_offset":13000,"ancestor_record_offset":12928,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":13048,"parent_record_offset":13024,"ancestor_record_offset":12928,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":12976,"parent_record_offset":12952,"ancestor_record_offset":12928,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":13080,"parent_record_offset":13048,"ancestor_record_offset":12928,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":13240,"parent_record_offset":13216,"ancestor_record_offset":12928,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":263,"record_offset":13288,"parent_record_offset":13264,"ancestor_record_offset":12928,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":263,"record_offset":13264,"parent_record_offset":13240,"ancestor_record_offset":12928,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":263,"record_offset":13456,"parent_record_offset":13048,"ancestor_record_offset":12928,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":13480,"parent_record_offset":13456,"ancestor_record_offset":12928,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$3::main_view::closure$2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 567
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFA8F
// 首条位置: {"module":929,"inlinee":64143}
// 编译实例 RVAs: 0xfabb20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$3,tuple$<bool> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 601
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":936,"record_offset":5316,"rva":16470880,"va":5385180000}
// 编译实例 RVAs: 0xfb5360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":936,"record_offset":5508,"parent_record_offset":5316,"ancestor_record_offset":5316,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00936.jsonl.gz"}
//   [pdb_inline_transitive] crash_reporter_gui::impl$3::main_view::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":936,"record_offset":5528,"parent_record_offset":5508,"ancestor_record_offset":5316,"inlinee":64269,"source_line":601,"archive":"_pdb/C/modules/00936.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$3::main_view::closure$3
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 601
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFB0D
// 首条位置: {"module":936,"inlinee":64269}
// 编译实例 RVAs: 0xfb5360
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
// 源码行: 848
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":263,"record_offset":1316,"rva":6993872,"va":5375702992}
// 编译实例 RVAs: 0x6ab7d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-24476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375702992,"raw_hex":"e864a0ffff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375703159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-24624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375702992,"raw_hex":"e8d09fffff","size":5,"soft_fail":false,"target_va":5375678704,"va":5375703323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6980953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375702992,"raw_hex":"e8a77a95ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375703140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa9c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9429859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375702992,"raw_hex":"e863e38f00","size":5,"soft_fail":false,"target_va":5385133200,"va":5375703336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41353870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375702992,"raw_hex":"e88e027702","size":5,"soft_fail":false,"target_va":5417057056,"va":5375703181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40916903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375702992,"raw_hex":"e8a7577002","size":5,"soft_fail":false,"target_va":5416620769,"va":5375703861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9091766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375702992,"raw_hex":"e8b6ba8a00","size":5,"soft_fail":false,"target_va":5384794864,"va":5375703093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40917662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375702992,"raw_hex":"e89e5a7002","size":5,"soft_fail":false,"target_va":5416620794,"va":5375703127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$crash_reporter_gui..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9438651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375702992,"raw_hex":"e8bb059000","size":5,"soft_fail":false,"target_va":5385142704,"va":5375704048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7431669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375702992,"raw_hex":"e8f5657100","size":5,"soft_fail":false,"target_va":5383134720,"va":5375703046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5375702992,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5375703591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":263,"record_offset":1472,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":263,"record_offset":4000,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] crash_reporter_gui::text
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"crash-reporter-gui","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/crash-reporter-gui/src/lib.rs
//     evidence={"module":263,"record_offset":1552,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":18706,"source_line":316,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":2860,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":18712,"source_line":112,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::padding
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":3972,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":18714,"source_line":104,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::style
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":3368,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":18713,"source_line":127,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/helpers.rs
//     evidence={"module":263,"record_offset":2832,"parent_record_offset":1316,"ancestor_record_offset":1316,"inlinee":18710,"source_line":116,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":1316,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1316,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":1316,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":1316,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":1316,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":1316,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2016,"parent_record_offset":1868,"ancestor_record_offset":1316,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":263,"record_offset":2412,"parent_record_offset":2356,"ancestor_record_offset":1316,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":4148,"parent_record_offset":4104,"ancestor_record_offset":1316,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":4104,"parent_record_offset":4072,"ancestor_record_offset":1316,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":263,"record_offset":3464,"parent_record_offset":3436,"ancestor_record_offset":1316,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":263,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":1316,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":1316,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":2384,"parent_record_offset":2356,"ancestor_record_offset":1316,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":1316,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":1868,"parent_record_offset":1836,"ancestor_record_offset":1316,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1316,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":1316,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":263,"record_offset":1904,"parent_record_offset":1868,"ancestor_record_offset":1316,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":1316,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":1316,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":1316,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":263,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1316,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":263,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":1316,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$114::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":1616,"parent_record_offset":1592,"ancestor_record_offset":1316,"inlinee":10264,"source_line":3043,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":1592,"parent_record_offset":1552,"ancestor_record_offset":1316,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":263,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":1316,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":263,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":1316,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":3496,"parent_record_offset":3464,"ancestor_record_offset":1316,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":263,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":1316,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":263,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":1316,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":3560,"parent_record_offset":3524,"ancestor_record_offset":1316,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":263,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":1316,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":263,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":1316,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":263,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":1316,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":263,"record_offset":2144,"parent_record_offset":1760,"ancestor_record_offset":1316,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1316,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":263,"record_offset":2260,"parent_record_offset":1552,"ancestor_record_offset":1316,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/element.rs
//     evidence={"module":263,"record_offset":4072,"parent_record_offset":4032,"ancestor_record_offset":1316,"inlinee":18717,"source_line":32,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::Text::shaping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/widget/text.rs
//     evidence={"module":263,"record_offset":2232,"parent_record_offset":1552,"ancestor_record_offset":1316,"inlinee":18708,"source_line":113,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_widget","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/widget/src/button.rs
//     evidence={"module":263,"record_offset":4032,"parent_record_offset":4000,"ancestor_record_offset":1316,"inlinee":18716,"source_line":307,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$4::active
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 862
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x499C
// 首条位置: {"module":263,"record_offset":112732,"rva":7057840,"va":5375766960}
// 编译实例 RVAs: 0x6bb1b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9429979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375766960,"raw_hex":"e8dbe38f00","size":5,"soft_fail":false,"target_va":5385196960,"va":5375766976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$4::hovered
// 记录形式: compiled_procedure; instances=1
// 源码行: 876
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":263,"record_offset":112844,"rva":7058000,"va":5375767120}
// 编译实例 RVAs: 0x6bb250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9429819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767120,"raw_hex":"e83be38f00","size":5,"soft_fail":false,"target_va":5385196960,"va":5375767136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$4::pressed
// 记录形式: compiled_procedure; instances=1
// 源码行: 890
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":263,"record_offset":112956,"rva":7058160,"va":5375767280}
// 编译实例 RVAs: 0x6bb2f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9429659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767280,"raw_hex":"e89be28f00","size":5,"soft_fail":false,"target_va":5385196960,"va":5375767296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$5::appearance
// 记录形式: compiled_procedure; instances=1
// 源码行: 910
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":263,"record_offset":113292,"rva":7058608,"va":5375767728}
// 编译实例 RVAs: 0x6bb4b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] iced_style::container::Appearance::with_border
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/container.rs
//     evidence={"module":263,"record_offset":113416,"parent_record_offset":113292,"ancestor_record_offset":113292,"inlinee":18822,"source_line":20,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$59::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":263,"record_offset":113484,"parent_record_offset":113464,"ancestor_record_offset":113292,"inlinee":14394,"source_line":493,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::border::impl$13::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/border.rs
//     evidence={"module":263,"record_offset":113444,"parent_record_offset":113416,"ancestor_record_offset":113292,"inlinee":18824,"source_line":5,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::border::impl$20::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/border.rs
//     evidence={"module":263,"record_offset":113464,"parent_record_offset":113444,"ancestor_record_offset":113292,"inlinee":18826,"source_line":29,"archive":"_pdb/C/modules/00263.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$6::active
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 921
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x49A0
// 首条位置: {"module":263,"record_offset":113068,"rva":7058320,"va":5375767440}
// 编译实例 RVAs: 0x6bb390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9429499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767440,"raw_hex":"e8fbe18f00","size":5,"soft_fail":false,"target_va":5385196960,"va":5375767456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: crash_reporter_gui::impl$6::hovered
// 记录形式: compiled_procedure; instances=1
// 源码行: 929
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":263,"record_offset":113180,"rva":7058464,"va":5375767584}
// 编译实例 RVAs: 0x6bb420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_style..button..Appearance$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9429355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375767584,"raw_hex":"e86be18f00","size":5,"soft_fail":false,"target_va":5385196960,"va":5375767600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 15
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$1", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<crash_reporter_gui::Controls>", "constructor": "core::ptr::drop_in_place", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::State", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::program::state::State<crash_reporter_gui::Controls> >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::program::state::State<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::new<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::State::new", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::update<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::State::update", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::build_user_interface<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::build_user_interface", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$0", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> > >", "constructor": "core::iter::adapters::cloned::Cloned", "arguments": ["core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<iced_core::event::Event>,core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<iced_core::event::Event>", "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<crash_reporter_gui::Controls> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>", "constructor": "tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0", "arguments": ["tuple$<>", "tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "tuple$<enum2$<winit::event::Event<tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0", "arguments": ["tuple$<>", "tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::EventLoop::pump_events", "arguments": ["tuple$<>", "tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner_winit::run_gui_with_options<crash_reporter_gui::Controls>", "constructor": "tg_iced::gui_runner_winit::run_gui_with_options", "arguments": ["crash_reporter_gui::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 217
// [PDB 泛型关系，非字段关系] {"type": "enum2$<crash_reporter_gui::Message>", "constructor": "enum2$", "arguments": ["crash_reporter_gui::Message"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_runtime::command::action::Action", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > >", "constructor": "enum2$", "arguments": ["iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_runtime::command::Command", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::command::Command<enum2$<crash_reporter_gui::Message> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::command::Command<enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<crash_reporter_gui::Message> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::impl$0::update::closure_env$2", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::event::Status,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::event::Status", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::draw::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::draw::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::layout::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::layout::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::is_over::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::is_over::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::on_event::recurse<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::on_event::recurse", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::Element", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button::new<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::button::Button::new", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button::style<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,enum2$<iced_style::theme::Button> >", "constructor": "iced_widget::button::Button::style", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "enum2$<iced_style::theme::Button>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button::on_press<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::Button::on_press", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::widget::operation::Operation", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > >", "constructor": "dyn$", "arguments": ["iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > >", "constructor": "ref_mut$", "arguments": ["dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >", "constructor": "tuple$", "arguments": ["ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::button::impl$1::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::button::impl$1::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::push_mut<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec::push_mut", "arguments": ["enum2$<crash_reporter_gui::Message>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::Button", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$1::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$1::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$1::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$1::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$2::from<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$2::from", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$2::from<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$2::from", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$2::from<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$2::from", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::tag<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::tag", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::tag<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::tag", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::tag<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::tag", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::size<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::size", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::scrollable<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::scrollable", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::text_input<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::text_input", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::custom<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::custom", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::container<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::container", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::container::closure$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::container::closure$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::focusable<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::focusable", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::impl$0::container::closure_env$0", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_core::element::impl$4::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_core::element::impl$4::operate::impl$0::container::closure_env$0<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Output,enum2$<crash_reporter_gui::Message> >", "constructor": "assoc$", "arguments": ["Output", "enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > >", "constructor": "core::ops::function::Fn", "arguments": ["tuple$<enum2$<tg_integrity_report::controls::Message> >", "assoc$<Output,enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "constructor": "dyn$", "arguments": ["core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>,tuple$<enum2$<tg_integrity_report::controls::Message> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "tuple$<enum2$<tg_integrity_report::controls::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::mouse_interaction<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::mouse_interaction", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::mouse_interaction<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::mouse_interaction", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::Row", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::state<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::state", "arguments": ["iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::state<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::state", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::operate<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::operate", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::operate<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::operate", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::overlay<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::overlay", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::overlay<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::overlay", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::Checkbox", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_core::element::impl$4::operate::MapOperation", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation::finish<iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >,enum2$<tg_integrity_report::controls::Message> >", "constructor": "iced_core::widget::operation::Operation::finish", "arguments": ["iced_core::element::impl$4::operate::MapOperation<enum2$<crash_reporter_gui::Message> >", "enum2$<tg_integrity_report::controls::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::Element::map<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,enum2$<crash_reporter_gui::Message>,crash_reporter_gui::impl$3::main_view::closure_env$2>", "constructor": "iced_core::element::Element::map", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "enum2$<crash_reporter_gui::Message>", "crash_reporter_gui::impl$3::main_view::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::button::Button<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::Column", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::Map", "arguments": ["enum2$<tg_integrity_report::controls::Message>", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::element::Map<enum2$<tg_integrity_report::controls::Message>,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<enum2$<crash_reporter_gui::Message> >", "constructor": "core::option::Option", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<enum2$<crash_reporter_gui::Message> > >", "constructor": "enum2$", "arguments": ["core::option::Option<enum2$<crash_reporter_gui::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<crash_reporter_gui::Message> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<enum2$<crash_reporter_gui::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >", "constructor": "ref$", "arguments": ["dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::Iter", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$1", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<iced_core::widget::tree::Tree,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["iced_core::widget::tree::Tree", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>", "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["enum2$<crash_reporter_gui::Message>", "core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2$<tg_integrity_report::controls::Message>,alloc::alloc::Global>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "ref$", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::widget::tree::Tree,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::widget::tree::Tree", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::Element", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::layout::node::Node,core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::layout::node::Node", "core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::helpers::row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,alloc::vec::Vec<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "iced_widget::helpers::row", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "alloc::vec::Vec<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::helpers::column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,alloc::vec::Vec<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "iced_widget::helpers::column", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "alloc::vec::Vec<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "array$", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::push<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::column::Column::push", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::diff<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::diff", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::overlay<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::overlay", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::column::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::column::impl$2::operate::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::state<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::state", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::checkbox::impl$1::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::checkbox::impl$1::on_event", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::checkbox::Checkbox<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::Container::new<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::Scrollable<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::Container::new", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::Scrollable<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::Container::new<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::Container::new", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::row::Row<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::Container::style<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,enum2$<iced_style::theme::Container> >", "constructor": "iced_widget::container::Container::style", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "enum2$<iced_style::theme::Container>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$33::call<tuple$<enum2$<tg_integrity_report::controls::Message> >,dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$33::call", "arguments": ["tuple$<enum2$<tg_integrity_report::controls::Message> >", "dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::impl$32::call_mut<tuple$<enum2$<tg_integrity_report::controls::Message> >,dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global>", "constructor": "alloc::boxed::impl$32::call_mut", "arguments": ["tuple$<enum2$<tg_integrity_report::controls::Message> >", "dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::state<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::state", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_core::shell::Shell::merge<enum2$<crash_reporter_gui::Message>,enum2$<tg_integrity_report::controls::Message>,ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > > >", "constructor": "iced_core::shell::Shell::merge", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<tg_integrity_report::controls::Message>", "ref$<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::shell::Shell::merge<enum2$<crash_reporter_gui::Message>,enum2$<tg_integrity_report::controls::Message>,ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> > >", "constructor": "iced_core::shell::Shell::merge", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<tg_integrity_report::controls::Message>", "ref$<alloc::boxed::Box<dyn$<core::ops::function::Fn<tuple$<enum2$<tg_integrity_report::controls::Message> >,assoc$<Output,enum2$<crash_reporter_gui::Message> > > >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::notify_on_scroll<enum2$<crash_reporter_gui::Message> >", "constructor": "iced_widget::scrollable::notify_on_scroll", "arguments": ["enum2$<crash_reporter_gui::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::space::impl$1::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::space::impl$1::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::space::Space,enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::space::Space", "enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::iter_inner::impl$1::partial_drop<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,3>", "constructor": "core::array::iter::iter_inner::impl$1::partial_drop", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::iter_inner::impl$1::partial_drop<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "core::array::iter::iter_inner::impl$1::partial_drop", "arguments": ["iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::flex::resolve<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::layout::flex::resolve", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::mouse_interaction<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::mouse_interaction", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::draw<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::draw", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate::closure$0<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate::closure$0", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay", "arguments": ["enum2$<crash_reporter_gui::Message>", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 候选参数类型: ["iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::stone_foundation_surface::StoneFoundationSurfaces>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_platform_stairs::WoodenPlatformStairDrawables>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_arch_frame::WoodenArchFrames>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::walls::wooden_arch_frame::RecalculateWoodenArchFramesCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$3,tuple$<bool> >
// 信息 ID: ff9c98aa1d67b332f3dcd27d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["bool"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["crash_reporter_gui::impl$3::main_view::closure_env$3", "tuple$<bool>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::str::pattern::TwoWaySearcher::next<enum2$<core::str::pattern::MatchOnly> >
// 信息 ID: 832a225dd84a2a2fbae75e78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["core::str::pattern::MatchOnly"]; 字段布局 = unknown。
// [类型关系] core::str::pattern::TwoWaySearcher::next -> ["enum2$<core::str::pattern::MatchOnly>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::str::pattern::impl$33::next_match @ 0x6ab699 -> 0x6b8610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::Controls::main_view
// 信息 ID: 7250c590fbee8cb2a17b08a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_reporter_gui::impl$2::view @ 0x6b88a7 -> 0x6ac710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::Controls::more_info_button
// 信息 ID: 5de0c3c2c925913781476424
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::Controls::new
// 信息 ID: 0f370aad6b479aa102642ee7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_reporter_gui::run_crash_reporter_gui<tiny_glade::mock_crash_reporter::closure_env$1> @ 0x59973e -> 0x6abc50 (direct_call); 同地址别名未裁定。
// [静态直接引用] crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7> @ 0x59a92c -> 0x6abc50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::_::impl$0::deserialize
// 信息 ID: 654c1dee323c436102887357
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::options::Options::from_str_seed<core::marker::PhantomData<enum2$<crash_reporter_gui::CrashAdvice> >,enum2$<crash_reporter_gui::CrashAdvice> >; module=240, inline_record=53968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: edca1af186b313030c69523a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] crash_reporter_gui::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<crash_reporter_gui::_::impl$0::deserialize::__Field> > @ 0x1f5b46 -> 0x598cd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: cdd577618c40adfc1a26f829
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<crash_reporter_gui::_::impl$0::deserialize::__Field> >; module=64, inline_record=22340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 8573ff236c27e362c8608c07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<crash_reporter_gui::_::impl$0::deserialize::__Visitor>; module=64, inline_record=109076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::copy_to_clipboard
// 信息 ID: 026f6abde16367c4e1874953
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::impl$2::update; module=263, inline_record=110540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::external_crashing_module
// 信息 ID: d13bcd00016c70345453f90b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=8124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$0::program_outcome
// 信息 ID: f241666e86a49de4a0e77db5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_iced::gui_runner_winit::run_gui_with_options<crash_reporter_gui::Controls> @ 0x531923 -> 0x6b8830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$0::tick_message
// 信息 ID: 91689e61955c4e0c7e63f94a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_iced::gui_runner_winit::run_gui_with_options<crash_reporter_gui::Controls> @ 0x53188e -> 0x6b8810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$11::clone
// 信息 ID: 2c3cc411c859c89f782b46ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_widget::button::impl$1::on_event<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >; module=928, inline_record=12104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$0
// 信息 ID: af3604578689a0cfa4df8c10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=263, inline_record=4492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$1
// 信息 ID: e10dde300531bfc7f5fedad3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=263, inline_record=5568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$2
// 信息 ID: 5f4c616fbcbba7aa5ee9b82b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=263, inline_record=5768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$3
// 信息 ID: 28a19a0cae9771ac4fd9ca96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=263, inline_record=6024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$4
// 信息 ID: 13bcaa1e309fa00d04227ffc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=263, inline_record=6296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$5
// 信息 ID: 17f2b97bda579b32ebcca8d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=263, inline_record=6524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$1::new::closure$6
// 信息 ID: 588244ed2a7af0b78bd2d84c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "crash_reporter_gui::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::new; module=263, inline_record=6696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::update
// 信息 ID: 5c343e110f2f2a07f42cd123
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::command::Command::batch<enum2$<crash_reporter_gui::Message>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<enum2$<crash_reporter_gui::Message>,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<crash_reporter_gui::Controls> > > @ 0x52e4ad -> 0x6ba600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::update::closure$0
// 信息 ID: 7a251dbf23a5f4117be5c5e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$2::update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::impl$2::update; module=263, inline_record=110964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::view
// 信息 ID: d4c9964d322bd53bfe780569
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::program::state::build_user_interface<crash_reporter_gui::Controls> @ 0x1e210e -> 0x6b8840 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$2::view::closure$0
// 信息 ID: 937974edbe31c583a6184c51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4df6506252c83c8495a5ff87: iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] b9123650d11dc9236a011191: iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$2::view", "captures": "unknown"}
// [闭包环境类型引用] crash_reporter_gui::impl$2::view::closure_env$0 <- core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::map::Map<core::str::iter::Lines,crash_reporter_gui::impl$2::view::closure_env$0>,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)>; module=939, inline_record=3248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$3::main_view::closure$0
// 信息 ID: 114c258a58c401d9d757f288
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "crash_reporter_gui::impl$3::main_view", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=12780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$3::main_view::closure$1
// 信息 ID: 812f9d6f11a30a947d51cc0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "crash_reporter_gui::impl$3::main_view", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=12928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$3::main_view::closure$2
// 信息 ID: 0fee20af44dea8be0d55b4d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "crash_reporter_gui::impl$3::main_view", "captures": "unknown"}
// [闭包环境类型引用] crash_reporter_gui::impl$3::main_view::closure_env$2 <- core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$2,tuple$<enum2$<tg_integrity_report::controls::Message> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] crash_reporter_gui::impl$3::main_view::closure_env$2 <- iced_core::element::Element::map<enum2$<tg_integrity_report::controls::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,enum2$<crash_reporter_gui::Message>,crash_reporter_gui::impl$3::main_view::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$2,tuple$<enum2$<tg_integrity_report::controls::Message> > >; module=929, inline_record=12976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$3::main_view::closure$3
// 信息 ID: 160f3e1de2e970d6b74db9cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "crash_reporter_gui::impl$3::main_view", "captures": "unknown"}
// [闭包环境类型引用] crash_reporter_gui::impl$3::main_view::closure_env$3 <- core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$3,tuple$<bool> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<crash_reporter_gui::impl$3::main_view::closure_env$3,tuple$<bool> >; module=936, inline_record=5528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$4::active
// 信息 ID: f2a5cb6bcbbaaba158f58e27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_style::button::StyleSheet::disabled<crash_reporter_gui::ButtonStyle>; module=927, inline_record=2048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$4::hovered
// 信息 ID: 38eab7b61c80b63c962a3fa1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$4::pressed
// 信息 ID: 8c968774fa74e38ce2a48f6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$5::appearance
// 信息 ID: bbbef26c42e23a5301674a4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$6::active
// 信息 ID: 3bd8d4fac1b35a79fc5b7338
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_style::button::StyleSheet::pressed<crash_reporter_gui::HyperlinkStyle>; module=927, inline_record=1856; 不是独立调用边。
// [PDB 内联宿主] iced_style::button::StyleSheet::disabled<crash_reporter_gui::HyperlinkStyle>; module=927, inline_record=2244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::impl$6::hovered
// 信息 ID: 2722a8c97f49d20d8c6b95df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::run_crash_reporter_gui<tiny_glade::mock_crash_reporter::closure_env$1>
// 信息 ID: b62001097ae2e262fdeb1190
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] crash_reporter_gui::run_crash_reporter_gui -> ["tiny_glade::mock_crash_reporter::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::mock_crash_reporter @ 0x640722 -> 0x598e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7>
// 信息 ID: e4e5e0c636ecfecab27cc93d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] crash_reporter_gui::run_crash_reporter_gui -> ["tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x5439d1 -> 0x59a030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::text
// 信息 ID: 2909b054a7162b61836c0f5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4df6506252c83c8495a5ff87: iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::column::Column<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// [泛型宿主/闭包/直接引用方参数候选] b9123650d11dc9236a011191: iced_core::element::Element<enum2$<crash_reporter_gui::Message>,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)
// 引用方: 原程序直接转移 0 条；内联宿主 36 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::more_info_button; module=263, inline_record=1552; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=13708; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=15172; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=17484; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=19212; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=20860; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=23224; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=25704; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::text<ref$<str$> >
// 信息 ID: e141a88c9c782c1f57c09381
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] crash_reporter_gui::text -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: crash_reporter_gui::wide_button
// 信息 ID: 4462e85aa913ca9f57668a10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=23172; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=31344; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=51568; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=52820; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=54164; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=56116; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=63964; 不是独立调用边。
// [PDB 内联宿主] crash_reporter_gui::Controls::main_view; module=263, inline_record=79688; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

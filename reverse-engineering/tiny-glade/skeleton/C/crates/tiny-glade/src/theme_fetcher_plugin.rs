// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/theme_fetcher_plugin.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: dfd0382834db89cd3b2d390622cf617dc150cd53db911be681a409176aa94f92
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: theme_fetcher_plugin.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 30, 230
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":30,"record_offset":43592,"rva":1420656,"va":5370129776}
// 编译实例 RVAs: 0x15ad70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::lifecycle::Packet<()>>>::drop_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14572699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e89b5cde00","size":5,"soft_fail":false,"target_va":5384703584,"va":5370130880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14573053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8fd5dde00","size":5,"soft_fail":false,"target_va":5384703904,"va":5370130846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14526157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8cda6dd00","size":5,"soft_fail":false,"target_va":5384656240,"va":5370130078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::thread::builder::Builder>::name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14518738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8d289dd00","size":5,"soft_fail":false,"target_va":5384649296,"va":5370130553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14479057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8d1eedc00","size":5,"soft_fail":false,"target_va":5384609408,"va":5370130346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5548730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8baaa5400","size":5,"soft_fail":false,"target_va":5375678688,"va":5370129953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5547579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e83ba65400","size":5,"soft_fail":false,"target_va":5375678704,"va":5370131120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1407751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8f984eaff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370129938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-858518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e86ae6f2ff","size":5,"soft_fail":false,"target_va":5369273072,"va":5370131585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008a890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-855012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e81cf4f2ff","size":5,"soft_fail":false,"target_va":5369276560,"va":5370131567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140092950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-821513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8f776f3ff","size":5,"soft_fail":false,"target_va":5369309520,"va":5370131028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-792797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e823e7f3ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370131640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009d5e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-777519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8d122f4ff","size":5,"soft_fail":false,"target_va":5369353696,"va":5370131210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b6e30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-673159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e879baf5ff","size":5,"soft_fail":false,"target_va":5369458224,"va":5370131378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401cff90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t478745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8194e0700","size":5,"soft_fail":false,"target_va":5370609552,"va":5370130802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403facc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2751992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8f8fd2900","size":5,"soft_fail":false,"target_va":5372882112,"va":5370130115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a74c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9542439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8279b9100","size":5,"soft_fail":false,"target_va":5379673168,"va":5370130724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46927065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8d90ccc02","size":5,"soft_fail":false,"target_va":5417057056,"va":5370129986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46489854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8fe60c502","size":5,"soft_fail":false,"target_va":5416620769,"va":5370130910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46489861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e80561c502","size":5,"soft_fail":false,"target_va":5416620794,"va":5370130928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14493700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e80428dd00","size":5,"soft_fail":false,"target_va":5384624080,"va":5370130375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3795875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8a3eb3900","size":5,"soft_fail":false,"target_va":5373926624,"va":5370130744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46490646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e81664c502","size":5,"soft_fail":false,"target_va":5416621632,"va":5370130981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::standard_app_root_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13008744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e8687fc600","size":5,"soft_fail":false,"target_va":5383138784,"va":5370130035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::lifecycle::spawn_unchecked
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3237765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370129776,"raw_hex":"e885673100","size":5,"soft_fail":false,"target_va":5373368432,"va":5370130662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*56030343(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5370129776,"raw_hex":"ff1587f45603","size":6,"soft_fail":false,"va":5370130827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/app.rs
//     evidence={"module":30,"record_offset":45164,"parent_record_offset":43592,"ancestor_record_offset":43592,"inlinee":7551,"source_line":318,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":30,"record_offset":44184,"parent_record_offset":43592,"ancestor_record_offset":43592,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":30,"record_offset":44408,"parent_record_offset":43592,"ancestor_record_offset":43592,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":44352,"parent_record_offset":43592,"ancestor_record_offset":43592,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":45124,"parent_record_offset":43592,"ancestor_record_offset":43592,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":44240,"parent_record_offset":43592,"ancestor_record_offset":43592,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":44100,"parent_record_offset":43592,"ancestor_record_offset":43592,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":30,"record_offset":44072,"parent_record_offset":43592,"ancestor_record_offset":43592,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":30,"record_offset":43764,"parent_record_offset":43592,"ancestor_record_offset":43592,"inlinee":7548,"source_line":519,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":30,"record_offset":44380,"parent_record_offset":43592,"ancestor_record_offset":43592,"inlinee":7549,"source_line":78,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":30,"record_offset":45032,"parent_record_offset":43592,"ancestor_record_offset":43592,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":43960,"parent_record_offset":43936,"ancestor_record_offset":43592,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":43984,"parent_record_offset":43960,"ancestor_record_offset":43592,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":46188,"parent_record_offset":46164,"ancestor_record_offset":43592,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":46212,"parent_record_offset":46188,"ancestor_record_offset":43592,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":44008,"parent_record_offset":43984,"ancestor_record_offset":43592,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":46236,"parent_record_offset":46212,"ancestor_record_offset":43592,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":43936,"parent_record_offset":43904,"ancestor_record_offset":43592,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":46164,"parent_record_offset":46136,"ancestor_record_offset":43592,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":30,"record_offset":43904,"parent_record_offset":43860,"ancestor_record_offset":43592,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":30,"record_offset":43860,"parent_record_offset":43828,"ancestor_record_offset":43592,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":44712,"parent_record_offset":44676,"ancestor_record_offset":43592,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":46136,"parent_record_offset":46112,"ancestor_record_offset":43592,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":44788,"parent_record_offset":44748,"ancestor_record_offset":43592,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":44748,"parent_record_offset":44712,"ancestor_record_offset":43592,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":46112,"parent_record_offset":46088,"ancestor_record_offset":43592,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":30,"record_offset":44564,"parent_record_offset":44516,"ancestor_record_offset":43592,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":30,"record_offset":44600,"parent_record_offset":44564,"ancestor_record_offset":43592,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":30,"record_offset":44636,"parent_record_offset":44600,"ancestor_record_offset":43592,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":30,"record_offset":44516,"parent_record_offset":44480,"ancestor_record_offset":43592,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":30,"record_offset":44480,"parent_record_offset":44444,"ancestor_record_offset":43592,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":44444,"parent_record_offset":44408,"ancestor_record_offset":43592,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":45660,"parent_record_offset":45624,"ancestor_record_offset":43592,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":30,"record_offset":45460,"parent_record_offset":45436,"ancestor_record_offset":43592,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":30,"record_offset":44676,"parent_record_offset":44636,"ancestor_record_offset":43592,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":44208,"parent_record_offset":44184,"ancestor_record_offset":43592,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":30,"record_offset":44948,"parent_record_offset":44636,"ancestor_record_offset":43592,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":44972,"parent_record_offset":44948,"ancestor_record_offset":43592,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":45220,"parent_record_offset":45192,"ancestor_record_offset":43592,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":30,"record_offset":45692,"parent_record_offset":45660,"ancestor_record_offset":43592,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":45084,"parent_record_offset":45056,"ancestor_record_offset":43592,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":45496,"parent_record_offset":45460,"ancestor_record_offset":43592,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":44124,"parent_record_offset":44100,"ancestor_record_offset":43592,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":44148,"parent_record_offset":44124,"ancestor_record_offset":43592,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":45520,"parent_record_offset":45496,"ancestor_record_offset":43592,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":45552,"parent_record_offset":45460,"ancestor_record_offset":43592,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":44288,"parent_record_offset":44264,"ancestor_record_offset":43592,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":30,"record_offset":44264,"parent_record_offset":44240,"ancestor_record_offset":43592,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":30,"record_offset":44312,"parent_record_offset":44288,"ancestor_record_offset":43592,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":30,"record_offset":45348,"parent_record_offset":45324,"ancestor_record_offset":43592,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":30,"record_offset":43796,"parent_record_offset":43764,"ancestor_record_offset":43592,"inlinee":5896,"source_line":198,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\counter.rs
//     evidence={"module":30,"record_offset":43828,"parent_record_offset":43796,"ancestor_record_offset":43592,"inlinee":5885,"source_line":20,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":30,"record_offset":45056,"parent_record_offset":45032,"ancestor_record_offset":43592,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::impl$0::build::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42D3
// 首条位置: {"module":230,"inlinee":17107}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::update_daily_theme
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":30,"record_offset":9288,"rva":1405520,"va":5370114640}
// 编译实例 RVAs: 0x157250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14544033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e8a1ecdd00","size":5,"soft_fail":false,"target_va":5384659264,"va":5370115226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t14541122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e842e1dd00","size":5,"soft_fail":false,"target_va":5384656240,"va":5370115113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14540844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e82ce0dd00","size":5,"soft_fail":false,"target_va":5384656800,"va":5370115951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=20
//     evidence={"assembly":"\tcallq\t5563728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e850e55400","size":5,"soft_fail":false,"target_va":5375678704,"va":5370114971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400919f0
//     owner=unresolved_namespace; count=1
//     target_aliases=51 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-812680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e87899f3ff","size":5,"soft_fail":false,"target_va":5369305584,"va":5370118259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-777203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e80d24f4ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370116046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-773264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e87033f4ff","size":5,"soft_fail":false,"target_va":5369343504,"va":5370116763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-686840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e80885f5ff","size":5,"soft_fail":false,"target_va":5369431360,"va":5370118195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2470
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-676805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e83bacf5ff","size":5,"soft_fail":false,"target_va":5369439344,"va":5370116144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2d50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-673367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e8a9b9f5ff","size":5,"soft_fail":false,"target_va":5369441616,"va":5370114978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401581a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e80e000000","size":5,"soft_fail":false,"target_va":5370118560,"va":5370118541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015c980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e85b540000","size":5,"soft_fail":false,"target_va":5370136960,"va":5370115360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14015e080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e8f9650000","size":5,"soft_fail":false,"target_va":5370142848,"va":5370116738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043b990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3031517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e8dd412e00","size":5,"soft_fail":false,"target_va":5373147536,"va":5370116014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d66e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46200375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e837f6c002","size":5,"soft_fail":false,"target_va":5416316480,"va":5370116100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d67620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46202716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e85cffc002","size":5,"soft_fail":false,"target_va":5416318496,"va":5370115775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d678c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46203848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e8c803c102","size":5,"soft_fail":false,"target_va":5416319168,"va":5370115315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46205645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e8cd0ac102","size":5,"soft_fail":false,"target_va":5416321152,"va":5370115502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46941211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e81b44cc02","size":5,"soft_fail":false,"target_va":5417057056,"va":5370115840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..fmt..Display$u20$for$u20$anyhow..Error$GT$::fmt
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14507692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e8ac5edd00","size":5,"soft_fail":false,"target_va":5384624064,"va":5370116367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14507668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e8945edd00","size":5,"soft_fail":false,"target_va":5384624080,"va":5370116407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46504477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e81d9ac502","size":5,"soft_fail":false,"target_va":5416621632,"va":5370117150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::try_load_from_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10147185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e871d59a00","size":5,"soft_fail":false,"target_va":5380262880,"va":5370115690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t46459748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e864ebc402","size":5,"soft_fail":false,"target_va":5416576832,"va":5370117079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t46461312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e880f1c402","size":5,"soft_fail":false,"target_va":5416576144,"va":5370114827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13870142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e83ea4d300","size":5,"soft_fail":false,"target_va":5383984848,"va":5370114701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13870552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e8d8a5d300","size":5,"soft_fail":false,"target_va":5383985360,"va":5370114803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14636784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e8f056df00","size":5,"soft_fail":false,"target_va":5384751936,"va":5370115147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4176154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e81ab93f00","size":5,"soft_fail":false,"target_va":5374290912,"va":5370114753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-300151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e8896bfbff","size":5,"soft_fail":false,"target_va":5369814592,"va":5370114738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2511131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370114640,"raw_hex":"e81b512600","size":5,"soft_fail":false,"target_va":5372625904,"va":5370114768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":30,"record_offset":10028,"parent_record_offset":9288,"ancestor_record_offset":9288,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":9660,"parent_record_offset":9288,"ancestor_record_offset":9288,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":10428,"parent_record_offset":9288,"ancestor_record_offset":9288,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":30,"record_offset":9440,"parent_record_offset":9288,"ancestor_record_offset":9288,"inlinee":4957,"source_line":212,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":30,"record_offset":9596,"parent_record_offset":9288,"ancestor_record_offset":9288,"inlinee":7498,"source_line":789,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":10312,"parent_record_offset":10288,"ancestor_record_offset":9288,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":10336,"parent_record_offset":10312,"ancestor_record_offset":9288,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":10360,"parent_record_offset":10336,"ancestor_record_offset":9288,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":10288,"parent_record_offset":10232,"ancestor_record_offset":9288,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":10260,"parent_record_offset":10232,"ancestor_record_offset":9288,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":10232,"parent_record_offset":10208,"ancestor_record_offset":9288,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":10208,"parent_record_offset":10184,"ancestor_record_offset":9288,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":17316,"parent_record_offset":17264,"ancestor_record_offset":9288,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":17228,"parent_record_offset":17196,"ancestor_record_offset":9288,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":17264,"parent_record_offset":17228,"ancestor_record_offset":9288,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":30,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":9288,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":30,"record_offset":11596,"parent_record_offset":11556,"ancestor_record_offset":9288,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":30,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":9288,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":30,"record_offset":13300,"parent_record_offset":10544,"ancestor_record_offset":9288,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":30,"record_offset":11556,"parent_record_offset":11516,"ancestor_record_offset":9288,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":30,"record_offset":12336,"parent_record_offset":12296,"ancestor_record_offset":9288,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":30,"record_offset":11516,"parent_record_offset":10544,"ancestor_record_offset":9288,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":30,"record_offset":10056,"parent_record_offset":10028,"ancestor_record_offset":9288,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":30,"record_offset":17344,"parent_record_offset":17264,"ancestor_record_offset":9288,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":30,"record_offset":14188,"parent_record_offset":14160,"ancestor_record_offset":9288,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":14716,"parent_record_offset":10544,"ancestor_record_offset":9288,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":13264,"parent_record_offset":10544,"ancestor_record_offset":9288,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=93
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":10112,"parent_record_offset":10088,"ancestor_record_offset":9288,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":17372,"parent_record_offset":17264,"ancestor_record_offset":9288,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":17160,"parent_record_offset":10488,"ancestor_record_offset":9288,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":13196,"parent_record_offset":10544,"ancestor_record_offset":9288,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":9772,"parent_record_offset":9708,"ancestor_record_offset":9288,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":9504,"parent_record_offset":9468,"ancestor_record_offset":9288,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":9796,"parent_record_offset":9772,"ancestor_record_offset":9288,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":30,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":9288,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/mutex.rs
//     evidence={"module":30,"record_offset":9684,"parent_record_offset":9660,"ancestor_record_offset":9288,"inlinee":4964,"source_line":703,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":30,"record_offset":9468,"parent_record_offset":9440,"ancestor_record_offset":9288,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":30,"record_offset":9708,"parent_record_offset":9684,"ancestor_record_offset":9288,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":30,"record_offset":9560,"parent_record_offset":9468,"ancestor_record_offset":9288,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":30,"record_offset":9744,"parent_record_offset":9708,"ancestor_record_offset":9288,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":10744,"parent_record_offset":10544,"ancestor_record_offset":9288,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":30,"record_offset":10712,"parent_record_offset":10544,"ancestor_record_offset":9288,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":30,"record_offset":9620,"parent_record_offset":9596,"ancestor_record_offset":9288,"inlinee":5895,"source_line":916,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":30,"record_offset":10544,"parent_record_offset":10488,"ancestor_record_offset":9288,"inlinee":7502,"source_line":61,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":30,"record_offset":14748,"parent_record_offset":14716,"ancestor_record_offset":9288,"inlinee":7513,"source_line":79,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":30,"record_offset":10488,"parent_record_offset":10428,"ancestor_record_offset":9288,"inlinee":7500,"source_line":56,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":30,"record_offset":17196,"parent_record_offset":17160,"ancestor_record_offset":9288,"inlinee":7511,"source_line":57,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D4C
// 首条位置: {"module":30,"inlinee":7500}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":17160,"parent_record_offset":10488,"ancestor_record_offset":10488,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::theme_fetcher_plugin::try_load_theme_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":30,"record_offset":10544,"parent_record_offset":10488,"ancestor_record_offset":10488,"inlinee":7502,"source_line":61,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":11024,"parent_record_offset":11000,"ancestor_record_offset":10488,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":11048,"parent_record_offset":11024,"ancestor_record_offset":10488,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":10488,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":11000,"parent_record_offset":10944,"ancestor_record_offset":10488,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":10972,"parent_record_offset":10944,"ancestor_record_offset":10488,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":10488,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":10488,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":17316,"parent_record_offset":17264,"ancestor_record_offset":10488,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":17228,"parent_record_offset":17196,"ancestor_record_offset":10488,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":17264,"parent_record_offset":17228,"ancestor_record_offset":10488,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":30,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":10488,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":30,"record_offset":11596,"parent_record_offset":11556,"ancestor_record_offset":10488,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":30,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":10488,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":30,"record_offset":13300,"parent_record_offset":10544,"ancestor_record_offset":10488,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":30,"record_offset":11556,"parent_record_offset":11516,"ancestor_record_offset":10488,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":30,"record_offset":12336,"parent_record_offset":12296,"ancestor_record_offset":10488,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":30,"record_offset":11516,"parent_record_offset":10544,"ancestor_record_offset":10488,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":30,"record_offset":17344,"parent_record_offset":17264,"ancestor_record_offset":10488,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":30,"record_offset":14188,"parent_record_offset":14160,"ancestor_record_offset":10488,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":14716,"parent_record_offset":10544,"ancestor_record_offset":10488,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":13264,"parent_record_offset":10544,"ancestor_record_offset":10488,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=85
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":10776,"parent_record_offset":10744,"ancestor_record_offset":10488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":17372,"parent_record_offset":17264,"ancestor_record_offset":10488,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":13196,"parent_record_offset":10544,"ancestor_record_offset":10488,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":10744,"parent_record_offset":10544,"ancestor_record_offset":10488,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":30,"record_offset":10712,"parent_record_offset":10544,"ancestor_record_offset":10488,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":30,"record_offset":14748,"parent_record_offset":14716,"ancestor_record_offset":10488,"inlinee":7513,"source_line":79,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":30,"record_offset":17196,"parent_record_offset":17160,"ancestor_record_offset":10488,"inlinee":7511,"source_line":57,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D57
// 首条位置: {"module":30,"inlinee":7511}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":17228,"parent_record_offset":17196,"ancestor_record_offset":17196,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":17700,"parent_record_offset":17196,"ancestor_record_offset":17196,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":17584,"parent_record_offset":17560,"ancestor_record_offset":17196,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":17608,"parent_record_offset":17584,"ancestor_record_offset":17196,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":17632,"parent_record_offset":17608,"ancestor_record_offset":17196,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":17560,"parent_record_offset":17504,"ancestor_record_offset":17196,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":17532,"parent_record_offset":17504,"ancestor_record_offset":17196,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":17504,"parent_record_offset":17480,"ancestor_record_offset":17196,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":17480,"parent_record_offset":17456,"ancestor_record_offset":17196,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":17316,"parent_record_offset":17264,"ancestor_record_offset":17196,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":30,"record_offset":17264,"parent_record_offset":17228,"ancestor_record_offset":17196,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":30,"record_offset":17344,"parent_record_offset":17264,"ancestor_record_offset":17196,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":17408,"parent_record_offset":17264,"ancestor_record_offset":17196,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":17372,"parent_record_offset":17264,"ancestor_record_offset":17196,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::try_load_theme_info
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D4E
// 首条位置: {"module":30,"inlinee":7502}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":30,"record_offset":13300,"parent_record_offset":10544,"ancestor_record_offset":10544,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":30,"record_offset":11516,"parent_record_offset":10544,"ancestor_record_offset":10544,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":14716,"parent_record_offset":10544,"ancestor_record_offset":10544,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":30,"record_offset":13264,"parent_record_offset":10544,"ancestor_record_offset":10544,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":11756,"parent_record_offset":10544,"ancestor_record_offset":10544,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":13196,"parent_record_offset":10544,"ancestor_record_offset":10544,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":30,"record_offset":10744,"parent_record_offset":10544,"ancestor_record_offset":10544,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":30,"record_offset":10712,"parent_record_offset":10544,"ancestor_record_offset":10544,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":11024,"parent_record_offset":11000,"ancestor_record_offset":10544,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":11048,"parent_record_offset":11024,"ancestor_record_offset":10544,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":10544,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":11000,"parent_record_offset":10944,"ancestor_record_offset":10544,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":10972,"parent_record_offset":10944,"ancestor_record_offset":10544,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":10544,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":10544,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":30,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":10544,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":30,"record_offset":11596,"parent_record_offset":11556,"ancestor_record_offset":10544,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":30,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":10544,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":30,"record_offset":11556,"parent_record_offset":11516,"ancestor_record_offset":10544,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":30,"record_offset":12336,"parent_record_offset":12296,"ancestor_record_offset":10544,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":30,"record_offset":14188,"parent_record_offset":14160,"ancestor_record_offset":10544,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=68
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":10776,"parent_record_offset":10744,"ancestor_record_offset":10544,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":14776,"parent_record_offset":14748,"ancestor_record_offset":10544,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":30,"record_offset":14748,"parent_record_offset":14716,"ancestor_record_offset":10544,"inlinee":7513,"source_line":79,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D59
// 首条位置: {"module":30,"inlinee":7513}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":14844,"parent_record_offset":14748,"ancestor_record_offset":14748,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":30,"record_offset":14776,"parent_record_offset":14748,"ancestor_record_offset":14748,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":15020,"parent_record_offset":14996,"ancestor_record_offset":14748,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":15044,"parent_record_offset":15020,"ancestor_record_offset":14748,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":15068,"parent_record_offset":15044,"ancestor_record_offset":14748,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":30,"record_offset":14996,"parent_record_offset":14940,"ancestor_record_offset":14748,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":14968,"parent_record_offset":14940,"ancestor_record_offset":14748,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":14940,"parent_record_offset":14916,"ancestor_record_offset":14748,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":30,"record_offset":14916,"parent_record_offset":14892,"ancestor_record_offset":14748,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00030.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":30,"record_offset":14812,"parent_record_offset":14776,"ancestor_record_offset":14748,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00030.jsonl.gz"}
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
// 完整结构及原始类型记录: theme_fetcher_plugin.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::theme_fetcher_plugin::ActiveEventReceiver
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 31
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$7::init_state<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_ecs::system::system_param::impl$7::init_state", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_ecs::change_detection::Res", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$106::init_access<bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily> >", "constructor": "bevy_ecs::system::system_param::impl$106::init_access", "arguments": ["bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::schedule::Schedules::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>,bevy_app::main_schedule::Update,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::schedule::schedule::Schedules::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "bevy_app::main_schedule::Update", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>,bevy_app::main_schedule::Update,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_app::sub_app::SubApp::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "bevy_app::main_schedule::Update", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::decorator_plugin::DecoratorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::mod_editor::ModEditorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::components::drawable::PrevTransformPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::render::fsr_plugin::plugin_impl::FsrPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::dlss_plugin::plugin_impl::DlssPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::rollercoaster::RollercoasterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::startup::load_glade_tod_and_weather::SnowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::egui_input::EguiInputPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::grass2::Grass2Plugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::decorator_plugin::DecoratorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::mod_editor::ModEditorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::backend::icc::IccProfilePlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::CritterPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::sheep::SheepPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::gpu_lut::GpuLutPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::components::drawable::PrevTransformPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::render::fsr_plugin::plugin_impl::FsrPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::dlss_plugin::plugin_impl::DlssPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::rollercoaster::RollercoasterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::startup::load_glade_tod_and_weather::SnowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::egui_input::EguiInputPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::grass2::Grass2Plugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: theme_fetcher_plugin.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: theme_fetcher_plugin.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::impl$0::build
// 信息 ID: 9e8fd5e8fb0204d62c15e498
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::impl$0::build::closure$0
// 信息 ID: 11aa888de4b716159bc408ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- std::thread::lifecycle::spawn_unchecked<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- core::ptr::drop_in_place<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<std::thread::lifecycle::spawn_unchecked::closure$1::closure$0::closure_env$0<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >; module=230, inline_record=1300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::try_load_theme_info
// 信息 ID: 1a4dae35da079dba71a939ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=30, inline_record=10544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
// 信息 ID: feaf26e263e2ae6ace4146f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::try_load_theme_info", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=30, inline_record=14748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::update_daily_theme
// 信息 ID: 490302dbac4c9615d3fd9d58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c4ff6cd0298cc47ef1ee6925: void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > @ 0x2c5801 -> 0x157250 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> @ 0x6a44f4 -> 0x157250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0
// 信息 ID: 42f9ee82a1ffbdcad1e813a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::update_daily_theme", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=30, inline_record=10488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
// 信息 ID: a2d2f52185f5c8508f3ddadd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::update_daily_theme", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=30, inline_record=17196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

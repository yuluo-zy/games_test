// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/crash_pickup/client.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: fbc22f48b16c7f337c1e5f93f151748149adf854bfdb9ae5ce74214178bd9f4c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: client.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 186, 230
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use crash_handler; // namespace candidate
//   package identities: [{"name":"crash-handler","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50","version":"0.6.1"}]
// use minidumper; // namespace candidate
//   package identities: [{"name":"minidumper","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50","version":"0.8.2"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::client::start
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":186,"record_offset":536,"rva":5464544,"va":5374173664}
// 编译实例 RVAs: 0x5361e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::lifecycle::Packet<()>>>::drop_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10529565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e81daba000","size":5,"soft_fail":false,"target_va":5384703584,"va":5374174014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10529905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e871aca000","size":5,"soft_fail":false,"target_va":5384703904,"va":5374173994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$minidumper..ipc..windows..Socket$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2831053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e8cd322b00","size":5,"soft_fail":false,"target_va":5377005376,"va":5374174318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1504843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e84bf61600","size":5,"soft_fail":false,"target_va":5375678688,"va":5374173840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: client.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5451638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e88ad0acff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374173825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: client.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4835603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e8ed36b6ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374174446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f2860
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: client.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2374072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e848c6dbff","size":5,"soft_fail":false,"target_va":5371799648,"va":5374173715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404379b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: client.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1042959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e8f115f0ff","size":5,"soft_fail":false,"target_va":5373131184,"va":5374174138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42446557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e8ddae8702","size":5,"soft_fail":false,"target_va":5416620769,"va":5374174207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<minidumper::errors::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e8721f0000","size":5,"soft_fail":false,"target_va":5374181840,"va":5374173785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42447351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e8f7b18702","size":5,"soft_fail":false,"target_va":5416621632,"va":5374174276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_handler::windows::CrashHandler::attach
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"crash-handler","version":"0.6.1","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     evidence={"assembly":"\tcallq\t2831851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e8eb352b00","size":5,"soft_fail":false,"target_va":5377005936,"va":5374174080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10554379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e80b0ca100","size":5,"soft_fail":false,"target_va":5384728160,"va":5374173776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::lifecycle::spawn_unchecked
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-803544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374173664,"raw_hex":"e828bdf3ff","size":5,"soft_fail":false,"target_va":5373370400,"va":5374173939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*51987192(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374173664,"raw_hex":"ff15f8421903","size":6,"soft_fail":false,"va":5374173978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":186,"record_offset":1144,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":1404,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":186,"record_offset":716,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":186,"record_offset":2696,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] crash_handler::make_crash_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"crash-handler","version":"0.6.1","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/crash-handler/src/lib.rs
//     evidence={"module":186,"record_offset":2248,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":15621,"source_line":124,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::with_name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/minidumper/src/ipc/client.rs
//     evidence={"module":186,"record_offset":684,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7556,"source_line":24,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_direct] std::thread::functions::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":186,"record_offset":1232,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":7489,"source_line":125,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":536,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":536,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":536,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":186,"record_offset":896,"parent_record_offset":864,"ancestor_record_offset":536,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":186,"record_offset":864,"parent_record_offset":820,"ancestor_record_offset":536,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":186,"record_offset":820,"parent_record_offset":788,"ancestor_record_offset":536,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":186,"record_offset":1840,"parent_record_offset":1804,"ancestor_record_offset":536,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":186,"record_offset":788,"parent_record_offset":756,"ancestor_record_offset":536,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":186,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":536,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":186,"record_offset":756,"parent_record_offset":716,"ancestor_record_offset":536,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":186,"record_offset":1016,"parent_record_offset":820,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":186,"record_offset":1872,"parent_record_offset":1840,"ancestor_record_offset":536,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":186,"record_offset":1360,"parent_record_offset":1232,"ancestor_record_offset":536,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":186,"record_offset":1320,"parent_record_offset":1292,"ancestor_record_offset":536,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":1172,"parent_record_offset":1144,"ancestor_record_offset":536,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":1680,"parent_record_offset":1644,"ancestor_record_offset":536,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":536,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":536,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":186,"record_offset":1736,"parent_record_offset":1644,"ancestor_record_offset":536,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":186,"record_offset":1532,"parent_record_offset":1508,"ancestor_record_offset":536,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":186,"record_offset":1268,"parent_record_offset":1232,"ancestor_record_offset":536,"inlinee":7491,"source_line":185,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::builder::Builder::spawn_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\builder.rs
//     evidence={"module":186,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":536,"inlinee":7492,"source_line":254,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$6::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":186,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":536,"inlinee":15623,"source_line":14,"archive":"_pdb/C/modules/00186.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::client::start::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42D5
// 首条位置: {"module":230,"inlinee":17109}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":230,"record_offset":1644,"parent_record_offset":1564,"ancestor_record_offset":1564,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":1964,"parent_record_offset":1564,"ancestor_record_offset":1564,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":230,"record_offset":1900,"parent_record_offset":1564,"ancestor_record_offset":1564,"inlinee":11666,"source_line":646,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_millis
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":230,"record_offset":1612,"parent_record_offset":1564,"ancestor_record_offset":1564,"inlinee":7493,"source_line":244,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::ping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/minidumper/src/ipc/client.rs
//     evidence={"module":230,"record_offset":1672,"parent_record_offset":1564,"ancestor_record_offset":1564,"inlinee":17110,"source_line":186,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":230,"record_offset":2076,"parent_record_offset":2040,"ancestor_record_offset":1564,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":230,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":1564,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":1840,"parent_record_offset":1672,"ancestor_record_offset":1564,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":2016,"parent_record_offset":1992,"ancestor_record_offset":1564,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":230,"record_offset":2108,"parent_record_offset":2076,"ancestor_record_offset":1564,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":230,"record_offset":1928,"parent_record_offset":1900,"ancestor_record_offset":1564,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":230,"record_offset":1720,"parent_record_offset":1672,"ancestor_record_offset":1564,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":230,"record_offset":1868,"parent_record_offset":1672,"ancestor_record_offset":1564,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":2136,"parent_record_offset":2076,"ancestor_record_offset":1564,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":2168,"parent_record_offset":2040,"ancestor_record_offset":1564,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/minidumper/src/ipc/windows.rs
//     evidence={"module":230,"record_offset":1756,"parent_record_offset":1672,"ancestor_record_offset":1564,"inlinee":17111,"source_line":474,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv_vectored
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/minidumper/src/ipc/windows.rs
//     evidence={"module":230,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":1564,"inlinee":17112,"source_line":479,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::client::start::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42F3
// 首条位置: {"module":230,"inlinee":17139}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":230,"record_offset":34948,"parent_record_offset":34812,"ancestor_record_offset":34812,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":35300,"parent_record_offset":34812,"ancestor_record_offset":34812,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":230,"record_offset":35268,"parent_record_offset":34812,"ancestor_record_offset":34812,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":230,"record_offset":35192,"parent_record_offset":34812,"ancestor_record_offset":34812,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::ping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/minidumper/src/ipc/client.rs
//     evidence={"module":230,"record_offset":34976,"parent_record_offset":34812,"ancestor_record_offset":34812,"inlinee":17110,"source_line":186,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::crash_service::note_crash_started
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/crash_service.rs
//     evidence={"module":230,"record_offset":34864,"parent_record_offset":34812,"ancestor_record_offset":34812,"inlinee":8039,"source_line":169,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":230,"record_offset":35160,"parent_record_offset":34976,"ancestor_record_offset":34812,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":230,"record_offset":35236,"parent_record_offset":35192,"ancestor_record_offset":34812,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":230,"record_offset":35016,"parent_record_offset":34976,"ancestor_record_offset":34812,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":230,"record_offset":35132,"parent_record_offset":34976,"ancestor_record_offset":34812,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":34888,"parent_record_offset":34864,"ancestor_record_offset":34812,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":230,"record_offset":34912,"parent_record_offset":34888,"ancestor_record_offset":34812,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/minidumper/src/ipc/windows.rs
//     evidence={"module":230,"record_offset":35048,"parent_record_offset":34976,"ancestor_record_offset":34812,"inlinee":17111,"source_line":474,"archive":"_pdb/C/modules/00230.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv_vectored
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=28eb7b10d7e36fbd7a3cdf713562b36e538a2a50#28eb7b10d7e36fbd7a3cdf713562b36e538a2a50"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/28eb7b1/minidumper/src/ipc/windows.rs
//     evidence={"module":230,"record_offset":35072,"parent_record_offset":35048,"ancestor_record_offset":34812,"inlinee":17112,"source_line":479,"archive":"_pdb/C/modules/00230.jsonl.gz"}
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
// 完整结构及原始类型记录: client.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: client.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: client.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::client::start
// 信息 ID: 3344c74030640d2c67fc3667
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::crash_pickup::CrashReporter::spawn_server @ 0x5f7ee8 -> 0x5361e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::client::start::closure$0
// 信息 ID: c5bb418e2e52454abedeae20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::client::start", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- std::thread::lifecycle::spawn_unchecked<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- core::ptr::drop_in_place<std::thread::lifecycle::spawn_unchecked::closure_env$1<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >; module=230, inline_record=1564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::client::start::closure$1
// 信息 ID: 555d294ff1973a522d5909f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::client::start", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$1 <- core::ptr::drop_in_place<crash_handler::make_crash_event::Wrapper<tiny_glade::crash_pickup::client::start::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$1 <- crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1>; module=230, inline_record=34812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

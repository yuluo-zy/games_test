// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/integrity_report_main.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: db8f8a023bc596e541e272203fa55b9c523ef62a4ccd629b00ea42114a1229e6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: integrity_report_main.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 180
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use tg_integrity_report; // namespace candidate
//   package identities: [{"name":"tg-integrity-report","source":"local_or_path","version":"0.1.0"}]
// use tg_package_integrity; // namespace candidate
//   package identities: [{"name":"tg-package-integrity","source":"local_or_path","version":"0.1.0"}]
// use winit; // namespace candidate
//   package identities: [{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::integrity_report_main::verify_package_integrity_and_show_report
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":180,"record_offset":16528,"rva":5384688,"va":5374093808}
// 编译实例 RVAs: 0x5229f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::lock_contended
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42518542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e80ec88802","size":5,"soft_fail":false,"target_va":5416616112,"va":5374097565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42518638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e86ec88802","size":5,"soft_fail":false,"target_va":5416616304,"va":5374097661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10554508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e88c0ca100","size":5,"soft_fail":false,"target_va":5384648448,"va":5374093935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10554185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e8490ba100","size":5,"soft_fail":false,"target_va":5384648032,"va":5374093842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t10515263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e83f73a000","size":5,"soft_fail":false,"target_va":5384609408,"va":5374094140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2189716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e894692100","size":5,"soft_fail":false,"target_va":5376286320,"va":5374096599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1582272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e8c0241800","size":5,"soft_fail":false,"target_va":5375678688,"va":5374096411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1582377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e829251800","size":5,"soft_fail":false,"target_va":5375678704,"va":5374096322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5374209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e8fffeadff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374096396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087be0
//     owner=unresolved_namespace; count=1
//     target_aliases=122 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4833556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e8ec3eb6ff","size":5,"soft_fail":false,"target_va":5369265120,"va":5374098671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008c0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4815256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e86886b6ff","size":5,"soft_fail":false,"target_va":5369282736,"va":5374097987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4759316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e8ec60b7ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374098159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4758587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e8c563b7ff","size":5,"soft_fail":false,"target_va":5369339968,"va":5374098550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4740939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e8b5a8b7ff","size":5,"soft_fail":false,"target_va":5369357424,"va":5374098358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a28c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4723323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e885edb7ff","size":5,"soft_fail":false,"target_va":5369374912,"va":5374098230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b1e10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4659830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e88ae5b8ff","size":5,"soft_fail":false,"target_va":5369437712,"va":5374097537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b3700
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4654331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e805fbb8ff","size":5,"soft_fail":false,"target_va":5369444096,"va":5374098422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c2980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-396621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e8b3f2f9ff","size":5,"soft_fail":false,"target_va":5373700480,"va":5374097096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42960426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e82a868f02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374096625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42523082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e8cad98802","size":5,"soft_fail":false,"target_va":5416620769,"va":5374097682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42522956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e84cd98802","size":5,"soft_fail":false,"target_va":5416620794,"va":5374097833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e8c6000000","size":5,"soft_fail":false,"target_va":5374098832,"va":5374098629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42523980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e84cdd8802","size":5,"soft_fail":false,"target_va":5416621632,"va":5374097647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42522115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e803d68802","size":5,"soft_fail":false,"target_va":5416619696,"va":5374097576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::run_integrity_report_gui
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1670449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e8317d1900","size":5,"soft_fail":false,"target_va":5375767968,"va":5374097514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_package_integrity::verify_package_manifest
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2073901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e82da51f00","size":5,"soft_fail":false,"target_va":5376167776,"va":5374093870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1161085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374093808,"raw_hex":"e88348eeff","size":5,"soft_fail":false,"target_va":5372935104,"va":5374096184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5374093808,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5374096292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":180,"record_offset":22932,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":180,"record_offset":17492,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":180,"record_offset":16820,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":180,"record_offset":22864,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":180,"record_offset":21496,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":180,"record_offset":19932,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":180,"record_offset":16876,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":180,"record_offset":16736,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":180,"record_offset":19088,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":13712,"source_line":200,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":180,"record_offset":21524,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":13624,"source_line":305,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":21580,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":13626,"source_line":508,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":20032,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":13604,"source_line":205,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":20632,"parent_record_offset":16528,"ancestor_record_offset":16528,"inlinee":13609,"source_line":307,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":20440,"parent_record_offset":20416,"ancestor_record_offset":16528,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":20464,"parent_record_offset":20440,"ancestor_record_offset":16528,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":19572,"parent_record_offset":19548,"ancestor_record_offset":16528,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":19596,"parent_record_offset":19572,"ancestor_record_offset":16528,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":20488,"parent_record_offset":20464,"ancestor_record_offset":16528,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":19620,"parent_record_offset":19596,"ancestor_record_offset":16528,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":20416,"parent_record_offset":20388,"ancestor_record_offset":16528,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":19548,"parent_record_offset":19416,"ancestor_record_offset":16528,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":180,"record_offset":23012,"parent_record_offset":22968,"ancestor_record_offset":16528,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":180,"record_offset":22968,"parent_record_offset":22932,"ancestor_record_offset":16528,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":180,"record_offset":19416,"parent_record_offset":19388,"ancestor_record_offset":16528,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":20328,"parent_record_offset":20300,"ancestor_record_offset":16528,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":21332,"parent_record_offset":21304,"ancestor_record_offset":16528,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":21304,"parent_record_offset":21280,"ancestor_record_offset":16528,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":20388,"parent_record_offset":20356,"ancestor_record_offset":16528,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":20356,"parent_record_offset":20328,"ancestor_record_offset":16528,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":21280,"parent_record_offset":21256,"ancestor_record_offset":16528,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":180,"record_offset":20208,"parent_record_offset":20180,"ancestor_record_offset":16528,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":180,"record_offset":20236,"parent_record_offset":20208,"ancestor_record_offset":16528,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":180,"record_offset":20264,"parent_record_offset":20236,"ancestor_record_offset":16528,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":180,"record_offset":20180,"parent_record_offset":20152,"ancestor_record_offset":16528,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":180,"record_offset":20152,"parent_record_offset":20116,"ancestor_record_offset":16528,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":180,"record_offset":20704,"parent_record_offset":20676,"ancestor_record_offset":16528,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":180,"record_offset":21844,"parent_record_offset":21820,"ancestor_record_offset":16528,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":180,"record_offset":21820,"parent_record_offset":21776,"ancestor_record_offset":16528,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":180,"record_offset":20300,"parent_record_offset":20264,"ancestor_record_offset":16528,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":180,"record_offset":19448,"parent_record_offset":19416,"ancestor_record_offset":16528,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":180,"record_offset":20676,"parent_record_offset":20632,"ancestor_record_offset":16528,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":180,"record_offset":19512,"parent_record_offset":19476,"ancestor_record_offset":16528,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":180,"record_offset":19476,"parent_record_offset":19448,"ancestor_record_offset":16528,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":180,"record_offset":16844,"parent_record_offset":16820,"ancestor_record_offset":16528,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":180,"record_offset":20544,"parent_record_offset":20264,"ancestor_record_offset":16528,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":180,"record_offset":20568,"parent_record_offset":20544,"ancestor_record_offset":16528,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":180,"record_offset":19296,"parent_record_offset":19088,"ancestor_record_offset":16528,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":180,"record_offset":21868,"parent_record_offset":21844,"ancestor_record_offset":16528,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":180,"record_offset":22360,"parent_record_offset":21776,"ancestor_record_offset":16528,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":180,"record_offset":19264,"parent_record_offset":19180,"ancestor_record_offset":16528,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":22280,"parent_record_offset":22252,"ancestor_record_offset":16528,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":22660,"parent_record_offset":22472,"ancestor_record_offset":16528,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":19208,"parent_record_offset":19180,"ancestor_record_offset":16528,"inlinee":4686,"source_line":788,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":21972,"parent_record_offset":21940,"ancestor_record_offset":16528,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":22752,"parent_record_offset":22720,"ancestor_record_offset":16528,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":16760,"parent_record_offset":16736,"ancestor_record_offset":16528,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":21996,"parent_record_offset":21972,"ancestor_record_offset":16528,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":16784,"parent_record_offset":16760,"ancestor_record_offset":16528,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":22684,"parent_record_offset":22660,"ancestor_record_offset":16528,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":19232,"parent_record_offset":19208,"ancestor_record_offset":16528,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":180,"record_offset":16924,"parent_record_offset":16900,"ancestor_record_offset":16528,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":180,"record_offset":16900,"parent_record_offset":16876,"ancestor_record_offset":16528,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":180,"record_offset":16948,"parent_record_offset":16924,"ancestor_record_offset":16528,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":180,"record_offset":22152,"parent_record_offset":22124,"ancestor_record_offset":16528,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":180,"record_offset":22124,"parent_record_offset":22096,"ancestor_record_offset":16528,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":180,"record_offset":22472,"parent_record_offset":22432,"ancestor_record_offset":16528,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":180,"record_offset":22252,"parent_record_offset":22060,"ancestor_record_offset":16528,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":180,"record_offset":22060,"parent_record_offset":22032,"ancestor_record_offset":16528,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":180,"record_offset":21904,"parent_record_offset":21776,"ancestor_record_offset":16528,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":180,"record_offset":22032,"parent_record_offset":21904,"ancestor_record_offset":16528,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":180,"record_offset":22432,"parent_record_offset":22400,"ancestor_record_offset":16528,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":180,"record_offset":21940,"parent_record_offset":21904,"ancestor_record_offset":16528,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":180,"record_offset":22720,"parent_record_offset":22432,"ancestor_record_offset":16528,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":180,"record_offset":22096,"parent_record_offset":22060,"ancestor_record_offset":16528,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":180,"record_offset":19180,"parent_record_offset":19088,"ancestor_record_offset":16528,"inlinee":13716,"source_line":120,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":180,"record_offset":19124,"parent_record_offset":19088,"ancestor_record_offset":16528,"inlinee":13714,"source_line":69,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":180,"record_offset":19148,"parent_record_offset":19124,"ancestor_record_offset":16528,"inlinee":13715,"source_line":80,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoop::window_target
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":180,"record_offset":21548,"parent_record_offset":21524,"ancestor_record_offset":16528,"inlinee":13625,"source_line":210,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":22820,"parent_record_offset":21776,"ancestor_record_offset":16528,"inlinee":12895,"source_line":348,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":21700,"parent_record_offset":21580,"ancestor_record_offset":16528,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":21644,"parent_record_offset":21580,"ancestor_record_offset":16528,"inlinee":13627,"source_line":100,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":21776,"parent_record_offset":21740,"ancestor_record_offset":16528,"inlinee":12814,"source_line":164,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$31::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":20068,"parent_record_offset":20032,"ancestor_record_offset":16528,"inlinee":13606,"source_line":116,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":20116,"parent_record_offset":20068,"ancestor_record_offset":16528,"inlinee":13608,"source_line":162,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$9::build::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":21740,"parent_record_offset":21700,"ancestor_record_offset":16528,"inlinee":13629,"source_line":514,"archive":"_pdb/C/modules/00180.jsonl.gz"}
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
// 完整结构及原始类型记录: integrity_report_main.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: integrity_report_main.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: integrity_report_main.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::integrity_report_main::verify_package_integrity_and_show_report
// 信息 ID: 4ea054bb328486c95a8fc304
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x64370a -> 0x5229f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

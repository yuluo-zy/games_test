// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/integrity_report_main.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: db8f8a023bc596e541e272203fa55b9c523ef62a4ccd629b00ea42114a1229e6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: integrity_report_main.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 83
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
// 首条位置: {"module":83,"record_offset":28724,"rva":2446000,"va":5371155120}
// 编译实例 RVAs: 0x2552b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t11489026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8024faf00","size":5,"soft_fail":false,"target_va":5382644528,"va":5371155497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4462321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8f1164400","size":5,"soft_fail":false,"target_va":5375621104,"va":5371158778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4462044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8dc154400","size":5,"soft_fail":false,"target_va":5375620560,"va":5371158511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400430c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2176720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e830c9deff","size":5,"soft_fail":false,"target_va":5368983744,"va":5371160459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3653463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e857bf3700","size":5,"soft_fail":false,"target_va":5374814320,"va":5371160852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dc880
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3695344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8f0623800","size":5,"soft_fail":false,"target_va":5374855296,"va":5371159947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3740599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8b7133900","size":5,"soft_fail":false,"target_va":5374900736,"va":5371160132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3740784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e870143900","size":5,"soft_fail":false,"target_va":5374901408,"va":5371160619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ebc10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3757264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8d0543900","size":5,"soft_fail":false,"target_va":5374917648,"va":5371160379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405eeac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3769376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e820843900","size":5,"soft_fail":false,"target_va":5374929600,"va":5371160219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f97d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3814481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e851343a00","size":5,"soft_fail":false,"target_va":5374973904,"va":5371159418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3923168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8e0dc3b00","size":5,"soft_fail":false,"target_va":5375081488,"va":5371158315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3923267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e843dd3b00","size":5,"soft_fail":false,"target_va":5375081504,"va":5371158232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=9
//     target_aliases=69 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4360719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e80f8a4200","size":5,"soft_fail":false,"target_va":5375516032,"va":5371155308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40688627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8f3db6c02","size":5,"soft_fail":false,"target_va":5411847168,"va":5371158536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40381103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8af2a6802","size":5,"soft_fail":false,"target_va":5411540992,"va":5371159884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40381202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8122b6802","size":5,"soft_fail":false,"target_va":5411540963,"va":5371159756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tg_package_integrity::PackageVerificationError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8bce6ffff","size":5,"soft_fail":false,"target_va":5371154256,"va":5371160719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40383274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e82a336802","size":5,"soft_fail":false,"target_va":5411542880,"va":5371159601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40379072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8c0226802","size":5,"soft_fail":false,"target_va":5411538608,"va":5371159531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40379242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e86a236802","size":5,"soft_fail":false,"target_va":5411538768,"va":5371159521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40379341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8cd236802","size":5,"soft_fail":false,"target_va":5411538960,"va":5371159614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11593173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8d5e5b000","size":5,"soft_fail":false,"target_va":5382748432,"va":5371155254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11592450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e802e3b000","size":5,"soft_fail":false,"target_va":5382747616,"va":5371155161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::run_integrity_report_gui
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-integrity-report","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4031847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e867853d00","size":5,"soft_fail":false,"target_va":5375191248,"va":5371159396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_package_integrity::verify_package_manifest
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-package-integrity","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4354262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8d6704200","size":5,"soft_fail":false,"target_va":5375509456,"va":5371155189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3042740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e8b46d2e00","size":5,"soft_fail":false,"target_va":5374200800,"va":5371158055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1932774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371155120,"raw_hex":"e81a82e2ff","size":5,"soft_fail":false,"target_va":5369226208,"va":5371158977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5371155120,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5371158203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":83,"record_offset":35408,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":83,"record_offset":29808,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":83,"record_offset":29016,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":83,"record_offset":35340,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":83,"record_offset":32320,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":83,"record_offset":29048,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":83,"record_offset":28932,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":83,"record_offset":31644,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":10937,"source_line":200,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":83,"record_offset":34020,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":5996,"source_line":305,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":83,"record_offset":34076,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":5999,"source_line":508,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":83,"record_offset":32420,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":5917,"source_line":205,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":83,"record_offset":33836,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":5956,"source_line":222,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":83,"record_offset":33024,"parent_record_offset":28724,"ancestor_record_offset":28724,"inlinee":5922,"source_line":307,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":83,"record_offset":32856,"parent_record_offset":32832,"ancestor_record_offset":28724,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":83,"record_offset":32880,"parent_record_offset":32856,"ancestor_record_offset":28724,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":83,"record_offset":32120,"parent_record_offset":32096,"ancestor_record_offset":28724,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":83,"record_offset":35480,"parent_record_offset":35444,"ancestor_record_offset":28724,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":83,"record_offset":32832,"parent_record_offset":32804,"ancestor_record_offset":28724,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":83,"record_offset":32096,"parent_record_offset":32012,"ancestor_record_offset":28724,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":83,"record_offset":35444,"parent_record_offset":35408,"ancestor_record_offset":28724,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":83,"record_offset":32012,"parent_record_offset":31984,"ancestor_record_offset":28724,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":83,"record_offset":32744,"parent_record_offset":32716,"ancestor_record_offset":28724,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":83,"record_offset":33728,"parent_record_offset":33700,"ancestor_record_offset":28724,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":83,"record_offset":33700,"parent_record_offset":33676,"ancestor_record_offset":28724,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":83,"record_offset":32804,"parent_record_offset":32772,"ancestor_record_offset":28724,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":83,"record_offset":32772,"parent_record_offset":32744,"ancestor_record_offset":28724,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":83,"record_offset":33676,"parent_record_offset":33652,"ancestor_record_offset":28724,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":83,"record_offset":32680,"parent_record_offset":32652,"ancestor_record_offset":28724,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":83,"record_offset":32652,"parent_record_offset":32624,"ancestor_record_offset":28724,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":83,"record_offset":32596,"parent_record_offset":32568,"ancestor_record_offset":28724,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":83,"record_offset":32624,"parent_record_offset":32596,"ancestor_record_offset":28724,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":83,"record_offset":32568,"parent_record_offset":32540,"ancestor_record_offset":28724,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":83,"record_offset":32540,"parent_record_offset":32504,"ancestor_record_offset":28724,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":83,"record_offset":33096,"parent_record_offset":33068,"ancestor_record_offset":28724,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":83,"record_offset":34328,"parent_record_offset":34304,"ancestor_record_offset":28724,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":83,"record_offset":34304,"parent_record_offset":34264,"ancestor_record_offset":28724,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":83,"record_offset":32716,"parent_record_offset":32680,"ancestor_record_offset":28724,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":83,"record_offset":32040,"parent_record_offset":32012,"ancestor_record_offset":28724,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":83,"record_offset":33068,"parent_record_offset":33024,"ancestor_record_offset":28724,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":83,"record_offset":32956,"parent_record_offset":32932,"ancestor_record_offset":28724,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":83,"record_offset":32064,"parent_record_offset":32040,"ancestor_record_offset":28724,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":83,"record_offset":29104,"parent_record_offset":29072,"ancestor_record_offset":28724,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":83,"record_offset":29132,"parent_record_offset":29072,"ancestor_record_offset":28724,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":83,"record_offset":32932,"parent_record_offset":32680,"ancestor_record_offset":28724,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":83,"record_offset":31880,"parent_record_offset":31644,"ancestor_record_offset":28724,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":83,"record_offset":34352,"parent_record_offset":34328,"ancestor_record_offset":28724,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":83,"record_offset":35588,"parent_record_offset":35560,"ancestor_record_offset":28724,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":83,"record_offset":34844,"parent_record_offset":34264,"ancestor_record_offset":28724,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":83,"record_offset":31840,"parent_record_offset":31748,"ancestor_record_offset":28724,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":83,"record_offset":34764,"parent_record_offset":34736,"ancestor_record_offset":28724,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":83,"record_offset":35136,"parent_record_offset":34948,"ancestor_record_offset":28724,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":83,"record_offset":31784,"parent_record_offset":31748,"ancestor_record_offset":28724,"inlinee":10942,"source_line":683,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":83,"record_offset":34456,"parent_record_offset":34424,"ancestor_record_offset":28724,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":83,"record_offset":35228,"parent_record_offset":35196,"ancestor_record_offset":28724,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":83,"record_offset":28956,"parent_record_offset":28932,"ancestor_record_offset":28724,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":83,"record_offset":34480,"parent_record_offset":34456,"ancestor_record_offset":28724,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":83,"record_offset":28980,"parent_record_offset":28956,"ancestor_record_offset":28724,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":83,"record_offset":35160,"parent_record_offset":35136,"ancestor_record_offset":28724,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":83,"record_offset":31808,"parent_record_offset":31784,"ancestor_record_offset":28724,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":83,"record_offset":29160,"parent_record_offset":29072,"ancestor_record_offset":28724,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":83,"record_offset":29072,"parent_record_offset":29048,"ancestor_record_offset":28724,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":83,"record_offset":29184,"parent_record_offset":29160,"ancestor_record_offset":28724,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":83,"record_offset":34636,"parent_record_offset":34608,"ancestor_record_offset":28724,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":83,"record_offset":34608,"parent_record_offset":34580,"ancestor_record_offset":28724,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":83,"record_offset":34948,"parent_record_offset":34912,"ancestor_record_offset":28724,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":83,"record_offset":34736,"parent_record_offset":34544,"ancestor_record_offset":28724,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":83,"record_offset":34544,"parent_record_offset":34516,"ancestor_record_offset":28724,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":83,"record_offset":34388,"parent_record_offset":34264,"ancestor_record_offset":28724,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":83,"record_offset":34516,"parent_record_offset":34388,"ancestor_record_offset":28724,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":83,"record_offset":34912,"parent_record_offset":34884,"ancestor_record_offset":28724,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":83,"record_offset":34424,"parent_record_offset":34388,"ancestor_record_offset":28724,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":83,"record_offset":35196,"parent_record_offset":34912,"ancestor_record_offset":28724,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":83,"record_offset":34580,"parent_record_offset":34544,"ancestor_record_offset":28724,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalSize::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":83,"record_offset":33920,"parent_record_offset":33896,"ancestor_record_offset":28724,"inlinee":6845,"source_line":439,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":83,"record_offset":33948,"parent_record_offset":33920,"ancestor_record_offset":28724,"inlinee":5953,"source_line":109,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$33::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":83,"record_offset":33896,"parent_record_offset":33872,"ancestor_record_offset":28724,"inlinee":6843,"source_line":531,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":83,"record_offset":31748,"parent_record_offset":31644,"ancestor_record_offset":28724,"inlinee":10941,"source_line":120,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":83,"record_offset":31692,"parent_record_offset":31644,"ancestor_record_offset":28724,"inlinee":10939,"source_line":69,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":83,"record_offset":31716,"parent_record_offset":31692,"ancestor_record_offset":28724,"inlinee":10940,"source_line":80,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoop::window_target
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":83,"record_offset":34044,"parent_record_offset":34020,"ancestor_record_offset":28724,"inlinee":5998,"source_line":210,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":83,"record_offset":35296,"parent_record_offset":34264,"ancestor_record_offset":28724,"inlinee":6012,"source_line":348,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":83,"record_offset":34196,"parent_record_offset":34076,"ancestor_record_offset":28724,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":83,"record_offset":34140,"parent_record_offset":34076,"ancestor_record_offset":28724,"inlinee":6001,"source_line":100,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":83,"record_offset":34264,"parent_record_offset":34232,"ancestor_record_offset":28724,"inlinee":6005,"source_line":164,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$30::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":83,"record_offset":32456,"parent_record_offset":32420,"ancestor_record_offset":28724,"inlinee":5919,"source_line":116,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":83,"record_offset":32504,"parent_record_offset":32456,"ancestor_record_offset":28724,"inlinee":5921,"source_line":162,"archive":"_pdb/B/modules/00083.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$9::build::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":83,"record_offset":34232,"parent_record_offset":34196,"ancestor_record_offset":28724,"inlinee":6004,"source_line":514,"archive":"_pdb/B/modules/00083.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [文件级接口候选，具体函数归属 unknown] bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 候选参数类型: ["country_core::systems::collision::world::SubworldColliderHandle", "ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >"]

// ------------------------------------------------------------------
// 函数: tiny_glade::integrity_report_main::verify_package_integrity_and_show_report
// 信息 ID: 0530ac0f3e53daf088e397e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x3dd65e -> 0x2552b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

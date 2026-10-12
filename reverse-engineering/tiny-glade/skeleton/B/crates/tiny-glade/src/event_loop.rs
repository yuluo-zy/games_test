// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/event_loop.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d04dee5d15498cc98ee4043a7a72f9d31ab927dc9f3381c5ae500be8a135cd22
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: event_loop.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 92, 184, 194, 234, 240
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"}]
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use egui_winit; // namespace candidate
//   package identities: [{"name":"egui-winit","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use winit; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.9.0"},{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::run_event_loop
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":92,"record_offset":14624,"rva":2548896,"va":5371258016}
// 编译实例 RVAs: 0x26e4a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..vec_deque..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::try_fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2850778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8da7f2b00","size":5,"soft_fail":false,"target_va":5374110320,"va":5371259537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t4359110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8c6834200","size":5,"soft_fail":false,"target_va":5375621152,"va":5371262037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t11384475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e89bb6ad00","size":5,"soft_fail":false,"target_va":5382644528,"va":5371260048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4361384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8a88c4200","size":5,"soft_fail":false,"target_va":5375621104,"va":5371259715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2281972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e80c2eddff","size":5,"soft_fail":false,"target_va":5368983552,"va":5371265519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140268ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e81e91ffff","size":5,"soft_fail":false,"target_va":5371235488,"va":5371263869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140511380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2760693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8f51f2a00","size":5,"soft_fail":false,"target_va":5374022528,"va":5371261830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3548667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8fb253600","size":5,"soft_fail":false,"target_va":5374814320,"va":5371265648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d3f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3553909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8753a3600","size":5,"soft_fail":false,"target_va":5374820112,"va":5371266198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=3
//     target_aliases=92 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3635730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8127a3700","size":5,"soft_fail":false,"target_va":5374900736,"va":5371265001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e8000
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3638550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e816853700","size":5,"soft_fail":false,"target_va":5374902272,"va":5371263717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f0d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3672329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e809093800","size":5,"soft_fail":false,"target_va":5374938384,"va":5371266050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f18c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3677786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e85a1e3800","size":5,"soft_fail":false,"target_va":5374941376,"va":5371263585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f97d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3710277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8459d3800","size":5,"soft_fail":false,"target_va":5374973904,"va":5371263622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140607fc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3769714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e872853900","size":5,"soft_fail":false,"target_va":5375033280,"va":5371263561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140608df0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3773334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e896933900","size":5,"soft_fail":false,"target_va":5375036912,"va":5371263573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=26
//     target_aliases=1 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3822247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8a7523a00","size":5,"soft_fail":false,"target_va":5375081504,"va":5371259252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=5
//     target_aliases=69 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4256169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8a9f14000","size":5,"soft_fail":false,"target_va":5375516032,"va":5371259858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140697a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4362256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e810904200","size":5,"soft_fail":false,"target_va":5375621776,"va":5371259515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d63680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11484653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8ed3daf00","size":5,"soft_fail":false,"target_va":5382747776,"va":5371263118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a1fc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40052278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e836266302","size":5,"soft_fail":false,"target_va":5411315648,"va":5371263365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3939777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8c11d3c00","size":5,"soft_fail":false,"target_va":5375203392,"va":5371263610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11392258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e802d5ad00","size":5,"soft_fail":false,"target_va":5382655024,"va":5371262761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40278135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e877986602","size":5,"soft_fail":false,"target_va":5411542064,"va":5371263924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40278854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8469b6602","size":5,"soft_fail":false,"target_va":5411542880,"va":5371264021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui_winit::State::handle_platform_output
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui-winit","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4232867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8a3964000","size":5,"soft_fail":false,"target_va":5375491904,"va":5371259032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t40276671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8bf926602","size":5,"soft_fail":false,"target_va":5411538608,"va":5371261932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::Sender$LT$T$GT$::send
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1402409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e829661500","size":5,"soft_fail":false,"target_va":5372663984,"va":5371261570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2637591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8173f2800","size":5,"soft_fail":false,"target_va":5373896208,"va":5371258612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-31140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e85c86ffff","size":5,"soft_fail":false,"target_va":5371227456,"va":5371258591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2824295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e867182b00","size":5,"soft_fail":false,"target_va":5374082928,"va":5371258628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::os::errno
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11519589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e865c6af00","size":5,"soft_fail":false,"target_va":5382781808,"va":5371262214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40276848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e870936602","size":5,"soft_fail":false,"target_va":5411538768,"va":5371261915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40279858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8329f6602","size":5,"soft_fail":false,"target_va":5411538960,"va":5371259097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11427990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e89660ae00","size":5,"soft_fail":false,"target_va":5382691200,"va":5371263205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11489358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e84e50af00","size":5,"soft_fail":false,"target_va":5382747616,"va":5371258253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::icc::get_monitor_icc_profile
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1297896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8e8cd1300","size":5,"soft_fail":false,"target_va":5372560384,"va":5371262483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::setup::find_best_fullscreen_video_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2266935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8c968ddff","size":5,"soft_fail":false,"target_va":5368992704,"va":5371259634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2940262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e866dd2c00","size":5,"soft_fail":false,"target_va":5374198784,"va":5371258517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2951325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e89d082d00","size":5,"soft_fail":false,"target_va":5374210752,"va":5371259422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::VideoMode::size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t4361856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8808e4200","size":5,"soft_fail":false,"target_va":5375621552,"va":5371259691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::current_monitor
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t4363727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8cf954200","size":5,"soft_fail":false,"target_va":5375621888,"va":5371258156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::is_minimized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t4354869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e835734200","size":5,"soft_fail":false,"target_va":5375614240,"va":5371259366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t4383245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e80de24200","size":5,"soft_fail":false,"target_va":5375645312,"va":5371262062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::Window::set_cursor_grab
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-4202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e896efffff","size":5,"soft_fail":false,"target_va":5371254944,"va":5371259141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::Window::set_cursor_visible
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-7298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e87ee3ffff","size":5,"soft_fail":false,"target_va":5371255664,"va":5371262957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::Window::try_set_fullscreen
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t4363681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371258016,"raw_hex":"e8a1954200","size":5,"soft_fail":false,"target_va":5375623808,"va":5371260122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5371258016,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5371259223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":92,"record_offset":25348,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11223,"source_line":1500,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":92,"record_offset":17360,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":4733,"source_line":1827,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":92,"record_offset":25464,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11227,"source_line":240,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":92,"record_offset":14972,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":4594,"source_line":153,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":92,"record_offset":15320,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":92,"record_offset":26660,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11236,"source_line":1407,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":92,"record_offset":19360,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":92,"record_offset":18400,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":92,"record_offset":18456,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":92,"record_offset":17276,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":15940,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":92,"record_offset":23036,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":8084,"source_line":607,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":92,"record_offset":15808,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::mouse_capture::OsCursorVisibilityCmdOrigin::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":92,"record_offset":26492,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11235,"source_line":609,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::mouse_capture::OsCursorVisibilityCmdOrigin::is_empty
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":92,"record_offset":26380,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11233,"source_line":585,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::mouse_capture::OsCursorVisibilityCmdOrigin::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":92,"record_offset":26408,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11234,"source_line":615,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":92,"record_offset":19392,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":92,"record_offset":19276,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":92,"record_offset":15252,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":7838,"source_line":785,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":92,"record_offset":23008,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":7835,"source_line":607,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":92,"record_offset":15348,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":92,"record_offset":15856,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11201,"source_line":663,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::setup::impl$1::from_winit
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":92,"record_offset":18956,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":5986,"source_line":206,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::set_control_flow
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":92,"record_offset":15028,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11206,"source_line":351,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":92,"record_offset":15224,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":8450,"source_line":183,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":92,"record_offset":18316,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11214,"source_line":1499,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":92,"record_offset":14860,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11202,"source_line":1488,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":92,"record_offset":24020,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11219,"source_line":1377,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_minimized
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":92,"record_offset":17840,"parent_record_offset":14624,"ancestor_record_offset":14624,"inlinee":11210,"source_line":1017,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":92,"record_offset":16888,"parent_record_offset":16864,"ancestor_record_offset":14624,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":92,"record_offset":16864,"parent_record_offset":16776,"ancestor_record_offset":14624,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":92,"record_offset":16776,"parent_record_offset":16748,"ancestor_record_offset":14624,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":92,"record_offset":25668,"parent_record_offset":25464,"ancestor_record_offset":14624,"inlinee":4907,"source_line":181,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":92,"record_offset":17432,"parent_record_offset":17400,"ancestor_record_offset":14624,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":92,"record_offset":17400,"parent_record_offset":17360,"ancestor_record_offset":14624,"inlinee":4734,"source_line":204,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":92,"record_offset":14996,"parent_record_offset":14972,"ancestor_record_offset":14624,"inlinee":4596,"source_line":558,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":92,"record_offset":17712,"parent_record_offset":17544,"ancestor_record_offset":14624,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":92,"record_offset":17544,"parent_record_offset":17360,"ancestor_record_offset":14624,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":92,"record_offset":17576,"parent_record_offset":17544,"ancestor_record_offset":14624,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::Drain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":92,"record_offset":25376,"parent_record_offset":25348,"ancestor_record_offset":14624,"inlinee":11225,"source_line":37,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":92,"record_offset":25940,"parent_record_offset":25912,"ancestor_record_offset":14624,"inlinee":11229,"source_line":94,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":92,"record_offset":26000,"parent_record_offset":25972,"ancestor_record_offset":14624,"inlinee":11231,"source_line":115,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":92,"record_offset":17464,"parent_record_offset":17432,"ancestor_record_offset":14624,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":92,"record_offset":17744,"parent_record_offset":17712,"ancestor_record_offset":14624,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":92,"record_offset":17496,"parent_record_offset":17464,"ancestor_record_offset":14624,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":92,"record_offset":18808,"parent_record_offset":18772,"ancestor_record_offset":14624,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":92,"record_offset":18772,"parent_record_offset":18744,"ancestor_record_offset":14624,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":92,"record_offset":17792,"parent_record_offset":17768,"ancestor_record_offset":14624,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":92,"record_offset":17768,"parent_record_offset":17744,"ancestor_record_offset":14624,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":92,"record_offset":18744,"parent_record_offset":18716,"ancestor_record_offset":14624,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_inner_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":92,"record_offset":18088,"parent_record_offset":17964,"ancestor_record_offset":14624,"inlinee":6797,"source_line":281,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":92,"record_offset":17996,"parent_record_offset":17964,"ancestor_record_offset":14624,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":92,"record_offset":17964,"parent_record_offset":17924,"ancestor_record_offset":14624,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":92,"record_offset":18144,"parent_record_offset":18120,"ancestor_record_offset":14624,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":92,"record_offset":16808,"parent_record_offset":16776,"ancestor_record_offset":14624,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":15124,"parent_record_offset":15100,"ancestor_record_offset":14624,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":15100,"parent_record_offset":15076,"ancestor_record_offset":14624,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":15880,"parent_record_offset":15856,"ancestor_record_offset":14624,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":92,"record_offset":26736,"parent_record_offset":26708,"ancestor_record_offset":14624,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":92,"record_offset":19016,"parent_record_offset":18956,"ancestor_record_offset":14624,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$103::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":92,"record_offset":24896,"parent_record_offset":24872,"ancestor_record_offset":14624,"inlinee":5977,"source_line":77,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$108::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":92,"record_offset":19116,"parent_record_offset":19092,"ancestor_record_offset":14624,"inlinee":5955,"source_line":77,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":92,"record_offset":18424,"parent_record_offset":18400,"ancestor_record_offset":14624,"inlinee":5946,"source_line":115,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":92,"record_offset":15148,"parent_record_offset":15124,"ancestor_record_offset":14624,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":92,"record_offset":16832,"parent_record_offset":16808,"ancestor_record_offset":14624,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":92,"record_offset":18844,"parent_record_offset":18808,"ancestor_record_offset":14624,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":92,"record_offset":25560,"parent_record_offset":25532,"ancestor_record_offset":14624,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":92,"record_offset":19448,"parent_record_offset":19416,"ancestor_record_offset":14624,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":92,"record_offset":19476,"parent_record_offset":19416,"ancestor_record_offset":14624,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=80
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":16404,"parent_record_offset":16376,"ancestor_record_offset":14624,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":92,"record_offset":16576,"parent_record_offset":16548,"ancestor_record_offset":14624,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":92,"record_offset":16624,"parent_record_offset":16600,"ancestor_record_offset":14624,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":92,"record_offset":16600,"parent_record_offset":16576,"ancestor_record_offset":14624,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":92,"record_offset":18028,"parent_record_offset":17996,"ancestor_record_offset":14624,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":15172,"parent_record_offset":15148,"ancestor_record_offset":14624,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":92,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":14624,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":15728,"parent_record_offset":15700,"ancestor_record_offset":14624,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":16192,"parent_record_offset":16004,"ancestor_record_offset":14624,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":15416,"parent_record_offset":15384,"ancestor_record_offset":14624,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":16280,"parent_record_offset":16252,"ancestor_record_offset":14624,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":18180,"parent_record_offset":18144,"ancestor_record_offset":14624,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":15632,"parent_record_offset":15600,"ancestor_record_offset":14624,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":18056,"parent_record_offset":17996,"ancestor_record_offset":14624,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":15440,"parent_record_offset":15416,"ancestor_record_offset":14624,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":15656,"parent_record_offset":15632,"ancestor_record_offset":14624,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":16216,"parent_record_offset":16192,"ancestor_record_offset":14624,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":18204,"parent_record_offset":18180,"ancestor_record_offset":14624,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":16304,"parent_record_offset":16280,"ancestor_record_offset":14624,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":18236,"parent_record_offset":18144,"ancestor_record_offset":14624,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":92,"record_offset":26708,"parent_record_offset":26660,"ancestor_record_offset":14624,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":92,"record_offset":19504,"parent_record_offset":19416,"ancestor_record_offset":14624,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":92,"record_offset":19416,"parent_record_offset":19392,"ancestor_record_offset":14624,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":92,"record_offset":19528,"parent_record_offset":19504,"ancestor_record_offset":14624,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::last_os_error
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error.rs
//     evidence={"module":92,"record_offset":25304,"parent_record_offset":24280,"ancestor_record_offset":14624,"inlinee":6877,"source_line":642,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":92,"record_offset":16548,"parent_record_offset":16516,"ancestor_record_offset":14624,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":92,"record_offset":16516,"parent_record_offset":16488,"ancestor_record_offset":14624,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":92,"record_offset":15600,"parent_record_offset":15572,"ancestor_record_offset":14624,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":92,"record_offset":15572,"parent_record_offset":15544,"ancestor_record_offset":14624,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":92,"record_offset":15276,"parent_record_offset":15252,"ancestor_record_offset":14624,"inlinee":7804,"source_line":915,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":92,"record_offset":16004,"parent_record_offset":15968,"ancestor_record_offset":14624,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":92,"record_offset":15700,"parent_record_offset":15504,"ancestor_record_offset":14624,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":92,"record_offset":15504,"parent_record_offset":15476,"ancestor_record_offset":14624,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":92,"record_offset":15476,"parent_record_offset":15348,"ancestor_record_offset":14624,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":92,"record_offset":15968,"parent_record_offset":15940,"ancestor_record_offset":14624,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":92,"record_offset":15384,"parent_record_offset":15348,"ancestor_record_offset":14624,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":92,"record_offset":16252,"parent_record_offset":15968,"ancestor_record_offset":14624,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":92,"record_offset":15544,"parent_record_offset":15504,"ancestor_record_offset":14624,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::event_loop::run_event_loop::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/event_loop.rs
//     evidence={"module":92,"record_offset":18488,"parent_record_offset":18456,"ancestor_record_offset":14624,"inlinee":11218,"source_line":425,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":92,"record_offset":24116,"parent_record_offset":24088,"ancestor_record_offset":14624,"inlinee":5984,"source_line":281,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":92,"record_offset":19068,"parent_record_offset":19040,"ancestor_record_offset":14624,"inlinee":5953,"source_line":109,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Position::to_physical
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":92,"record_offset":24796,"parent_record_offset":24280,"ancestor_record_offset":14624,"inlinee":6862,"source_line":563,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$17::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":92,"record_offset":25048,"parent_record_offset":25024,"ancestor_record_offset":14624,"inlinee":6866,"source_line":296,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":92,"record_offset":19040,"parent_record_offset":19016,"ancestor_record_offset":14624,"inlinee":5972,"source_line":466,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":92,"record_offset":24088,"parent_record_offset":24060,"ancestor_record_offset":14624,"inlinee":5982,"source_line":573,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":92,"record_offset":18988,"parent_record_offset":18956,"ancestor_record_offset":14624,"inlinee":5988,"source_line":51,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::set_control_flow
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":92,"record_offset":15052,"parent_record_offset":15028,"ancestor_record_offset":14624,"inlinee":11207,"source_line":517,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::runner::EventLoopRunner::set_control_flow
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop/runner.rs
//     evidence={"module":92,"record_offset":15076,"parent_record_offset":15052,"ancestor_record_offset":14624,"inlinee":11209,"source_line":150,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":92,"record_offset":14932,"parent_record_offset":14908,"ancestor_record_offset":14624,"inlinee":11205,"source_line":1023,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":92,"record_offset":25276,"parent_record_offset":24280,"ancestor_record_offset":14624,"inlinee":6012,"source_line":348,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":92,"record_offset":17868,"parent_record_offset":17840,"ancestor_record_offset":14624,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":92,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":14624,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":92,"record_offset":24336,"parent_record_offset":24280,"ancestor_record_offset":14624,"inlinee":6848,"source_line":456,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":92,"record_offset":24280,"parent_record_offset":24248,"ancestor_record_offset":14624,"inlinee":11222,"source_line":461,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_minimized
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":92,"record_offset":17924,"parent_record_offset":17896,"ancestor_record_offset":14624,"inlinee":11213,"source_line":638,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":92,"record_offset":18512,"parent_record_offset":18488,"ancestor_record_offset":14624,"inlinee":11202,"source_line":1488,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_minimized::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":92,"record_offset":17896,"parent_record_offset":17868,"ancestor_record_offset":14624,"inlinee":11212,"source_line":1019,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$13::set_cursor_position::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":92,"record_offset":24248,"parent_record_offset":24212,"ancestor_record_offset":14624,"inlinee":11221,"source_line":1380,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::available_monitors::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":92,"record_offset":18364,"parent_record_offset":18340,"ancestor_record_offset":14624,"inlinee":11216,"source_line":1500,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":92,"record_offset":14908,"parent_record_offset":14884,"ancestor_record_offset":14624,"inlinee":11204,"source_line":1490,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3D40
// 首条位置: {"module":240,"inlinee":15680}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":240,"record_offset":67588,"parent_record_offset":67420,"ancestor_record_offset":67420,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_direct] egui::context::impl$37::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":240,"record_offset":67460,"parent_record_offset":67420,"ancestor_record_offset":67420,"inlinee":7761,"source_line":678,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":240,"record_offset":67488,"parent_record_offset":67460,"ancestor_record_offset":67420,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":240,"record_offset":67516,"parent_record_offset":67488,"ancestor_record_offset":67420,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":240,"record_offset":67544,"parent_record_offset":67516,"ancestor_record_offset":67420,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::impl$5::partial_cmp
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38B3
// 首条位置: {"module":194,"inlinee":14515}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$58::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":65820,"parent_record_offset":65796,"ancestor_record_offset":65796,"inlinee":14516,"source_line":1820,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::EguiEventIo::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":92,"record_offset":9532,"rva":2547280,"va":5371256400}
// 编译实例 RVAs: 0x26de50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d1dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3554814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256400,"raw_hex":"e8fe3d3600","size":5,"soft_fail":false,"target_va":5374811584,"va":5371256765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3824937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256400,"raw_hex":"e8295d3a00","size":5,"soft_fail":false,"target_va":5375081488,"va":5371256546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40590692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256400,"raw_hex":"e8645d6b02","size":5,"soft_fail":false,"target_va":5411847168,"va":5371256471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40284355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256400,"raw_hex":"e8c3b06602","size":5,"soft_fail":false,"target_va":5411540992,"va":5371256632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10301953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256400,"raw_hex":"e801329d00","size":5,"soft_fail":false,"target_va":5381558640,"va":5371256682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":92,"record_offset":9692,"parent_record_offset":9532,"ancestor_record_offset":9532,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":9920,"parent_record_offset":9532,"ancestor_record_offset":9532,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":92,"record_offset":9664,"parent_record_offset":9532,"ancestor_record_offset":9532,"inlinee":6795,"source_line":298,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":92,"record_offset":9816,"parent_record_offset":9792,"ancestor_record_offset":9532,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":92,"record_offset":9840,"parent_record_offset":9816,"ancestor_record_offset":9532,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":92,"record_offset":9760,"parent_record_offset":9724,"ancestor_record_offset":9532,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":92,"record_offset":9792,"parent_record_offset":9760,"ancestor_record_offset":9532,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":92,"record_offset":9724,"parent_record_offset":9692,"ancestor_record_offset":9532,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":92,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":9532,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":9944,"parent_record_offset":9920,"ancestor_record_offset":9532,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":9868,"parent_record_offset":9840,"ancestor_record_offset":9532,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":10004,"parent_record_offset":9968,"ancestor_record_offset":9532,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":10028,"parent_record_offset":10004,"ancestor_record_offset":9532,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":10060,"parent_record_offset":9968,"ancestor_record_offset":9532,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::EguiEventIo::take_egui_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":92,"record_offset":10256,"rva":2547664,"va":5371256784}
// 编译实例 RVAs: 0x26dfd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2274152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e8984cddff","size":5,"soft_fail":false,"target_va":5368983552,"va":5371257699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1883778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e87e41e3ff","size":5,"soft_fail":false,"target_va":5369374080,"va":5371257853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3556331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e8eb433600","size":5,"soft_fail":false,"target_va":5374814320,"va":5371257984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a1640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40056015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e8cf346302","size":5,"soft_fail":false,"target_va":5411313216,"va":5371257196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40283988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e854af6602","size":5,"soft_fail":false,"target_va":5411541584,"va":5371257591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40285311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e87fb46602","size":5,"soft_fail":false,"target_va":5411542880,"va":5371257564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40286216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e808b86602","size":5,"soft_fail":false,"target_va":5411543360,"va":5371257139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui_winit::State::take_egui_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui-winit","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4224390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e886754000","size":5,"soft_fail":false,"target_va":5375481712,"va":5371257317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10128254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e87e8b9a00","size":5,"soft_fail":false,"target_va":5381385536,"va":5371257277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10133472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e8e09f9a00","size":5,"soft_fail":false,"target_va":5381390640,"va":5371257163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10129068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e8ac8e9a00","size":5,"soft_fail":false,"target_va":5381386480,"va":5371257407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10127804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e8bc899a00","size":5,"soft_fail":false,"target_va":5381384704,"va":5371256895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40281115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e81ba46602","size":5,"soft_fail":false,"target_va":5411538608,"va":5371257488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40281285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e8c5a46602","size":5,"soft_fail":false,"target_va":5411538768,"va":5371257478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40281381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e825a56602","size":5,"soft_fail":false,"target_va":5411538960,"va":5371257574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40279457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371256784,"raw_hex":"e8a19d6602","size":5,"soft_fail":false,"target_va":5411537104,"va":5371257642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":92,"record_offset":10416,"parent_record_offset":10256,"ancestor_record_offset":10256,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":13000,"parent_record_offset":10256,"ancestor_record_offset":10256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":92,"record_offset":10900,"parent_record_offset":10256,"ancestor_record_offset":10256,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":11920,"parent_record_offset":10256,"ancestor_record_offset":10256,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":10944,"parent_record_offset":10256,"ancestor_record_offset":10256,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":92,"record_offset":10444,"parent_record_offset":10256,"ancestor_record_offset":10256,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":92,"record_offset":12916,"parent_record_offset":10256,"ancestor_record_offset":10256,"inlinee":11201,"source_line":663,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":12296,"parent_record_offset":12268,"ancestor_record_offset":10256,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":12348,"parent_record_offset":12324,"ancestor_record_offset":10256,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":12324,"parent_record_offset":12296,"ancestor_record_offset":10256,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":12236,"parent_record_offset":12204,"ancestor_record_offset":10256,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":12268,"parent_record_offset":12236,"ancestor_record_offset":10256,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":12440,"parent_record_offset":12268,"ancestor_record_offset":10256,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":92,"record_offset":12552,"parent_record_offset":12528,"ancestor_record_offset":10256,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":92,"record_offset":12372,"parent_record_offset":12348,"ancestor_record_offset":10256,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":92,"record_offset":11784,"parent_record_offset":11752,"ancestor_record_offset":10256,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":92,"record_offset":12072,"parent_record_offset":12032,"ancestor_record_offset":10256,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":92,"record_offset":11868,"parent_record_offset":11840,"ancestor_record_offset":10256,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":92,"record_offset":12172,"parent_record_offset":12032,"ancestor_record_offset":10256,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":12504,"parent_record_offset":12204,"ancestor_record_offset":10256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":92,"record_offset":11356,"parent_record_offset":11332,"ancestor_record_offset":10256,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":92,"record_offset":11332,"parent_record_offset":11308,"ancestor_record_offset":10256,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":92,"record_offset":11540,"parent_record_offset":11512,"ancestor_record_offset":10256,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":92,"record_offset":12396,"parent_record_offset":12372,"ancestor_record_offset":10256,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":92,"record_offset":11284,"parent_record_offset":11260,"ancestor_record_offset":10256,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":92,"record_offset":11308,"parent_record_offset":11284,"ancestor_record_offset":10256,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":92,"record_offset":11612,"parent_record_offset":11584,"ancestor_record_offset":10256,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":92,"record_offset":11260,"parent_record_offset":11144,"ancestor_record_offset":10256,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":92,"record_offset":11752,"parent_record_offset":11720,"ancestor_record_offset":10256,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":92,"record_offset":10972,"parent_record_offset":10944,"ancestor_record_offset":10256,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":92,"record_offset":11112,"parent_record_offset":11080,"ancestor_record_offset":10256,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":92,"record_offset":11080,"parent_record_offset":11044,"ancestor_record_offset":10256,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":92,"record_offset":11020,"parent_record_offset":10996,"ancestor_record_offset":10256,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":92,"record_offset":11044,"parent_record_offset":11020,"ancestor_record_offset":10256,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":92,"record_offset":10996,"parent_record_offset":10972,"ancestor_record_offset":10256,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":92,"record_offset":11584,"parent_record_offset":11044,"ancestor_record_offset":10256,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":92,"record_offset":11720,"parent_record_offset":11688,"ancestor_record_offset":10256,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":92,"record_offset":11816,"parent_record_offset":11720,"ancestor_record_offset":10256,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":92,"record_offset":11688,"parent_record_offset":11664,"ancestor_record_offset":10256,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":92,"record_offset":11664,"parent_record_offset":10944,"ancestor_record_offset":10256,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":92,"record_offset":11840,"parent_record_offset":11816,"ancestor_record_offset":10256,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":92,"record_offset":11144,"parent_record_offset":11112,"ancestor_record_offset":10256,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":92,"record_offset":11428,"parent_record_offset":11144,"ancestor_record_offset":10256,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":92,"record_offset":11400,"parent_record_offset":11144,"ancestor_record_offset":10256,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":92,"record_offset":11204,"parent_record_offset":11144,"ancestor_record_offset":10256,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":10820,"parent_record_offset":10792,"ancestor_record_offset":10256,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":14348,"parent_record_offset":14160,"ancestor_record_offset":10256,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":10512,"parent_record_offset":10480,"ancestor_record_offset":10256,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":14440,"parent_record_offset":14408,"ancestor_record_offset":10256,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":10724,"parent_record_offset":10692,"ancestor_record_offset":10256,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":10256,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":10748,"parent_record_offset":10724,"ancestor_record_offset":10256,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":14372,"parent_record_offset":14348,"ancestor_record_offset":10256,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":92,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":10256,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":11956,"parent_record_offset":11920,"ancestor_record_offset":10256,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":13072,"parent_record_offset":13036,"ancestor_record_offset":10256,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":13640,"parent_record_offset":13368,"ancestor_record_offset":10256,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":92,"record_offset":12100,"parent_record_offset":12072,"ancestor_record_offset":10256,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":12204,"parent_record_offset":12032,"ancestor_record_offset":10256,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":92,"record_offset":12476,"parent_record_offset":12204,"ancestor_record_offset":10256,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":92,"record_offset":10692,"parent_record_offset":10664,"ancestor_record_offset":10256,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":92,"record_offset":10664,"parent_record_offset":10636,"ancestor_record_offset":10256,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":92,"record_offset":14160,"parent_record_offset":14120,"ancestor_record_offset":10256,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":92,"record_offset":10792,"parent_record_offset":10600,"ancestor_record_offset":10256,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":92,"record_offset":10600,"parent_record_offset":10572,"ancestor_record_offset":10256,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":92,"record_offset":10572,"parent_record_offset":10444,"ancestor_record_offset":10256,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":92,"record_offset":14120,"parent_record_offset":14088,"ancestor_record_offset":10256,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":92,"record_offset":10480,"parent_record_offset":10444,"ancestor_record_offset":10256,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":92,"record_offset":14408,"parent_record_offset":14120,"ancestor_record_offset":10256,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":92,"record_offset":12128,"parent_record_offset":12100,"ancestor_record_offset":10256,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":92,"record_offset":12032,"parent_record_offset":11992,"ancestor_record_offset":10256,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":92,"record_offset":11992,"parent_record_offset":11956,"ancestor_record_offset":10256,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":92,"record_offset":10636,"parent_record_offset":10600,"ancestor_record_offset":10256,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::run_event_loop::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3C98
// 首条位置: {"module":234,"inlinee":15512}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":6264,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":234,"record_offset":8124,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":234,"record_offset":10512,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":234,"record_offset":8208,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":234,"record_offset":10544,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":234,"record_offset":8180,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":6876,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":234,"record_offset":7732,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":8084,"source_line":607,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":234,"record_offset":6752,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::mouse_capture::OsCursorVisibilityCmdOrigin::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":234,"record_offset":8380,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":11233,"source_line":585,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":234,"record_offset":10428,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":234,"record_offset":7704,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":7835,"source_line":607,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":234,"record_offset":6292,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":234,"record_offset":6792,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":11201,"source_line":663,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] winit::keyboard::ModifiersState::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":234,"record_offset":9944,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":14510,"source_line":703,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":234,"record_offset":7928,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":11202,"source_line":1488,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":234,"record_offset":8968,"parent_record_offset":5808,"ancestor_record_offset":5808,"inlinee":11219,"source_line":1377,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":234,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":5808,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":234,"record_offset":9016,"parent_record_offset":8968,"ancestor_record_offset":5808,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":234,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":5808,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":9344,"parent_record_offset":9312,"ancestor_record_offset":5808,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":234,"record_offset":7756,"parent_record_offset":7732,"ancestor_record_offset":5808,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":6672,"parent_record_offset":6644,"ancestor_record_offset":5808,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":7128,"parent_record_offset":6940,"ancestor_record_offset":5808,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":6360,"parent_record_offset":6328,"ancestor_record_offset":5808,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":7216,"parent_record_offset":7188,"ancestor_record_offset":5808,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":6576,"parent_record_offset":6544,"ancestor_record_offset":5808,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":5808,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":6600,"parent_record_offset":6576,"ancestor_record_offset":5808,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":5808,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":5808,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::from_raw_os_error
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error.rs
//     evidence={"module":234,"record_offset":9840,"parent_record_offset":9816,"ancestor_record_offset":5808,"inlinee":6878,"source_line":674,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::last_os_error
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error.rs
//     evidence={"module":234,"record_offset":9816,"parent_record_offset":9252,"ancestor_record_offset":5808,"inlinee":6877,"source_line":642,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::new_os
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":234,"record_offset":9864,"parent_record_offset":9840,"ancestor_record_offset":5808,"inlinee":6880,"source_line":176,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":234,"record_offset":6544,"parent_record_offset":6516,"ancestor_record_offset":5808,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":234,"record_offset":6516,"parent_record_offset":6488,"ancestor_record_offset":5808,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":234,"record_offset":6940,"parent_record_offset":6904,"ancestor_record_offset":5808,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":234,"record_offset":6644,"parent_record_offset":6448,"ancestor_record_offset":5808,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":234,"record_offset":6448,"parent_record_offset":6420,"ancestor_record_offset":5808,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":234,"record_offset":6420,"parent_record_offset":6292,"ancestor_record_offset":5808,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":234,"record_offset":6904,"parent_record_offset":6876,"ancestor_record_offset":5808,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":234,"record_offset":6328,"parent_record_offset":6292,"ancestor_record_offset":5808,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":234,"record_offset":7188,"parent_record_offset":6904,"ancestor_record_offset":5808,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":234,"record_offset":6488,"parent_record_offset":6448,"ancestor_record_offset":5808,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":234,"record_offset":9072,"parent_record_offset":9044,"ancestor_record_offset":5808,"inlinee":5984,"source_line":281,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":234,"record_offset":8260,"parent_record_offset":8232,"ancestor_record_offset":5808,"inlinee":5953,"source_line":109,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$19::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":234,"record_offset":8232,"parent_record_offset":8208,"ancestor_record_offset":5808,"inlinee":5975,"source_line":308,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":234,"record_offset":9044,"parent_record_offset":9016,"ancestor_record_offset":5808,"inlinee":5982,"source_line":573,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$6::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":234,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":5808,"inlinee":11463,"source_line":145,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::keyboard::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":234,"record_offset":9972,"parent_record_offset":9944,"ancestor_record_offset":5808,"inlinee":14512,"source_line":703,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":234,"record_offset":8000,"parent_record_offset":7976,"ancestor_record_offset":5808,"inlinee":11205,"source_line":1023,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":234,"record_offset":9788,"parent_record_offset":9252,"ancestor_record_offset":5808,"inlinee":6012,"source_line":348,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":234,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":5808,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":234,"record_offset":9312,"parent_record_offset":9252,"ancestor_record_offset":5808,"inlinee":6848,"source_line":456,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":234,"record_offset":9252,"parent_record_offset":9212,"ancestor_record_offset":5808,"inlinee":11222,"source_line":461,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$13::set_cursor_position::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":234,"record_offset":9212,"parent_record_offset":9172,"ancestor_record_offset":5808,"inlinee":11221,"source_line":1380,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":234,"record_offset":7976,"parent_record_offset":7952,"ancestor_record_offset":5808,"inlinee":11204,"source_line":1490,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::run_event_loop::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 421
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x37A0
// 首条位置: {"module":184,"inlinee":14240}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":184,"record_offset":39748,"parent_record_offset":39704,"ancestor_record_offset":39704,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":184,"record_offset":39776,"parent_record_offset":39704,"ancestor_record_offset":39704,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":40008,"parent_record_offset":39704,"ancestor_record_offset":39704,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":40208,"parent_record_offset":40184,"ancestor_record_offset":39704,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":40184,"parent_record_offset":40132,"ancestor_record_offset":39704,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":40156,"parent_record_offset":40132,"ancestor_record_offset":39704,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":40132,"parent_record_offset":40108,"ancestor_record_offset":39704,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":184,"record_offset":40108,"parent_record_offset":40088,"ancestor_record_offset":39704,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":184,"record_offset":39864,"parent_record_offset":39836,"ancestor_record_offset":39704,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":184,"record_offset":39892,"parent_record_offset":39864,"ancestor_record_offset":39704,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":184,"record_offset":39808,"parent_record_offset":39776,"ancestor_record_offset":39704,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":184,"record_offset":39836,"parent_record_offset":39808,"ancestor_record_offset":39704,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":40028,"parent_record_offset":40008,"ancestor_record_offset":39704,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":184,"record_offset":39920,"parent_record_offset":39892,"ancestor_record_offset":39704,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":184,"record_offset":39948,"parent_record_offset":39920,"ancestor_record_offset":39704,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::run_event_loop::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 425
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2BD2
// 首条位置: {"module":92,"inlinee":11218}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":92,"record_offset":18512,"parent_record_offset":18488,"ancestor_record_offset":18488,"inlinee":11202,"source_line":1488,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":92,"record_offset":18584,"parent_record_offset":18560,"ancestor_record_offset":18488,"inlinee":11205,"source_line":1023,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":92,"record_offset":18536,"parent_record_offset":18512,"ancestor_record_offset":18488,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00092.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":92,"record_offset":18560,"parent_record_offset":18536,"ancestor_record_offset":18488,"inlinee":11204,"source_line":1490,"archive":"_pdb/B/modules/00092.jsonl.gz"}
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
// 完整结构及原始类型记录: event_loop.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::event_loop::CursorRequestId
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop::join_head_and_tail_wrapping<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop::join_head_and_tail_wrapping", "arguments": ["tiny_glade::event_loop::CursorRequestId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["tiny_glade::event_loop::CursorRequestId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::wrap_copy<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::wrap_copy", "arguments": ["tiny_glade::event_loop::CursorRequestId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::drain::Drain<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::drain::Drain", "arguments": ["tiny_glade::event_loop::CursorRequestId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::drain::Drain<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::drain::Drain<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::event_loop::EguiEventIo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::event_loop::EguiEventIo>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::event_loop::EguiEventIo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::event_loop::GameEvent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 26
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::event_loop::GameEvent>", "constructor": "enum2$", "arguments": ["tiny_glade::event_loop::GameEvent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::impl$18::fmt<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpsc::impl$18::fmt", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::try_recv<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::list::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::error::SendTimeoutError", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "enum2$", "arguments": ["std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::try_recv<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::array::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::GameEvent> >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::GameEvent> >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::impl$20::drop::closure_env$2", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::GameEvent> >,std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::GameEvent> >", "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::try_recv<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::event_loop::GameEvent> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::event_loop::GameEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tiny_glade::event_loop::GameEvent> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tiny_glade::event_loop::GameEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tiny_glade::event_loop::GameEvent> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tiny_glade::event_loop::GameEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Sender<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpsc::Sender", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Sender<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SendError<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpsc::SendError", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SendError<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SendError<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::event_loop::OsWindowEvent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 30
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::event_loop::OsWindowEvent>", "constructor": "enum2$", "arguments": ["tiny_glade::event_loop::OsWindowEvent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::try_recv<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::list::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::error::SendTimeoutError", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "enum2$", "arguments": ["std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::try_recv<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::array::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::impl$20::drop::closure_env$2", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >,std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "std::sync::mpmc::impl$20::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::try_recv<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::zero::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::OsWindowEvent> > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::OsWindowEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::OsWindowEvent> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::OsWindowEvent> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::event_loop::OsWindowEvent> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<tiny_glade::event_loop::OsWindowEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::impl$7::drop::DropGuard<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global>", "constructor": "alloc::vec::drain::impl$7::drop::DropGuard", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::impl$7::drop::DropGuard<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::impl$7::drop::DropGuard<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Sender<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpsc::Sender", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Sender<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SendError<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpsc::SendError", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SendError<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SendError<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: event_loop.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: event_loop.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::EguiEventIo::new
// 信息 ID: 95d79b4effeed36a58252078
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x59468c -> 0x26de50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::EguiEventIo::take_egui_input
// 信息 ID: 8428eaacad6f14d3e688c2a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4ded3f -> 0x26dfd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::impl$1::clone
// 信息 ID: 8268b24774977b8d05651a9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=240, inline_record=67420; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=240, inline_record=74536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::impl$5::partial_cmp
// 信息 ID: 7a14e08c59a6231f672e2bb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=65796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::run_event_loop
// 信息 ID: 8f04498bf3876e5743a0ddea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x59913d -> 0x26e4a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::run_event_loop::closure$0
// 信息 ID: 8a19651cfae970eb0a68ea48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::event_loop::run_event_loop", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::event_loop::run_event_loop::closure_env$0 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tiny_glade::event_loop::run_event_loop::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::event_loop::run_event_loop::closure_env$0 <- core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tiny_glade::event_loop::run_event_loop::closure_env$0>,tuple$<enum2$<winit::event::Event<tuple$<> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::event_loop::run_event_loop::closure_env$0 <- winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tiny_glade::event_loop::run_event_loop::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tiny_glade::event_loop::run_event_loop::closure_env$0>; module=234, inline_record=5808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::run_event_loop::closure$2
// 信息 ID: 6076fd53c6e3541e96ef82b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::event_loop::run_event_loop", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::event_loop::run_event_loop::closure_env$2 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::event_loop::run_event_loop::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::event_loop::run_event_loop::closure_env$2 <- alloc::collections::vec_deque::into_iter::impl$2::try_fold<winit::platform_impl::platform::monitor::MonitorHandle,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::event_loop::run_event_loop::closure_env$2> >,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::event_loop::run_event_loop::closure_env$2> > >; module=184, inline_record=39704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::run_event_loop::closure$3
// 信息 ID: 57d4e9143b502ac4fea38022
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::event_loop::run_event_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::event_loop::run_event_loop; module=92, inline_record=18488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

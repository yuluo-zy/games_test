// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/winit_event_loop.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 581baf81746a8f0bad5b279b4d99b3bc09953be8ff15ff8336a86880093e0740
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: winit_event_loop.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 120, 180, 228
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use smol_str; // namespace candidate
//   package identities: [{"name":"smol_str","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use winit; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.11.0"},{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_event_loop::run_event_loop
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":180,"record_offset":496,"rva":5374608,"va":5374083728}
// 编译实例 RVAs: 0x520290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42527703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8d7eb8802","size":5,"soft_fail":false,"target_va":5416616304,"va":5374088596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10564066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8e231a100","size":5,"soft_fail":false,"target_va":5384648032,"va":5374083961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..vec_deque..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::try_fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1092100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8fc55efff","size":5,"soft_fail":false,"target_va":5372992976,"va":5374085071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t10523840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8c094a000","size":5,"soft_fail":false,"target_va":5384609408,"va":5374085563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t1593582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8ee501800","size":5,"soft_fail":false,"target_va":5375678704,"va":5374085117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140085b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4833755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8253eb6ff","size":5,"soft_fail":false,"target_va":5369256768,"va":5374090518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140088c40
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4819733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8eb74b6ff","size":5,"soft_fail":false,"target_va":5369269312,"va":5374089040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=9
//     target_aliases=3 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4745940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e82c95b7ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374084783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009f150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4725714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e82ee4b7ff","size":5,"soft_fail":false,"target_va":5369360720,"va":5374086429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a5380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4698862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8124db8ff","size":5,"soft_fail":false,"target_va":5369385856,"va":5374084713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400adf70
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4662400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e880dbb8ff","size":5,"soft_fail":false,"target_va":5369421680,"va":5374084075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c26a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4583326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e86210baff","size":5,"soft_fail":false,"target_va":5369505440,"va":5374088761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c3eb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4577179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e86528baff","size":5,"soft_fail":false,"target_va":5369511600,"va":5374088774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140741180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2230723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8c3092200","size":5,"soft_fail":false,"target_va":5376315776,"va":5374085048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140957020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4418739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8b36c4300","size":5,"soft_fail":false,"target_va":5378502688,"va":5374083944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f33320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10558999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8171ea100","size":5,"soft_fail":false,"target_va":5384647456,"va":5374088452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bec0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42970754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e882ae8f02","size":5,"soft_fail":false,"target_va":5417057984,"va":5374087225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow::h581f7130cbfacad7 (.llvm.12452156013871766364)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t603872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8e0360900","size":5,"soft_fail":false,"target_va":5374690992,"va":5374087115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9551172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e844bd9100","size":5,"soft_fail":false,"target_va":5383637632,"va":5374086455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10535736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e838c3a000","size":5,"soft_fail":false,"target_va":5384624080,"va":5374088339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_capture::CursorVisibilityState::mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4441160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e848c44300","size":5,"soft_fail":false,"target_va":5378529680,"va":5374088515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_capture::CursorVisibilityState::os_cursor_visible
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4441948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e85cc74300","size":5,"soft_fail":false,"target_va":5378529600,"va":5374087647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::mouse_capture::CustomCursorSsboGpuData::update_visibility
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4443441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e831cd4300","size":5,"soft_fail":false,"target_va":5378529824,"va":5374086378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42531084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e80cf98802","size":5,"soft_fail":false,"target_va":5416619696,"va":5374088607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::Sender$LT$T$GT$::send
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-4385888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8a013bdff","size":5,"soft_fail":false,"target_va":5369698176,"va":5374084059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t207127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e817290300","size":5,"soft_fail":false,"target_va":5374291584,"va":5374084452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4271057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e82fd4beff","size":5,"soft_fail":false,"target_va":5369813376,"va":5374084428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1457289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e877c3e9ff","size":5,"soft_fail":false,"target_va":5372627184,"va":5374084468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::io::error::windows::errno
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10660639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e81faba200","size":5,"soft_fail":false,"target_va":5384748160,"va":5374087516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10639573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8d558a200","size":5,"soft_fail":false,"target_va":5384728160,"va":5374088582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::icc::get_monitor_icc_profile
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3358135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e849c2ccff","size":5,"soft_fail":false,"target_va":5370729968,"va":5374088098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::winit_event_loop::extract_available_monitors_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8771a0000","size":5,"soft_fail":false,"target_va":5374090784,"va":5374084004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::winit_setup::DesktopModes::ensure
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-328971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8f5fafaff","size":5,"soft_fail":false,"target_va":5373756624,"va":5374085590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::winit_setup::find_best_fullscreen_video_mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-321114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8a619fbff","size":5,"soft_fail":false,"target_va":5373764064,"va":5374085173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1151244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8f46eeeff","size":5,"soft_fail":false,"target_va":5372933088,"va":5374084327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1146323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e82d82eeff","size":5,"soft_fail":false,"target_va":5372938288,"va":5374084606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::VideoMode::size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2235453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e83d1c2200","size":5,"soft_fail":false,"target_va":5376320688,"va":5374085230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::current_monitor
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2236454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e826202200","size":5,"soft_fail":false,"target_va":5376320320,"va":5374083861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::is_minimized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2235054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8ae1a2200","size":5,"soft_fail":false,"target_va":5376319952,"va":5374084893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2221655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e857e62100","size":5,"soft_fail":false,"target_va":5376308960,"va":5374087300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::Window::set_cursor_grab
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t14786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8c2390000","size":5,"soft_fail":false,"target_va":5374099552,"va":5374084761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::Window::set_cursor_visible
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t12510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8de300000","size":5,"soft_fail":false,"target_va":5374100256,"va":5374087741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::Window::try_set_fullscreen
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2198809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374083728,"raw_hex":"e8198d2100","size":5,"soft_fail":false,"target_va":5376284480,"va":5374085666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*52074932(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374083728,"raw_hex":"ff15b4991a03","size":6,"soft_fail":false,"va":5374087478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":7284,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":15291,"source_line":1808,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":1684,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":5253,"source_line":2205,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":180,"record_offset":7372,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":15295,"source_line":239,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":832,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":12059,"source_line":165,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":180,"record_offset":888,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":180,"record_offset":8332,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":15279,"source_line":1446,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":180,"record_offset":3832,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":180,"record_offset":2788,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":180,"record_offset":7256,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":180,"record_offset":2844,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":180,"record_offset":1628,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":180,"record_offset":944,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":180,"record_offset":5392,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":11666,"source_line":646,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":180,"record_offset":3888,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":180,"record_offset":3748,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":180,"record_offset":1196,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":7498,"source_line":789,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":180,"record_offset":916,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::winit_setup::impl$2::from_winit
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_setup.rs
//     evidence={"module":180,"record_offset":3400,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":10977,"source_line":262,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::set_control_flow
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":180,"record_offset":1000,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":15460,"source_line":351,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":180,"record_offset":1168,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":11386,"source_line":183,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":2704,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":15467,"source_line":1499,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":720,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":12872,"source_line":1488,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":5560,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":12879,"source_line":1377,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_minimized
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":2228,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":15463,"source_line":1017,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":1264,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":13599,"source_line":940,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":496,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":496,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":496,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":3264,"parent_record_offset":3160,"ancestor_record_offset":496,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":180,"record_offset":9776,"parent_record_offset":9752,"ancestor_record_offset":496,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":7576,"parent_record_offset":7372,"ancestor_record_offset":496,"inlinee":12111,"source_line":208,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":1792,"parent_record_offset":1760,"ancestor_record_offset":496,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":1760,"parent_record_offset":1716,"ancestor_record_offset":496,"inlinee":5255,"source_line":236,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":496,"inlinee":11703,"source_line":781,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":2076,"parent_record_offset":1904,"ancestor_record_offset":496,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::push_back_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":1716,"parent_record_offset":1684,"ancestor_record_offset":496,"inlinee":5254,"source_line":2223,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":1904,"parent_record_offset":1716,"ancestor_record_offset":496,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":1940,"parent_record_offset":1904,"ancestor_record_offset":496,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::Drain::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":180,"record_offset":7312,"parent_record_offset":7284,"ancestor_record_offset":496,"inlinee":15293,"source_line":37,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":180,"record_offset":7848,"parent_record_offset":7820,"ancestor_record_offset":496,"inlinee":4882,"source_line":94,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":180,"record_offset":7904,"parent_record_offset":7876,"ancestor_record_offset":496,"inlinee":4884,"source_line":115,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":1824,"parent_record_offset":1792,"ancestor_record_offset":496,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":496,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":1856,"parent_record_offset":1824,"ancestor_record_offset":496,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":3196,"parent_record_offset":3160,"ancestor_record_offset":496,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":3160,"parent_record_offset":3132,"ancestor_record_offset":496,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":496,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":496,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":3132,"parent_record_offset":3104,"ancestor_record_offset":496,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_inner_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":180,"record_offset":2476,"parent_record_offset":2352,"ancestor_record_offset":496,"inlinee":10497,"source_line":309,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":180,"record_offset":1400,"parent_record_offset":1368,"ancestor_record_offset":496,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":180,"record_offset":1368,"parent_record_offset":1336,"ancestor_record_offset":496,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":180,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":496,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":180,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":496,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":180,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":496,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":180,"record_offset":3460,"parent_record_offset":3400,"ancestor_record_offset":496,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$106::from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":180,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":496,"inlinee":12892,"source_line":79,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":180,"record_offset":3560,"parent_record_offset":3536,"ancestor_record_offset":496,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::math::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":180,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":496,"inlinee":12865,"source_line":1780,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":180,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":496,"inlinee":5091,"source_line":115,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":180,"record_offset":1120,"parent_record_offset":1096,"ancestor_record_offset":496,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":180,"record_offset":3232,"parent_record_offset":3196,"ancestor_record_offset":496,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":180,"record_offset":7468,"parent_record_offset":7440,"ancestor_record_offset":496,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":180,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":496,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":180,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":496,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":180,"record_offset":1436,"parent_record_offset":1400,"ancestor_record_offset":496,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":180,"record_offset":2188,"parent_record_offset":1904,"ancestor_record_offset":496,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":180,"record_offset":5416,"parent_record_offset":5392,"ancestor_record_offset":496,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":6316,"parent_record_offset":6128,"ancestor_record_offset":496,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":6408,"parent_record_offset":6376,"ancestor_record_offset":496,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":2568,"parent_record_offset":2532,"ancestor_record_offset":496,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":496,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":1464,"parent_record_offset":1400,"ancestor_record_offset":496,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":496,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":6340,"parent_record_offset":6316,"ancestor_record_offset":496,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":496,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":6432,"parent_record_offset":6408,"ancestor_record_offset":496,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":2624,"parent_record_offset":2532,"ancestor_record_offset":496,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":180,"record_offset":8388,"parent_record_offset":8332,"ancestor_record_offset":496,"inlinee":6433,"source_line":79,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":180,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":496,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":180,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":496,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":180,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":496,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::f64::impl$0::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f64.rs
//     evidence={"module":180,"record_offset":5732,"parent_record_offset":5708,"ancestor_record_offset":496,"inlinee":12863,"source_line":101,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::from_raw_os_error
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":180,"record_offset":7068,"parent_record_offset":7044,"ancestor_record_offset":496,"inlinee":6257,"source_line":687,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::last_os_error
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":180,"record_offset":7044,"parent_record_offset":5980,"ancestor_record_offset":496,"inlinee":6256,"source_line":655,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::new_os
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":180,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":496,"inlinee":6259,"source_line":167,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":180,"record_offset":6216,"parent_record_offset":6188,"ancestor_record_offset":496,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":180,"record_offset":6188,"parent_record_offset":6160,"ancestor_record_offset":496,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpmc\mod.rs
//     evidence={"module":180,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":496,"inlinee":5895,"source_line":916,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":180,"record_offset":6128,"parent_record_offset":6092,"ancestor_record_offset":496,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":180,"record_offset":6092,"parent_record_offset":6064,"ancestor_record_offset":496,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":180,"record_offset":6376,"parent_record_offset":6092,"ancestor_record_offset":496,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":180,"record_offset":6160,"parent_record_offset":6128,"ancestor_record_offset":496,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::winit_event_loop::run_event_loop::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_event_loop.rs
//     evidence={"module":180,"record_offset":2876,"parent_record_offset":2844,"ancestor_record_offset":496,"inlinee":15470,"source_line":396,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":180,"record_offset":5656,"parent_record_offset":5628,"ancestor_record_offset":496,"inlinee":12883,"source_line":281,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":180,"record_offset":3512,"parent_record_offset":3484,"ancestor_record_offset":496,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Position::to_physical
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":180,"record_offset":6480,"parent_record_offset":5980,"ancestor_record_offset":496,"inlinee":12890,"source_line":563,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$17::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":180,"record_offset":6760,"parent_record_offset":6736,"ancestor_record_offset":496,"inlinee":12894,"source_line":296,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":180,"record_offset":3592,"parent_record_offset":3512,"ancestor_record_offset":496,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":180,"record_offset":3484,"parent_record_offset":3460,"ancestor_record_offset":496,"inlinee":10979,"source_line":466,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":180,"record_offset":5628,"parent_record_offset":5600,"ancestor_record_offset":496,"inlinee":12881,"source_line":573,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$5::from_f64
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":180,"record_offset":5708,"parent_record_offset":5684,"ancestor_record_offset":496,"inlinee":12885,"source_line":140,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":180,"record_offset":3432,"parent_record_offset":3400,"ancestor_record_offset":496,"inlinee":5416,"source_line":51,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::set_control_flow
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":180,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":496,"inlinee":15461,"source_line":517,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::runner::EventLoopRunner::set_control_flow
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop/runner.rs
//     evidence={"module":180,"record_offset":1048,"parent_record_offset":1024,"ancestor_record_offset":496,"inlinee":15462,"source_line":150,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":496,"inlinee":12875,"source_line":1023,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":7016,"parent_record_offset":5980,"ancestor_record_offset":496,"inlinee":12895,"source_line":348,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":496,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":496,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":6032,"parent_record_offset":5980,"ancestor_record_offset":496,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":5980,"parent_record_offset":5952,"ancestor_record_offset":496,"inlinee":12888,"source_line":461,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_minimized
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":2312,"parent_record_offset":2284,"ancestor_record_offset":496,"inlinee":15466,"source_line":638,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":496,"inlinee":13602,"source_line":147,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":496,"inlinee":12872,"source_line":1488,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_minimized::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":2284,"parent_record_offset":2256,"ancestor_record_offset":496,"inlinee":15465,"source_line":1019,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":496,"inlinee":13601,"source_line":942,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$13::set_cursor_position::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":5952,"parent_record_offset":5920,"ancestor_record_offset":496,"inlinee":12887,"source_line":1380,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::available_monitors::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":496,"inlinee":15469,"source_line":1500,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":496,"inlinee":12874,"source_line":1490,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_event_loop::run_event_loop::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x322D
// 首条位置: {"module":120,"inlinee":12845}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":120,"record_offset":15312,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":120,"record_offset":17452,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":120,"record_offset":21176,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":120,"record_offset":17564,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":120,"record_offset":17508,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":120,"record_offset":17172,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":120,"record_offset":17424,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":12876,"source_line":1964,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":120,"record_offset":16824,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":120,"record_offset":16564,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":120,"record_offset":16508,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":11666,"source_line":646,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":120,"record_offset":21092,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\mpsc.rs
//     evidence={"module":120,"record_offset":16480,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":12867,"source_line":607,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::dpi_scaling::platform::logical_size_for_windowed_resolution
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/dpi_scaling.rs
//     evidence={"module":120,"record_offset":15340,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":12847,"source_line":77,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] winit::keyboard::ModifiersState::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":120,"record_offset":16760,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":12869,"source_line":696,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":17060,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":12872,"source_line":1488,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":18352,"parent_record_offset":14780,"ancestor_record_offset":14780,"inlinee":12879,"source_line":1377,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":120,"record_offset":20912,"parent_record_offset":20884,"ancestor_record_offset":14780,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":120,"record_offset":20940,"parent_record_offset":20912,"ancestor_record_offset":14780,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":120,"record_offset":20968,"parent_record_offset":20940,"ancestor_record_offset":14780,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":120,"record_offset":20884,"parent_record_offset":20856,"ancestor_record_offset":14780,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":120,"record_offset":20856,"parent_record_offset":20828,"ancestor_record_offset":14780,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":120,"record_offset":16952,"parent_record_offset":16924,"ancestor_record_offset":14780,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":120,"record_offset":16924,"parent_record_offset":16892,"ancestor_record_offset":14780,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":120,"record_offset":16104,"parent_record_offset":15916,"ancestor_record_offset":14780,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$106::from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":120,"record_offset":19396,"parent_record_offset":19372,"ancestor_record_offset":14780,"inlinee":12892,"source_line":79,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$115::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":120,"record_offset":16128,"parent_record_offset":16104,"ancestor_record_offset":14780,"inlinee":10983,"source_line":79,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::classify
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":120,"record_offset":16068,"parent_record_offset":15980,"ancestor_record_offset":14780,"inlinee":12859,"source_line":724,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_negative
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":120,"record_offset":16012,"parent_record_offset":15980,"ancestor_record_offset":14780,"inlinee":12857,"source_line":795,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::is_sign_positive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":120,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":14780,"inlinee":12856,"source_line":761,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":120,"record_offset":16036,"parent_record_offset":16012,"ancestor_record_offset":14780,"inlinee":12858,"source_line":1179,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::math::round
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":120,"record_offset":16316,"parent_record_offset":16292,"ancestor_record_offset":14780,"inlinee":12865,"source_line":1780,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":120,"record_offset":17196,"parent_record_offset":17172,"ancestor_record_offset":14780,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":120,"record_offset":21200,"parent_record_offset":21176,"ancestor_record_offset":14780,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":120,"record_offset":17476,"parent_record_offset":17452,"ancestor_record_offset":14780,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":120,"record_offset":15488,"parent_record_offset":15456,"ancestor_record_offset":14780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":120,"record_offset":16984,"parent_record_offset":16952,"ancestor_record_offset":14780,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":120,"record_offset":16532,"parent_record_offset":16508,"ancestor_record_offset":14780,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15740,"parent_record_offset":15552,"ancestor_record_offset":14780,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15832,"parent_record_offset":15800,"ancestor_record_offset":14780,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15672,"parent_record_offset":15640,"ancestor_record_offset":14780,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":17012,"parent_record_offset":16952,"ancestor_record_offset":14780,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15696,"parent_record_offset":15672,"ancestor_record_offset":14780,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15764,"parent_record_offset":15740,"ancestor_record_offset":14780,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":15856,"parent_record_offset":15832,"ancestor_record_offset":14780,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$33::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":120,"record_offset":16864,"parent_record_offset":16824,"ancestor_record_offset":14780,"inlinee":6698,"source_line":37,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$38::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":120,"record_offset":16892,"parent_record_offset":16864,"ancestor_record_offset":14780,"inlinee":6700,"source_line":410,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::f64::impl$0::round
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f64.rs
//     evidence={"module":120,"record_offset":16292,"parent_record_offset":16268,"ancestor_record_offset":14780,"inlinee":12863,"source_line":101,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::from_raw_os_error
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":120,"record_offset":19884,"parent_record_offset":19860,"ancestor_record_offset":14780,"inlinee":6257,"source_line":687,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::last_os_error
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":120,"record_offset":19860,"parent_record_offset":18784,"ancestor_record_offset":14780,"inlinee":6256,"source_line":655,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::new_os
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":120,"record_offset":19908,"parent_record_offset":19884,"ancestor_record_offset":14780,"inlinee":6259,"source_line":167,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":120,"record_offset":15640,"parent_record_offset":15612,"ancestor_record_offset":14780,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":120,"record_offset":15612,"parent_record_offset":15584,"ancestor_record_offset":14780,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":120,"record_offset":15552,"parent_record_offset":15516,"ancestor_record_offset":14780,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":120,"record_offset":15516,"parent_record_offset":15488,"ancestor_record_offset":14780,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":120,"record_offset":15800,"parent_record_offset":15516,"ancestor_record_offset":14780,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":120,"record_offset":15584,"parent_record_offset":15552,"ancestor_record_offset":14780,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::winit_event_loop::run_event_loop::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/winit_event_loop.rs
//     evidence={"module":120,"record_offset":17592,"parent_record_offset":17564,"ancestor_record_offset":14780,"inlinee":12878,"source_line":246,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::LogicalSize::cast
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":16216,"parent_record_offset":15916,"ancestor_record_offset":14780,"inlinee":12861,"source_line":360,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":18444,"parent_record_offset":18420,"ancestor_record_offset":14780,"inlinee":12883,"source_line":281,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalSize::to_logical
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":15916,"parent_record_offset":15340,"ancestor_record_offset":14780,"inlinee":12853,"source_line":431,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":16244,"parent_record_offset":16216,"ancestor_record_offset":14780,"inlinee":10981,"source_line":109,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Position::to_physical
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":19296,"parent_record_offset":18784,"ancestor_record_offset":14780,"inlinee":12890,"source_line":563,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$17::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":19576,"parent_record_offset":19552,"ancestor_record_offset":14780,"inlinee":12894,"source_line":296,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$2::from_f64
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":16268,"parent_record_offset":16244,"ancestor_record_offset":14780,"inlinee":10985,"source_line":125,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":18420,"parent_record_offset":18396,"ancestor_record_offset":14780,"inlinee":12881,"source_line":573,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$5::from_f64
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":18496,"parent_record_offset":18472,"ancestor_record_offset":14780,"inlinee":12885,"source_line":140,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::validate_scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":120,"record_offset":15956,"parent_record_offset":15916,"ancestor_record_offset":14780,"inlinee":12855,"source_line":161,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::keyboard::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":120,"record_offset":16788,"parent_record_offset":16760,"ancestor_record_offset":14780,"inlinee":12871,"source_line":696,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":14780,"inlinee":12875,"source_line":1023,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":19832,"parent_record_offset":18784,"ancestor_record_offset":14780,"inlinee":12895,"source_line":348,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":15400,"parent_record_offset":15372,"ancestor_record_offset":14780,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":15456,"parent_record_offset":15428,"ancestor_record_offset":14780,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":18784,"parent_record_offset":18748,"ancestor_record_offset":14780,"inlinee":12888,"source_line":461,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":15372,"parent_record_offset":15340,"ancestor_record_offset":14780,"inlinee":12848,"source_line":568,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":15428,"parent_record_offset":15400,"ancestor_record_offset":14780,"inlinee":12850,"source_line":569,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$13::set_cursor_position::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":18748,"parent_record_offset":18708,"ancestor_record_offset":14780,"inlinee":12887,"source_line":1380,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":17108,"parent_record_offset":17084,"ancestor_record_offset":14780,"inlinee":12874,"source_line":1490,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_event_loop::run_event_loop::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 246
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x324E
// 首条位置: {"module":120,"inlinee":12878}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_event_loop::run_event_loop::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 392
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x42C9
// 首条位置: {"module":228,"inlinee":17097}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":69588,"parent_record_offset":69532,"ancestor_record_offset":69532,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":69852,"parent_record_offset":69532,"ancestor_record_offset":69532,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":69620,"parent_record_offset":69532,"ancestor_record_offset":69532,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":70316,"parent_record_offset":70292,"ancestor_record_offset":69532,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":70340,"parent_record_offset":70316,"ancestor_record_offset":69532,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":70364,"parent_record_offset":70340,"ancestor_record_offset":69532,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":228,"record_offset":70292,"parent_record_offset":70240,"ancestor_record_offset":69532,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":69796,"parent_record_offset":69772,"ancestor_record_offset":69532,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":69772,"parent_record_offset":69748,"ancestor_record_offset":69532,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":228,"record_offset":69748,"parent_record_offset":69724,"ancestor_record_offset":69532,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":228,"record_offset":69968,"parent_record_offset":69928,"ancestor_record_offset":69532,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":228,"record_offset":70000,"parent_record_offset":69968,"ancestor_record_offset":69532,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":228,"record_offset":69892,"parent_record_offset":69852,"ancestor_record_offset":69532,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":228,"record_offset":69928,"parent_record_offset":69892,"ancestor_record_offset":69532,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":228,"record_offset":69648,"parent_record_offset":69620,"ancestor_record_offset":69532,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":228,"record_offset":70028,"parent_record_offset":70000,"ancestor_record_offset":69532,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":228,"record_offset":70060,"parent_record_offset":70028,"ancestor_record_offset":69532,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_event_loop::run_event_loop::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 396
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3C6E
// 首条位置: {"module":180,"inlinee":15470}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2876,"inlinee":12872,"source_line":1488,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":2876,"inlinee":12875,"source_line":1023,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":2876,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":2876,"inlinee":12874,"source_line":1490,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::winit_event_loop::extract_available_monitors_info
// 记录形式: compiled_procedure; instances=1
// 源码行: 610
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":180,"record_offset":10536,"rva":5381664,"va":5374090784}
// 编译实例 RVAs: 0x521e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10518032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e8107ea000","size":5,"soft_fail":false,"target_va":5384609408,"va":5374091371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1586974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e81e371800","size":5,"soft_fail":false,"target_va":5375678688,"va":5374091709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t1586477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e82d351800","size":5,"soft_fail":false,"target_va":5375678704,"va":5374092222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5369507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e85d11aeff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374091694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4752569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e8477bb7ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5374091412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b47d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4644905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e8d71fb9ff","size":5,"soft_fail":false,"target_va":5369448400,"va":5374093300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b4f00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4643382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e8ca25b9ff","size":5,"soft_fail":false,"target_va":5369450240,"va":5374093617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401d5e60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3458488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e8483acbff","size":5,"soft_fail":false,"target_va":5370633824,"va":5374092307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ff1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2240836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e8bcceddff","size":5,"soft_fail":false,"target_va":5371851168,"va":5374091999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403024e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2227635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e84d02deff","size":5,"soft_fail":false,"target_va":5371864288,"va":5374091918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140549560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t160041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e829710200","size":5,"soft_fail":false,"target_va":5374252384,"va":5374092338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b7090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t610619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e83b510900","size":5,"soft_fail":false,"target_va":5374701712,"va":5374091088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140741180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2224862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e8def22100","size":5,"soft_fail":false,"target_va":5376315776,"va":5374090909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42259049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e869d28402","size":5,"soft_fail":false,"target_va":5416351168,"va":5374092114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: winit_event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42965362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e872998f02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374091689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42528275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e813ee8802","size":5,"soft_fail":false,"target_va":5416620769,"va":5374092489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::BTreeMap$LT$K$C$V$C$A$GT$::insert
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2050421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e88bb6e0ff","size":5,"soft_fail":false,"target_va":5372041376,"va":5374091792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2046201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e807c7e0ff","size":5,"soft_fail":false,"target_va":5372046912,"va":5374093108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42529739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e8cbf38802","size":5,"soft_fail":false,"target_va":5416622208,"va":5374092464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::winit_setup::DesktopModes::ensure
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-335400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e8d8e1faff","size":5,"soft_fail":false,"target_va":5373756624,"va":5374092019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2229218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374090784,"raw_hex":"e8e2032200","size":5,"soft_fail":false,"target_va":5376320352,"va":5374091129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*52070891(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374090784,"raw_hex":"ff15eb891a03","size":6,"soft_fail":false,"va":5374091623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":180,"record_offset":12808,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":11475,"source_line":131,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":180,"record_offset":13108,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":180,"record_offset":10856,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":180,"record_offset":10828,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":180,"record_offset":12724,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":180,"record_offset":13336,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":180,"record_offset":12968,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::impl$16::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":180,"record_offset":10720,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":11548,"source_line":71,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::video_modes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":180,"record_offset":11096,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":15179,"source_line":164,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":10744,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":15467,"source_line":1499,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":11928,"parent_record_offset":11904,"ancestor_record_offset":10536,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":11952,"parent_record_offset":11928,"ancestor_record_offset":10536,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":12580,"parent_record_offset":12556,"ancestor_record_offset":10536,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":12604,"parent_record_offset":12580,"ancestor_record_offset":10536,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":11976,"parent_record_offset":11952,"ancestor_record_offset":10536,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":12628,"parent_record_offset":12604,"ancestor_record_offset":10536,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":11904,"parent_record_offset":11872,"ancestor_record_offset":10536,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":180,"record_offset":12556,"parent_record_offset":12532,"ancestor_record_offset":10536,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":180,"record_offset":11872,"parent_record_offset":11836,"ancestor_record_offset":10536,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":180,"record_offset":11836,"parent_record_offset":11132,"ancestor_record_offset":10536,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":180,"record_offset":12532,"parent_record_offset":12508,"ancestor_record_offset":10536,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":180,"record_offset":12216,"parent_record_offset":12192,"ancestor_record_offset":10536,"inlinee":4643,"source_line":207,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$35::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":180,"record_offset":11404,"parent_record_offset":11372,"ancestor_record_offset":10536,"inlinee":13815,"source_line":1882,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":180,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":10536,"inlinee":4868,"source_line":1904,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::drop_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":180,"record_offset":12352,"parent_record_offset":12320,"ancestor_record_offset":10536,"inlinee":4870,"source_line":1194,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::BTreeSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\set.rs
//     evidence={"module":180,"record_offset":12028,"parent_record_offset":11132,"ancestor_record_offset":10536,"inlinee":15173,"source_line":898,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::BTreeSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\set.rs
//     evidence={"module":180,"record_offset":11260,"parent_record_offset":11132,"ancestor_record_offset":10536,"inlinee":15172,"source_line":344,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::impl$18::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\set.rs
//     evidence={"module":180,"record_offset":11372,"parent_record_offset":11132,"ancestor_record_offset":10536,"inlinee":15175,"source_line":1534,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":11028,"parent_record_offset":10904,"ancestor_record_offset":10536,"inlinee":12111,"source_line":208,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":10904,"parent_record_offset":10880,"ancestor_record_offset":10536,"inlinee":12109,"source_line":2064,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":10936,"parent_record_offset":10904,"ancestor_record_offset":10536,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":180,"record_offset":10960,"parent_record_offset":10936,"ancestor_record_offset":10536,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":180,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":10536,"inlinee":15156,"source_line":47,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":13212,"parent_record_offset":13180,"ancestor_record_offset":10536,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":13716,"parent_record_offset":13688,"ancestor_record_offset":10536,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":13688,"parent_record_offset":13664,"ancestor_record_offset":10536,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":10536,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":13236,"parent_record_offset":13212,"ancestor_record_offset":10536,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":180,"record_offset":13664,"parent_record_offset":13640,"ancestor_record_offset":10536,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":180,"record_offset":12836,"parent_record_offset":12808,"ancestor_record_offset":10536,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":180,"record_offset":13180,"parent_record_offset":13136,"ancestor_record_offset":10536,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":180,"record_offset":13136,"parent_record_offset":13108,"ancestor_record_offset":10536,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":180,"record_offset":12748,"parent_record_offset":12724,"ancestor_record_offset":10536,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":180,"record_offset":13360,"parent_record_offset":13336,"ancestor_record_offset":10536,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ordering::is_le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":180,"record_offset":11516,"parent_record_offset":11492,"ancestor_record_offset":10536,"inlinee":5208,"source_line":518,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":180,"record_offset":11444,"parent_record_offset":11132,"ancestor_record_offset":10536,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":180,"record_offset":11804,"parent_record_offset":11780,"ancestor_record_offset":10536,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":180,"record_offset":12900,"parent_record_offset":12864,"ancestor_record_offset":10536,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":180,"record_offset":12772,"parent_record_offset":12748,"ancestor_record_offset":10536,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":180,"record_offset":12436,"parent_record_offset":12352,"ancestor_record_offset":10536,"inlinee":4641,"source_line":807,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":180,"record_offset":11692,"parent_record_offset":11668,"ancestor_record_offset":10536,"inlinee":12821,"source_line":469,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":180,"record_offset":11668,"parent_record_offset":11132,"ancestor_record_offset":10536,"inlinee":12820,"source_line":681,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":180,"record_offset":13744,"parent_record_offset":13716,"ancestor_record_offset":10536,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":180,"record_offset":10984,"parent_record_offset":10960,"ancestor_record_offset":10536,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":180,"record_offset":11492,"parent_record_offset":11468,"ancestor_record_offset":10536,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":180,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":10536,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":180,"record_offset":12168,"parent_record_offset":11132,"ancestor_record_offset":10536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":180,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":10536,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":180,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":10536,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":180,"record_offset":11052,"parent_record_offset":11028,"ancestor_record_offset":10536,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":180,"record_offset":13300,"parent_record_offset":13136,"ancestor_record_offset":10536,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":180,"record_offset":11740,"parent_record_offset":11716,"ancestor_record_offset":10536,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":180,"record_offset":12380,"parent_record_offset":12352,"ancestor_record_offset":10536,"inlinee":4871,"source_line":684,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":180,"record_offset":12404,"parent_record_offset":12380,"ancestor_record_offset":10536,"inlinee":4872,"source_line":259,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":180,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":10536,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":180,"record_offset":12864,"parent_record_offset":12836,"ancestor_record_offset":10536,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":11312,"parent_record_offset":11288,"ancestor_record_offset":10536,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":180,"record_offset":11336,"parent_record_offset":11312,"ancestor_record_offset":10536,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":180,"record_offset":11604,"parent_record_offset":11580,"ancestor_record_offset":10536,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":180,"record_offset":11556,"parent_record_offset":11132,"ancestor_record_offset":10536,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":180,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":10536,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":180,"record_offset":11628,"parent_record_offset":11604,"ancestor_record_offset":10536,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":180,"record_offset":11288,"parent_record_offset":11132,"ancestor_record_offset":10536,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::video_modes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":180,"record_offset":11132,"parent_record_offset":11096,"ancestor_record_offset":10536,"inlinee":15170,"source_line":229,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::util::has_flag
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/util.rs
//     evidence={"module":180,"record_offset":11780,"parent_record_offset":11132,"ancestor_record_offset":10536,"inlinee":8170,"source_line":53,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":180,"record_offset":10768,"parent_record_offset":10744,"ancestor_record_offset":10536,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00180.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::available_monitors::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":180,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":10536,"inlinee":15469,"source_line":1500,"archive":"_pdb/C/modules/00180.jsonl.gz"}
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
// 完整结构及原始类型记录: winit_event_loop.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: winit_event_loop.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: winit_event_loop.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >,ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >", "ref$<tuple$<tuple$<bool,u32>,country_core::render::render_config::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_event_loop::extract_available_monitors_info
// 信息 ID: c6977a465607397de4a90798
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::winit_event_loop::run_event_loop @ 0x5203a4 -> 0x521e20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_event_loop::run_event_loop
// 信息 ID: 2a8063797cce722c2274051b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x430781 -> 0x520290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_event_loop::run_event_loop::closure$0
// 信息 ID: f58e35b2aedc5119d1b28e36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::winit_event_loop::run_event_loop", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::winit_event_loop::run_event_loop::closure_env$0 <- core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tiny_glade::winit_event_loop::run_event_loop::closure_env$0>,tuple$<enum2$<winit::event::Event<tuple$<> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::winit_event_loop::run_event_loop::closure_env$0 <- winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tiny_glade::winit_event_loop::run_event_loop::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::winit_event_loop::run_event_loop::closure_env$0 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tiny_glade::winit_event_loop::run_event_loop::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tiny_glade::winit_event_loop::run_event_loop::closure_env$0>; module=120, inline_record=14780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_event_loop::run_event_loop::closure$0::closure$1
// 信息 ID: d74e1f73fdbdc805bd343cab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::winit_event_loop::run_event_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tiny_glade::winit_event_loop::run_event_loop::closure_env$0>; module=120, inline_record=17592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_event_loop::run_event_loop::closure$2
// 信息 ID: 5e5d2f68488986365b5c7c92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::winit_event_loop::run_event_loop", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::winit_event_loop::run_event_loop::closure_env$2 <- alloc::collections::vec_deque::into_iter::impl$2::try_fold<winit::platform_impl::platform::monitor::MonitorHandle,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::winit_event_loop::run_event_loop::closure_env$2> >,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::winit_event_loop::run_event_loop::closure_env$2 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::winit_event_loop::run_event_loop::closure_env$2> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::winit_event_loop::run_event_loop::closure_env$2> > >; module=228, inline_record=69532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::winit_event_loop::run_event_loop::closure$3
// 信息 ID: ed88fac1001c4b7eacbba03a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::winit_event_loop::run_event_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::winit_event_loop::run_event_loop; module=180, inline_record=2876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-iced/src/gui_runner_winit.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f0548b9bdc8a6fce768f7c82f410d26ccaa8ad1b878a9defc33cdfb9feb8f3a0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_runner_winit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 120, 184, 945, 990, 1001, 2508, 3746, 3820
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use iced_core; // namespace candidate
//   package identities: [{"name":"iced_core","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_graphics; // namespace candidate
//   package identities: [{"name":"iced_graphics","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_runtime; // namespace candidate
//   package identities: [{"name":"iced_runtime","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_style; // namespace candidate
//   package identities: [{"name":"iced_style","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_tiny_skia; // namespace candidate
//   package identities: [{"name":"iced_tiny_skia","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use iced_winit; // namespace candidate
//   package identities: [{"name":"iced_winit","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380","version":"0.12.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tg_gpu_selector; // namespace candidate
//   package identities: [{"name":"tg-gpu-selector","source":"local_or_path","version":"0.1.0"}]
// use tg_iced; // namespace candidate
//   package identities: [{"name":"tg-iced","source":"local_or_path","version":"0.1.0"}]
// use winit; // namespace candidate
//   package identities: [{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::gpu_selection::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":184,"record_offset":30992,"rva":5449712,"va":5374158832}
// 编译实例 RVAs: 0x5327f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42453938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8b2cb8702","size":5,"soft_fail":false,"target_va":5416616304,"va":5374162361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10488446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e87e0aa000","size":5,"soft_fail":false,"target_va":5384648032,"va":5374159581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1685492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8f4b71900","size":5,"soft_fail":false,"target_va":5375844784,"va":5374159287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1519490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8822f1700","size":5,"soft_fail":false,"target_va":5375678688,"va":5374159193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t1518238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e89e2a1700","size":5,"soft_fail":false,"target_va":5375678704,"va":5374160461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5436991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8c109adff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374159178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007fed0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4928473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e827ccb4ff","size":5,"soft_fail":false,"target_va":5369233104,"va":5374161572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140080710
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4925958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8fad5b4ff","size":5,"soft_fail":false,"target_va":5369235216,"va":5374161169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089690
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4892866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e83e57b5ff","size":5,"soft_fail":false,"target_va":5369271952,"va":5374164813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008c0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4881721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8c782b5ff","size":5,"soft_fail":false,"target_va":5369282736,"va":5374164452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a650
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4818899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e82d78b6ff","size":5,"soft_fail":false,"target_va":5369341520,"va":5374160414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ac970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4748339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8cd8bb7ff","size":5,"soft_fail":false,"target_va":5369416048,"va":5374164382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400af890
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4733485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8d3c5b7ff","size":5,"soft_fail":false,"target_va":5369428112,"va":5374161592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b3050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4720345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e827f9b7ff","size":5,"soft_fail":false,"target_va":5369442384,"va":5374162724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bfcd0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4669486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8d2bfb8ff","size":5,"soft_fail":false,"target_va":5369494736,"va":5374164217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c46b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4647966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8e213b9ff","size":5,"soft_fail":false,"target_va":5369513648,"va":5374161609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401e2420
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3480290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e81ee5caff","size":5,"soft_fail":false,"target_va":5370684448,"va":5374164733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140597b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t414096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e890510600","size":5,"soft_fail":false,"target_va":5374573440,"va":5374159339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c4270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1642783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e81f111900","size":5,"soft_fail":false,"target_va":5375804016,"va":5374161228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42191736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e878cb8302","size":5,"soft_fail":false,"target_va":5416351392,"va":5374159651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42898144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8e0928e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374158907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42458458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e85add8702","size":5,"soft_fail":false,"target_va":5416620769,"va":5374162306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t1654852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e844401900","size":5,"soft_fail":false,"target_va":5375816416,"va":5374161559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42459895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8f7e28702","size":5,"soft_fail":false,"target_va":5416622240,"va":5374162340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > > >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-17460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8ccbbffff","size":5,"soft_fail":false,"target_va":5374143600,"va":5374161055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1777262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e86e1e1b00","size":5,"soft_fail":false,"target_va":5375936352,"va":5374159085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-3494441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8d7adcaff","size":5,"soft_fail":false,"target_va":5370664992,"va":5374159428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-3485729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8dfcfcaff","size":5,"soft_fail":false,"target_va":5370674512,"va":5374160236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1684970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8eab51900","size":5,"soft_fail":false,"target_va":5375844112,"va":5374159137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1682843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e89bad1900","size":5,"soft_fail":false,"target_va":5375842656,"va":5374159808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42457286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8c6d88702","size":5,"soft_fail":false,"target_va":5416619696,"va":5374162405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10566897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8f13ca100","size":5,"soft_fail":false,"target_va":5384728160,"va":5374161258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1227854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8b243edff","size":5,"soft_fail":false,"target_va":5372933648,"va":5374161497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1221226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e8965dedff","size":5,"soft_fail":false,"target_va":5372938288,"va":5374159509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2149951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374158832,"raw_hex":"e83fce2000","size":5,"soft_fail":false,"target_va":5376308960,"va":5374159004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*52003591(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374158832,"raw_hex":"ff1507831903","size":6,"soft_fail":false,"va":5374158955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner_winit::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":184,"record_offset":31180,"parent_record_offset":30992,"ancestor_record_offset":30992,"inlinee":15604,"source_line":33,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":184,"record_offset":31156,"parent_record_offset":30992,"ancestor_record_offset":30992,"inlinee":15602,"source_line":24,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":32672,"parent_record_offset":32648,"ancestor_record_offset":30992,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":32696,"parent_record_offset":32672,"ancestor_record_offset":30992,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":34028,"parent_record_offset":34004,"ancestor_record_offset":30992,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":34052,"parent_record_offset":34028,"ancestor_record_offset":30992,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":32720,"parent_record_offset":32696,"ancestor_record_offset":30992,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":34076,"parent_record_offset":34052,"ancestor_record_offset":30992,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":32648,"parent_record_offset":32616,"ancestor_record_offset":30992,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":34004,"parent_record_offset":33916,"ancestor_record_offset":30992,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":32616,"parent_record_offset":32572,"ancestor_record_offset":30992,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":32572,"parent_record_offset":32536,"ancestor_record_offset":30992,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":34436,"parent_record_offset":34392,"ancestor_record_offset":30992,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":36160,"parent_record_offset":36132,"ancestor_record_offset":30992,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33428,"parent_record_offset":33396,"ancestor_record_offset":30992,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33944,"parent_record_offset":33916,"ancestor_record_offset":30992,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33916,"parent_record_offset":33892,"ancestor_record_offset":30992,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33476,"parent_record_offset":33452,"ancestor_record_offset":30992,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33452,"parent_record_offset":33428,"ancestor_record_offset":30992,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33892,"parent_record_offset":33868,"ancestor_record_offset":30992,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":33180,"parent_record_offset":33152,"ancestor_record_offset":30992,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":32412,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":31780,"parent_record_offset":31748,"ancestor_record_offset":30992,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":32792,"parent_record_offset":32768,"ancestor_record_offset":30992,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":33396,"parent_record_offset":33352,"ancestor_record_offset":30992,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":33324,"parent_record_offset":33296,"ancestor_record_offset":30992,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":33352,"parent_record_offset":33324,"ancestor_record_offset":30992,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":34624,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":33804,"parent_record_offset":33776,"ancestor_record_offset":30992,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":34904,"parent_record_offset":34868,"ancestor_record_offset":30992,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":34868,"parent_record_offset":34840,"ancestor_record_offset":30992,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":35156,"parent_record_offset":35132,"ancestor_record_offset":30992,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":34680,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":34976,"parent_record_offset":34952,"ancestor_record_offset":30992,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":184,"record_offset":36192,"parent_record_offset":36160,"ancestor_record_offset":30992,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":184,"record_offset":34952,"parent_record_offset":34928,"ancestor_record_offset":30992,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":184,"record_offset":34928,"parent_record_offset":34904,"ancestor_record_offset":30992,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":36256,"parent_record_offset":36220,"ancestor_record_offset":30992,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":184,"record_offset":31612,"parent_record_offset":31588,"ancestor_record_offset":30992,"inlinee":12821,"source_line":469,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":36220,"parent_record_offset":36192,"ancestor_record_offset":30992,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":31588,"parent_record_offset":31544,"ancestor_record_offset":30992,"inlinee":12820,"source_line":681,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":184,"record_offset":33972,"parent_record_offset":33944,"ancestor_record_offset":30992,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":184,"record_offset":33672,"parent_record_offset":33648,"ancestor_record_offset":30992,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":33584,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":33648,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":31716,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":184,"record_offset":35048,"parent_record_offset":35024,"ancestor_record_offset":30992,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":31940,"parent_record_offset":31904,"ancestor_record_offset":30992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":34648,"parent_record_offset":34624,"ancestor_record_offset":30992,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":35024,"parent_record_offset":35000,"ancestor_record_offset":30992,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":31636,"parent_record_offset":31612,"ancestor_record_offset":30992,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":34752,"parent_record_offset":34680,"ancestor_record_offset":30992,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":33216,"parent_record_offset":33180,"ancestor_record_offset":30992,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":35000,"parent_record_offset":34976,"ancestor_record_offset":30992,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":34780,"parent_record_offset":34680,"ancestor_record_offset":30992,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":34720,"parent_record_offset":34680,"ancestor_record_offset":30992,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":34804,"parent_record_offset":34780,"ancestor_record_offset":30992,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":33516,"parent_record_offset":33352,"ancestor_record_offset":30992,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":31660,"parent_record_offset":31636,"ancestor_record_offset":30992,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":184,"record_offset":32932,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32200,"parent_record_offset":32012,"ancestor_record_offset":30992,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32292,"parent_record_offset":32260,"ancestor_record_offset":30992,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32440,"parent_record_offset":32412,"ancestor_record_offset":30992,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32828,"parent_record_offset":32792,"ancestor_record_offset":30992,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32132,"parent_record_offset":32100,"ancestor_record_offset":30992,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32464,"parent_record_offset":32440,"ancestor_record_offset":30992,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32156,"parent_record_offset":32132,"ancestor_record_offset":30992,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32224,"parent_record_offset":32200,"ancestor_record_offset":30992,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32852,"parent_record_offset":32828,"ancestor_record_offset":30992,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32316,"parent_record_offset":32292,"ancestor_record_offset":30992,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32884,"parent_record_offset":32792,"ancestor_record_offset":30992,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":184,"record_offset":33696,"parent_record_offset":33672,"ancestor_record_offset":30992,"inlinee":15594,"source_line":7,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":184,"record_offset":34392,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":15597,"source_line":104,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::program::state::State::is_queue_empty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":33556,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":15590,"source_line":74,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":33296,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":9582,"source_line":62,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":184,"record_offset":33732,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":184,"record_offset":32500,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":15587,"source_line":31,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":184,"record_offset":32536,"parent_record_offset":32500,"ancestor_record_offset":30992,"inlinee":15589,"source_line":139,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":184,"record_offset":32100,"parent_record_offset":32072,"ancestor_record_offset":30992,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":184,"record_offset":32072,"parent_record_offset":32044,"ancestor_record_offset":30992,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":184,"record_offset":32012,"parent_record_offset":31972,"ancestor_record_offset":30992,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":184,"record_offset":31972,"parent_record_offset":31940,"ancestor_record_offset":30992,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":184,"record_offset":32260,"parent_record_offset":31972,"ancestor_record_offset":30992,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":184,"record_offset":32044,"parent_record_offset":32012,"ancestor_record_offset":30992,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":184,"record_offset":31748,"parent_record_offset":31716,"ancestor_record_offset":30992,"inlinee":15599,"source_line":46,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":184,"record_offset":33616,"parent_record_offset":33584,"ancestor_record_offset":30992,"inlinee":15600,"source_line":186,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":184,"record_offset":35804,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":11386,"source_line":183,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":31544,"parent_record_offset":31516,"ancestor_record_offset":30992,"inlinee":12819,"source_line":219,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":33072,"parent_record_offset":33048,"ancestor_record_offset":30992,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":31488,"parent_record_offset":31460,"ancestor_record_offset":30992,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":31904,"parent_record_offset":31872,"ancestor_record_offset":30992,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":33120,"parent_record_offset":33096,"ancestor_record_offset":30992,"inlinee":13602,"source_line":147,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":31460,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":12815,"source_line":746,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":31808,"parent_record_offset":31748,"ancestor_record_offset":30992,"inlinee":12848,"source_line":568,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":33048,"parent_record_offset":31180,"ancestor_record_offset":30992,"inlinee":13599,"source_line":940,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":31872,"parent_record_offset":31840,"ancestor_record_offset":30992,"inlinee":12850,"source_line":569,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":31516,"parent_record_offset":31488,"ancestor_record_offset":30992,"inlinee":12818,"source_line":747,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":33096,"parent_record_offset":33072,"ancestor_record_offset":30992,"inlinee":13601,"source_line":942,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::vulkan_config::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":990,"record_offset":5920,"rva":16713904,"va":5385423024}
// 编译实例 RVAs: 0xff08b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31189224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8e8e8db01","size":5,"soft_fail":false,"target_va":5416616304,"va":5385427075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-775890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e82e29f4ff","size":5,"soft_fail":false,"target_va":5384648032,"va":5385423917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9578763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8f5d66dff","size":5,"soft_fail":false,"target_va":5375844784,"va":5385423542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t16164626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e812a7f600","size":5,"soft_fail":false,"target_va":5401588224,"va":5385423593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9744765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8834e6bff","size":5,"soft_fail":false,"target_va":5375678688,"va":5385423448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=18
//     evidence={"assembly":"\tcallq\t-9746098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e84e496bff","size":5,"soft_fail":false,"target_va":5375678704,"va":5385424797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16701246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8c22801ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5385423433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008c0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16146473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8d79f09ff","size":5,"soft_fail":false,"target_va":5369282736,"va":5385429204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a650
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16083235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8dd960aff","size":5,"soft_fail":false,"target_va":5369341520,"va":5385424750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400afd60
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15998221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8f3e20bff","size":5,"soft_fail":false,"target_va":5369429344,"va":5385427560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b3050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15985081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e847160cff","size":5,"soft_fail":false,"target_va":5369442384,"va":5385427460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401e2420
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14745042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e82e021fff","size":5,"soft_fail":false,"target_va":5370684448,"va":5385429485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141f586c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16151542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8f673f600","size":5,"soft_fail":false,"target_va":5401577152,"va":5385425605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30927400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e828ead701","size":5,"soft_fail":false,"target_va":5416351392,"va":5385423987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31193659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e83bfadb01","size":5,"soft_fail":false,"target_va":5416620769,"va":5385427105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-9609616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8705e6dff","size":5,"soft_fail":false,"target_va":5375816416,"va":5385426027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31195181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e82d00dc01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385427054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$iced_runtime..command..action..Action$LT$tg_gpu_selector..vulkan_config..Message$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-88322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8fea6feff","size":5,"soft_fail":false,"target_va":5385341248,"va":5385429565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-80338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e82ec6feff","size":5,"soft_fail":false,"target_va":5385345744,"va":5385426077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..Command$LT$tg_gpu_selector..vulkan_config..Message$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-80750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e892c4feff","size":5,"soft_fail":false,"target_va":5385348224,"va":5385428969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$tg_gpu_selector..vulkan_config..Message$GT$$GT$::hf955e266d794144e (.llvm.1958498355941987001)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16177863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8c7daf600","size":5,"soft_fail":false,"target_va":5401605776,"va":5385427908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$tg_gpu_selector..vulkan_config..Message$GT$$GT$::hf955e266d794144e (.llvm.1981347926437597606)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-87078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8daabfeff","size":5,"soft_fail":false,"target_va":5385338432,"va":5385425505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..program..state..State$LT$tg_gpu_selector..vulkan_config..Controls$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-88141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8b3a7feff","size":5,"soft_fail":false,"target_va":5385337904,"va":5385426040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_tiny_skia..window..compositor..Surface$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-81105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e82fc3feff","size":5,"soft_fail":false,"target_va":5385344960,"va":5385426060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_gpu_selector..vulkan_config..Controls$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-84422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e83ab6feff","size":5,"soft_fail":false,"target_va":5385344720,"va":5385429137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::vulkan_config::Message> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8fdefffff","size":5,"soft_fail":false,"target_va":5385421296,"va":5385425390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-9486993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e86f3d6fff","size":5,"soft_fail":false,"target_va":5375936352,"va":5385423340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t16173452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e88cc9f600","size":5,"soft_fail":false,"target_va":5401597216,"va":5385423759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t16173151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e85fc8f600","size":5,"soft_fail":false,"target_va":5401597728,"va":5385424572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-9579285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8ebd46dff","size":5,"soft_fail":false,"target_va":5375844112,"va":5385423392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-9581493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e84bcc6dff","size":5,"soft_fail":false,"target_va":5375842656,"va":5385424144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31192547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8e3f5db01","size":5,"soft_fail":false,"target_va":5416619696,"va":5385427144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-697540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e83c5bf5ff","size":5,"soft_fail":false,"target_va":5384728160,"va":5385425695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t16175549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8bdd1f600","size":5,"soft_fail":false,"target_va":5401601488,"va":5385425934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t16179393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e8c1e0f600","size":5,"soft_fail":false,"target_va":5401603248,"va":5385423850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-9114297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385423024,"raw_hex":"e847ed74ff","size":5,"soft_fail":false,"target_va":5376308960,"va":5385423252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*40739334(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5385423024,"raw_hex":"ff1506a26d02","size":6,"soft_fail":false,"va":5385423212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner_winit::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":990,"record_offset":6112,"parent_record_offset":5920,"ancestor_record_offset":5920,"inlinee":15604,"source_line":33,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":990,"record_offset":6084,"parent_record_offset":5920,"ancestor_record_offset":5920,"inlinee":15602,"source_line":24,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":990,"record_offset":7624,"parent_record_offset":7600,"ancestor_record_offset":5920,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":990,"record_offset":7648,"parent_record_offset":7624,"ancestor_record_offset":5920,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":990,"record_offset":9008,"parent_record_offset":8984,"ancestor_record_offset":5920,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":990,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":5920,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":990,"record_offset":7672,"parent_record_offset":7648,"ancestor_record_offset":5920,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":990,"record_offset":9056,"parent_record_offset":9032,"ancestor_record_offset":5920,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":990,"record_offset":7600,"parent_record_offset":7568,"ancestor_record_offset":5920,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":990,"record_offset":8984,"parent_record_offset":8896,"ancestor_record_offset":5920,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":990,"record_offset":7568,"parent_record_offset":7524,"ancestor_record_offset":5920,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":990,"record_offset":7524,"parent_record_offset":7488,"ancestor_record_offset":5920,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":990,"record_offset":9416,"parent_record_offset":9372,"ancestor_record_offset":5920,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":990,"record_offset":11300,"parent_record_offset":11272,"ancestor_record_offset":5920,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":990,"record_offset":8408,"parent_record_offset":8376,"ancestor_record_offset":5920,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":990,"record_offset":8924,"parent_record_offset":8896,"ancestor_record_offset":5920,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":990,"record_offset":8896,"parent_record_offset":8872,"ancestor_record_offset":5920,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":990,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":5920,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":990,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":5920,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":990,"record_offset":8872,"parent_record_offset":8848,"ancestor_record_offset":5920,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":990,"record_offset":8160,"parent_record_offset":8132,"ancestor_record_offset":5920,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":990,"record_offset":7364,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":990,"record_offset":6732,"parent_record_offset":6700,"ancestor_record_offset":5920,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":990,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":5920,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":990,"record_offset":8376,"parent_record_offset":8332,"ancestor_record_offset":5920,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":990,"record_offset":8304,"parent_record_offset":8276,"ancestor_record_offset":5920,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":990,"record_offset":8332,"parent_record_offset":8304,"ancestor_record_offset":5920,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":990,"record_offset":9604,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":990,"record_offset":8784,"parent_record_offset":8756,"ancestor_record_offset":5920,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":990,"record_offset":9884,"parent_record_offset":9848,"ancestor_record_offset":5920,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":990,"record_offset":9848,"parent_record_offset":9820,"ancestor_record_offset":5920,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":990,"record_offset":10136,"parent_record_offset":10112,"ancestor_record_offset":5920,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":990,"record_offset":9660,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":990,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":5920,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":990,"record_offset":11332,"parent_record_offset":11300,"ancestor_record_offset":5920,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\clone.rs
//     evidence={"module":990,"record_offset":10868,"parent_record_offset":10836,"ancestor_record_offset":5920,"inlinee":6914,"source_line":236,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":990,"record_offset":9932,"parent_record_offset":9908,"ancestor_record_offset":5920,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":990,"record_offset":9908,"parent_record_offset":9884,"ancestor_record_offset":5920,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":990,"record_offset":11396,"parent_record_offset":11360,"ancestor_record_offset":5920,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":990,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":5920,"inlinee":12821,"source_line":469,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":990,"record_offset":11360,"parent_record_offset":11332,"ancestor_record_offset":5920,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":990,"record_offset":6540,"parent_record_offset":6496,"ancestor_record_offset":5920,"inlinee":12820,"source_line":681,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":990,"record_offset":8952,"parent_record_offset":8924,"ancestor_record_offset":5920,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":990,"record_offset":8652,"parent_record_offset":8628,"ancestor_record_offset":5920,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":990,"record_offset":8564,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":990,"record_offset":8628,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":990,"record_offset":6668,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":990,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":5920,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":990,"record_offset":10028,"parent_record_offset":10004,"ancestor_record_offset":5920,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":990,"record_offset":6892,"parent_record_offset":6856,"ancestor_record_offset":5920,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":990,"record_offset":9628,"parent_record_offset":9604,"ancestor_record_offset":5920,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":990,"record_offset":10004,"parent_record_offset":9980,"ancestor_record_offset":5920,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":990,"record_offset":6588,"parent_record_offset":6564,"ancestor_record_offset":5920,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":990,"record_offset":9732,"parent_record_offset":9660,"ancestor_record_offset":5920,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":990,"record_offset":8196,"parent_record_offset":8160,"ancestor_record_offset":5920,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":990,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":5920,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":990,"record_offset":9760,"parent_record_offset":9660,"ancestor_record_offset":5920,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":990,"record_offset":9700,"parent_record_offset":9660,"ancestor_record_offset":5920,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":990,"record_offset":9784,"parent_record_offset":9760,"ancestor_record_offset":5920,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":990,"record_offset":8496,"parent_record_offset":8332,"ancestor_record_offset":5920,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":990,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":5920,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":990,"record_offset":7884,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":990,"record_offset":7152,"parent_record_offset":6964,"ancestor_record_offset":5920,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":990,"record_offset":7244,"parent_record_offset":7212,"ancestor_record_offset":5920,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":990,"record_offset":7392,"parent_record_offset":7364,"ancestor_record_offset":5920,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":990,"record_offset":7780,"parent_record_offset":7744,"ancestor_record_offset":5920,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":990,"record_offset":7084,"parent_record_offset":7052,"ancestor_record_offset":5920,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":990,"record_offset":7416,"parent_record_offset":7392,"ancestor_record_offset":5920,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":990,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":5920,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":990,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":5920,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":990,"record_offset":7804,"parent_record_offset":7780,"ancestor_record_offset":5920,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":990,"record_offset":7268,"parent_record_offset":7244,"ancestor_record_offset":5920,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":990,"record_offset":7836,"parent_record_offset":7744,"ancestor_record_offset":5920,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":990,"record_offset":8676,"parent_record_offset":8652,"ancestor_record_offset":5920,"inlinee":15594,"source_line":7,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":990,"record_offset":9372,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":15597,"source_line":104,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::program::state::State::is_queue_empty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":990,"record_offset":8536,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":15590,"source_line":74,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":990,"record_offset":8276,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":9582,"source_line":62,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":990,"record_offset":8712,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":990,"record_offset":7452,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":15587,"source_line":31,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":990,"record_offset":7488,"parent_record_offset":7452,"ancestor_record_offset":5920,"inlinee":15589,"source_line":139,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$24::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/init.rs
//     evidence={"module":990,"record_offset":10892,"parent_record_offset":10868,"ancestor_record_offset":5920,"inlinee":11527,"source_line":93,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":990,"record_offset":7052,"parent_record_offset":7024,"ancestor_record_offset":5920,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":990,"record_offset":7024,"parent_record_offset":6996,"ancestor_record_offset":5920,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":990,"record_offset":6964,"parent_record_offset":6924,"ancestor_record_offset":5920,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":990,"record_offset":6924,"parent_record_offset":6892,"ancestor_record_offset":5920,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":990,"record_offset":7212,"parent_record_offset":6924,"ancestor_record_offset":5920,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":990,"record_offset":6996,"parent_record_offset":6964,"ancestor_record_offset":5920,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::vulkan_config::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":990,"record_offset":10784,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":64666,"source_line":49,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::vulkan_config::impl$4::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":990,"record_offset":10836,"parent_record_offset":10808,"ancestor_record_offset":5920,"inlinee":64658,"source_line":16,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::vulkan_config::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":990,"record_offset":8000,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":64673,"source_line":16,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":990,"record_offset":6700,"parent_record_offset":6668,"ancestor_record_offset":5920,"inlinee":15599,"source_line":46,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":990,"record_offset":8596,"parent_record_offset":8564,"ancestor_record_offset":5920,"inlinee":15600,"source_line":186,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":990,"record_offset":10944,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":11386,"source_line":183,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":990,"record_offset":6496,"parent_record_offset":6468,"ancestor_record_offset":5920,"inlinee":12819,"source_line":219,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":990,"record_offset":8052,"parent_record_offset":8028,"ancestor_record_offset":5920,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":990,"record_offset":6440,"parent_record_offset":6412,"ancestor_record_offset":5920,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":990,"record_offset":6856,"parent_record_offset":6824,"ancestor_record_offset":5920,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":990,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":5920,"inlinee":13602,"source_line":147,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":990,"record_offset":6412,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":12815,"source_line":746,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":990,"record_offset":6760,"parent_record_offset":6700,"ancestor_record_offset":5920,"inlinee":12848,"source_line":568,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":990,"record_offset":8028,"parent_record_offset":6112,"ancestor_record_offset":5920,"inlinee":13599,"source_line":940,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":990,"record_offset":6824,"parent_record_offset":6792,"ancestor_record_offset":5920,"inlinee":12850,"source_line":569,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":990,"record_offset":6468,"parent_record_offset":6440,"ancestor_record_offset":5920,"inlinee":12818,"source_line":747,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":990,"record_offset":8076,"parent_record_offset":8052,"ancestor_record_offset":5920,"inlinee":13601,"source_line":942,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_winit::run_gui<tg_iced::dialog::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1001,"record_offset":5888,"rva":16775056,"va":5385484176}
// 编译实例 RVAs: 0xfff790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31128539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8dbfbda01","size":5,"soft_fail":false,"target_va":5416616304,"va":5385487760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-836994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e87e3af3ff","size":5,"soft_fail":false,"target_va":5384648032,"va":5385485021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9639884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e834e86cff","size":5,"soft_fail":false,"target_va":5375844784,"va":5385484663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t16209648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8f056f700","size":5,"soft_fail":false,"target_va":5401694368,"va":5385484715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9805886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8c25f6aff","size":5,"soft_fail":false,"target_va":5375678688,"va":5385484569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-9807218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e88e5a6aff","size":5,"soft_fail":false,"target_va":5375678704,"va":5385485917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16762367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8013a00ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5385484554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008c0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16207145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8d7b208ff","size":5,"soft_fail":false,"target_va":5369282736,"va":5385489876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a650
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16144347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e825a809ff","size":5,"soft_fail":false,"target_va":5369341520,"va":5385485862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ffa0
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16125430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e80af209ff","size":5,"soft_fail":false,"target_va":5369364384,"va":5385489809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b3050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16045769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e837290bff","size":5,"soft_fail":false,"target_va":5369442384,"va":5385488148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401e2420
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14805714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e82e151eff","size":5,"soft_fail":false,"target_va":5370684448,"va":5385490157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141f78370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16218798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8ae7af700","size":5,"soft_fail":false,"target_va":5401707376,"va":5385488573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30866296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e878fbd601","size":5,"soft_fail":false,"target_va":5416351392,"va":5385485091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31132974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e82e0ddb01","size":5,"soft_fail":false,"target_va":5416620769,"va":5385487790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-9670578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e84e706cff","size":5,"soft_fail":false,"target_va":5375816416,"va":5385486989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31134496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e82013db01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385487739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$iced_runtime..command..action..Action$LT$tg_iced..dialog..Message$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-59378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e80e18ffff","size":5,"soft_fail":false,"target_va":5385430864,"va":5385490237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-49204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8cc3fffff","size":5,"soft_fail":false,"target_va":5385437840,"va":5385487039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..Command$LT$tg_iced..dialog..Message$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-54389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e88b2bffff","size":5,"soft_fail":false,"target_va":5385435248,"va":5385489632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$tg_iced..dialog..Message$GT$$GT$::h8225831984f5bf01 (.llvm.7367605847545050435)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-50518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8aa3affff","size":5,"soft_fail":false,"target_va":5385436112,"va":5385486625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..program..state..State$LT$tg_iced..dialog..Controls$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-51391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e84137ffff","size":5,"soft_fail":false,"target_va":5385435616,"va":5385487002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_tiny_skia..window..compositor..Surface$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-52627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e86d32ffff","size":5,"soft_fail":false,"target_va":5385434400,"va":5385487022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<tg_iced::dialog::Message> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e826f0ffff","size":5,"soft_fail":false,"target_va":5385482448,"va":5385486501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-9548114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8ae4e6eff","size":5,"soft_fail":false,"target_va":5375936352,"va":5385484461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t16218502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e88679f700","size":5,"soft_fail":false,"target_va":5401703360,"va":5385484853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t16218159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e82f78f700","size":5,"soft_fail":false,"target_va":5401703840,"va":5385485676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-9640406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e82ae66cff","size":5,"soft_fail":false,"target_va":5375844112,"va":5385484513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-9642597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e89bdd6cff","size":5,"soft_fail":false,"target_va":5375842656,"va":5385485248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31131862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8d608db01","size":5,"soft_fail":false,"target_va":5416619696,"va":5385487829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-758537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8f76cf4ff","size":5,"soft_fail":false,"target_va":5384728160,"va":5385486692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-28808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e8788fffff","size":5,"soft_fail":false,"target_va":5385458128,"va":5385486931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-19348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e86cb4ffff","size":5,"soft_fail":false,"target_va":5385465584,"va":5385484927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-9175425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385484176,"raw_hex":"e87ffe73ff","size":5,"soft_fail":false,"target_va":5376308960,"va":5385484380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*40678208(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5385484176,"raw_hex":"ff1540b36c02","size":6,"soft_fail":false,"va":5385484338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner_winit::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":1001,"record_offset":6064,"parent_record_offset":5888,"ancestor_record_offset":5888,"inlinee":15604,"source_line":33,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":1001,"record_offset":6040,"parent_record_offset":5888,"ancestor_record_offset":5888,"inlinee":15602,"source_line":24,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1001,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":5888,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1001,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":5888,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1001,"record_offset":8940,"parent_record_offset":8916,"ancestor_record_offset":5888,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1001,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":5888,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1001,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":5888,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1001,"record_offset":8988,"parent_record_offset":8964,"ancestor_record_offset":5888,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1001,"record_offset":7540,"parent_record_offset":7508,"ancestor_record_offset":5888,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1001,"record_offset":8916,"parent_record_offset":8828,"ancestor_record_offset":5888,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1001,"record_offset":7508,"parent_record_offset":7464,"ancestor_record_offset":5888,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1001,"record_offset":7464,"parent_record_offset":7428,"ancestor_record_offset":5888,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1001,"record_offset":9348,"parent_record_offset":9304,"ancestor_record_offset":5888,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1001,"record_offset":11100,"parent_record_offset":11072,"ancestor_record_offset":5888,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1001,"record_offset":8320,"parent_record_offset":8288,"ancestor_record_offset":5888,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1001,"record_offset":8856,"parent_record_offset":8828,"ancestor_record_offset":5888,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1001,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":5888,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1001,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":5888,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1001,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":5888,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1001,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":5888,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1001,"record_offset":8072,"parent_record_offset":8044,"ancestor_record_offset":5888,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1001,"record_offset":7304,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1001,"record_offset":6672,"parent_record_offset":6640,"ancestor_record_offset":5888,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1001,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":5888,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1001,"record_offset":8288,"parent_record_offset":8244,"ancestor_record_offset":5888,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1001,"record_offset":8216,"parent_record_offset":8188,"ancestor_record_offset":5888,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1001,"record_offset":8244,"parent_record_offset":8216,"ancestor_record_offset":5888,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1001,"record_offset":9536,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1001,"record_offset":8704,"parent_record_offset":8668,"ancestor_record_offset":5888,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1001,"record_offset":9816,"parent_record_offset":9780,"ancestor_record_offset":5888,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1001,"record_offset":9780,"parent_record_offset":9752,"ancestor_record_offset":5888,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1001,"record_offset":10068,"parent_record_offset":10044,"ancestor_record_offset":5888,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1001,"record_offset":9592,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1001,"record_offset":9888,"parent_record_offset":9864,"ancestor_record_offset":5888,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1001,"record_offset":11132,"parent_record_offset":11100,"ancestor_record_offset":5888,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1001,"record_offset":9864,"parent_record_offset":9840,"ancestor_record_offset":5888,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1001,"record_offset":9840,"parent_record_offset":9816,"ancestor_record_offset":5888,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1001,"record_offset":11196,"parent_record_offset":11160,"ancestor_record_offset":5888,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1001,"record_offset":6504,"parent_record_offset":6480,"ancestor_record_offset":5888,"inlinee":12821,"source_line":469,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1001,"record_offset":11160,"parent_record_offset":11132,"ancestor_record_offset":5888,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1001,"record_offset":6480,"parent_record_offset":6436,"ancestor_record_offset":5888,"inlinee":12820,"source_line":681,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1001,"record_offset":8884,"parent_record_offset":8856,"ancestor_record_offset":5888,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1001,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":5888,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1001,"record_offset":8476,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1001,"record_offset":8540,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1001,"record_offset":6608,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1001,"record_offset":9960,"parent_record_offset":9936,"ancestor_record_offset":5888,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1001,"record_offset":6832,"parent_record_offset":6796,"ancestor_record_offset":5888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1001,"record_offset":9560,"parent_record_offset":9536,"ancestor_record_offset":5888,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1001,"record_offset":9936,"parent_record_offset":9912,"ancestor_record_offset":5888,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1001,"record_offset":6528,"parent_record_offset":6504,"ancestor_record_offset":5888,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1001,"record_offset":9664,"parent_record_offset":9592,"ancestor_record_offset":5888,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1001,"record_offset":8108,"parent_record_offset":8072,"ancestor_record_offset":5888,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1001,"record_offset":9912,"parent_record_offset":9888,"ancestor_record_offset":5888,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1001,"record_offset":9692,"parent_record_offset":9592,"ancestor_record_offset":5888,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1001,"record_offset":9632,"parent_record_offset":9592,"ancestor_record_offset":5888,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1001,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":5888,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1001,"record_offset":8408,"parent_record_offset":8244,"ancestor_record_offset":5888,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1001,"record_offset":6552,"parent_record_offset":6528,"ancestor_record_offset":5888,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1001,"record_offset":7824,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1001,"record_offset":7092,"parent_record_offset":6904,"ancestor_record_offset":5888,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1001,"record_offset":7184,"parent_record_offset":7152,"ancestor_record_offset":5888,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1001,"record_offset":7332,"parent_record_offset":7304,"ancestor_record_offset":5888,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1001,"record_offset":7720,"parent_record_offset":7684,"ancestor_record_offset":5888,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1001,"record_offset":7024,"parent_record_offset":6992,"ancestor_record_offset":5888,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1001,"record_offset":7356,"parent_record_offset":7332,"ancestor_record_offset":5888,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1001,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":5888,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1001,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":5888,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1001,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":5888,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1001,"record_offset":7208,"parent_record_offset":7184,"ancestor_record_offset":5888,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1001,"record_offset":7776,"parent_record_offset":7684,"ancestor_record_offset":5888,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":1001,"record_offset":8588,"parent_record_offset":8564,"ancestor_record_offset":5888,"inlinee":15594,"source_line":7,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":1001,"record_offset":9304,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":15597,"source_line":104,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::program::state::State::is_queue_empty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":1001,"record_offset":8448,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":15590,"source_line":74,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":1001,"record_offset":8188,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":9582,"source_line":62,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":1001,"record_offset":8624,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":1001,"record_offset":7392,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":15587,"source_line":31,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":1001,"record_offset":7428,"parent_record_offset":7392,"ancestor_record_offset":5888,"inlinee":15589,"source_line":139,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":1001,"record_offset":6992,"parent_record_offset":6964,"ancestor_record_offset":5888,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":1001,"record_offset":6964,"parent_record_offset":6936,"ancestor_record_offset":5888,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":1001,"record_offset":6904,"parent_record_offset":6864,"ancestor_record_offset":5888,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":1001,"record_offset":6864,"parent_record_offset":6832,"ancestor_record_offset":5888,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1001,"record_offset":7152,"parent_record_offset":6864,"ancestor_record_offset":5888,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":1001,"record_offset":6936,"parent_record_offset":6904,"ancestor_record_offset":5888,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::dialog::impl$2::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/dialog.rs
//     evidence={"module":1001,"record_offset":10716,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":19005,"source_line":153,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":1001,"record_offset":6640,"parent_record_offset":6608,"ancestor_record_offset":5888,"inlinee":15599,"source_line":46,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":1001,"record_offset":8508,"parent_record_offset":8476,"ancestor_record_offset":5888,"inlinee":15600,"source_line":186,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":1001,"record_offset":10744,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":11386,"source_line":183,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":1001,"record_offset":6436,"parent_record_offset":6408,"ancestor_record_offset":5888,"inlinee":12819,"source_line":219,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":1001,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":5888,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":1001,"record_offset":6380,"parent_record_offset":6352,"ancestor_record_offset":5888,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":1001,"record_offset":6796,"parent_record_offset":6764,"ancestor_record_offset":5888,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":1001,"record_offset":8012,"parent_record_offset":7988,"ancestor_record_offset":5888,"inlinee":13602,"source_line":147,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":1001,"record_offset":6352,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":12815,"source_line":746,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":1001,"record_offset":6700,"parent_record_offset":6640,"ancestor_record_offset":5888,"inlinee":12848,"source_line":568,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":1001,"record_offset":7940,"parent_record_offset":6064,"ancestor_record_offset":5888,"inlinee":13599,"source_line":940,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":1001,"record_offset":6764,"parent_record_offset":6732,"ancestor_record_offset":5888,"inlinee":12850,"source_line":569,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":1001,"record_offset":6408,"parent_record_offset":6380,"ancestor_record_offset":5888,"inlinee":12818,"source_line":747,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":1001,"record_offset":7988,"parent_record_offset":7964,"ancestor_record_offset":5888,"inlinee":13601,"source_line":942,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_winit::run_gui<tg_integrity_report::controls::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":945,"record_offset":5924,"rva":16489776,"va":5385198896}
// 编译实例 RVAs: 0xfb9d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31413840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e85056df01","size":5,"soft_fail":false,"target_va":5416616304,"va":5385202459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-551714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8de94f7ff","size":5,"soft_fail":false,"target_va":5384648032,"va":5385199741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9354604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8944271ff","size":5,"soft_fail":false,"target_va":5375844784,"va":5385199383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15985344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8c0eaf300","size":5,"soft_fail":false,"target_va":5401184784,"va":5385199435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9520606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e822ba6eff","size":5,"soft_fail":false,"target_va":5375678688,"va":5385199289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-9521938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8eeb46eff","size":5,"soft_fail":false,"target_va":5375678704,"va":5385200637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16477087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8619404ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5385199274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008c0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15921833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8570d0dff","size":5,"soft_fail":false,"target_va":5369282736,"va":5385204564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a650
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15859067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e885020eff","size":5,"soft_fail":false,"target_va":5369341520,"va":5385200582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b3050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15760457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8b7830fff","size":5,"soft_fail":false,"target_va":5369442384,"va":5385202836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401e2420
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14520402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8ae6f22ff","size":5,"soft_fail":false,"target_va":5370684448,"va":5385204845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31151580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8dc55db01","size":5,"soft_fail":false,"target_va":5416351392,"va":5385199807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31418275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8a367df01","size":5,"soft_fail":false,"target_va":5416620769,"va":5385202489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-9385294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8b2ca70ff","size":5,"soft_fail":false,"target_va":5375816416,"va":5385201705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31419797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8956ddf01","size":5,"soft_fail":false,"target_va":5416622240,"va":5385202438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$iced_runtime..command..action..Action$LT$tg_integrity_report..controls..Message$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15949038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8ee5cf300","size":5,"soft_fail":false,"target_va":5401153968,"va":5385204925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15954832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e89073f300","size":5,"soft_fail":false,"target_va":5401156592,"va":5385201755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..Command$LT$tg_integrity_report..controls..Message$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15954747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e83b73f300","size":5,"soft_fail":false,"target_va":5401159072,"va":5385204320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$tg_integrity_report..controls..Message$GT$$GT$::h342d7b9f0c0a8036 (.llvm.16523719838322645898)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15999022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e82e20f400","size":5,"soft_fail":false,"target_va":5401202288,"va":5385203261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$tg_integrity_report..controls..Message$GT$$GT$::h342d7b9f0c0a8036 (.llvm.3102096760774865050)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t15949786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8da5ff300","size":5,"soft_fail":false,"target_va":5401151152,"va":5385201361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..program..state..State$LT$tg_integrity_report..controls..Controls$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15948869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8455cf300","size":5,"soft_fail":false,"target_va":5401150592,"va":5385201718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_tiny_skia..window..compositor..Surface$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15954641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8d172f300","size":5,"soft_fail":false,"target_va":5401156384,"va":5385201738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_integrity_report..controls..Controls$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15951466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e86a66f300","size":5,"soft_fail":false,"target_va":5401155968,"va":5385204497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e81ff0ffff","size":5,"soft_fail":false,"target_va":5385197168,"va":5385201228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-9262834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e80ea972ff","size":5,"soft_fail":false,"target_va":5375936352,"va":5385199181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t15994195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8530df400","size":5,"soft_fail":false,"target_va":5401193776,"va":5385199576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t15993859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8030cf400","size":5,"soft_fail":false,"target_va":5401194256,"va":5385200392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-9355126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e88a4071ff","size":5,"soft_fail":false,"target_va":5375844112,"va":5385199233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-9357313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8ff3771ff","size":5,"soft_fail":false,"target_va":5375842656,"va":5385199964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31417163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e84b63df01","size":5,"soft_fail":false,"target_va":5416619696,"va":5385202528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-473271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e849c7f8ff","size":5,"soft_fail":false,"target_va":5384728160,"va":5385201426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t15996301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e88d15f400","size":5,"soft_fail":false,"target_va":5401197968,"va":5385201662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t16000073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e84924f400","size":5,"soft_fail":false,"target_va":5401199728,"va":5385199650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-8890144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385198896,"raw_hex":"e8e05878ff","size":5,"soft_fail":false,"target_va":5376308960,"va":5385199099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*40963491(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5385198896,"raw_hex":"ff15a30d7102","size":6,"soft_fail":false,"va":5385199055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner_winit::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":945,"record_offset":6112,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":15604,"source_line":33,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/lib.rs
//     evidence={"module":945,"record_offset":6088,"parent_record_offset":5924,"ancestor_record_offset":5924,"inlinee":15602,"source_line":24,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":945,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":5924,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":945,"record_offset":7632,"parent_record_offset":7608,"ancestor_record_offset":5924,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":945,"record_offset":8984,"parent_record_offset":8960,"ancestor_record_offset":5924,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":945,"record_offset":9008,"parent_record_offset":8984,"ancestor_record_offset":5924,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":945,"record_offset":7656,"parent_record_offset":7632,"ancestor_record_offset":5924,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":945,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":5924,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":945,"record_offset":7584,"parent_record_offset":7552,"ancestor_record_offset":5924,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":945,"record_offset":8960,"parent_record_offset":8872,"ancestor_record_offset":5924,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":945,"record_offset":7552,"parent_record_offset":7508,"ancestor_record_offset":5924,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":945,"record_offset":7508,"parent_record_offset":7472,"ancestor_record_offset":5924,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":945,"record_offset":9392,"parent_record_offset":9348,"ancestor_record_offset":5924,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":945,"record_offset":11116,"parent_record_offset":11088,"ancestor_record_offset":5924,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":945,"record_offset":8364,"parent_record_offset":8332,"ancestor_record_offset":5924,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":945,"record_offset":8900,"parent_record_offset":8872,"ancestor_record_offset":5924,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":945,"record_offset":8872,"parent_record_offset":8848,"ancestor_record_offset":5924,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":945,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":5924,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":945,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":5924,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":945,"record_offset":8848,"parent_record_offset":8824,"ancestor_record_offset":5924,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":945,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":5924,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":945,"record_offset":7348,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":945,"record_offset":6716,"parent_record_offset":6684,"ancestor_record_offset":5924,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":945,"record_offset":7728,"parent_record_offset":7704,"ancestor_record_offset":5924,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":945,"record_offset":8332,"parent_record_offset":8288,"ancestor_record_offset":5924,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":945,"record_offset":8260,"parent_record_offset":8232,"ancestor_record_offset":5924,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":945,"record_offset":8288,"parent_record_offset":8260,"ancestor_record_offset":5924,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":945,"record_offset":9580,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":945,"record_offset":8748,"parent_record_offset":8712,"ancestor_record_offset":5924,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":945,"record_offset":9860,"parent_record_offset":9824,"ancestor_record_offset":5924,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":945,"record_offset":9824,"parent_record_offset":9796,"ancestor_record_offset":5924,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":945,"record_offset":10112,"parent_record_offset":10088,"ancestor_record_offset":5924,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":945,"record_offset":9636,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":945,"record_offset":9932,"parent_record_offset":9908,"ancestor_record_offset":5924,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":945,"record_offset":11148,"parent_record_offset":11116,"ancestor_record_offset":5924,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":945,"record_offset":9908,"parent_record_offset":9884,"ancestor_record_offset":5924,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":945,"record_offset":9884,"parent_record_offset":9860,"ancestor_record_offset":5924,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":945,"record_offset":11212,"parent_record_offset":11176,"ancestor_record_offset":5924,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":945,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":5924,"inlinee":12821,"source_line":469,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":945,"record_offset":11176,"parent_record_offset":11148,"ancestor_record_offset":5924,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":945,"record_offset":6524,"parent_record_offset":6480,"ancestor_record_offset":5924,"inlinee":12820,"source_line":681,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":945,"record_offset":8928,"parent_record_offset":8900,"ancestor_record_offset":5924,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":945,"record_offset":8608,"parent_record_offset":8584,"ancestor_record_offset":5924,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":945,"record_offset":8520,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":945,"record_offset":8584,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":945,"record_offset":6652,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":945,"record_offset":10004,"parent_record_offset":9980,"ancestor_record_offset":5924,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":945,"record_offset":6876,"parent_record_offset":6840,"ancestor_record_offset":5924,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":945,"record_offset":9604,"parent_record_offset":9580,"ancestor_record_offset":5924,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":945,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":5924,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":945,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":5924,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":945,"record_offset":9708,"parent_record_offset":9636,"ancestor_record_offset":5924,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":945,"record_offset":8152,"parent_record_offset":8116,"ancestor_record_offset":5924,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":945,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":5924,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":945,"record_offset":9736,"parent_record_offset":9636,"ancestor_record_offset":5924,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":945,"record_offset":9676,"parent_record_offset":9636,"ancestor_record_offset":5924,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":945,"record_offset":9760,"parent_record_offset":9736,"ancestor_record_offset":5924,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":945,"record_offset":8452,"parent_record_offset":8288,"ancestor_record_offset":5924,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":945,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":5924,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":945,"record_offset":7868,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":945,"record_offset":7136,"parent_record_offset":6948,"ancestor_record_offset":5924,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":945,"record_offset":7228,"parent_record_offset":7196,"ancestor_record_offset":5924,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":945,"record_offset":7376,"parent_record_offset":7348,"ancestor_record_offset":5924,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":945,"record_offset":7764,"parent_record_offset":7728,"ancestor_record_offset":5924,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":945,"record_offset":7068,"parent_record_offset":7036,"ancestor_record_offset":5924,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":945,"record_offset":7400,"parent_record_offset":7376,"ancestor_record_offset":5924,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":945,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":5924,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":945,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":5924,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":945,"record_offset":7788,"parent_record_offset":7764,"ancestor_record_offset":5924,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":945,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":5924,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":945,"record_offset":7820,"parent_record_offset":7728,"ancestor_record_offset":5924,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":945,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":5924,"inlinee":15594,"source_line":7,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":945,"record_offset":9348,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":15597,"source_line":104,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::program::state::State::is_queue_empty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":945,"record_offset":8492,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":15590,"source_line":74,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":945,"record_offset":8232,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":9582,"source_line":62,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":945,"record_offset":8668,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":945,"record_offset":7436,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":15587,"source_line":31,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":945,"record_offset":7472,"parent_record_offset":7436,"ancestor_record_offset":5924,"inlinee":15589,"source_line":139,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":945,"record_offset":7036,"parent_record_offset":7008,"ancestor_record_offset":5924,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":945,"record_offset":7008,"parent_record_offset":6980,"ancestor_record_offset":5924,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":945,"record_offset":6948,"parent_record_offset":6908,"ancestor_record_offset":5924,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":945,"record_offset":6908,"parent_record_offset":6876,"ancestor_record_offset":5924,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":945,"record_offset":7196,"parent_record_offset":6908,"ancestor_record_offset":5924,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":945,"record_offset":6980,"parent_record_offset":6948,"ancestor_record_offset":5924,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":945,"record_offset":6684,"parent_record_offset":6652,"ancestor_record_offset":5924,"inlinee":15599,"source_line":46,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":945,"record_offset":8552,"parent_record_offset":8520,"ancestor_record_offset":5924,"inlinee":15600,"source_line":186,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":945,"record_offset":10760,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":11386,"source_line":183,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":945,"record_offset":6480,"parent_record_offset":6452,"ancestor_record_offset":5924,"inlinee":12819,"source_line":219,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":945,"record_offset":8008,"parent_record_offset":7984,"ancestor_record_offset":5924,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":945,"record_offset":6424,"parent_record_offset":6396,"ancestor_record_offset":5924,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":945,"record_offset":6840,"parent_record_offset":6808,"ancestor_record_offset":5924,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":945,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":5924,"inlinee":13602,"source_line":147,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":945,"record_offset":6396,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":12815,"source_line":746,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":945,"record_offset":6744,"parent_record_offset":6684,"ancestor_record_offset":5924,"inlinee":12848,"source_line":568,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":945,"record_offset":7984,"parent_record_offset":6112,"ancestor_record_offset":5924,"inlinee":13599,"source_line":940,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":945,"record_offset":6808,"parent_record_offset":6776,"ancestor_record_offset":5924,"inlinee":12850,"source_line":569,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":945,"record_offset":6452,"parent_record_offset":6424,"ancestor_record_offset":5924,"inlinee":12818,"source_line":747,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":945,"record_offset":8032,"parent_record_offset":8008,"ancestor_record_offset":5924,"inlinee":13601,"source_line":942,"archive":"_pdb/C/modules/00945.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_winit::run_gui_with_options
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3CF4
// 首条位置: {"module":184,"inlinee":15604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":32412,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":32384,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":34624,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":34680,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":33584,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":33648,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":31716,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=55
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":33776,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":184,"record_offset":32932,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":184,"record_offset":34392,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":15597,"source_line":104,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::is_queue_empty
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":33556,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":15590,"source_line":74,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":33296,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":9582,"source_line":62,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":184,"record_offset":33732,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":184,"record_offset":32500,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":15587,"source_line":31,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] tg_gpu_selector::vulkan_config::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":990,"record_offset":10784,"parent_record_offset":6112,"ancestor_record_offset":6112,"inlinee":64666,"source_line":49,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_direct] tg_gpu_selector::vulkan_config::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":990,"record_offset":8000,"parent_record_offset":6112,"ancestor_record_offset":6112,"inlinee":64673,"source_line":16,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::dialog::impl$2::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/dialog.rs
//     evidence={"module":1001,"record_offset":10716,"parent_record_offset":6064,"ancestor_record_offset":6064,"inlinee":19005,"source_line":153,"archive":"_pdb/C/modules/01001.jsonl.gz"}
//   [pdb_inline_direct] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":184,"record_offset":35804,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":11386,"source_line":183,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":31460,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":12815,"source_line":746,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":33048,"parent_record_offset":31180,"ancestor_record_offset":31180,"inlinee":13599,"source_line":940,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":32672,"parent_record_offset":32648,"ancestor_record_offset":31180,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":32696,"parent_record_offset":32672,"ancestor_record_offset":31180,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=60
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":34028,"parent_record_offset":34004,"ancestor_record_offset":31180,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=60
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":34052,"parent_record_offset":34028,"ancestor_record_offset":31180,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":32720,"parent_record_offset":32696,"ancestor_record_offset":31180,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=60
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":34076,"parent_record_offset":34052,"ancestor_record_offset":31180,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":32648,"parent_record_offset":32616,"ancestor_record_offset":31180,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=60
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":34004,"parent_record_offset":33916,"ancestor_record_offset":31180,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":32616,"parent_record_offset":32572,"ancestor_record_offset":31180,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":32572,"parent_record_offset":32536,"ancestor_record_offset":31180,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":34436,"parent_record_offset":34392,"ancestor_record_offset":31180,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":36160,"parent_record_offset":36132,"ancestor_record_offset":31180,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33428,"parent_record_offset":33396,"ancestor_record_offset":31180,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33944,"parent_record_offset":33916,"ancestor_record_offset":31180,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33916,"parent_record_offset":33892,"ancestor_record_offset":31180,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33476,"parent_record_offset":33452,"ancestor_record_offset":31180,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33452,"parent_record_offset":33428,"ancestor_record_offset":31180,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":33892,"parent_record_offset":33868,"ancestor_record_offset":31180,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":33180,"parent_record_offset":33152,"ancestor_record_offset":31180,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":33152,"parent_record_offset":33120,"ancestor_record_offset":31180,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":31780,"parent_record_offset":31748,"ancestor_record_offset":31180,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":32792,"parent_record_offset":32768,"ancestor_record_offset":31180,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":33396,"parent_record_offset":33352,"ancestor_record_offset":31180,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":33324,"parent_record_offset":33296,"ancestor_record_offset":31180,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":33352,"parent_record_offset":33324,"ancestor_record_offset":31180,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":33804,"parent_record_offset":33776,"ancestor_record_offset":31180,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":34904,"parent_record_offset":34868,"ancestor_record_offset":31180,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":34868,"parent_record_offset":34840,"ancestor_record_offset":31180,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":35156,"parent_record_offset":35132,"ancestor_record_offset":31180,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":34976,"parent_record_offset":34952,"ancestor_record_offset":31180,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":184,"record_offset":36192,"parent_record_offset":36160,"ancestor_record_offset":31180,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\clone.rs
//     evidence={"module":990,"record_offset":10868,"parent_record_offset":10836,"ancestor_record_offset":6112,"inlinee":6914,"source_line":236,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":184,"record_offset":34952,"parent_record_offset":34928,"ancestor_record_offset":31180,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":184,"record_offset":34928,"parent_record_offset":34904,"ancestor_record_offset":31180,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":36256,"parent_record_offset":36220,"ancestor_record_offset":31180,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":184,"record_offset":31612,"parent_record_offset":31588,"ancestor_record_offset":31180,"inlinee":12821,"source_line":469,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":36220,"parent_record_offset":36192,"ancestor_record_offset":31180,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":31588,"parent_record_offset":31544,"ancestor_record_offset":31180,"inlinee":12820,"source_line":681,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":184,"record_offset":33972,"parent_record_offset":33944,"ancestor_record_offset":31180,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":184,"record_offset":33672,"parent_record_offset":33648,"ancestor_record_offset":31180,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":990,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":6112,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":184,"record_offset":35048,"parent_record_offset":35024,"ancestor_record_offset":31180,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=139
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":31940,"parent_record_offset":31904,"ancestor_record_offset":31180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":34648,"parent_record_offset":34624,"ancestor_record_offset":31180,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":35024,"parent_record_offset":35000,"ancestor_record_offset":31180,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":31636,"parent_record_offset":31612,"ancestor_record_offset":31180,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":34752,"parent_record_offset":34680,"ancestor_record_offset":31180,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":33216,"parent_record_offset":33180,"ancestor_record_offset":31180,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":35000,"parent_record_offset":34976,"ancestor_record_offset":31180,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":34780,"parent_record_offset":34680,"ancestor_record_offset":31180,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":34720,"parent_record_offset":34680,"ancestor_record_offset":31180,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":34804,"parent_record_offset":34780,"ancestor_record_offset":31180,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":33516,"parent_record_offset":33352,"ancestor_record_offset":31180,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":31660,"parent_record_offset":31636,"ancestor_record_offset":31180,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32200,"parent_record_offset":32012,"ancestor_record_offset":31180,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32292,"parent_record_offset":32260,"ancestor_record_offset":31180,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32440,"parent_record_offset":32412,"ancestor_record_offset":31180,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32828,"parent_record_offset":32792,"ancestor_record_offset":31180,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32132,"parent_record_offset":32100,"ancestor_record_offset":31180,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32464,"parent_record_offset":32440,"ancestor_record_offset":31180,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32156,"parent_record_offset":32132,"ancestor_record_offset":31180,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32224,"parent_record_offset":32200,"ancestor_record_offset":31180,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32852,"parent_record_offset":32828,"ancestor_record_offset":31180,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32316,"parent_record_offset":32292,"ancestor_record_offset":31180,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":32884,"parent_record_offset":32792,"ancestor_record_offset":31180,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":184,"record_offset":33696,"parent_record_offset":33672,"ancestor_record_offset":31180,"inlinee":15594,"source_line":7,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":184,"record_offset":32536,"parent_record_offset":32500,"ancestor_record_offset":31180,"inlinee":15589,"source_line":139,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$24::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/init.rs
//     evidence={"module":990,"record_offset":10892,"parent_record_offset":10868,"ancestor_record_offset":6112,"inlinee":11527,"source_line":93,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":184,"record_offset":32100,"parent_record_offset":32072,"ancestor_record_offset":31180,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":184,"record_offset":32072,"parent_record_offset":32044,"ancestor_record_offset":31180,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":184,"record_offset":32012,"parent_record_offset":31972,"ancestor_record_offset":31180,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":184,"record_offset":31972,"parent_record_offset":31940,"ancestor_record_offset":31180,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":184,"record_offset":32260,"parent_record_offset":31972,"ancestor_record_offset":31180,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":184,"record_offset":32044,"parent_record_offset":32012,"ancestor_record_offset":31180,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::vulkan_config::impl$4::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":990,"record_offset":10836,"parent_record_offset":10808,"ancestor_record_offset":6112,"inlinee":64658,"source_line":16,"archive":"_pdb/C/modules/00990.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":184,"record_offset":31748,"parent_record_offset":31716,"ancestor_record_offset":31180,"inlinee":15599,"source_line":46,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":184,"record_offset":33616,"parent_record_offset":33584,"ancestor_record_offset":31180,"inlinee":15600,"source_line":186,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":31544,"parent_record_offset":31516,"ancestor_record_offset":31180,"inlinee":12819,"source_line":219,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":33072,"parent_record_offset":33048,"ancestor_record_offset":31180,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":31488,"parent_record_offset":31460,"ancestor_record_offset":31180,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":31904,"parent_record_offset":31872,"ancestor_record_offset":31180,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":33120,"parent_record_offset":33096,"ancestor_record_offset":31180,"inlinee":13602,"source_line":147,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":31808,"parent_record_offset":31748,"ancestor_record_offset":31180,"inlinee":12848,"source_line":568,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":31872,"parent_record_offset":31840,"ancestor_record_offset":31180,"inlinee":12850,"source_line":569,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":31516,"parent_record_offset":31488,"ancestor_record_offset":31180,"inlinee":12818,"source_line":747,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":33096,"parent_record_offset":33072,"ancestor_record_offset":31180,"inlinee":13601,"source_line":942,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_winit::run_gui_with_options<crash_reporter_gui::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":184,"record_offset":20912,"rva":5443392,"va":5374152512}
// 编译实例 RVAs: 0x530f40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::sys::sync::mutex::futex::Mutex>::wake
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t42460128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8e0e38702","size":5,"soft_fail":false,"target_va":5416616304,"va":5374156171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10494782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e83e23a000","size":5,"soft_fail":false,"target_va":5384648032,"va":5374153245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1691821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8add01900","size":5,"soft_fail":false,"target_va":5375844784,"va":5374152958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1525819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e83b481700","size":5,"soft_fail":false,"target_va":5375678688,"va":5374152864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t1524574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e85e431700","size":5,"soft_fail":false,"target_va":5375678704,"va":5374154125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5430662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e87a22adff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374152849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140080710
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4919622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8baeeb4ff","size":5,"soft_fail":false,"target_va":5369235216,"va":5374154833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089690
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4886754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e81e6fb5ff","size":5,"soft_fail":false,"target_va":5369271952,"va":5374158701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008c0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4875609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8a79ab5ff","size":5,"soft_fail":false,"target_va":5369282736,"va":5374158340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009a650
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4812563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8ed90b6ff","size":5,"soft_fail":false,"target_va":5369341520,"va":5374154078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009da40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4803462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e87ab4b6ff","size":5,"soft_fail":false,"target_va":5369354816,"va":5374158273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400af890
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4727284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e80cdeb7ff","size":5,"soft_fail":false,"target_va":5369428112,"va":5374155391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b3050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4714137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e86711b8ff","size":5,"soft_fail":false,"target_va":5369442384,"va":5374156516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bfcd0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4663374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8b2d7b8ff","size":5,"soft_fail":false,"target_va":5369494736,"va":5374158105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c3200
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4647024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e89017b9ff","size":5,"soft_fail":false,"target_va":5369508352,"va":5374155371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c46b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4641765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e81b2cb9ff","size":5,"soft_fail":false,"target_va":5369513648,"va":5374155408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401e2420
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3474178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8fefccaff","size":5,"soft_fail":false,"target_va":5370684448,"va":5374158621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401e2550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3473023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e88101cbff","size":5,"soft_fail":false,"target_va":5370684752,"va":5374157770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140597b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t420425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8496a0600","size":5,"soft_fail":false,"target_va":5374573440,"va":5374153010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b8810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1601405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e87d6f1800","size":5,"soft_fail":false,"target_va":5375756304,"va":5374154894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b8830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1601288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8086f1800","size":5,"soft_fail":false,"target_va":5375756336,"va":5374155043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42195409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8d1d98302","size":5,"soft_fail":false,"target_va":5416350384,"va":5374154970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42198072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e838e48302","size":5,"soft_fail":false,"target_va":5416351392,"va":5374153315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_runner_winit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42904001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8c1a98e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374153050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42464666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e89af58702","size":5,"soft_fail":false,"target_va":5416620769,"va":5374156098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t1661053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e87d581900","size":5,"soft_fail":false,"target_va":5375816416,"va":5374155358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42466103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e837fb8702","size":5,"soft_fail":false,"target_va":5416622240,"va":5374156132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > > >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-11123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e88dd4ffff","size":5,"soft_fail":false,"target_va":5374143600,"va":5374154718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1783591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e827371b00","size":5,"soft_fail":false,"target_va":5375936352,"va":5374152756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-3487539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8cdc8caff","size":5,"soft_fail":false,"target_va":5370665568,"va":5374153102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-3487761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8efc7caff","size":5,"soft_fail":false,"target_va":5370666144,"va":5374153900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1691299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8a3ce1900","size":5,"soft_fail":false,"target_va":5375844112,"va":5374152808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t1689179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e85bc61900","size":5,"soft_fail":false,"target_va":5375842656,"va":5374153472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t42463494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e806f18702","size":5,"soft_fail":false,"target_va":5416619696,"va":5374156197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::functions::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10573091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e82355a100","size":5,"soft_fail":false,"target_va":5384728160,"va":5374155064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1222809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e86757edff","size":5,"soft_fail":false,"target_va":5372932496,"va":5374155300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-1214893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e85376edff","size":5,"soft_fail":false,"target_va":5372938288,"va":5374153176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t2156280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374152512,"raw_hex":"e8f8e62000","size":5,"soft_fail":false,"target_va":5376308960,"va":5374152675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*52009936(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374152512,"raw_hex":"ff15d09b1903","size":6,"soft_fail":false,"va":5374152610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":22068,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":22040,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":24280,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":24336,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":23240,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":23304,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":184,"record_offset":21400,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":23432,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":184,"record_offset":22588,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":184,"record_offset":24048,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":15597,"source_line":104,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::is_queue_empty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":23212,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":15590,"source_line":74,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":22952,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":9582,"source_line":62,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::queue_message
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":184,"record_offset":25460,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":9583,"source_line":69,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":184,"record_offset":23388,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":184,"record_offset":22156,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":15587,"source_line":31,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":184,"record_offset":25720,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":11386,"source_line":183,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":21144,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":12815,"source_line":746,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":22704,"parent_record_offset":20912,"ancestor_record_offset":20912,"inlinee":13599,"source_line":940,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":22328,"parent_record_offset":22304,"ancestor_record_offset":20912,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":22352,"parent_record_offset":22328,"ancestor_record_offset":20912,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":23684,"parent_record_offset":23660,"ancestor_record_offset":20912,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":23708,"parent_record_offset":23684,"ancestor_record_offset":20912,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":22376,"parent_record_offset":22352,"ancestor_record_offset":20912,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":23732,"parent_record_offset":23708,"ancestor_record_offset":20912,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":22304,"parent_record_offset":22272,"ancestor_record_offset":20912,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":184,"record_offset":23660,"parent_record_offset":23572,"ancestor_record_offset":20912,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":22272,"parent_record_offset":22228,"ancestor_record_offset":20912,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":22228,"parent_record_offset":22192,"ancestor_record_offset":20912,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":24092,"parent_record_offset":24048,"ancestor_record_offset":20912,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":184,"record_offset":26076,"parent_record_offset":26048,"ancestor_record_offset":20912,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":23084,"parent_record_offset":23052,"ancestor_record_offset":20912,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":23600,"parent_record_offset":23572,"ancestor_record_offset":20912,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":23572,"parent_record_offset":23548,"ancestor_record_offset":20912,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":23132,"parent_record_offset":23108,"ancestor_record_offset":20912,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":23108,"parent_record_offset":23084,"ancestor_record_offset":20912,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":184,"record_offset":23548,"parent_record_offset":23524,"ancestor_record_offset":20912,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":22836,"parent_record_offset":22808,"ancestor_record_offset":20912,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":22808,"parent_record_offset":22776,"ancestor_record_offset":20912,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":21460,"parent_record_offset":21432,"ancestor_record_offset":20912,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":22448,"parent_record_offset":22424,"ancestor_record_offset":20912,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":23052,"parent_record_offset":23008,"ancestor_record_offset":20912,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":22980,"parent_record_offset":22952,"ancestor_record_offset":20912,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":23008,"parent_record_offset":22980,"ancestor_record_offset":20912,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":184,"record_offset":23460,"parent_record_offset":23432,"ancestor_record_offset":20912,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":24560,"parent_record_offset":24524,"ancestor_record_offset":20912,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":24524,"parent_record_offset":24496,"ancestor_record_offset":20912,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":24812,"parent_record_offset":24788,"ancestor_record_offset":20912,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":184,"record_offset":24632,"parent_record_offset":24608,"ancestor_record_offset":20912,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":184,"record_offset":26108,"parent_record_offset":26076,"ancestor_record_offset":20912,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":184,"record_offset":24608,"parent_record_offset":24584,"ancestor_record_offset":20912,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":184,"record_offset":24584,"parent_record_offset":24560,"ancestor_record_offset":20912,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":26172,"parent_record_offset":26136,"ancestor_record_offset":20912,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":184,"record_offset":21296,"parent_record_offset":21272,"ancestor_record_offset":20912,"inlinee":12821,"source_line":469,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":26136,"parent_record_offset":26108,"ancestor_record_offset":20912,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":184,"record_offset":21272,"parent_record_offset":21228,"ancestor_record_offset":20912,"inlinee":12820,"source_line":681,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":184,"record_offset":23628,"parent_record_offset":23600,"ancestor_record_offset":20912,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":184,"record_offset":23328,"parent_record_offset":23304,"ancestor_record_offset":20912,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":184,"record_offset":24704,"parent_record_offset":24680,"ancestor_record_offset":20912,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":21604,"parent_record_offset":21572,"ancestor_record_offset":20912,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":24304,"parent_record_offset":24280,"ancestor_record_offset":20912,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":24680,"parent_record_offset":24656,"ancestor_record_offset":20912,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":184,"record_offset":21320,"parent_record_offset":21296,"ancestor_record_offset":20912,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":24408,"parent_record_offset":24336,"ancestor_record_offset":20912,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":22872,"parent_record_offset":22836,"ancestor_record_offset":20912,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":24656,"parent_record_offset":24632,"ancestor_record_offset":20912,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":24436,"parent_record_offset":24336,"ancestor_record_offset":20912,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":184,"record_offset":24376,"parent_record_offset":24336,"ancestor_record_offset":20912,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":24460,"parent_record_offset":24436,"ancestor_record_offset":20912,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":23172,"parent_record_offset":23008,"ancestor_record_offset":20912,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":21344,"parent_record_offset":21320,"ancestor_record_offset":20912,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21856,"parent_record_offset":21668,"ancestor_record_offset":20912,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21948,"parent_record_offset":21916,"ancestor_record_offset":20912,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":22096,"parent_record_offset":22068,"ancestor_record_offset":20912,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":22484,"parent_record_offset":22448,"ancestor_record_offset":20912,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21788,"parent_record_offset":21756,"ancestor_record_offset":20912,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":22120,"parent_record_offset":22096,"ancestor_record_offset":20912,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21812,"parent_record_offset":21788,"ancestor_record_offset":20912,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21880,"parent_record_offset":21856,"ancestor_record_offset":20912,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":20912,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21972,"parent_record_offset":21948,"ancestor_record_offset":20912,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":22540,"parent_record_offset":22448,"ancestor_record_offset":20912,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":184,"record_offset":23352,"parent_record_offset":23328,"ancestor_record_offset":20912,"inlinee":15594,"source_line":7,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":184,"record_offset":22192,"parent_record_offset":22156,"ancestor_record_offset":20912,"inlinee":15589,"source_line":139,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":184,"record_offset":21756,"parent_record_offset":21728,"ancestor_record_offset":20912,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":184,"record_offset":21728,"parent_record_offset":21700,"ancestor_record_offset":20912,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":184,"record_offset":21668,"parent_record_offset":21632,"ancestor_record_offset":20912,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":184,"record_offset":21632,"parent_record_offset":21604,"ancestor_record_offset":20912,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":184,"record_offset":21916,"parent_record_offset":21632,"ancestor_record_offset":20912,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":184,"record_offset":21700,"parent_record_offset":21668,"ancestor_record_offset":20912,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":184,"record_offset":21432,"parent_record_offset":21400,"ancestor_record_offset":20912,"inlinee":15599,"source_line":46,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":184,"record_offset":23272,"parent_record_offset":23240,"ancestor_record_offset":20912,"inlinee":15600,"source_line":186,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":21228,"parent_record_offset":21200,"ancestor_record_offset":20912,"inlinee":12819,"source_line":219,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":22728,"parent_record_offset":22704,"ancestor_record_offset":20912,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":21172,"parent_record_offset":21144,"ancestor_record_offset":20912,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":21572,"parent_record_offset":21544,"ancestor_record_offset":20912,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":22776,"parent_record_offset":22752,"ancestor_record_offset":20912,"inlinee":13602,"source_line":147,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":21488,"parent_record_offset":21432,"ancestor_record_offset":20912,"inlinee":12848,"source_line":568,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":21544,"parent_record_offset":21516,"ancestor_record_offset":20912,"inlinee":12850,"source_line":569,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":21200,"parent_record_offset":21172,"ancestor_record_offset":20912,"inlinee":12818,"source_line":747,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":22752,"parent_record_offset":22728,"ancestor_record_offset":20912,"inlinee":13601,"source_line":942,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_winit::run_gui_with_options::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3CEF
// 首条位置: {"module":184,"inlinee":15599}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":184,"record_offset":21460,"parent_record_offset":21432,"ancestor_record_offset":21432,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":21488,"parent_record_offset":21432,"ancestor_record_offset":21432,"inlinee":12848,"source_line":568,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":184,"record_offset":21604,"parent_record_offset":21572,"ancestor_record_offset":21432,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21856,"parent_record_offset":21668,"ancestor_record_offset":21432,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21948,"parent_record_offset":21916,"ancestor_record_offset":21432,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21788,"parent_record_offset":21756,"ancestor_record_offset":21432,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21812,"parent_record_offset":21788,"ancestor_record_offset":21432,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21880,"parent_record_offset":21856,"ancestor_record_offset":21432,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":184,"record_offset":21972,"parent_record_offset":21948,"ancestor_record_offset":21432,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":184,"record_offset":21756,"parent_record_offset":21728,"ancestor_record_offset":21432,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":184,"record_offset":21728,"parent_record_offset":21700,"ancestor_record_offset":21432,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":184,"record_offset":21668,"parent_record_offset":21632,"ancestor_record_offset":21432,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":184,"record_offset":21632,"parent_record_offset":21604,"ancestor_record_offset":21432,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":184,"record_offset":21916,"parent_record_offset":21632,"ancestor_record_offset":21432,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":184,"record_offset":21700,"parent_record_offset":21668,"ancestor_record_offset":21432,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":21516,"parent_record_offset":21488,"ancestor_record_offset":21432,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":184,"record_offset":21572,"parent_record_offset":21544,"ancestor_record_offset":21432,"inlinee":12851,"source_line":456,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":184,"record_offset":21544,"parent_record_offset":21516,"ancestor_record_offset":21432,"inlinee":12850,"source_line":569,"archive":"_pdb/C/modules/00184.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_winit::run_gui_with_options::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3207
// 首条位置: {"module":120,"inlinee":12807}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":120,"record_offset":9448,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":120,"record_offset":8932,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":120,"record_offset":9188,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":120,"record_offset":9356,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::debug::Debug::overlay
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/debug/null.rs
//     evidence={"module":120,"record_offset":9300,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":12831,"source_line":44,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":120,"record_offset":10052,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":9582,"source_line":62,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] iced_winit::conversion::mouse_interaction
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/winit/src/conversion.rs
//     evidence={"module":120,"record_offset":9476,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":12835,"source_line":393,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::exit
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":120,"record_offset":9016,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":12825,"source_line":363,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":8664,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":12815,"source_line":746,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::request_redraw
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":7564,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":12809,"source_line":599,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_cursor_icon
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":9504,"parent_record_offset":7324,"ancestor_record_offset":7324,"inlinee":12836,"source_line":1352,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":120,"record_offset":10200,"parent_record_offset":10168,"ancestor_record_offset":7324,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":120,"record_offset":10248,"parent_record_offset":10224,"ancestor_record_offset":7324,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":120,"record_offset":10224,"parent_record_offset":10200,"ancestor_record_offset":7324,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":120,"record_offset":7708,"parent_record_offset":7668,"ancestor_record_offset":7324,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":120,"record_offset":10168,"parent_record_offset":10120,"ancestor_record_offset":7324,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":120,"record_offset":9324,"parent_record_offset":9300,"ancestor_record_offset":7324,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":120,"record_offset":10084,"parent_record_offset":10052,"ancestor_record_offset":7324,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":120,"record_offset":10120,"parent_record_offset":10084,"ancestor_record_offset":7324,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":120,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":7324,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":120,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":7324,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":120,"record_offset":8980,"parent_record_offset":8956,"ancestor_record_offset":7324,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":120,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":7324,"inlinee":12821,"source_line":469,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":120,"record_offset":9140,"parent_record_offset":9116,"ancestor_record_offset":7324,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":120,"record_offset":8804,"parent_record_offset":8760,"ancestor_record_offset":7324,"inlinee":12820,"source_line":681,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":120,"record_offset":8236,"parent_record_offset":7668,"ancestor_record_offset":7324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":120,"record_offset":8852,"parent_record_offset":8828,"ancestor_record_offset":7324,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":120,"record_offset":10288,"parent_record_offset":10120,"ancestor_record_offset":7324,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":120,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":7324,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":120,"record_offset":8196,"parent_record_offset":7668,"ancestor_record_offset":7324,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":7324,"inlinee":4346,"source_line":726,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":8488,"parent_record_offset":8300,"ancestor_record_offset":7324,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":7804,"parent_record_offset":7772,"ancestor_record_offset":7324,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":8580,"parent_record_offset":8548,"ancestor_record_offset":7324,"inlinee":4809,"source_line":2899,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":8020,"parent_record_offset":7988,"ancestor_record_offset":7324,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":7324,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":8044,"parent_record_offset":8020,"ancestor_record_offset":7324,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":7324,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":120,"record_offset":8604,"parent_record_offset":8580,"ancestor_record_offset":7324,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::interaction::impl$3::eq
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/interaction.rs
//     evidence={"module":120,"record_offset":9416,"parent_record_offset":9392,"ancestor_record_offset":7324,"inlinee":12833,"source_line":2,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::size::impl$12::eq
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/size.rs
//     evidence={"module":120,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":7324,"inlinee":12823,"source_line":4,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":120,"record_offset":9256,"parent_record_offset":9228,"ancestor_record_offset":7324,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":120,"record_offset":7988,"parent_record_offset":7960,"ancestor_record_offset":7324,"inlinee":4771,"source_line":463,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\panicking.rs
//     evidence={"module":120,"record_offset":7960,"parent_record_offset":7932,"ancestor_record_offset":7324,"inlinee":4769,"source_line":615,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":120,"record_offset":8300,"parent_record_offset":8264,"ancestor_record_offset":7324,"inlinee":4765,"source_line":128,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":120,"record_offset":8088,"parent_record_offset":7892,"ancestor_record_offset":7324,"inlinee":4344,"source_line":140,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison.rs
//     evidence={"module":120,"record_offset":7892,"parent_record_offset":7864,"ancestor_record_offset":7324,"inlinee":4982,"source_line":118,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":120,"record_offset":7736,"parent_record_offset":7668,"ancestor_record_offset":7324,"inlinee":4978,"source_line":490,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":120,"record_offset":7864,"parent_record_offset":7736,"ancestor_record_offset":7324,"inlinee":4981,"source_line":719,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sync\poison\mutex.rs
//     evidence={"module":120,"record_offset":8264,"parent_record_offset":8236,"ancestor_record_offset":7324,"inlinee":4805,"source_line":743,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":120,"record_offset":7772,"parent_record_offset":7736,"ancestor_record_offset":7324,"inlinee":4979,"source_line":31,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":120,"record_offset":8548,"parent_record_offset":8264,"ancestor_record_offset":7324,"inlinee":4807,"source_line":89,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::functions::panicking
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\functions.rs
//     evidence={"module":120,"record_offset":7932,"parent_record_offset":7892,"ancestor_record_offset":7324,"inlinee":4767,"source_line":216,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner_winit::run_gui_with_options::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner_winit.rs
//     evidence={"module":120,"record_offset":9228,"parent_record_offset":9188,"ancestor_record_offset":7324,"inlinee":12841,"source_line":121,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::exit
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":120,"record_offset":9044,"parent_record_offset":9016,"ancestor_record_offset":7324,"inlinee":12827,"source_line":525,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::runner::EventLoopRunner::set_exit_code
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop/runner.rs
//     evidence={"module":120,"record_offset":9068,"parent_record_offset":9044,"ancestor_record_offset":7324,"inlinee":12829,"source_line":158,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":8760,"parent_record_offset":8728,"ancestor_record_offset":7324,"inlinee":12819,"source_line":219,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":7600,"parent_record_offset":7564,"ancestor_record_offset":7324,"inlinee":12811,"source_line":112,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":8696,"parent_record_offset":8664,"ancestor_record_offset":7324,"inlinee":12816,"source_line":117,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":7668,"parent_record_offset":7636,"ancestor_record_offset":7324,"inlinee":12814,"source_line":164,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_cursor_icon
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":120,"record_offset":9588,"parent_record_offset":9560,"ancestor_record_offset":7324,"inlinee":12839,"source_line":401,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::request_redraw::closure$0
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":7636,"parent_record_offset":7600,"ancestor_record_offset":7324,"inlinee":12813,"source_line":600,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":8728,"parent_record_offset":8696,"ancestor_record_offset":7324,"inlinee":12818,"source_line":747,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$13::set_cursor_icon::closure$0
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":120,"record_offset":9560,"parent_record_offset":9532,"ancestor_record_offset":7324,"inlinee":12838,"source_line":1354,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_winit::run_gui_with_options::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3229
// 首条位置: {"module":120,"inlinee":12841}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":120,"record_offset":9256,"parent_record_offset":9228,"ancestor_record_offset":9228,"inlinee":12843,"source_line":123,"archive":"_pdb/C/modules/00120.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner_winit::run_gui_with_options::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3CF0
// 首条位置: {"module":184,"inlinee":15600}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: gui_runner_winit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gui_runner_winit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gui_runner_winit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::gpu_selection::Controls>
// 信息 ID: f117f75351bf75d78fe367c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner_winit::run_gui -> ["tg_gpu_selector::gpu_selection::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42e04f -> 0x5327f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42f626 -> 0x5327f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::vulkan_config::Controls>
// 信息 ID: 53a8424ebf06e70f3f9c4c6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner_winit::run_gui -> ["tg_gpu_selector::vulkan_config::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_gpu_selector::run_vulkan_configuration_gui @ 0x6c69fc -> 0xff08b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_winit::run_gui<tg_iced::dialog::Controls>
// 信息 ID: 78303d01a2d599b6d8961c83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner_winit::run_gui -> ["tg_iced::dialog::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_iced::dialog::MessageDialog::show @ 0x6c9822 -> 0xfff790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_winit::run_gui<tg_integrity_report::controls::Controls>
// 信息 ID: 342782f114909fe4ee0240e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner_winit::run_gui -> ["tg_integrity_report::controls::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_integrity_report::run_integrity_report_gui @ 0x6bb5e7 -> 0xfb9d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_winit::run_gui_with_options
// 信息 ID: ffc25e5aee53bfdee8619c18
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=184, inline_record=31180; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_integrity_report::controls::Controls>; module=945, inline_record=6112; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=990, inline_record=6112; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_iced::dialog::Controls>; module=1001, inline_record=6064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_winit::run_gui_with_options::closure$0
// 信息 ID: 8fffc099eebc4f337f19589f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_iced::gui_runner_winit::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui_with_options<crash_reporter_gui::Controls>; module=184, inline_record=21432; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=184, inline_record=31748; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_integrity_report::controls::Controls>; module=945, inline_record=6684; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=990, inline_record=6700; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_iced::dialog::Controls>; module=1001, inline_record=6640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_winit::run_gui_with_options::closure$1
// 信息 ID: 5404ce2433dbdc62bdd6e31a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_iced::gui_runner_winit::run_gui_with_options", "captures": "unknown"}
// [闭包环境类型引用] tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1 <- core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1 <- core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 15 条。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; module=120, inline_record=7324; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >; module=120, inline_record=11076; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >; module=2508, inline_record=6588; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >; module=3746, inline_record=3584; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::vulkan_config::Controls> >; module=3820, inline_record=3580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_winit::run_gui_with_options::closure$1::closure$0
// 信息 ID: e6204ef24d3df3c165ac3a9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_iced::gui_runner_winit::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; module=120, inline_record=9228; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >; module=120, inline_record=12980; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >; module=2508, inline_record=8492; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >; module=3746, inline_record=5488; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner_winit::run_gui_with_options::closure_env$1<tg_gpu_selector::vulkan_config::Controls> >; module=3820, inline_record=5484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_winit::run_gui_with_options::closure$2
// 信息 ID: d0572aa34198e349e65673b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_iced::gui_runner_winit::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui_with_options<crash_reporter_gui::Controls>; module=184, inline_record=23272; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=184, inline_record=33616; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_integrity_report::controls::Controls>; module=945, inline_record=8552; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=990, inline_record=8596; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner_winit::run_gui<tg_iced::dialog::Controls>; module=1001, inline_record=8508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner_winit::run_gui_with_options<crash_reporter_gui::Controls>
// 信息 ID: fd6ca39f23f03137ea5b71c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner_winit::run_gui_with_options -> ["crash_reporter_gui::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_reporter_gui::run_crash_reporter_gui<tiny_glade::mock_crash_reporter::closure_env$1> @ 0x59986c -> 0x530f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$7> @ 0x59aa5a -> 0x530f40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

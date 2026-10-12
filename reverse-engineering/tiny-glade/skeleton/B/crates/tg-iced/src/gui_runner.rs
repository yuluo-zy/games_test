// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tg-iced/src/gui_runner.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 42ed54e92e324095ec21d6baff0c0fb0d577431c1e42d360a5ec714d6582c015
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_runner.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 37, 234, 901, 937, 943, 2213, 2333, 3428, 3505, 4366
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
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
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
// 函数符号: tg_iced::gui_runner::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xD9B6
// 首条位置: {"module":901,"inlinee":55734}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":937,"record_offset":4020,"rva":14613200,"va":5383322320}
// 编译实例 RVAs: 0xdefad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8092436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8ec8484ff","size":5,"soft_fail":false,"target_va":5375230368,"va":5383322799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12950227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8d39ac500","size":5,"soft_fail":false,"target_va":5396273088,"va":5383322856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dc880
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8473945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8a7b27eff","size":5,"soft_fail":false,"target_va":5374855296,"va":5383329236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e8100
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8422475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8b57b7fff","size":5,"soft_fail":false,"target_va":5374902528,"va":5383324998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fa590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8351721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8179080ff","size":5,"soft_fail":false,"target_va":5374977424,"va":5383329140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8241219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8bd3f82ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5383322702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8243547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8a53682ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5383325046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140debf20
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8fab6ffff","size":5,"soft_fail":false,"target_va":5383307040,"va":5383325729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140deca00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8deb8ffff","size":5,"soft_fail":false,"target_va":5383309824,"va":5383328029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140deed10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e875e4ffff","size":5,"soft_fail":false,"target_va":5383318800,"va":5383325846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a4f200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12968606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e89ee2c500","size":5,"soft_fail":false,"target_va":5396296192,"va":5383327581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28524764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8dc40b301","size":5,"soft_fail":false,"target_va":5411847168,"va":5383322399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28213931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8ab82ae01","size":5,"soft_fail":false,"target_va":5411540992,"va":5383327056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-8122738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e88e0e84ff","size":5,"soft_fail":false,"target_va":5375203536,"va":5383326269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28214936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e89886ae01","size":5,"soft_fail":false,"target_va":5411541904,"va":5383326963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-9969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e80fd9ffff","size":5,"soft_fail":false,"target_va":5383316352,"va":5383326316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..program..state..State$LT$tg_gpu_selector..gpu_selection..Controls$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-20334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e892b0ffff","size":5,"soft_fail":false,"target_va":5383305952,"va":5383326281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_tiny_skia..window..compositor..Surface$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-11297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8dfd3ffff","size":5,"soft_fail":false,"target_va":5383315008,"va":5383326300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_gpu_selector..gpu_selection..Controls$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8fdc4ffff","size":5,"soft_fail":false,"target_va":5383314368,"va":5383329470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e878ecffff","size":5,"soft_fail":false,"target_va":5383320624,"va":5383325619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-8009904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e850c785ff","size":5,"soft_fail":false,"target_va":5375312688,"va":5383322587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t12959114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e88abdc500","size":5,"soft_fail":false,"target_va":5396282048,"va":5383322929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t12961709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8adc7c500","size":5,"soft_fail":false,"target_va":5396286528,"va":5383324814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-8101475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e89d6184ff","size":5,"soft_fail":false,"target_va":5375221168,"va":5383322638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-8102595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e83d5d84ff","size":5,"soft_fail":false,"target_va":5375220512,"va":5383323102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28211594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e88a79ae01","size":5,"soft_fail":false,"target_va":5411538608,"va":5383327009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28211972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8047bae01","size":5,"soft_fail":false,"target_va":5411538960,"va":5383326983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t12965103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8efd4c500","size":5,"soft_fail":false,"target_va":5396291168,"va":5383326060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t12969617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e891e6c500","size":5,"soft_fail":false,"target_va":5396292624,"va":5383323002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-7677200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383322320,"raw_hex":"e8f0da8aff","size":5,"soft_fail":false,"target_va":5375645312,"va":5383322507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*37367077(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5383322320,"raw_hex":"ff15252d3a02","size":6,"soft_fail":false,"va":5383322453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":937,"record_offset":4180,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":55734,"source_line":28,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::gui_runner::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":937,"record_offset":4204,"parent_record_offset":4020,"ancestor_record_offset":4020,"inlinee":8462,"source_line":46,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":4020,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":4020,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":4020,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":5588,"parent_record_offset":5552,"ancestor_record_offset":4020,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":5620,"parent_record_offset":5588,"ancestor_record_offset":4020,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":6780,"parent_record_offset":6692,"ancestor_record_offset":4020,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":937,"record_offset":5552,"parent_record_offset":5516,"ancestor_record_offset":4020,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":937,"record_offset":9512,"parent_record_offset":9484,"ancestor_record_offset":4020,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":937,"record_offset":6720,"parent_record_offset":6692,"ancestor_record_offset":4020,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":937,"record_offset":6692,"parent_record_offset":6668,"ancestor_record_offset":4020,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":937,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":4020,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":937,"record_offset":6152,"parent_record_offset":6124,"ancestor_record_offset":4020,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":937,"record_offset":5392,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":937,"record_offset":4784,"parent_record_offset":4756,"ancestor_record_offset":4020,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":937,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":4020,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":937,"record_offset":7256,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":937,"record_offset":6568,"parent_record_offset":6536,"ancestor_record_offset":4020,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":937,"record_offset":7540,"parent_record_offset":7500,"ancestor_record_offset":4020,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":937,"record_offset":7500,"parent_record_offset":7472,"ancestor_record_offset":4020,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":937,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":4020,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":937,"record_offset":7312,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":937,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":4020,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":937,"record_offset":9544,"parent_record_offset":9512,"ancestor_record_offset":4020,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":937,"record_offset":4684,"parent_record_offset":4528,"ancestor_record_offset":4020,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":937,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":4020,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":937,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":4020,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":937,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":4020,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":937,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":4020,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":937,"record_offset":9568,"parent_record_offset":9544,"ancestor_record_offset":4020,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":937,"record_offset":4572,"parent_record_offset":4528,"ancestor_record_offset":4020,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":937,"record_offset":6748,"parent_record_offset":6720,"ancestor_record_offset":4020,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":937,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":4020,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":937,"record_offset":6268,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":937,"record_offset":6332,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":937,"record_offset":4728,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":937,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":4020,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":937,"record_offset":4928,"parent_record_offset":4896,"ancestor_record_offset":4020,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":937,"record_offset":7280,"parent_record_offset":7256,"ancestor_record_offset":4020,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":937,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":4020,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":937,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":4020,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":937,"record_offset":7384,"parent_record_offset":7312,"ancestor_record_offset":4020,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":937,"record_offset":6188,"parent_record_offset":6152,"ancestor_record_offset":4020,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":937,"record_offset":7412,"parent_record_offset":7312,"ancestor_record_offset":4020,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":937,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":4020,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":937,"record_offset":7352,"parent_record_offset":7312,"ancestor_record_offset":4020,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":937,"record_offset":7436,"parent_record_offset":7412,"ancestor_record_offset":4020,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":937,"record_offset":5696,"parent_record_offset":5668,"ancestor_record_offset":4020,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":937,"record_offset":5904,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":5180,"parent_record_offset":4992,"ancestor_record_offset":4020,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":5272,"parent_record_offset":5240,"ancestor_record_offset":4020,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":5420,"parent_record_offset":5392,"ancestor_record_offset":4020,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":5800,"parent_record_offset":5764,"ancestor_record_offset":4020,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":5112,"parent_record_offset":5080,"ancestor_record_offset":4020,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":5444,"parent_record_offset":5420,"ancestor_record_offset":4020,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":4020,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":4020,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":4020,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":4020,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":5856,"parent_record_offset":5764,"ancestor_record_offset":4020,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":937,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":4020,"inlinee":8442,"source_line":7,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":937,"record_offset":7056,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":8446,"source_line":104,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":937,"record_offset":6416,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":8444,"source_line":123,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":937,"record_offset":5480,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":8434,"source_line":31,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":937,"record_offset":5516,"parent_record_offset":5480,"ancestor_record_offset":4020,"inlinee":8436,"source_line":139,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":937,"record_offset":5080,"parent_record_offset":5052,"ancestor_record_offset":4020,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":937,"record_offset":5052,"parent_record_offset":5024,"ancestor_record_offset":4020,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":937,"record_offset":4992,"parent_record_offset":4956,"ancestor_record_offset":4020,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":937,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":4020,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":937,"record_offset":5240,"parent_record_offset":4956,"ancestor_record_offset":4020,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":937,"record_offset":5024,"parent_record_offset":4992,"ancestor_record_offset":4020,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::gpu_selection::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/gpu_selection.rs
//     evidence={"module":937,"record_offset":8296,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":55845,"source_line":58,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":937,"record_offset":4756,"parent_record_offset":4728,"ancestor_record_offset":4020,"inlinee":8429,"source_line":59,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":937,"record_offset":6300,"parent_record_offset":6268,"ancestor_record_offset":4020,"inlinee":8440,"source_line":194,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":937,"record_offset":8332,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":8450,"source_line":183,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":937,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":4020,"inlinee":6859,"source_line":219,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":937,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":4020,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":937,"record_offset":4472,"parent_record_offset":4444,"ancestor_record_offset":4020,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":937,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":4020,"inlinee":6848,"source_line":456,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":937,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":4020,"inlinee":6838,"source_line":147,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":4444,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":8424,"source_line":746,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":4812,"parent_record_offset":4756,"ancestor_record_offset":4020,"inlinee":8430,"source_line":568,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":6020,"parent_record_offset":4204,"ancestor_record_offset":4020,"inlinee":8437,"source_line":940,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":4868,"parent_record_offset":4840,"ancestor_record_offset":4020,"inlinee":8432,"source_line":569,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":4500,"parent_record_offset":4472,"ancestor_record_offset":4020,"inlinee":8427,"source_line":747,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":4020,"inlinee":8439,"source_line":942,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":937,"record_offset":12144,"rva":14620368,"va":5383329488}
// 编译实例 RVAs: 0xdf16d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8099663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e8b16884ff","size":5,"soft_fail":false,"target_va":5375230368,"va":5383330026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12943007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e89f7ec500","size":5,"soft_fail":false,"target_va":5396273088,"va":5383330076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dc880
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8481721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e847947eff","size":5,"soft_fail":false,"target_va":5374855296,"va":5383337012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e8100
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8429795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e81d5f7fff","size":5,"soft_fail":false,"target_va":5374902528,"va":5383332318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fa590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8359497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e8b77180ff","size":5,"soft_fail":false,"target_va":5374977424,"va":5383336916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe270
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8343517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e823b080ff","size":5,"soft_fail":false,"target_va":5374993008,"va":5383336520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8248446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e8822382ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5383329929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8250866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e80e1a82ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5383332365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140debf20
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e84a9affff","size":5,"soft_fail":false,"target_va":5383307040,"va":5383333073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140deca00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e88e9affff","size":5,"soft_fail":false,"target_va":5383309824,"va":5383335789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140deed10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e845c7ffff","size":5,"soft_fail":false,"target_va":5383318800,"va":5383333318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a4f200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12960855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e857c4c500","size":5,"soft_fail":false,"target_va":5396296192,"va":5383335332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a50530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12967918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e8eedfc500","size":5,"soft_fail":false,"target_va":5396301104,"va":5383333181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28206187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e86b64ae01","size":5,"soft_fail":false,"target_va":5411540992,"va":5383334800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-8130090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e8d6f183ff","size":5,"soft_fail":false,"target_va":5375203536,"va":5383333621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28207192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e85868ae01","size":5,"soft_fail":false,"target_va":5411541904,"va":5383334707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-17321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e857bcffff","size":5,"soft_fail":false,"target_va":5383316352,"va":5383333668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..program..state..State$LT$tg_gpu_selector..vulkan_config..Controls$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-27126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e80a96ffff","size":5,"soft_fail":false,"target_va":5383306512,"va":5383333633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_tiny_skia..window..compositor..Surface$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-18649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e827b7ffff","size":5,"soft_fail":false,"target_va":5383315008,"va":5383333652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_gpu_selector..vulkan_config..Controls$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e82aa8ffff","size":5,"soft_fail":false,"target_va":5383314768,"va":5383337249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e8d6cfffff","size":5,"soft_fail":false,"target_va":5383320624,"va":5383332949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-8017131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e815ab85ff","size":5,"soft_fail":false,"target_va":5375312688,"va":5383329814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t12952326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e806a3c500","size":5,"soft_fail":false,"target_va":5396282560,"va":5383330229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t12950945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e8a19dc500","size":5,"soft_fail":false,"target_va":5396283088,"va":5383332138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-8108702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e8624584ff","size":5,"soft_fail":false,"target_va":5375221168,"va":5383329865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-8109923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e89d4084ff","size":5,"soft_fail":false,"target_va":5375220512,"va":5383330430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28203850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e84a5bae01","size":5,"soft_fail":false,"target_va":5411538608,"va":5383334753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28204228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e8c45cae01","size":5,"soft_fail":false,"target_va":5411538960,"va":5383334727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t12957031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e867b5c500","size":5,"soft_fail":false,"target_va":5396290576,"va":5383333540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t12962300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e8fcc9c500","size":5,"soft_fail":false,"target_va":5396292624,"va":5383330319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-7684427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383329488,"raw_hex":"e8b5be8aff","size":5,"soft_fail":false,"target_va":5375645312,"va":5383329734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*37359848(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5383329488,"raw_hex":"ff15e8103a02","size":6,"soft_fail":false,"va":5383329682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":937,"record_offset":12304,"parent_record_offset":12144,"ancestor_record_offset":12144,"inlinee":55734,"source_line":28,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::gui_runner::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":937,"record_offset":12332,"parent_record_offset":12144,"ancestor_record_offset":12144,"inlinee":8462,"source_line":46,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":13768,"parent_record_offset":13744,"ancestor_record_offset":12144,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":13792,"parent_record_offset":13768,"ancestor_record_offset":12144,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":14940,"parent_record_offset":14916,"ancestor_record_offset":12144,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":13712,"parent_record_offset":13676,"ancestor_record_offset":12144,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":13744,"parent_record_offset":13712,"ancestor_record_offset":12144,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":937,"record_offset":14916,"parent_record_offset":14828,"ancestor_record_offset":12144,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":937,"record_offset":13676,"parent_record_offset":13640,"ancestor_record_offset":12144,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":937,"record_offset":16948,"parent_record_offset":16920,"ancestor_record_offset":12144,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":937,"record_offset":14856,"parent_record_offset":14828,"ancestor_record_offset":12144,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":937,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":12144,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":937,"record_offset":14804,"parent_record_offset":14780,"ancestor_record_offset":12144,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":937,"record_offset":14304,"parent_record_offset":14276,"ancestor_record_offset":12144,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":937,"record_offset":13516,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":937,"record_offset":12908,"parent_record_offset":12880,"ancestor_record_offset":12144,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":937,"record_offset":13888,"parent_record_offset":13864,"ancestor_record_offset":12144,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":937,"record_offset":15392,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":937,"record_offset":14716,"parent_record_offset":14688,"ancestor_record_offset":12144,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":937,"record_offset":15676,"parent_record_offset":15636,"ancestor_record_offset":12144,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":937,"record_offset":15636,"parent_record_offset":15608,"ancestor_record_offset":12144,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":937,"record_offset":15928,"parent_record_offset":15904,"ancestor_record_offset":12144,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":937,"record_offset":15448,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":937,"record_offset":15748,"parent_record_offset":15724,"ancestor_record_offset":12144,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":937,"record_offset":16980,"parent_record_offset":16948,"ancestor_record_offset":12144,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":937,"record_offset":16516,"parent_record_offset":16484,"ancestor_record_offset":12144,"inlinee":4549,"source_line":165,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":937,"record_offset":12808,"parent_record_offset":12652,"ancestor_record_offset":12144,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":937,"record_offset":12768,"parent_record_offset":12744,"ancestor_record_offset":12144,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":937,"record_offset":15724,"parent_record_offset":15700,"ancestor_record_offset":12144,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":937,"record_offset":15700,"parent_record_offset":15676,"ancestor_record_offset":12144,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":937,"record_offset":12720,"parent_record_offset":12696,"ancestor_record_offset":12144,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":937,"record_offset":17004,"parent_record_offset":16980,"ancestor_record_offset":12144,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":937,"record_offset":12696,"parent_record_offset":12652,"ancestor_record_offset":12144,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":937,"record_offset":14884,"parent_record_offset":14856,"ancestor_record_offset":12144,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":937,"record_offset":14508,"parent_record_offset":14484,"ancestor_record_offset":12144,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":937,"record_offset":14420,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":937,"record_offset":14484,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":937,"record_offset":12852,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":937,"record_offset":16456,"parent_record_offset":16432,"ancestor_record_offset":12144,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":937,"record_offset":15820,"parent_record_offset":15796,"ancestor_record_offset":12144,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=65
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":937,"record_offset":13052,"parent_record_offset":13020,"ancestor_record_offset":12144,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":937,"record_offset":15416,"parent_record_offset":15392,"ancestor_record_offset":12144,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":937,"record_offset":15796,"parent_record_offset":15772,"ancestor_record_offset":12144,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":937,"record_offset":12744,"parent_record_offset":12720,"ancestor_record_offset":12144,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":937,"record_offset":15520,"parent_record_offset":15448,"ancestor_record_offset":12144,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":937,"record_offset":14340,"parent_record_offset":14304,"ancestor_record_offset":12144,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":937,"record_offset":15548,"parent_record_offset":15448,"ancestor_record_offset":12144,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":937,"record_offset":15772,"parent_record_offset":15748,"ancestor_record_offset":12144,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":937,"record_offset":15488,"parent_record_offset":15448,"ancestor_record_offset":12144,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":937,"record_offset":15572,"parent_record_offset":15548,"ancestor_record_offset":12144,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":937,"record_offset":13820,"parent_record_offset":13792,"ancestor_record_offset":12144,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":937,"record_offset":14028,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":13304,"parent_record_offset":13116,"ancestor_record_offset":12144,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":13396,"parent_record_offset":13364,"ancestor_record_offset":12144,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":13544,"parent_record_offset":13516,"ancestor_record_offset":12144,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":13924,"parent_record_offset":13888,"ancestor_record_offset":12144,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":13236,"parent_record_offset":13204,"ancestor_record_offset":12144,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":13568,"parent_record_offset":13544,"ancestor_record_offset":12144,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":12144,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":13328,"parent_record_offset":13304,"ancestor_record_offset":12144,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":13948,"parent_record_offset":13924,"ancestor_record_offset":12144,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":13420,"parent_record_offset":13396,"ancestor_record_offset":12144,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":937,"record_offset":13980,"parent_record_offset":13888,"ancestor_record_offset":12144,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":937,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":12144,"inlinee":8442,"source_line":7,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":937,"record_offset":15192,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":8446,"source_line":104,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":937,"record_offset":14568,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":8444,"source_line":123,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":937,"record_offset":13604,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":8434,"source_line":31,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":937,"record_offset":13640,"parent_record_offset":13604,"ancestor_record_offset":12144,"inlinee":8436,"source_line":139,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":937,"record_offset":16540,"parent_record_offset":16516,"ancestor_record_offset":12144,"inlinee":10696,"source_line":93,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":937,"record_offset":13204,"parent_record_offset":13176,"ancestor_record_offset":12144,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":937,"record_offset":13176,"parent_record_offset":13148,"ancestor_record_offset":12144,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":937,"record_offset":13116,"parent_record_offset":13080,"ancestor_record_offset":12144,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":937,"record_offset":13080,"parent_record_offset":13052,"ancestor_record_offset":12144,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":937,"record_offset":13364,"parent_record_offset":13080,"ancestor_record_offset":12144,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":937,"record_offset":13148,"parent_record_offset":13116,"ancestor_record_offset":12144,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::vulkan_config::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":937,"record_offset":16432,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":55849,"source_line":51,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::vulkan_config::impl$4::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":937,"record_offset":16484,"parent_record_offset":16456,"ancestor_record_offset":12144,"inlinee":55851,"source_line":18,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::vulkan_config::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":937,"record_offset":14144,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":55847,"source_line":18,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":937,"record_offset":12880,"parent_record_offset":12852,"ancestor_record_offset":12144,"inlinee":8429,"source_line":59,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":937,"record_offset":14452,"parent_record_offset":14420,"ancestor_record_offset":12144,"inlinee":8440,"source_line":194,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":937,"record_offset":16592,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":8450,"source_line":183,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":937,"record_offset":12652,"parent_record_offset":12624,"ancestor_record_offset":12144,"inlinee":6859,"source_line":219,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":937,"record_offset":14196,"parent_record_offset":14172,"ancestor_record_offset":12144,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":937,"record_offset":12596,"parent_record_offset":12568,"ancestor_record_offset":12144,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":937,"record_offset":13020,"parent_record_offset":12992,"ancestor_record_offset":12144,"inlinee":6848,"source_line":456,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":937,"record_offset":14244,"parent_record_offset":14220,"ancestor_record_offset":12144,"inlinee":6838,"source_line":147,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":12568,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":8424,"source_line":746,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":12936,"parent_record_offset":12880,"ancestor_record_offset":12144,"inlinee":8430,"source_line":568,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":14172,"parent_record_offset":12332,"ancestor_record_offset":12144,"inlinee":8437,"source_line":940,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":12992,"parent_record_offset":12964,"ancestor_record_offset":12144,"inlinee":8432,"source_line":569,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":12624,"parent_record_offset":12596,"ancestor_record_offset":12144,"inlinee":8427,"source_line":747,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":937,"record_offset":14220,"parent_record_offset":14196,"ancestor_record_offset":12144,"inlinee":8439,"source_line":942,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":943,"record_offset":3988,"rva":14682688,"va":5383391808}
// 编译实例 RVAs: 0xe00a40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8161963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8557583ff","size":5,"soft_fail":false,"target_va":5375230368,"va":5383392326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t13062668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e80c52c700","size":5,"soft_fail":false,"target_va":5396455056,"va":5383392383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dc880
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8543481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e807a37dff","size":5,"soft_fail":false,"target_va":5374855296,"va":5383398772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e8100
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8492059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8e56b7eff","size":5,"soft_fail":false,"target_va":5374902528,"va":5383394582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8475142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8faad7eff","size":5,"soft_fail":false,"target_va":5374923872,"va":5383399009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fa590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8421257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e877807fff","size":5,"soft_fail":false,"target_va":5374977424,"va":5383398676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8310746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8263081ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5383392229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8313131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8d52681ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5383394630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a79260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13071198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e85e73c700","size":5,"soft_fail":false,"target_va":5396468320,"va":5383397117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28144395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e80b73ad01","size":5,"soft_fail":false,"target_va":5411540992,"va":5383396592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-8192175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e851ff82ff","size":5,"soft_fail":false,"target_va":5375203536,"va":5383395706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28145400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8f876ad01","size":5,"soft_fail":false,"target_va":5411541904,"va":5383396499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$iced_runtime..command..action..Action$LT$tg_iced..dialog..Message$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-47570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e82e46ffff","size":5,"soft_fail":false,"target_va":5383350000,"va":5383397565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-38830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e85268ffff","size":5,"soft_fail":false,"target_va":5383356928,"va":5383395753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..Command$LT$tg_iced..dialog..Message$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-41063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8995fffff","size":5,"soft_fail":false,"target_va":5383354384,"va":5383395442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$tg_iced..dialog..Message$GT$$GT$::h3b2154c2eb66ee85 (.llvm.13565244980142266012)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-40086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e86a63ffff","size":5,"soft_fail":false,"target_va":5383355232,"va":5383395313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..program..state..State$LT$tg_iced..dialog..Controls$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-40987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8e55fffff","size":5,"soft_fail":false,"target_va":5383354736,"va":5383395718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_tiny_skia..window..compositor..Surface$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-42190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8325bffff","size":5,"soft_fail":false,"target_va":5383353552,"va":5383395737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<tg_iced::dialog::Message> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e818ecffff","size":5,"soft_fail":false,"target_va":5383390112,"va":5383395203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-8079431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8b9b784ff","size":5,"soft_fail":false,"target_va":5375312688,"va":5383392114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t13071503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e88f74c700","size":5,"soft_fail":false,"target_va":5396464016,"va":5383392508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t13070132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8346fc700","size":5,"soft_fail":false,"target_va":5396464528,"va":5383394391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-8171002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8065283ff","size":5,"soft_fail":false,"target_va":5375221168,"va":5383392165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-8172179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e86d4d83ff","size":5,"soft_fail":false,"target_va":5375220512,"va":5383392686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28142058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8ea69ad01","size":5,"soft_fail":false,"target_va":5411538608,"va":5383396545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28142436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8646bad01","size":5,"soft_fail":false,"target_va":5411538960,"va":5383396519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-31366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e87a85ffff","size":5,"soft_fail":false,"target_va":5383364288,"va":5383395649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-24330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e8f6a0ffff","size":5,"soft_fail":false,"target_va":5383368256,"va":5383392581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-7746727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383391808,"raw_hex":"e859cb89ff","size":5,"soft_fail":false,"target_va":5375645312,"va":5383392034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*37297550(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5383391808,"raw_hex":"ff158e1d3902","size":6,"soft_fail":false,"va":5383391980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":943,"record_offset":4132,"parent_record_offset":3988,"ancestor_record_offset":3988,"inlinee":55734,"source_line":28,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::gui_runner::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":943,"record_offset":4156,"parent_record_offset":3988,"ancestor_record_offset":3988,"inlinee":8462,"source_line":46,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":943,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":3988,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":943,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":3988,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":943,"record_offset":6764,"parent_record_offset":6740,"ancestor_record_offset":3988,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":943,"record_offset":5548,"parent_record_offset":5512,"ancestor_record_offset":3988,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":943,"record_offset":5580,"parent_record_offset":5548,"ancestor_record_offset":3988,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":943,"record_offset":6740,"parent_record_offset":6652,"ancestor_record_offset":3988,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":943,"record_offset":5512,"parent_record_offset":5476,"ancestor_record_offset":3988,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":943,"record_offset":8640,"parent_record_offset":8612,"ancestor_record_offset":3988,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":943,"record_offset":6680,"parent_record_offset":6652,"ancestor_record_offset":3988,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":943,"record_offset":6652,"parent_record_offset":6628,"ancestor_record_offset":3988,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":943,"record_offset":6628,"parent_record_offset":6604,"ancestor_record_offset":3988,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":943,"record_offset":6112,"parent_record_offset":6084,"ancestor_record_offset":3988,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":943,"record_offset":5352,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":943,"record_offset":4744,"parent_record_offset":4716,"ancestor_record_offset":3988,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":943,"record_offset":5724,"parent_record_offset":5700,"ancestor_record_offset":3988,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":943,"record_offset":7216,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":943,"record_offset":6528,"parent_record_offset":6496,"ancestor_record_offset":3988,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":943,"record_offset":7500,"parent_record_offset":7460,"ancestor_record_offset":3988,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":943,"record_offset":7460,"parent_record_offset":7432,"ancestor_record_offset":3988,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":943,"record_offset":7752,"parent_record_offset":7728,"ancestor_record_offset":3988,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":943,"record_offset":7272,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":943,"record_offset":7572,"parent_record_offset":7548,"ancestor_record_offset":3988,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":943,"record_offset":8672,"parent_record_offset":8640,"ancestor_record_offset":3988,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":943,"record_offset":4644,"parent_record_offset":4488,"ancestor_record_offset":3988,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":943,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":3988,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":943,"record_offset":7548,"parent_record_offset":7524,"ancestor_record_offset":3988,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":943,"record_offset":7524,"parent_record_offset":7500,"ancestor_record_offset":3988,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":943,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":3988,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":943,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":3988,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":943,"record_offset":4532,"parent_record_offset":4488,"ancestor_record_offset":3988,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":943,"record_offset":6708,"parent_record_offset":6680,"ancestor_record_offset":3988,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":943,"record_offset":6316,"parent_record_offset":6292,"ancestor_record_offset":3988,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":943,"record_offset":6228,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":943,"record_offset":6292,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":943,"record_offset":4688,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":943,"record_offset":7644,"parent_record_offset":7620,"ancestor_record_offset":3988,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":943,"record_offset":4888,"parent_record_offset":4856,"ancestor_record_offset":3988,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":943,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":3988,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":943,"record_offset":7620,"parent_record_offset":7596,"ancestor_record_offset":3988,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":943,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":3988,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":943,"record_offset":7344,"parent_record_offset":7272,"ancestor_record_offset":3988,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":943,"record_offset":6148,"parent_record_offset":6112,"ancestor_record_offset":3988,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":943,"record_offset":7372,"parent_record_offset":7272,"ancestor_record_offset":3988,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":943,"record_offset":7596,"parent_record_offset":7572,"ancestor_record_offset":3988,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":943,"record_offset":7312,"parent_record_offset":7272,"ancestor_record_offset":3988,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":943,"record_offset":7396,"parent_record_offset":7372,"ancestor_record_offset":3988,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":943,"record_offset":5656,"parent_record_offset":5628,"ancestor_record_offset":3988,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":943,"record_offset":5864,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":943,"record_offset":5140,"parent_record_offset":4952,"ancestor_record_offset":3988,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":943,"record_offset":5232,"parent_record_offset":5200,"ancestor_record_offset":3988,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":943,"record_offset":5380,"parent_record_offset":5352,"ancestor_record_offset":3988,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":943,"record_offset":5760,"parent_record_offset":5724,"ancestor_record_offset":3988,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":943,"record_offset":5072,"parent_record_offset":5040,"ancestor_record_offset":3988,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":943,"record_offset":5404,"parent_record_offset":5380,"ancestor_record_offset":3988,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":943,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":3988,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":943,"record_offset":5164,"parent_record_offset":5140,"ancestor_record_offset":3988,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":943,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":3988,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":943,"record_offset":5256,"parent_record_offset":5232,"ancestor_record_offset":3988,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":943,"record_offset":5816,"parent_record_offset":5724,"ancestor_record_offset":3988,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":943,"record_offset":6340,"parent_record_offset":6316,"ancestor_record_offset":3988,"inlinee":8442,"source_line":7,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":943,"record_offset":7016,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":8446,"source_line":104,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":943,"record_offset":6376,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":8444,"source_line":123,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":943,"record_offset":5440,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":8434,"source_line":31,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":943,"record_offset":5476,"parent_record_offset":5440,"ancestor_record_offset":3988,"inlinee":8436,"source_line":139,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":943,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":3988,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":943,"record_offset":5012,"parent_record_offset":4984,"ancestor_record_offset":3988,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":943,"record_offset":4952,"parent_record_offset":4916,"ancestor_record_offset":3988,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":943,"record_offset":4916,"parent_record_offset":4888,"ancestor_record_offset":3988,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":943,"record_offset":5200,"parent_record_offset":4916,"ancestor_record_offset":3988,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":943,"record_offset":4984,"parent_record_offset":4952,"ancestor_record_offset":3988,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::dialog::impl$2::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/dialog.rs
//     evidence={"module":943,"record_offset":8256,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":17078,"source_line":147,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":943,"record_offset":4716,"parent_record_offset":4688,"ancestor_record_offset":3988,"inlinee":8429,"source_line":59,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":943,"record_offset":6260,"parent_record_offset":6228,"ancestor_record_offset":3988,"inlinee":8440,"source_line":194,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":943,"record_offset":8284,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":8450,"source_line":183,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":943,"record_offset":4488,"parent_record_offset":4460,"ancestor_record_offset":3988,"inlinee":6859,"source_line":219,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":943,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":3988,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":943,"record_offset":4432,"parent_record_offset":4404,"ancestor_record_offset":3988,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":943,"record_offset":4856,"parent_record_offset":4828,"ancestor_record_offset":3988,"inlinee":6848,"source_line":456,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":943,"record_offset":6052,"parent_record_offset":6028,"ancestor_record_offset":3988,"inlinee":6838,"source_line":147,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":943,"record_offset":4404,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":8424,"source_line":746,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":943,"record_offset":4772,"parent_record_offset":4716,"ancestor_record_offset":3988,"inlinee":8430,"source_line":568,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":943,"record_offset":5980,"parent_record_offset":4156,"ancestor_record_offset":3988,"inlinee":8437,"source_line":940,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":943,"record_offset":4828,"parent_record_offset":4800,"ancestor_record_offset":3988,"inlinee":8432,"source_line":569,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":943,"record_offset":4460,"parent_record_offset":4432,"ancestor_record_offset":3988,"inlinee":8427,"source_line":747,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":943,"record_offset":6028,"parent_record_offset":6004,"ancestor_record_offset":3988,"inlinee":8439,"source_line":942,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":901,"record_offset":4024,"rva":14466784,"va":5383175904}
// 编译实例 RVAs: 0xdcbee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7946048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8c0c086ff","size":5,"soft_fail":false,"target_va":5375230368,"va":5383176411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t12702759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e827d4c100","size":5,"soft_fail":false,"target_va":5395879232,"va":5383176468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dc880
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8327561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e877ee80ff","size":5,"soft_fail":false,"target_va":5374855296,"va":5383182852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e8100
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8276155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e845b781ff","size":5,"soft_fail":false,"target_va":5374902528,"va":5383178678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fa590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8205337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8e7cb82ff","size":5,"soft_fail":false,"target_va":5374977424,"va":5383182756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8094834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e88e7b84ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5383176317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8097227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8357284ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5383178726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28360320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e880beb001","size":5,"soft_fail":false,"target_va":5411540992,"va":5383180667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-7976269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8b34a86ff","size":5,"soft_fail":false,"target_va":5375203536,"va":5383179800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28361325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e86dc2b001","size":5,"soft_fail":false,"target_va":5411541904,"va":5383180574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$iced_runtime..command..action..Action$LT$tg_integrity_report..controls..Message$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12623278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8ae9dc000","size":5,"soft_fail":false,"target_va":5395804928,"va":5383181645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t12628068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e864b0c000","size":5,"soft_fail":false,"target_va":5395807920,"va":5383179847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..Command$LT$tg_integrity_report..controls..Message$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t12630815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e81fbbc000","size":5,"soft_fail":false,"target_va":5395810368,"va":5383179548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$tg_integrity_report..controls..Message$GT$$GT$::h7dae3a7ce3dddaed (.llvm.13634732314637635666)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t12622714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e87a9bc000","size":5,"soft_fail":false,"target_va":5395802144,"va":5383179425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$tg_integrity_report..controls..Message$GT$$GT$::h7dae3a7ce3dddaed (.llvm.13760847750482213960)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12715934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e89e07c200","size":5,"soft_fail":false,"target_va":5395897136,"va":5383181197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..program..state..State$LT$tg_integrity_report..controls..Controls$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t12621767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8c797c000","size":5,"soft_fail":false,"target_va":5395801584,"va":5383179812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_tiny_skia..window..compositor..Surface$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t12627876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8a4afc000","size":5,"soft_fail":false,"target_va":5395807712,"va":5383179831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_integrity_report..controls..Controls$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12624202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e84aa1c000","size":5,"soft_fail":false,"target_va":5395807296,"va":5383183089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e811ecffff","size":5,"soft_fail":false,"target_va":5383174208,"va":5383179306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-7863519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8210388ff","size":5,"soft_fail":false,"target_va":5375312688,"va":5383176202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t12711593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8a9f6c100","size":5,"soft_fail":false,"target_va":5395888192,"va":5383176594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t12710217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e849f1c100","size":5,"soft_fail":false,"target_va":5395888704,"va":5383178482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-7955090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e86e9d86ff","size":5,"soft_fail":false,"target_va":5375221168,"va":5383176253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t-7956259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8dd9886ff","size":5,"soft_fail":false,"target_va":5375220512,"va":5383176766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28357983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e85fb5b001","size":5,"soft_fail":false,"target_va":5411538608,"va":5383180620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28358361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8d9b6b001","size":5,"soft_fail":false,"target_va":5411538960,"va":5383180594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t12712989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e81dfcc100","size":5,"soft_fail":false,"target_va":5395892752,"va":5383179758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t12717872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8300fc200","size":5,"soft_fail":false,"target_va":5395894544,"va":5383176667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t-7530812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383175904,"raw_hex":"e8c4168dff","size":5,"soft_fail":false,"target_va":5375645312,"va":5383176119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*37513465(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5383175904,"raw_hex":"ff15f9683c02","size":6,"soft_fail":false,"va":5383176065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":901,"record_offset":4184,"parent_record_offset":4024,"ancestor_record_offset":4024,"inlinee":55734,"source_line":28,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::gui_runner::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":901,"record_offset":4208,"parent_record_offset":4024,"ancestor_record_offset":4024,"inlinee":8462,"source_line":46,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":4024,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":4024,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":4024,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":5588,"parent_record_offset":5552,"ancestor_record_offset":4024,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":5620,"parent_record_offset":5588,"ancestor_record_offset":4024,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":6780,"parent_record_offset":6692,"ancestor_record_offset":4024,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":901,"record_offset":5552,"parent_record_offset":5516,"ancestor_record_offset":4024,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":901,"record_offset":8652,"parent_record_offset":8624,"ancestor_record_offset":4024,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":901,"record_offset":6720,"parent_record_offset":6692,"ancestor_record_offset":4024,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":901,"record_offset":6692,"parent_record_offset":6668,"ancestor_record_offset":4024,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":901,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":4024,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":901,"record_offset":6152,"parent_record_offset":6124,"ancestor_record_offset":4024,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":901,"record_offset":5392,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":901,"record_offset":4784,"parent_record_offset":4756,"ancestor_record_offset":4024,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":901,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":4024,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":901,"record_offset":7256,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":901,"record_offset":6568,"parent_record_offset":6536,"ancestor_record_offset":4024,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":901,"record_offset":7540,"parent_record_offset":7500,"ancestor_record_offset":4024,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":901,"record_offset":7500,"parent_record_offset":7472,"ancestor_record_offset":4024,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":901,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":4024,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":901,"record_offset":7312,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":901,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":4024,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":901,"record_offset":8684,"parent_record_offset":8652,"ancestor_record_offset":4024,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":901,"record_offset":4684,"parent_record_offset":4528,"ancestor_record_offset":4024,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":901,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":4024,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":901,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":4024,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":901,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":4024,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":901,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":4024,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":901,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":4024,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":901,"record_offset":4572,"parent_record_offset":4528,"ancestor_record_offset":4024,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":901,"record_offset":6748,"parent_record_offset":6720,"ancestor_record_offset":4024,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":901,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":4024,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":901,"record_offset":6268,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":901,"record_offset":6332,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":901,"record_offset":4728,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":901,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":4024,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":901,"record_offset":4928,"parent_record_offset":4896,"ancestor_record_offset":4024,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":901,"record_offset":7280,"parent_record_offset":7256,"ancestor_record_offset":4024,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":901,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":4024,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":901,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":4024,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":901,"record_offset":7384,"parent_record_offset":7312,"ancestor_record_offset":4024,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":901,"record_offset":6188,"parent_record_offset":6152,"ancestor_record_offset":4024,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":901,"record_offset":7412,"parent_record_offset":7312,"ancestor_record_offset":4024,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":901,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":4024,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":901,"record_offset":7352,"parent_record_offset":7312,"ancestor_record_offset":4024,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":901,"record_offset":7436,"parent_record_offset":7412,"ancestor_record_offset":4024,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":901,"record_offset":5696,"parent_record_offset":5668,"ancestor_record_offset":4024,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":901,"record_offset":5904,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5180,"parent_record_offset":4992,"ancestor_record_offset":4024,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5272,"parent_record_offset":5240,"ancestor_record_offset":4024,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5420,"parent_record_offset":5392,"ancestor_record_offset":4024,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5800,"parent_record_offset":5764,"ancestor_record_offset":4024,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5112,"parent_record_offset":5080,"ancestor_record_offset":4024,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5444,"parent_record_offset":5420,"ancestor_record_offset":4024,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":4024,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":4024,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":4024,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":4024,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5856,"parent_record_offset":5764,"ancestor_record_offset":4024,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":901,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":4024,"inlinee":8442,"source_line":7,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":901,"record_offset":7056,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":8446,"source_line":104,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":901,"record_offset":6416,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":8444,"source_line":123,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":901,"record_offset":5480,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":8434,"source_line":31,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":901,"record_offset":5516,"parent_record_offset":5480,"ancestor_record_offset":4024,"inlinee":8436,"source_line":139,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":901,"record_offset":5080,"parent_record_offset":5052,"ancestor_record_offset":4024,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":901,"record_offset":5052,"parent_record_offset":5024,"ancestor_record_offset":4024,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":901,"record_offset":4992,"parent_record_offset":4956,"ancestor_record_offset":4024,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":901,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":4024,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":901,"record_offset":5240,"parent_record_offset":4956,"ancestor_record_offset":4024,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":901,"record_offset":5024,"parent_record_offset":4992,"ancestor_record_offset":4024,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":901,"record_offset":4756,"parent_record_offset":4728,"ancestor_record_offset":4024,"inlinee":8429,"source_line":59,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":901,"record_offset":6300,"parent_record_offset":6268,"ancestor_record_offset":4024,"inlinee":8440,"source_line":194,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":901,"record_offset":8296,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":8450,"source_line":183,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":901,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":4024,"inlinee":6859,"source_line":219,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":901,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":4024,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":901,"record_offset":4472,"parent_record_offset":4444,"ancestor_record_offset":4024,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":901,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":4024,"inlinee":6848,"source_line":456,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":901,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":4024,"inlinee":6838,"source_line":147,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":4444,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":8424,"source_line":746,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":4812,"parent_record_offset":4756,"ancestor_record_offset":4024,"inlinee":8430,"source_line":568,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":6020,"parent_record_offset":4208,"ancestor_record_offset":4024,"inlinee":8437,"source_line":940,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":4868,"parent_record_offset":4840,"ancestor_record_offset":4024,"inlinee":8432,"source_line":569,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":4500,"parent_record_offset":4472,"ancestor_record_offset":4024,"inlinee":8427,"source_line":747,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":4024,"inlinee":8439,"source_line":942,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui_with_options
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x210E
// 首条位置: {"module":901,"inlinee":8462}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":901,"record_offset":5392,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":901,"record_offset":5364,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":901,"record_offset":7256,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":901,"record_offset":7312,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":901,"record_offset":6268,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":901,"record_offset":6332,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":901,"record_offset":4728,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":901,"record_offset":6536,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":901,"record_offset":5904,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":901,"record_offset":7056,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":8446,"source_line":104,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":901,"record_offset":6416,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":8444,"source_line":123,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":901,"record_offset":5480,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":8434,"source_line":31,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] tg_gpu_selector::gpu_selection::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/gpu_selection.rs
//     evidence={"module":937,"record_offset":8296,"parent_record_offset":4204,"ancestor_record_offset":4204,"inlinee":55845,"source_line":58,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_direct] tg_gpu_selector::vulkan_config::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":937,"record_offset":16432,"parent_record_offset":12332,"ancestor_record_offset":12332,"inlinee":55849,"source_line":51,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_direct] tg_gpu_selector::vulkan_config::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":937,"record_offset":14144,"parent_record_offset":12332,"ancestor_record_offset":12332,"inlinee":55847,"source_line":18,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::dialog::impl$2::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/dialog.rs
//     evidence={"module":943,"record_offset":8256,"parent_record_offset":4156,"ancestor_record_offset":4156,"inlinee":17078,"source_line":147,"archive":"_pdb/B/modules/00943.jsonl.gz"}
//   [pdb_inline_direct] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":901,"record_offset":8296,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":8450,"source_line":183,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":4444,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":8424,"source_line":746,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":6020,"parent_record_offset":4208,"ancestor_record_offset":4208,"inlinee":8437,"source_line":940,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":5644,"parent_record_offset":5620,"ancestor_record_offset":4208,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":4208,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=60
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":4208,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":5588,"parent_record_offset":5552,"ancestor_record_offset":4208,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":5620,"parent_record_offset":5588,"ancestor_record_offset":4208,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=60
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":901,"record_offset":6780,"parent_record_offset":6692,"ancestor_record_offset":4208,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":901,"record_offset":5552,"parent_record_offset":5516,"ancestor_record_offset":4208,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":901,"record_offset":8652,"parent_record_offset":8624,"ancestor_record_offset":4208,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":901,"record_offset":6720,"parent_record_offset":6692,"ancestor_record_offset":4208,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":901,"record_offset":6692,"parent_record_offset":6668,"ancestor_record_offset":4208,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":901,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":4208,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":901,"record_offset":6152,"parent_record_offset":6124,"ancestor_record_offset":4208,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":901,"record_offset":6124,"parent_record_offset":6092,"ancestor_record_offset":4208,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":901,"record_offset":4784,"parent_record_offset":4756,"ancestor_record_offset":4208,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":901,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":4208,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":901,"record_offset":6568,"parent_record_offset":6536,"ancestor_record_offset":4208,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":901,"record_offset":7540,"parent_record_offset":7500,"ancestor_record_offset":4208,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":901,"record_offset":7500,"parent_record_offset":7472,"ancestor_record_offset":4208,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":901,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":4208,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":901,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":4208,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":901,"record_offset":8684,"parent_record_offset":8652,"ancestor_record_offset":4208,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":937,"record_offset":16516,"parent_record_offset":16484,"ancestor_record_offset":12332,"inlinee":4549,"source_line":165,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":901,"record_offset":4684,"parent_record_offset":4528,"ancestor_record_offset":4208,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":901,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":4208,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":901,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":4208,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":901,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":4208,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":901,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":4208,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":901,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":4208,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":901,"record_offset":4572,"parent_record_offset":4528,"ancestor_record_offset":4208,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":901,"record_offset":6748,"parent_record_offset":6720,"ancestor_record_offset":4208,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":901,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":4208,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":937,"record_offset":16456,"parent_record_offset":16432,"ancestor_record_offset":12332,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":901,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":4208,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=139
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":901,"record_offset":4928,"parent_record_offset":4896,"ancestor_record_offset":4208,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":901,"record_offset":7280,"parent_record_offset":7256,"ancestor_record_offset":4208,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":901,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":4208,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":901,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":4208,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":901,"record_offset":7384,"parent_record_offset":7312,"ancestor_record_offset":4208,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":901,"record_offset":6188,"parent_record_offset":6152,"ancestor_record_offset":4208,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":901,"record_offset":7412,"parent_record_offset":7312,"ancestor_record_offset":4208,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":901,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":4208,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":901,"record_offset":7352,"parent_record_offset":7312,"ancestor_record_offset":4208,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":901,"record_offset":7436,"parent_record_offset":7412,"ancestor_record_offset":4208,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":901,"record_offset":5696,"parent_record_offset":5668,"ancestor_record_offset":4208,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5180,"parent_record_offset":4992,"ancestor_record_offset":4208,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5272,"parent_record_offset":5240,"ancestor_record_offset":4208,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5420,"parent_record_offset":5392,"ancestor_record_offset":4208,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5800,"parent_record_offset":5764,"ancestor_record_offset":4208,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5112,"parent_record_offset":5080,"ancestor_record_offset":4208,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5444,"parent_record_offset":5420,"ancestor_record_offset":4208,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":4208,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":4208,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":4208,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":4208,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":901,"record_offset":5856,"parent_record_offset":5764,"ancestor_record_offset":4208,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":901,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":4208,"inlinee":8442,"source_line":7,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":901,"record_offset":5516,"parent_record_offset":5480,"ancestor_record_offset":4208,"inlinee":8436,"source_line":139,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":937,"record_offset":16540,"parent_record_offset":16516,"ancestor_record_offset":12332,"inlinee":10696,"source_line":93,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":901,"record_offset":5080,"parent_record_offset":5052,"ancestor_record_offset":4208,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":901,"record_offset":5052,"parent_record_offset":5024,"ancestor_record_offset":4208,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":901,"record_offset":4992,"parent_record_offset":4956,"ancestor_record_offset":4208,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":901,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":4208,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":901,"record_offset":5240,"parent_record_offset":4956,"ancestor_record_offset":4208,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":901,"record_offset":5024,"parent_record_offset":4992,"ancestor_record_offset":4208,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::vulkan_config::impl$4::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-gpu-selector","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":937,"record_offset":16484,"parent_record_offset":16456,"ancestor_record_offset":12332,"inlinee":55851,"source_line":18,"archive":"_pdb/B/modules/00937.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":901,"record_offset":4756,"parent_record_offset":4728,"ancestor_record_offset":4208,"inlinee":8429,"source_line":59,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":901,"record_offset":6300,"parent_record_offset":6268,"ancestor_record_offset":4208,"inlinee":8440,"source_line":194,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":901,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":4208,"inlinee":6859,"source_line":219,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":901,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":4208,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":901,"record_offset":4472,"parent_record_offset":4444,"ancestor_record_offset":4208,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":901,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":4208,"inlinee":6848,"source_line":456,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":901,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":4208,"inlinee":6838,"source_line":147,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":4812,"parent_record_offset":4756,"ancestor_record_offset":4208,"inlinee":8430,"source_line":568,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":4868,"parent_record_offset":4840,"ancestor_record_offset":4208,"inlinee":8432,"source_line":569,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":4500,"parent_record_offset":4472,"ancestor_record_offset":4208,"inlinee":8427,"source_line":747,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":901,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":4208,"inlinee":8439,"source_line":942,"archive":"_pdb/B/modules/00901.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls>
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":37,"record_offset":10292,"rva":949360,"va":5369658480}
// 编译实例 RVAs: 0xe7c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5571410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e852035500","size":5,"soft_fail":false,"target_va":5375230368,"va":5369658953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e88d360000","size":5,"soft_fail":false,"target_va":5369673056,"va":5369659086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e82d310000","size":5,"soft_fail":false,"target_va":5369673568,"va":5369660974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ed1b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8c13f0000","size":5,"soft_fail":false,"target_va":5369680304,"va":5369663978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403df940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3111609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8b97a2f00","size":5,"soft_fail":false,"target_va":5372770624,"va":5369659010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5178478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e86e044f00","size":5,"soft_fail":false,"target_va":5374842928,"va":5369664445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dc880
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5189639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e807304f00","size":5,"soft_fail":false,"target_va":5374855296,"va":5369665652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e8100
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5241357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e80dfa4f00","size":5,"soft_fail":false,"target_va":5374902528,"va":5369661166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405eb1b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5249098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e84a185000","size":5,"soft_fail":false,"target_va":5374914992,"va":5369665889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8c80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5308518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e866005100","size":5,"soft_fail":false,"target_va":5374971008,"va":5369662485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fa590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5311863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8770d5100","size":5,"soft_fail":false,"target_va":5374977424,"va":5369665556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406075d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5368552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8e8ea5100","size":5,"soft_fail":false,"target_va":5375030736,"va":5369662179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060b030
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5383209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e829245200","size":5,"soft_fail":false,"target_va":5375045680,"va":5369662466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060ba00
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5386298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e83a305200","size":5,"soft_fail":false,"target_va":5375048192,"va":5369661889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060c4f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5388486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8c6385200","size":5,"soft_fail":false,"target_va":5375050992,"va":5369662501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5422627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e823be5200","size":5,"soft_fail":false,"target_va":5375081488,"va":5369658856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5420286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8feb45200","size":5,"soft_fail":false,"target_va":5375081504,"va":5369661213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14061f610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5466994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8726b5300","size":5,"soft_fail":false,"target_va":5375129104,"va":5369662105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14061f620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5467170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8226c5300","size":5,"soft_fail":false,"target_va":5375129120,"va":5369661945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42188125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e85dbd8302","size":5,"soft_fail":false,"target_va":5411847168,"va":5369659038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t41877650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e892007f02","size":5,"soft_fail":false,"target_va":5411540992,"va":5369663337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4694490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8daa14700","size":5,"soft_fail":false,"target_va":5374356528,"va":5369662033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t5541077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8d58c5400","size":5,"soft_fail":false,"target_va":5375203536,"va":5369662454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41878655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e87f047f02","size":5,"soft_fail":false,"target_va":5411541904,"va":5369663244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$crash_reporter_gui..Message$GT$$GT$::h7fba67ddaf714c57 (.llvm.12749940219289859111)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t47223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e877b80000","size":5,"soft_fail":false,"target_va":5369711104,"va":5369663876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e87cecffff","size":5,"soft_fail":false,"target_va":5369656784,"va":5369661775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_graphics","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t5653942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8b6455600","size":5,"soft_fail":false,"target_va":5375312688,"va":5369658741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t5562371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e803e05400","size":5,"soft_fail":false,"target_va":5375221168,"va":5369658792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     evidence={"assembly":"\tcallq\t5561245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e89ddb5400","size":5,"soft_fail":false,"target_va":5375220512,"va":5369659262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41875313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e871f77e02","size":5,"soft_fail":false,"target_va":5411538608,"va":5369663290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41875691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e8ebf87e02","size":5,"soft_fail":false,"target_va":5411538960,"va":5369663264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t4536942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e86e3a4500","size":5,"soft_fail":false,"target_va":5374199344,"va":5369662397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t4546708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e894604500","size":5,"soft_fail":false,"target_va":5374205872,"va":5369659159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t5986646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369658480,"raw_hex":"e856595b00","size":5,"soft_fail":false,"target_va":5375645312,"va":5369658661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*51030946(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5369658480,"raw_hex":"ff15a2ab0a03","size":6,"soft_fail":false,"va":5369658584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":37,"record_offset":11464,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":37,"record_offset":11436,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":37,"record_offset":13308,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":37,"record_offset":13364,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":37,"record_offset":12336,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":37,"record_offset":12400,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":37,"record_offset":10792,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":37,"record_offset":12604,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":37,"record_offset":11976,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/command.rs
//     evidence={"module":37,"record_offset":13108,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":8446,"source_line":104,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::queue_message
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":37,"record_offset":14348,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":8448,"source_line":69,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":37,"record_offset":12484,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":8444,"source_line":123,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":37,"record_offset":11552,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":8434,"source_line":31,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform/pump_events.rs
//     evidence={"module":37,"record_offset":14576,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":8450,"source_line":183,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":37,"record_offset":10508,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":8424,"source_line":746,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":37,"record_offset":12092,"parent_record_offset":10292,"ancestor_record_offset":10292,"inlinee":8437,"source_line":940,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":37,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":10292,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":37,"record_offset":11740,"parent_record_offset":11716,"ancestor_record_offset":10292,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":37,"record_offset":12856,"parent_record_offset":12832,"ancestor_record_offset":10292,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":37,"record_offset":11660,"parent_record_offset":11624,"ancestor_record_offset":10292,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":37,"record_offset":11692,"parent_record_offset":11660,"ancestor_record_offset":10292,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":37,"record_offset":12832,"parent_record_offset":12744,"ancestor_record_offset":10292,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":37,"record_offset":11624,"parent_record_offset":11588,"ancestor_record_offset":10292,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":37,"record_offset":14932,"parent_record_offset":14904,"ancestor_record_offset":10292,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":37,"record_offset":14452,"parent_record_offset":14420,"ancestor_record_offset":10292,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":37,"record_offset":12772,"parent_record_offset":12744,"ancestor_record_offset":10292,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":37,"record_offset":12744,"parent_record_offset":12720,"ancestor_record_offset":10292,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":37,"record_offset":14500,"parent_record_offset":14476,"ancestor_record_offset":10292,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":37,"record_offset":14476,"parent_record_offset":14452,"ancestor_record_offset":10292,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":37,"record_offset":12720,"parent_record_offset":12696,"ancestor_record_offset":10292,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":37,"record_offset":12224,"parent_record_offset":12196,"ancestor_record_offset":10292,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":37,"record_offset":12196,"parent_record_offset":12164,"ancestor_record_offset":10292,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":37,"record_offset":10856,"parent_record_offset":10828,"ancestor_record_offset":10292,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":37,"record_offset":11836,"parent_record_offset":11812,"ancestor_record_offset":10292,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":37,"record_offset":14420,"parent_record_offset":14376,"ancestor_record_offset":10292,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":37,"record_offset":14376,"parent_record_offset":14348,"ancestor_record_offset":10292,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":37,"record_offset":12632,"parent_record_offset":12604,"ancestor_record_offset":10292,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":37,"record_offset":13592,"parent_record_offset":13552,"ancestor_record_offset":10292,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":37,"record_offset":13552,"parent_record_offset":13524,"ancestor_record_offset":10292,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":37,"record_offset":13844,"parent_record_offset":13820,"ancestor_record_offset":10292,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":37,"record_offset":13664,"parent_record_offset":13640,"ancestor_record_offset":10292,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":37,"record_offset":14964,"parent_record_offset":14932,"ancestor_record_offset":10292,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":37,"record_offset":10748,"parent_record_offset":10592,"ancestor_record_offset":10292,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":37,"record_offset":10708,"parent_record_offset":10684,"ancestor_record_offset":10292,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":37,"record_offset":13640,"parent_record_offset":13616,"ancestor_record_offset":10292,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":37,"record_offset":13616,"parent_record_offset":13592,"ancestor_record_offset":10292,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":37,"record_offset":10660,"parent_record_offset":10636,"ancestor_record_offset":10292,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":37,"record_offset":14988,"parent_record_offset":14964,"ancestor_record_offset":10292,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":37,"record_offset":10636,"parent_record_offset":10592,"ancestor_record_offset":10292,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":37,"record_offset":12800,"parent_record_offset":12772,"ancestor_record_offset":10292,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":37,"record_offset":12424,"parent_record_offset":12400,"ancestor_record_offset":10292,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":37,"record_offset":13736,"parent_record_offset":13712,"ancestor_record_offset":10292,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":37,"record_offset":11000,"parent_record_offset":10968,"ancestor_record_offset":10292,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":37,"record_offset":13332,"parent_record_offset":13308,"ancestor_record_offset":10292,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":37,"record_offset":13712,"parent_record_offset":13688,"ancestor_record_offset":10292,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":37,"record_offset":10684,"parent_record_offset":10660,"ancestor_record_offset":10292,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":37,"record_offset":13436,"parent_record_offset":13364,"ancestor_record_offset":10292,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":37,"record_offset":12256,"parent_record_offset":12224,"ancestor_record_offset":10292,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":37,"record_offset":13464,"parent_record_offset":13364,"ancestor_record_offset":10292,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":37,"record_offset":13688,"parent_record_offset":13664,"ancestor_record_offset":10292,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":37,"record_offset":13404,"parent_record_offset":13364,"ancestor_record_offset":10292,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":37,"record_offset":13488,"parent_record_offset":13464,"ancestor_record_offset":10292,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":37,"record_offset":11768,"parent_record_offset":11740,"ancestor_record_offset":10292,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":37,"record_offset":14540,"parent_record_offset":14376,"ancestor_record_offset":10292,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11252,"parent_record_offset":11064,"ancestor_record_offset":10292,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11344,"parent_record_offset":11312,"ancestor_record_offset":10292,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11492,"parent_record_offset":11464,"ancestor_record_offset":10292,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11872,"parent_record_offset":11836,"ancestor_record_offset":10292,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11184,"parent_record_offset":11152,"ancestor_record_offset":10292,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11516,"parent_record_offset":11492,"ancestor_record_offset":10292,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":10292,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":10292,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11896,"parent_record_offset":11872,"ancestor_record_offset":10292,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11368,"parent_record_offset":11344,"ancestor_record_offset":10292,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11928,"parent_record_offset":11836,"ancestor_record_offset":10292,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/cursor.rs
//     evidence={"module":37,"record_offset":12448,"parent_record_offset":12424,"ancestor_record_offset":10292,"inlinee":8442,"source_line":7,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"iced_tiny_skia","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/tiny_skia/src/window/compositor.rs
//     evidence={"module":37,"record_offset":11588,"parent_record_offset":11552,"ancestor_record_offset":10292,"inlinee":8436,"source_line":139,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":37,"record_offset":11152,"parent_record_offset":11124,"ancestor_record_offset":10292,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":37,"record_offset":11124,"parent_record_offset":11096,"ancestor_record_offset":10292,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":37,"record_offset":11064,"parent_record_offset":11028,"ancestor_record_offset":10292,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":37,"record_offset":11028,"parent_record_offset":11000,"ancestor_record_offset":10292,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":37,"record_offset":11312,"parent_record_offset":11028,"ancestor_record_offset":10292,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":37,"record_offset":11096,"parent_record_offset":11064,"ancestor_record_offset":10292,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":37,"record_offset":10828,"parent_record_offset":10792,"ancestor_record_offset":10292,"inlinee":8429,"source_line":59,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":37,"record_offset":12368,"parent_record_offset":12336,"ancestor_record_offset":10292,"inlinee":8440,"source_line":194,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":37,"record_offset":10592,"parent_record_offset":10564,"ancestor_record_offset":10292,"inlinee":6859,"source_line":219,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":37,"record_offset":12116,"parent_record_offset":12092,"ancestor_record_offset":10292,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":37,"record_offset":10536,"parent_record_offset":10508,"ancestor_record_offset":10292,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":37,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":10292,"inlinee":6848,"source_line":456,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":37,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":10292,"inlinee":6838,"source_line":147,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":37,"record_offset":10884,"parent_record_offset":10828,"ancestor_record_offset":10292,"inlinee":8430,"source_line":568,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":37,"record_offset":10940,"parent_record_offset":10912,"ancestor_record_offset":10292,"inlinee":8432,"source_line":569,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":37,"record_offset":10564,"parent_record_offset":10536,"ancestor_record_offset":10292,"inlinee":8427,"source_line":747,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":37,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":10292,"inlinee":8439,"source_line":942,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui_with_options::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20ED
// 首条位置: {"module":37,"inlinee":8429}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":37,"record_offset":10856,"parent_record_offset":10828,"ancestor_record_offset":10828,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":37,"record_offset":10884,"parent_record_offset":10828,"ancestor_record_offset":10828,"inlinee":8430,"source_line":568,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":37,"record_offset":11000,"parent_record_offset":10968,"ancestor_record_offset":10828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11252,"parent_record_offset":11064,"ancestor_record_offset":10828,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11344,"parent_record_offset":11312,"ancestor_record_offset":10828,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11184,"parent_record_offset":11152,"ancestor_record_offset":10828,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":10828,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":10828,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":37,"record_offset":11368,"parent_record_offset":11344,"ancestor_record_offset":10828,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":37,"record_offset":11152,"parent_record_offset":11124,"ancestor_record_offset":10828,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":37,"record_offset":11124,"parent_record_offset":11096,"ancestor_record_offset":10828,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":37,"record_offset":11064,"parent_record_offset":11028,"ancestor_record_offset":10828,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":37,"record_offset":11028,"parent_record_offset":11000,"ancestor_record_offset":10828,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":37,"record_offset":11312,"parent_record_offset":11028,"ancestor_record_offset":10828,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":37,"record_offset":11096,"parent_record_offset":11064,"ancestor_record_offset":10828,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":37,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":10828,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":37,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":10828,"inlinee":6848,"source_line":456,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":37,"record_offset":10940,"parent_record_offset":10912,"ancestor_record_offset":10828,"inlinee":8432,"source_line":569,"archive":"_pdb/B/modules/00037.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui_with_options::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3C99
// 首条位置: {"module":234,"inlinee":15513}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":13524,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":234,"record_offset":13468,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":234,"record_offset":12680,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":234,"record_offset":13432,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::debug::Debug::overlay
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/debug/null.rs
//     evidence={"module":234,"record_offset":13376,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":15522,"source_line":44,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_runtime","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/runtime/src/program/state.rs
//     evidence={"module":234,"record_offset":13144,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":8485,"source_line":62,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] iced_winit::conversion::mouse_interaction
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_winit","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/winit/src/conversion.rs
//     evidence={"module":234,"record_offset":13552,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":15526,"source_line":393,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::exit
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/event_loop.rs
//     evidence={"module":234,"record_offset":12944,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":15519,"source_line":363,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":234,"record_offset":12388,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":8424,"source_line":746,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::request_redraw
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":234,"record_offset":11208,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":15514,"source_line":599,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_cursor_icon
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":234,"record_offset":13580,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":15527,"source_line":1352,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":234,"record_offset":13252,"parent_record_offset":13220,"ancestor_record_offset":10976,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":234,"record_offset":13300,"parent_record_offset":13276,"ancestor_record_offset":10976,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":234,"record_offset":13276,"parent_record_offset":13252,"ancestor_record_offset":10976,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":11376,"parent_record_offset":11352,"ancestor_record_offset":10976,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":11352,"parent_record_offset":11312,"ancestor_record_offset":10976,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":234,"record_offset":13220,"parent_record_offset":13172,"ancestor_record_offset":10976,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":234,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":10976,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":234,"record_offset":13172,"parent_record_offset":13144,"ancestor_record_offset":10976,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":234,"record_offset":13044,"parent_record_offset":13020,"ancestor_record_offset":10976,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":234,"record_offset":13020,"parent_record_offset":12996,"ancestor_record_offset":10976,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":234,"record_offset":12636,"parent_record_offset":12484,"ancestor_record_offset":10976,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":234,"record_offset":12596,"parent_record_offset":12572,"ancestor_record_offset":10976,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":234,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":10976,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":234,"record_offset":13068,"parent_record_offset":13044,"ancestor_record_offset":10976,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":234,"record_offset":12524,"parent_record_offset":12484,"ancestor_record_offset":10976,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":11932,"parent_record_offset":11312,"ancestor_record_offset":10976,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":234,"record_offset":12572,"parent_record_offset":12548,"ancestor_record_offset":10976,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":234,"record_offset":11400,"parent_record_offset":11376,"ancestor_record_offset":10976,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":13092,"parent_record_offset":13068,"ancestor_record_offset":10976,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":234,"record_offset":11892,"parent_record_offset":11312,"ancestor_record_offset":10976,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":11812,"parent_record_offset":11784,"ancestor_record_offset":10976,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":12184,"parent_record_offset":11996,"ancestor_record_offset":10976,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":11504,"parent_record_offset":11472,"ancestor_record_offset":10976,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":12276,"parent_record_offset":12244,"ancestor_record_offset":10976,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":11716,"parent_record_offset":11684,"ancestor_record_offset":10976,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":11528,"parent_record_offset":11504,"ancestor_record_offset":10976,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":11740,"parent_record_offset":11716,"ancestor_record_offset":10976,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":12208,"parent_record_offset":12184,"ancestor_record_offset":10976,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":12300,"parent_record_offset":12276,"ancestor_record_offset":10976,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::interaction::impl$3::eq
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_core","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/core/src/mouse/interaction.rs
//     evidence={"module":234,"record_offset":13492,"parent_record_offset":13468,"ancestor_record_offset":10976,"inlinee":15524,"source_line":2,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":234,"record_offset":12752,"parent_record_offset":12720,"ancestor_record_offset":10976,"inlinee":8444,"source_line":123,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":234,"record_offset":11684,"parent_record_offset":11656,"ancestor_record_offset":10976,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":234,"record_offset":11656,"parent_record_offset":11628,"ancestor_record_offset":10976,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":234,"record_offset":11996,"parent_record_offset":11960,"ancestor_record_offset":10976,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":234,"record_offset":11784,"parent_record_offset":11592,"ancestor_record_offset":10976,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":234,"record_offset":11592,"parent_record_offset":11564,"ancestor_record_offset":10976,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":234,"record_offset":11436,"parent_record_offset":11312,"ancestor_record_offset":10976,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":234,"record_offset":11564,"parent_record_offset":11436,"ancestor_record_offset":10976,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":234,"record_offset":11960,"parent_record_offset":11932,"ancestor_record_offset":10976,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":234,"record_offset":11472,"parent_record_offset":11436,"ancestor_record_offset":10976,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":234,"record_offset":12244,"parent_record_offset":11960,"ancestor_record_offset":10976,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":234,"record_offset":11628,"parent_record_offset":11592,"ancestor_record_offset":10976,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"tg-iced","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":234,"record_offset":12720,"parent_record_offset":12680,"ancestor_record_offset":10976,"inlinee":15518,"source_line":131,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::exit
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop.rs
//     evidence={"module":234,"record_offset":12972,"parent_record_offset":12944,"ancestor_record_offset":10976,"inlinee":15520,"source_line":525,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::runner::EventLoopRunner::set_exit_code
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/event_loop/runner.rs
//     evidence={"module":234,"record_offset":12996,"parent_record_offset":12972,"ancestor_record_offset":10976,"inlinee":15251,"source_line":158,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":234,"record_offset":12344,"parent_record_offset":11312,"ancestor_record_offset":10976,"inlinee":6012,"source_line":348,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":234,"record_offset":12484,"parent_record_offset":12452,"ancestor_record_offset":10976,"inlinee":6859,"source_line":219,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":234,"record_offset":11244,"parent_record_offset":11208,"ancestor_record_offset":10976,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":234,"record_offset":12420,"parent_record_offset":12388,"ancestor_record_offset":10976,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":234,"record_offset":11312,"parent_record_offset":11280,"ancestor_record_offset":10976,"inlinee":6005,"source_line":164,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_cursor_icon
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":234,"record_offset":13664,"parent_record_offset":13636,"ancestor_record_offset":10976,"inlinee":15530,"source_line":401,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::request_redraw::closure$0
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":234,"record_offset":11280,"parent_record_offset":11244,"ancestor_record_offset":10976,"inlinee":15516,"source_line":600,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":234,"record_offset":12452,"parent_record_offset":12420,"ancestor_record_offset":10976,"inlinee":8427,"source_line":747,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$13::set_cursor_icon::closure$0
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":234,"record_offset":13636,"parent_record_offset":13608,"ancestor_record_offset":10976,"inlinee":15529,"source_line":1354,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui_with_options::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3C9E
// 首条位置: {"module":234,"inlinee":15518}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"iced_style","version":"0.12.1","source":"git+https://github.com/PounceLight/iced.git?rev=2bccf7317e42422cb3b98c0de4f291d8c0865380#2bccf7317e42422cb3b98c0de4f291d8c0865380"}]
//     definition_source=/home/h3/.cargo/git/checkouts/iced-05cc82929dc27190/2bccf73/style/src/theme.rs
//     evidence={"module":234,"record_offset":12752,"parent_record_offset":12720,"ancestor_record_offset":12720,"inlinee":8444,"source_line":123,"archive":"_pdb/B/modules/00234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui_with_options::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20F8
// 首条位置: {"module":37,"inlinee":8440}
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
// 完整结构及原始类型记录: gui_runner.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_iced::gui_runner::ProgramWithOutcome
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gui_runner.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gui_runner.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::impl$0::default
// 信息 ID: 85e497db2cb4eea2eb324a1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>; module=901, inline_record=4184; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=937, inline_record=4180; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=937, inline_record=12304; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>; module=943, inline_record=4132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>
// 信息 ID: 28b8742d89056d7a10821aa6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner::run_gui -> ["tg_gpu_selector::gpu_selection::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_gpu_selector::run_gpu_selection_gui @ 0x630ee8 -> 0xdefad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>
// 信息 ID: c7c40fd0703498c86aad7941
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner::run_gui -> ["tg_gpu_selector::vulkan_config::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_gpu_selector::run_vulkan_configuration_gui @ 0x630fab -> 0xdf16d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>
// 信息 ID: 9662c0e6be753e631d8b8d2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner::run_gui -> ["tg_iced::dialog::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_iced::dialog::MessageDialog::show @ 0x632846 -> 0xe00a40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>
// 信息 ID: 9f279f051a8911ca33c9c4ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner::run_gui -> ["tg_integrity_report::controls::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_integrity_report::run_integrity_report_gui @ 0x62e917 -> 0xdcbee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options
// 信息 ID: 39f4cce3134d871875ef6b4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>; module=901, inline_record=4208; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=937, inline_record=4204; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=937, inline_record=12332; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>; module=943, inline_record=4156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options::closure$0
// 信息 ID: 7af24f802ad436e62561a011
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_iced::gui_runner::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls>; module=37, inline_record=10828; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>; module=901, inline_record=4756; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=937, inline_record=4756; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=937, inline_record=12880; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>; module=943, inline_record=4716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options::closure$1
// 信息 ID: fc58f3d1744a31ef562294c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_iced::gui_runner::run_gui_with_options", "captures": "unknown"}
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::vulkan_config::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 15 条。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; module=234, inline_record=10976; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >; module=2333, inline_record=5508; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >; module=3428, inline_record=2808; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >; module=3505, inline_record=4628; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::vulkan_config::Controls> >; module=3505, inline_record=6976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options::closure$1::closure$0
// 信息 ID: 8f1664e766687d7820471484
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_iced::gui_runner::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; module=234, inline_record=12720; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >; module=2333, inline_record=7252; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >; module=3428, inline_record=4552; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >; module=3505, inline_record=5220; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::vulkan_config::Controls> >; module=3505, inline_record=7568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options::closure$2
// 信息 ID: 65d28e15ae065f7f998032ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_iced::gui_runner::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls>; module=37, inline_record=12368; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>; module=901, inline_record=6300; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=937, inline_record=6300; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=937, inline_record=14452; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>; module=943, inline_record=6260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls>
// 信息 ID: 76a14dd9ecb832db11e2e77d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner::run_gui_with_options -> ["crash_reporter_gui::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_reporter_gui::run_crash_reporter_gui<tiny_glade::mock_crash_reporter::closure_env$1> @ 0x338118 -> 0xe7c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6> @ 0x339613 -> 0xe7c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

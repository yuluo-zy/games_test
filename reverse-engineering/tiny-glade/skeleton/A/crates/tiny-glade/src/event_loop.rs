// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/event_loop.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7a56e7c5d3f90f810364d15a5cb67bae89c093d6c82987a2bca0b58681a5e2a9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: event_loop.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 127, 140, 203, 256
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"PDB_registry_path_only","version":"0.27.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use winit; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.4.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::run_event_loop
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":203,"record_offset":8336,"rva":3205376,"va":5371914496}
// 编译实例 RVAs: 0x30e900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t1349490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e872971400","size":5,"soft_fail":false,"target_va":5373265456,"va":5371915961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2243796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e82cc3ddff","size":5,"soft_fail":false,"target_va":5369673744,"va":5371917535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140107320
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2129962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8d67fdfff","size":5,"soft_fail":false,"target_va":5369787168,"va":5371917125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c9460
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-285169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e80fa6fbff","size":5,"soft_fail":false,"target_va":5371630688,"va":5371915852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035fc30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t328453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e805030500","size":5,"soft_fail":false,"target_va":5372247088,"va":5371918630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t330875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e87b0c0500","size":5,"soft_fail":false,"target_va":5372249392,"va":5371918512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=3
//     target_aliases=89 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t399061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8d5160600","size":5,"soft_fail":false,"target_va":5372314208,"va":5371915142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t399797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8b5190600","size":5,"soft_fail":false,"target_va":5372316880,"va":5371917078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037aa80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t439580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e81cb50600","size":5,"soft_fail":false,"target_va":5372357248,"va":5371917663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037b500
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t442949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e845c20600","size":5,"soft_fail":false,"target_va":5372359936,"va":5371916982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140383620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t475968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e840430700","size":5,"soft_fail":false,"target_va":5372392992,"va":5371917019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038fec0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t527389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e81d0c0800","size":5,"soft_fail":false,"target_va":5372444352,"va":5371916958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140391020
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t531825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8711d0800","size":5,"soft_fail":false,"target_va":5372448800,"va":5371916970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd090
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t712689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8f1df0a00","size":5,"soft_fail":false,"target_va":5372629136,"va":5371916442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t832761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8f9b40c00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371916946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a67c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31028975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8ef76d901","size":5,"soft_fail":false,"target_va":5402945472,"va":5371916492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-265060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e89cf4fbff","size":5,"soft_fail":false,"target_va":5371651952,"va":5371917007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5447605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8b51f5300","size":5,"soft_fail":false,"target_va":5377364336,"va":5371916726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31062924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e88cfbd901","size":5,"soft_fail":false,"target_va":5402980128,"va":5371917199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui_winit::State::handle_platform_output
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1209745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e891751200","size":5,"soft_fail":false,"target_va":5373125376,"va":5371915626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5437057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e881f65200","size":5,"soft_fail":false,"target_va":5377353776,"va":5371916714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31059876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8a4efd901","size":5,"soft_fail":false,"target_va":5402975328,"va":5371915447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2132761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8e774dfff","size":5,"soft_fail":false,"target_va":5369782224,"va":5371914980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t75298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e822260100","size":5,"soft_fail":false,"target_va":5371990272,"va":5371914969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t651527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e807f10900","size":5,"soft_fail":false,"target_va":5372566528,"va":5371914996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::os::errno
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5562617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8f9e05400","size":5,"soft_fail":false,"target_va":5377478800,"va":5371916178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31060082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e872f0d901","size":5,"soft_fail":false,"target_va":5402975504,"va":5371915417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31060022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e836f0d901","size":5,"soft_fail":false,"target_va":5402975696,"va":5371915669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5476388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e824905300","size":5,"soft_fail":false,"target_va":5377393200,"va":5371916807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::icc::get_monitor_icc_profile
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-108509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e82358feff","size":5,"soft_fail":false,"target_va":5371808048,"va":5371916552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t277630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e87e3c0400","size":5,"soft_fail":false,"target_va":5372192528,"va":5371914893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t288021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e815650400","size":5,"soft_fail":false,"target_va":5372203328,"va":5371915302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::current_monitor
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1351183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e80f9e1400","size":5,"soft_fail":false,"target_va":5373265824,"va":5371914636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::is_minimized
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1325997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e8ad3b1400","size":5,"soft_fail":false,"target_va":5373241248,"va":5371915246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1337169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e851671400","size":5,"soft_fail":false,"target_va":5373253168,"va":5371915994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::Window::set_cursor_grab
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e824f2ffff","size":5,"soft_fail":false,"target_va":5371911568,"va":5371915111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::window::Window::set_cursor_visible
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371914496,"raw_hex":"e890f0ffff","size":5,"soft_fail":false,"target_va":5371912336,"va":5371916283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*39369390(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5371914496,"raw_hex":"ff15aeba5802","size":6,"soft_fail":false,"va":5371916140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":203,"record_offset":12248,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":12817,"source_line":1412,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":203,"record_offset":9004,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":10291,"source_line":1685,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":203,"record_offset":12364,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":12821,"source_line":241,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$2::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":203,"record_offset":8656,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":12811,"source_line":151,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":203,"record_offset":8808,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":203,"record_offset":12948,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":203,"record_offset":12976,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":203,"record_offset":12836,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":203,"record_offset":8920,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":203,"record_offset":8836,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":203,"record_offset":12652,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":203,"record_offset":10216,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":203,"record_offset":13004,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":203,"record_offset":12864,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":203,"record_offset":8740,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":7942,"source_line":789,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":203,"record_offset":12624,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":5570,"source_line":612,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":203,"record_offset":9760,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":203,"record_offset":10256,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform/pump_events.rs
//     evidence={"module":203,"record_offset":8712,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":12641,"source_line":183,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":203,"record_offset":8544,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":10654,"source_line":1480,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":203,"record_offset":10840,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":11152,"source_line":1369,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_minimized
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":203,"record_offset":9344,"parent_record_offset":8336,"ancestor_record_offset":8336,"inlinee":12813,"source_line":1017,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":203,"record_offset":13468,"parent_record_offset":13444,"ancestor_record_offset":8336,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":203,"record_offset":13444,"parent_record_offset":13388,"ancestor_record_offset":8336,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":203,"record_offset":12524,"parent_record_offset":12364,"ancestor_record_offset":8336,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":203,"record_offset":9248,"parent_record_offset":9004,"ancestor_record_offset":8336,"inlinee":6132,"source_line":171,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":203,"record_offset":9076,"parent_record_offset":9044,"ancestor_record_offset":8336,"inlinee":7711,"source_line":751,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":203,"record_offset":9044,"parent_record_offset":9004,"ancestor_record_offset":8336,"inlinee":7710,"source_line":188,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":203,"record_offset":8680,"parent_record_offset":8656,"ancestor_record_offset":8336,"inlinee":12812,"source_line":542,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":203,"record_offset":9272,"parent_record_offset":9248,"ancestor_record_offset":8336,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":203,"record_offset":9152,"parent_record_offset":9004,"ancestor_record_offset":8336,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":203,"record_offset":9176,"parent_record_offset":9152,"ancestor_record_offset":8336,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::Drain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":203,"record_offset":12276,"parent_record_offset":12248,"ancestor_record_offset":8336,"inlinee":12819,"source_line":38,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":203,"record_offset":12732,"parent_record_offset":12708,"ancestor_record_offset":8336,"inlinee":12823,"source_line":95,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\drain.rs
//     evidence={"module":203,"record_offset":12780,"parent_record_offset":12756,"ancestor_record_offset":8336,"inlinee":12825,"source_line":116,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":203,"record_offset":9108,"parent_record_offset":9076,"ancestor_record_offset":8336,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":203,"record_offset":13416,"parent_record_offset":13388,"ancestor_record_offset":8336,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":203,"record_offset":9304,"parent_record_offset":9272,"ancestor_record_offset":8336,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":203,"record_offset":13388,"parent_record_offset":13360,"ancestor_record_offset":8336,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":203,"record_offset":13948,"parent_record_offset":13912,"ancestor_record_offset":8336,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":203,"record_offset":9476,"parent_record_offset":9436,"ancestor_record_offset":8336,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":203,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":8336,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":203,"record_offset":10280,"parent_record_offset":10256,"ancestor_record_offset":8336,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":203,"record_offset":10872,"parent_record_offset":10840,"ancestor_record_offset":8336,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$77::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":203,"record_offset":11628,"parent_record_offset":11604,"ancestor_record_offset":8336,"inlinee":8217,"source_line":76,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":203,"record_offset":8944,"parent_record_offset":8920,"ancestor_record_offset":8336,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":203,"record_offset":13784,"parent_record_offset":13752,"ancestor_record_offset":8336,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":203,"record_offset":9200,"parent_record_offset":9176,"ancestor_record_offset":8336,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":203,"record_offset":8860,"parent_record_offset":8836,"ancestor_record_offset":8336,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":203,"record_offset":13980,"parent_record_offset":13948,"ancestor_record_offset":8336,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":203,"record_offset":8968,"parent_record_offset":8944,"ancestor_record_offset":8336,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":203,"record_offset":9876,"parent_record_offset":9816,"ancestor_record_offset":8336,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":10136,"parent_record_offset":10108,"ancestor_record_offset":8336,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":10572,"parent_record_offset":10396,"ancestor_record_offset":8336,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":9816,"parent_record_offset":9788,"ancestor_record_offset":8336,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":10660,"parent_record_offset":10632,"ancestor_record_offset":8336,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":9504,"parent_record_offset":9476,"ancestor_record_offset":8336,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":9624,"parent_record_offset":9588,"ancestor_record_offset":8336,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":10040,"parent_record_offset":10012,"ancestor_record_offset":8336,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":8336,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":9848,"parent_record_offset":9816,"ancestor_record_offset":8336,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":10064,"parent_record_offset":10040,"ancestor_record_offset":8336,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":10596,"parent_record_offset":10572,"ancestor_record_offset":8336,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":8336,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":10684,"parent_record_offset":10660,"ancestor_record_offset":8336,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":9680,"parent_record_offset":9588,"ancestor_record_offset":8336,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::from_raw_os_error
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\error.rs
//     evidence={"module":203,"record_offset":12060,"parent_record_offset":12036,"ancestor_record_offset":8336,"inlinee":9664,"source_line":634,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::last_os_error
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\error.rs
//     evidence={"module":203,"record_offset":12036,"parent_record_offset":11072,"ancestor_record_offset":8336,"inlinee":9663,"source_line":602,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::new_os
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":203,"record_offset":12084,"parent_record_offset":12060,"ancestor_record_offset":8336,"inlinee":9666,"source_line":174,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":203,"record_offset":10012,"parent_record_offset":9988,"ancestor_record_offset":8336,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":203,"record_offset":9988,"parent_record_offset":9964,"ancestor_record_offset":8336,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":203,"record_offset":8764,"parent_record_offset":8740,"ancestor_record_offset":8336,"inlinee":7892,"source_line":287,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":203,"record_offset":9912,"parent_record_offset":9760,"ancestor_record_offset":8336,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":203,"record_offset":10364,"parent_record_offset":10340,"ancestor_record_offset":8336,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":203,"record_offset":10396,"parent_record_offset":10364,"ancestor_record_offset":8336,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":203,"record_offset":10108,"parent_record_offset":9936,"ancestor_record_offset":8336,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":203,"record_offset":9936,"parent_record_offset":9912,"ancestor_record_offset":8336,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":203,"record_offset":9788,"parent_record_offset":9760,"ancestor_record_offset":8336,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":203,"record_offset":10632,"parent_record_offset":10364,"ancestor_record_offset":8336,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":203,"record_offset":9964,"parent_record_offset":9936,"ancestor_record_offset":8336,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":203,"record_offset":10928,"parent_record_offset":10900,"ancestor_record_offset":8336,"inlinee":8224,"source_line":281,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":203,"record_offset":10956,"parent_record_offset":10928,"ancestor_record_offset":8336,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Position::to_physical
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":203,"record_offset":11528,"parent_record_offset":11072,"ancestor_record_offset":8336,"inlinee":10545,"source_line":563,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$17::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":203,"record_offset":11780,"parent_record_offset":11756,"ancestor_record_offset":8336,"inlinee":10549,"source_line":296,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":203,"record_offset":10900,"parent_record_offset":10872,"ancestor_record_offset":8336,"inlinee":8222,"source_line":573,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":203,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":8336,"inlinee":10657,"source_line":872,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":203,"record_offset":12008,"parent_record_offset":11072,"ancestor_record_offset":8336,"inlinee":8244,"source_line":344,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":203,"record_offset":9376,"parent_record_offset":9344,"ancestor_record_offset":8336,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":203,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":8336,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":203,"record_offset":11116,"parent_record_offset":11072,"ancestor_record_offset":8336,"inlinee":8250,"source_line":452,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":203,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":8336,"inlinee":11155,"source_line":457,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_minimized
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":203,"record_offset":9436,"parent_record_offset":9408,"ancestor_record_offset":8336,"inlinee":12816,"source_line":634,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_minimized::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":203,"record_offset":9408,"parent_record_offset":9376,"ancestor_record_offset":8336,"inlinee":12815,"source_line":1019,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$13::set_cursor_position::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":203,"record_offset":11048,"parent_record_offset":11024,"ancestor_record_offset":8336,"inlinee":11154,"source_line":1372,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":203,"record_offset":8592,"parent_record_offset":8568,"ancestor_record_offset":8336,"inlinee":10656,"source_line":1482,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x35EC
// 首条位置: {"module":256,"inlinee":13804}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":51172,"parent_record_offset":51004,"ancestor_record_offset":51004,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_direct] egui::context::impl$37::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":256,"record_offset":51044,"parent_record_offset":51004,"ancestor_record_offset":51004,"inlinee":6618,"source_line":678,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":256,"record_offset":51072,"parent_record_offset":51044,"ancestor_record_offset":51004,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":51100,"parent_record_offset":51072,"ancestor_record_offset":51004,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":256,"record_offset":51128,"parent_record_offset":51100,"ancestor_record_offset":51004,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00256.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::impl$5::partial_cmp
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x299C
// 首条位置: {"module":127,"inlinee":10652}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$54::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":65856,"parent_record_offset":65832,"ancestor_record_offset":65832,"inlinee":10653,"source_line":1549,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$55::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":127,"record_offset":65880,"parent_record_offset":65856,"ancestor_record_offset":65832,"inlinee":7631,"source_line":1565,"archive":"_pdb/A/modules/00127.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::EguiEventIo::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":203,"record_offset":4336,"rva":3203984,"va":5371913104}
// 编译实例 RVAs: 0x30e390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140396ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t558526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913104,"raw_hex":"e8be850800","size":5,"soft_fail":false,"target_va":5372472000,"va":5371913469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t836441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913104,"raw_hex":"e859c30c00","size":5,"soft_fail":false,"target_va":5372749696,"va":5371913250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31680180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913104,"raw_hex":"e8b466e301","size":5,"soft_fail":false,"target_va":5403593360,"va":5371913175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31064803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913104,"raw_hex":"e8e302da01","size":5,"soft_fail":false,"target_va":5402978144,"va":5371913336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-260959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913104,"raw_hex":"e8a104fcff","size":5,"soft_fail":false,"target_va":5371652432,"va":5371913386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":203,"record_offset":4496,"parent_record_offset":4336,"ancestor_record_offset":4336,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":203,"record_offset":4724,"parent_record_offset":4336,"ancestor_record_offset":4336,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":203,"record_offset":4468,"parent_record_offset":4336,"ancestor_record_offset":4336,"inlinee":10490,"source_line":273,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":203,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":4336,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":203,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":4336,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":203,"record_offset":4564,"parent_record_offset":4528,"ancestor_record_offset":4336,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":203,"record_offset":4596,"parent_record_offset":4564,"ancestor_record_offset":4336,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":203,"record_offset":4528,"parent_record_offset":4496,"ancestor_record_offset":4336,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":203,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":4336,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":203,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":4336,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":203,"record_offset":4672,"parent_record_offset":4644,"ancestor_record_offset":4336,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":4808,"parent_record_offset":4772,"ancestor_record_offset":4336,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":4336,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":4864,"parent_record_offset":4772,"ancestor_record_offset":4336,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::EguiEventIo::take_egui_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":203,"record_offset":5060,"rva":3204368,"va":5371913488}
// 编译实例 RVAs: 0x30e510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2240723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e82dcfddff","size":5,"soft_fail":false,"target_va":5369673744,"va":5371914462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: event_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t334976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e8801c0500","size":5,"soft_fail":false,"target_va":5372249392,"va":5371914411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31064419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e86301da01","size":5,"soft_fail":false,"target_va":5402978688,"va":5371914264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e870f5ffff","size":5,"soft_fail":false,"target_va":5371911376,"va":5371914075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31066293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e8b508da01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371913830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31066744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e8780ada01","size":5,"soft_fail":false,"target_va":5402980656,"va":5371913907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui_winit::State::take_egui_input
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1200781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e88d521200","size":5,"soft_fail":false,"target_va":5373114848,"va":5371914062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4442885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e805cb4300","size":5,"soft_fail":false,"target_va":5376356912,"va":5371914022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4446794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e84ada4300","size":5,"soft_fail":false,"target_va":5376360560,"va":5371913761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4442548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e8b4c94300","size":5,"soft_fail":false,"target_va":5376356144,"va":5371913591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31061179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e8bbf4d901","size":5,"soft_fail":false,"target_va":5402975328,"va":5371914144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31061365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e875f5d901","size":5,"soft_fail":false,"target_va":5402975504,"va":5371914134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31061464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e8d8f5d901","size":5,"soft_fail":false,"target_va":5402975696,"va":5371914227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3d9350ee38791ca5 (.llvm.8305701337189838128)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t711382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371913488,"raw_hex":"e8d6da0a00","size":5,"soft_fail":false,"target_va":5372625168,"va":5371913781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":203,"record_offset":5220,"parent_record_offset":5060,"ancestor_record_offset":5060,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":203,"record_offset":7920,"parent_record_offset":5060,"ancestor_record_offset":5060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":203,"record_offset":5740,"parent_record_offset":5060,"ancestor_record_offset":5060,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":203,"record_offset":6800,"parent_record_offset":5060,"ancestor_record_offset":5060,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":203,"record_offset":5784,"parent_record_offset":5060,"ancestor_record_offset":5060,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":203,"record_offset":5248,"parent_record_offset":5060,"ancestor_record_offset":5060,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":203,"record_offset":7836,"parent_record_offset":5060,"ancestor_record_offset":5060,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":203,"record_offset":7188,"parent_record_offset":7160,"ancestor_record_offset":5060,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":203,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":5060,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":203,"record_offset":7216,"parent_record_offset":7188,"ancestor_record_offset":5060,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":203,"record_offset":7128,"parent_record_offset":7100,"ancestor_record_offset":5060,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":203,"record_offset":7160,"parent_record_offset":7128,"ancestor_record_offset":5060,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":203,"record_offset":7332,"parent_record_offset":7160,"ancestor_record_offset":5060,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":203,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":5060,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":203,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":5060,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":203,"record_offset":6664,"parent_record_offset":6572,"ancestor_record_offset":5060,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":203,"record_offset":7024,"parent_record_offset":6996,"ancestor_record_offset":5060,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":203,"record_offset":6380,"parent_record_offset":5920,"ancestor_record_offset":5060,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":203,"record_offset":7072,"parent_record_offset":6896,"ancestor_record_offset":5060,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":203,"record_offset":6748,"parent_record_offset":6720,"ancestor_record_offset":5060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":203,"record_offset":7396,"parent_record_offset":7100,"ancestor_record_offset":5060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":203,"record_offset":6156,"parent_record_offset":6132,"ancestor_record_offset":5060,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":203,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":5060,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":203,"record_offset":6340,"parent_record_offset":6308,"ancestor_record_offset":5060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":203,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":5060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":203,"record_offset":7796,"parent_record_offset":6864,"ancestor_record_offset":5060,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":203,"record_offset":5376,"parent_record_offset":5316,"ancestor_record_offset":5060,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":203,"record_offset":6608,"parent_record_offset":6572,"ancestor_record_offset":5060,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":203,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":5060,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":203,"record_offset":6108,"parent_record_offset":6084,"ancestor_record_offset":5060,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":203,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":5060,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":203,"record_offset":6084,"parent_record_offset":5976,"ancestor_record_offset":5060,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":203,"record_offset":6572,"parent_record_offset":6540,"ancestor_record_offset":5060,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":203,"record_offset":5816,"parent_record_offset":5784,"ancestor_record_offset":5060,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":203,"record_offset":5948,"parent_record_offset":5920,"ancestor_record_offset":5060,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":203,"record_offset":5920,"parent_record_offset":5888,"ancestor_record_offset":5060,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":203,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":5060,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":203,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":5060,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":203,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":5060,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":203,"record_offset":6412,"parent_record_offset":5888,"ancestor_record_offset":5060,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":203,"record_offset":6540,"parent_record_offset":6508,"ancestor_record_offset":5060,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":203,"record_offset":6696,"parent_record_offset":6540,"ancestor_record_offset":5060,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":203,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":5060,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":203,"record_offset":6484,"parent_record_offset":5784,"ancestor_record_offset":5060,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":203,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":5060,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":203,"record_offset":5976,"parent_record_offset":5948,"ancestor_record_offset":5060,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":203,"record_offset":6224,"parent_record_offset":5976,"ancestor_record_offset":5060,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":203,"record_offset":6196,"parent_record_offset":5976,"ancestor_record_offset":5060,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":203,"record_offset":6028,"parent_record_offset":5976,"ancestor_record_offset":5060,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":5660,"parent_record_offset":5632,"ancestor_record_offset":5060,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":8172,"parent_record_offset":7984,"ancestor_record_offset":5060,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":5316,"parent_record_offset":5284,"ancestor_record_offset":5060,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":8264,"parent_record_offset":8232,"ancestor_record_offset":5060,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":5564,"parent_record_offset":5532,"ancestor_record_offset":5060,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":5348,"parent_record_offset":5316,"ancestor_record_offset":5060,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":5060,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":8196,"parent_record_offset":8172,"ancestor_record_offset":5060,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":8288,"parent_record_offset":8264,"ancestor_record_offset":5060,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":203,"record_offset":6832,"parent_record_offset":6800,"ancestor_record_offset":5060,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":203,"record_offset":6936,"parent_record_offset":6896,"ancestor_record_offset":5060,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":203,"record_offset":7100,"parent_record_offset":6896,"ancestor_record_offset":5060,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":203,"record_offset":7368,"parent_record_offset":7100,"ancestor_record_offset":5060,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":203,"record_offset":5532,"parent_record_offset":5504,"ancestor_record_offset":5060,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":203,"record_offset":5504,"parent_record_offset":5476,"ancestor_record_offset":5060,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":203,"record_offset":5412,"parent_record_offset":5248,"ancestor_record_offset":5060,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":203,"record_offset":7948,"parent_record_offset":7920,"ancestor_record_offset":5060,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":203,"record_offset":7984,"parent_record_offset":7948,"ancestor_record_offset":5060,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":203,"record_offset":5632,"parent_record_offset":5440,"ancestor_record_offset":5060,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":203,"record_offset":5440,"parent_record_offset":5412,"ancestor_record_offset":5060,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":203,"record_offset":5284,"parent_record_offset":5248,"ancestor_record_offset":5060,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":203,"record_offset":8232,"parent_record_offset":7948,"ancestor_record_offset":5060,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":203,"record_offset":6964,"parent_record_offset":6936,"ancestor_record_offset":5060,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":203,"record_offset":6996,"parent_record_offset":6964,"ancestor_record_offset":5060,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":203,"record_offset":6896,"parent_record_offset":6864,"ancestor_record_offset":5060,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":203,"record_offset":6864,"parent_record_offset":6832,"ancestor_record_offset":5060,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":203,"record_offset":5476,"parent_record_offset":5440,"ancestor_record_offset":5060,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00203.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::event_loop::run_event_loop::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B8F
// 首条位置: {"module":140,"inlinee":11151}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":140,"record_offset":4396,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":140,"record_offset":6120,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":140,"record_offset":8492,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":140,"record_offset":8520,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":140,"record_offset":6412,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":140,"record_offset":6288,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":140,"record_offset":7000,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":140,"record_offset":4368,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":140,"record_offset":4312,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":140,"record_offset":4892,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":140,"record_offset":8548,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":140,"record_offset":8408,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mpsc\mod.rs
//     evidence={"module":140,"record_offset":4284,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":5570,"source_line":612,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":140,"record_offset":4424,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":140,"record_offset":4932,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] winit::keyboard::ModifiersState::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.4.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-2.4.2/src/lib.rs
//     evidence={"module":140,"record_offset":7924,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":10659,"source_line":698,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":140,"record_offset":6008,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":10654,"source_line":1480,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":140,"record_offset":7028,"parent_record_offset":3996,"ancestor_record_offset":3996,"inlinee":11152,"source_line":1369,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":140,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":3996,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":140,"record_offset":7064,"parent_record_offset":7028,"ancestor_record_offset":3996,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":140,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":3996,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":140,"record_offset":6168,"parent_record_offset":6144,"ancestor_record_offset":3996,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":140,"record_offset":7356,"parent_record_offset":7328,"ancestor_record_offset":3996,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":140,"record_offset":4336,"parent_record_offset":4312,"ancestor_record_offset":3996,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":4812,"parent_record_offset":4784,"ancestor_record_offset":3996,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":5348,"parent_record_offset":5172,"ancestor_record_offset":3996,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":4484,"parent_record_offset":4456,"ancestor_record_offset":3996,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":5436,"parent_record_offset":5408,"ancestor_record_offset":3996,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":4716,"parent_record_offset":4688,"ancestor_record_offset":3996,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":4516,"parent_record_offset":4484,"ancestor_record_offset":3996,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":4740,"parent_record_offset":4716,"ancestor_record_offset":3996,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":3996,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":3996,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::from_raw_os_error
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\error.rs
//     evidence={"module":140,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":3996,"inlinee":9664,"source_line":634,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::last_os_error
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\error.rs
//     evidence={"module":140,"record_offset":7796,"parent_record_offset":7280,"ancestor_record_offset":3996,"inlinee":9663,"source_line":602,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::new_os
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":140,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":3996,"inlinee":9666,"source_line":174,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":140,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":3996,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":140,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":3996,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":140,"record_offset":4580,"parent_record_offset":4424,"ancestor_record_offset":3996,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":140,"record_offset":5044,"parent_record_offset":5016,"ancestor_record_offset":3996,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":140,"record_offset":5072,"parent_record_offset":5044,"ancestor_record_offset":3996,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":140,"record_offset":4784,"parent_record_offset":4608,"ancestor_record_offset":3996,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":140,"record_offset":4608,"parent_record_offset":4580,"ancestor_record_offset":3996,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":140,"record_offset":4456,"parent_record_offset":4424,"ancestor_record_offset":3996,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":140,"record_offset":5408,"parent_record_offset":5140,"ancestor_record_offset":3996,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":140,"record_offset":4640,"parent_record_offset":4608,"ancestor_record_offset":3996,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":140,"record_offset":7120,"parent_record_offset":7092,"ancestor_record_offset":3996,"inlinee":8224,"source_line":281,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":140,"record_offset":7152,"parent_record_offset":7120,"ancestor_record_offset":3996,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":140,"record_offset":7092,"parent_record_offset":7064,"ancestor_record_offset":3996,"inlinee":8222,"source_line":573,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::keyboard::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.4.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-2.4.2/src/lib.rs
//     evidence={"module":140,"record_offset":7952,"parent_record_offset":7924,"ancestor_record_offset":3996,"inlinee":10661,"source_line":698,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":140,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":3996,"inlinee":10657,"source_line":872,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":140,"record_offset":7768,"parent_record_offset":7280,"ancestor_record_offset":3996,"inlinee":8244,"source_line":344,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":140,"record_offset":6032,"parent_record_offset":6008,"ancestor_record_offset":3996,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":140,"record_offset":7328,"parent_record_offset":7280,"ancestor_record_offset":3996,"inlinee":8250,"source_line":452,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_cursor_position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":140,"record_offset":7280,"parent_record_offset":7252,"ancestor_record_offset":3996,"inlinee":11155,"source_line":457,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$13::set_cursor_position::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":140,"record_offset":7252,"parent_record_offset":7220,"ancestor_record_offset":3996,"inlinee":11154,"source_line":1372,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":140,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":3996,"inlinee":10656,"source_line":1482,"archive":"_pdb/A/modules/00140.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["tiny_glade::event_loop::CursorRequestId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::wrap_copy<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::wrap_copy", "arguments": ["tiny_glade::event_loop::CursorRequestId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop::join_head_and_tail_wrapping<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop::join_head_and_tail_wrapping", "arguments": ["tiny_glade::event_loop::CursorRequestId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 19
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::event_loop::GameEvent>", "constructor": "enum2$", "arguments": ["tiny_glade::event_loop::GameEvent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::try_recv<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::array::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::GameEvent> >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::GameEvent> >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::impl$15::drop::closure_env$2", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::GameEvent> >,std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::GameEvent> >", "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::try_recv<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::list::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::try_recv<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::error::SendTimeoutError", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "enum2$", "arguments": ["std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<enum2$<tiny_glade::event_loop::GameEvent> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::GameEvent> >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<tiny_glade::event_loop::GameEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::event_loop::OsWindowEvent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 30
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::event_loop::OsWindowEvent>", "constructor": "enum2$", "arguments": ["tiny_glade::event_loop::OsWindowEvent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::try_recv<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::array::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >,std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "std::sync::mpmc::impl$6::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::impl$15::drop::closure_env$2", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >,std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "std::sync::mpmc::impl$15::drop::closure_env$2<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::try_recv<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::list::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::try_recv<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::zero::Channel::try_recv", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::error::SendTimeoutError", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > >", "constructor": "enum2$", "arguments": ["std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::OsWindowEvent> >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["enum2$<tiny_glade::event_loop::OsWindowEvent>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::OsWindowEvent> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::EguiEventIo::new
// 信息 ID: c9802d94b57f717f48a113bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c5cc9 -> 0x30e390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::EguiEventIo::take_egui_input
// 信息 ID: 056173e7a7672aa7ca873736
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24b53f -> 0x30e510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::impl$1::clone
// 信息 ID: ad7588be16cd213ee13c44e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game; module=256, inline_record=51004; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=256, inline_record=58076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::impl$5::partial_cmp
// 信息 ID: 918641f900197305e32639ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=65832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::run_event_loop
// 信息 ID: edacc87d0b0c17f0e29f13d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c9a8d -> 0x30e900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::event_loop::run_event_loop::closure$0
// 信息 ID: 4d732a3d07e4226834b9fd6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::event_loop::run_event_loop", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::event_loop::run_event_loop::closure_env$0 <- core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tiny_glade::event_loop::run_event_loop::closure_env$0>,tuple$<enum2$<winit::event::Event<tuple$<> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::event_loop::run_event_loop::closure_env$0 <- core::ptr::drop_in_place<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tiny_glade::event_loop::run_event_loop::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::event_loop::run_event_loop::closure_env$0 <- winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tiny_glade::event_loop::run_event_loop::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::event_loop::run_event_loop::closure_env$0 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tiny_glade::event_loop::run_event_loop::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tiny_glade::event_loop::run_event_loop::closure_env$0>; module=140, inline_record=3996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tg-iced/src/dialog.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 41bac85dc2f2b6aa7bd59a116208ae62ab08f0ad631b5ee22fb86b9556d07277
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dialog.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 270, 786, 2011, 2012, 2013
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use iced_core; // namespace candidate
//   package identities: []
// use iced_widget; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tg_iced; // namespace candidate
//   package identities: []
// use winit; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_iced::dialog::MessageDialog::show
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":270,"record_offset":3588,"rva":4138112,"va":5372847232}
// 编译实例 RVAs: 0x3f2480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t417013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8f55c0600","size":5,"soft_fail":false,"target_va":5373265456,"va":5372848438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t417415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8875e0600","size":5,"soft_fail":false,"target_va":5373265408,"va":5372847988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t404319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e85f2b0600","size":5,"soft_fail":false,"target_va":5373252112,"va":5372847788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d84b0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3255652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e89c52ceff","size":5,"soft_fail":false,"target_va":5369595056,"va":5372850703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3215446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8aaefceff","size":5,"soft_fail":false,"target_va":5369632448,"va":5372847889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=2
//     target_aliases=26 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1289184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e82054ecff","size":5,"soft_fail":false,"target_va":5371561776,"va":5372850955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-602022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e85ad0f6ff","size":5,"soft_fail":false,"target_va":5372249392,"va":5372851409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-575501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8f337f7ff","size":5,"soft_fail":false,"target_va":5372274416,"va":5372849912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=4
//     target_aliases=89 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-534193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e84fd9f7ff","size":5,"soft_fail":false,"target_va":5372314208,"va":5372848396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403706c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-535558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8fad3f7ff","size":5,"soft_fail":false,"target_va":5372315328,"va":5372850881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140375190
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-515853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8f320f8ff","size":5,"soft_fail":false,"target_va":5372334480,"va":5372850328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140377f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-503997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8434ff8ff","size":5,"soft_fail":false,"target_va":5372346176,"va":5372850168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-97904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e89081feff","size":5,"soft_fail":false,"target_va":5372749696,"va":5372847595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-97789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e80382feff","size":5,"soft_fail":false,"target_va":5372749712,"va":5372847496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140905660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5319020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e86c295100","size":5,"soft_fail":false,"target_va":5378168416,"va":5372849391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30745542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8c623d501","size":5,"soft_fail":false,"target_va":5403593360,"va":5372847813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30128576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8c0b9cb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372849563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4512844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e84cdc4400","size":5,"soft_fail":false,"target_va":5377364336,"va":5372851487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$winit..event_loop..EventLoop$LT$$LP$$RP$$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5274870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8f67c5000","size":5,"soft_fail":false,"target_va":5378124304,"va":5372849429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30130587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e89bc1cb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372849536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30126419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e853b1cb01","size":5,"soft_fail":false,"target_va":5402975328,"va":5372848904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30126618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e81ab2cb01","size":5,"soft_fail":false,"target_va":5402975504,"va":5372848881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30126700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e86cb2cb01","size":5,"soft_fail":false,"target_va":5402975696,"va":5372848991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5291227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8dbbc5000","size":5,"soft_fail":false,"target_va":5378138560,"va":5372847328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5293641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e849c65000","size":5,"soft_fail":false,"target_va":5378142768,"va":5372849122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t417642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e86a5f0600","size":5,"soft_fail":false,"target_va":5373265840,"va":5372848193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::primary_monitor
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t417642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e86a5f0600","size":5,"soft_fail":false,"target_va":5373265808,"va":5372848161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::init
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5311728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372847232,"raw_hex":"e8f00c5100","size":5,"soft_fail":false,"target_va":5378160512,"va":5372848779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5372847232,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5372847467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":270,"record_offset":8976,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":270,"record_offset":6000,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":270,"record_offset":6232,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8210,"source_line":1547,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":270,"record_offset":8528,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":270,"record_offset":4416,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::dialog::Controls::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/dialog.rs
//     evidence={"module":270,"record_offset":8948,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":14747,"source_line":153,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::impl$18::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":270,"record_offset":6700,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8219,"source_line":302,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":270,"record_offset":3780,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8182,"source_line":200,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":270,"record_offset":5804,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8203,"source_line":329,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":270,"record_offset":7232,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8226,"source_line":305,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":270,"record_offset":6288,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8211,"source_line":125,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":270,"record_offset":5860,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8207,"source_line":118,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::request_user_attention
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":270,"record_offset":8596,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8261,"source_line":1281,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::build
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":270,"record_offset":7260,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8227,"source_line":508,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":270,"record_offset":4516,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8190,"source_line":205,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":270,"record_offset":5444,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8196,"source_line":222,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":270,"record_offset":6900,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8220,"source_line":274,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":270,"record_offset":5144,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8195,"source_line":307,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":270,"record_offset":5776,"parent_record_offset":3588,"ancestor_record_offset":3588,"inlinee":8201,"source_line":340,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":4920,"parent_record_offset":4896,"ancestor_record_offset":3588,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":4944,"parent_record_offset":4920,"ancestor_record_offset":3588,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":3588,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":9052,"parent_record_offset":9012,"ancestor_record_offset":3588,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":3588,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":4192,"parent_record_offset":4108,"ancestor_record_offset":3588,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":270,"record_offset":9012,"parent_record_offset":8976,"ancestor_record_offset":3588,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":270,"record_offset":4108,"parent_record_offset":4080,"ancestor_record_offset":3588,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":5340,"parent_record_offset":5312,"ancestor_record_offset":3588,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":4868,"parent_record_offset":4840,"ancestor_record_offset":3588,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":4840,"parent_record_offset":4812,"ancestor_record_offset":3588,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":5016,"parent_record_offset":4840,"ancestor_record_offset":3588,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":3588,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":4776,"parent_record_offset":4748,"ancestor_record_offset":3588,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":3588,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":4692,"parent_record_offset":4664,"ancestor_record_offset":3588,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":4720,"parent_record_offset":4692,"ancestor_record_offset":3588,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":4664,"parent_record_offset":4636,"ancestor_record_offset":3588,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":270,"record_offset":4636,"parent_record_offset":4600,"ancestor_record_offset":3588,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$46::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":270,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":3588,"inlinee":6525,"source_line":2729,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":270,"record_offset":7492,"parent_record_offset":7468,"ancestor_record_offset":3588,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":270,"record_offset":7468,"parent_record_offset":7428,"ancestor_record_offset":3588,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":4812,"parent_record_offset":4776,"ancestor_record_offset":3588,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":270,"record_offset":4136,"parent_record_offset":4108,"ancestor_record_offset":3588,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":270,"record_offset":5180,"parent_record_offset":5144,"ancestor_record_offset":3588,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$77::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":270,"record_offset":6544,"parent_record_offset":6520,"ancestor_record_offset":3588,"inlinee":8217,"source_line":76,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":270,"record_offset":5612,"parent_record_offset":5588,"ancestor_record_offset":3588,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":270,"record_offset":5076,"parent_record_offset":5052,"ancestor_record_offset":3588,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":270,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":3588,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":270,"record_offset":5052,"parent_record_offset":4776,"ancestor_record_offset":3588,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":3988,"parent_record_offset":3780,"ancestor_record_offset":3588,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":270,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":3588,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":4972,"parent_record_offset":4944,"ancestor_record_offset":3588,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":270,"record_offset":7672,"parent_record_offset":7612,"ancestor_record_offset":3588,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":270,"record_offset":6348,"parent_record_offset":6312,"ancestor_record_offset":3588,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":270,"record_offset":5948,"parent_record_offset":5900,"ancestor_record_offset":3588,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":270,"record_offset":6384,"parent_record_offset":6312,"ancestor_record_offset":3588,"inlinee":8213,"source_line":1407,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":270,"record_offset":3952,"parent_record_offset":3868,"ancestor_record_offset":3588,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":270,"record_offset":8496,"parent_record_offset":7260,"ancestor_record_offset":3588,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":270,"record_offset":7940,"parent_record_offset":7912,"ancestor_record_offset":3588,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":270,"record_offset":8300,"parent_record_offset":8124,"ancestor_record_offset":3588,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":270,"record_offset":3896,"parent_record_offset":3868,"ancestor_record_offset":3588,"inlinee":8187,"source_line":669,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":270,"record_offset":7612,"parent_record_offset":7584,"ancestor_record_offset":3588,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":270,"record_offset":8388,"parent_record_offset":8360,"ancestor_record_offset":3588,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":270,"record_offset":7844,"parent_record_offset":7816,"ancestor_record_offset":3588,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":270,"record_offset":7644,"parent_record_offset":7612,"ancestor_record_offset":3588,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":270,"record_offset":7868,"parent_record_offset":7844,"ancestor_record_offset":3588,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":270,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":3588,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":270,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":3588,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":270,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":3588,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":270,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":3588,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":270,"record_offset":7552,"parent_record_offset":7428,"ancestor_record_offset":3588,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":270,"record_offset":7708,"parent_record_offset":7552,"ancestor_record_offset":3588,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":270,"record_offset":8092,"parent_record_offset":8064,"ancestor_record_offset":3588,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":270,"record_offset":8124,"parent_record_offset":8092,"ancestor_record_offset":3588,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":270,"record_offset":7912,"parent_record_offset":7736,"ancestor_record_offset":3588,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":270,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":3588,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":270,"record_offset":7584,"parent_record_offset":7552,"ancestor_record_offset":3588,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":270,"record_offset":8360,"parent_record_offset":8092,"ancestor_record_offset":3588,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":270,"record_offset":7768,"parent_record_offset":7736,"ancestor_record_offset":3588,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":270,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":3588,"inlinee":8224,"source_line":281,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalSize::cast
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":270,"record_offset":5528,"parent_record_offset":5504,"ancestor_record_offset":3588,"inlinee":8200,"source_line":439,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=13
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":270,"record_offset":5564,"parent_record_offset":5528,"ancestor_record_offset":3588,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$19::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":270,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":3588,"inlinee":8215,"source_line":308,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":270,"record_offset":6028,"parent_record_offset":6000,"ancestor_record_offset":3588,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$33::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":270,"record_offset":5504,"parent_record_offset":5480,"ancestor_record_offset":3588,"inlinee":8198,"source_line":531,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":270,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":3588,"inlinee":8222,"source_line":573,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":270,"record_offset":3868,"parent_record_offset":3780,"ancestor_record_offset":3588,"inlinee":8186,"source_line":120,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":270,"record_offset":3812,"parent_record_offset":3780,"ancestor_record_offset":3588,"inlinee":8184,"source_line":69,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":270,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":3588,"inlinee":8185,"source_line":80,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/event_loop.rs
//     evidence={"module":270,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":3588,"inlinee":8205,"source_line":494,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":270,"record_offset":6312,"parent_record_offset":6288,"ancestor_record_offset":3588,"inlinee":8212,"source_line":211,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":270,"record_offset":5900,"parent_record_offset":5860,"ancestor_record_offset":3588,"inlinee":8209,"source_line":185,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":270,"record_offset":8456,"parent_record_offset":7428,"ancestor_record_offset":3588,"inlinee":8244,"source_line":344,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":270,"record_offset":7360,"parent_record_offset":7260,"ancestor_record_offset":3588,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":270,"record_offset":7324,"parent_record_offset":7260,"ancestor_record_offset":3588,"inlinee":8229,"source_line":96,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":270,"record_offset":7428,"parent_record_offset":7396,"ancestor_record_offset":3588,"inlinee":8233,"source_line":160,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_user_attention
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":270,"record_offset":8668,"parent_record_offset":8644,"ancestor_record_offset":3588,"inlinee":8264,"source_line":929,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::request_user_attention::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":270,"record_offset":8644,"parent_record_offset":8620,"ancestor_record_offset":3588,"inlinee":8263,"source_line":1283,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$30::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":270,"record_offset":4552,"parent_record_offset":4516,"ancestor_record_offset":3588,"inlinee":8192,"source_line":116,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":270,"record_offset":4600,"parent_record_offset":4552,"ancestor_record_offset":3588,"inlinee":8194,"source_line":162,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$9::build::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":270,"record_offset":7396,"parent_record_offset":7360,"ancestor_record_offset":3588,"inlinee":8232,"source_line":514,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::dialog::impl$4::view
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":270,"record_offset":9324,"rva":4142416,"va":5372851536}
// 编译实例 RVAs: 0x3f3550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$str$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t4736759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e8f7464800","size":5,"soft_fail":false,"target_va":5377588464,"va":5372851700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140045d50
//     owner=unresolved_namespace; count=10
//     target_aliases=101 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3861988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e81c12c5ff","size":5,"soft_fail":false,"target_va":5368995152,"va":5372857135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cf70
//     owner=unresolved_namespace; count=4
//     target_aliases=31 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-624075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e8357af6ff","size":5,"soft_fail":false,"target_va":5372235632,"va":5372859702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=17
//     target_aliases=1 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-102250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e89670feff","size":5,"soft_fail":false,"target_va":5372749696,"va":5372851941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-107135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e8815dfeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5372856842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9350
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4996069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e8e53b4c00","size":5,"soft_fail":false,"target_va":5377856336,"va":5372860262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b9490
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4996584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e8e83d4c00","size":5,"soft_fail":false,"target_va":5377856656,"va":5372860067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bf3b0
//     owner=unresolved_namespace; count=5
//     target_aliases=7 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5028945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e851bc4c00","size":5,"soft_fail":false,"target_va":5377881008,"va":5372852058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bf3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=22 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5025206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e8b6ad4c00","size":5,"soft_fail":false,"target_va":5377881024,"va":5372855813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5520
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5052561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e891184d00","size":5,"soft_fail":false,"target_va":5377905952,"va":5372853386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f1050
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5227628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e86cc44f00","size":5,"soft_fail":false,"target_va":5378084944,"va":5372857311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=17
//     evidence={"assembly":"\tcallq\t30121979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e8fb9fcb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372856160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_widget..button..Button$LT$tg_iced..dialog..Message$C$iced_style..theme..Theme$C$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$$GT$
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t5265071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e8af565000","size":5,"soft_fail":false,"target_va":5378122304,"va":5372857228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t30124045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e80da8cb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5372856078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::column::Column$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5323714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e8c23b5100","size":5,"soft_fail":false,"target_va":5378179104,"va":5372855385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_widget::row::Row$LT$Message$C$Theme$C$Renderer$GT$::with_children
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t5049896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e8280e4d00","size":5,"soft_fail":false,"target_va":5377902688,"va":5372852787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t3301087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372851536,"raw_hex":"e8df5e3200","size":5,"soft_fail":false,"target_va":5376152880,"va":5372851788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":270,"record_offset":20164,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":20480,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":270,"record_offset":17608,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::Button::on_press
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":270,"record_offset":11028,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14495,"source_line":112,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::button::impl$2::from
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":270,"record_offset":12160,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14497,"source_line":307,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::padding
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":270,"record_offset":19292,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14535,"source_line":76,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::column::Column::spacing
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":270,"record_offset":19260,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14498,"source_line":70,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_x
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":270,"record_offset":20108,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14506,"source_line":121,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::container::Container::center_y
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":270,"record_offset":20136,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14507,"source_line":127,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::button
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":270,"record_offset":10372,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14488,"source_line":116,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::container
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":270,"record_offset":19704,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14501,"source_line":57,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::helpers::text
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/helpers.rs
//     evidence={"module":270,"record_offset":9504,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14748,"source_line":146,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::align_items
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":270,"record_offset":18632,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14521,"source_line":91,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::spacing
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":270,"record_offset":14352,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14522,"source_line":67,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::Row::with_children
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":270,"record_offset":14104,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14749,"source_line":35,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::row::impl$3::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":270,"record_offset":18660,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14524,"source_line":322,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] iced_widget::scrollable::Scrollable::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/scrollable.rs
//     evidence={"module":270,"record_offset":19320,"parent_record_offset":9324,"ancestor_record_offset":9324,"inlinee":14500,"source_line":51,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":10656,"parent_record_offset":10632,"ancestor_record_offset":9324,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":10680,"parent_record_offset":10656,"ancestor_record_offset":9324,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":9896,"parent_record_offset":9872,"ancestor_record_offset":9324,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":10600,"parent_record_offset":10564,"ancestor_record_offset":9324,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":10632,"parent_record_offset":10600,"ancestor_record_offset":9324,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":9872,"parent_record_offset":9816,"ancestor_record_offset":9324,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":270,"record_offset":10564,"parent_record_offset":10528,"ancestor_record_offset":9324,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":9844,"parent_record_offset":9816,"ancestor_record_offset":9324,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":9816,"parent_record_offset":9788,"ancestor_record_offset":9324,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":270,"record_offset":9592,"parent_record_offset":9540,"ancestor_record_offset":9324,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$21::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":270,"record_offset":9672,"parent_record_offset":9648,"ancestor_record_offset":9324,"inlinee":4475,"source_line":2366,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":270,"record_offset":9540,"parent_record_offset":9504,"ancestor_record_offset":9324,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":270,"record_offset":14168,"parent_record_offset":14136,"ancestor_record_offset":9324,"inlinee":14752,"source_line":57,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":270,"record_offset":14196,"parent_record_offset":14136,"ancestor_record_offset":9324,"inlinee":6065,"source_line":262,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":270,"record_offset":14300,"parent_record_offset":14276,"ancestor_record_offset":9324,"inlinee":6073,"source_line":267,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":270,"record_offset":10456,"parent_record_offset":10408,"ancestor_record_offset":9324,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":270,"record_offset":9620,"parent_record_offset":9540,"ancestor_record_offset":9324,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$53::fmt
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":270,"record_offset":9648,"parent_record_offset":9540,"ancestor_record_offset":9324,"inlinee":5379,"source_line":2333,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":270,"record_offset":14228,"parent_record_offset":14196,"ancestor_record_offset":9324,"inlinee":6067,"source_line":40,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":270,"record_offset":14252,"parent_record_offset":14228,"ancestor_record_offset":9324,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\try_trait.rs
//     evidence={"module":270,"record_offset":14276,"parent_record_offset":14252,"ancestor_record_offset":9324,"inlinee":6071,"source_line":390,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":9740,"parent_record_offset":9540,"ancestor_record_offset":9324,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":10708,"parent_record_offset":10680,"ancestor_record_offset":9324,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":270,"record_offset":9704,"parent_record_offset":9540,"ancestor_record_offset":9324,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::element::Element::new
//     owner=git_dependency_definition; count=17
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/element.rs
//     evidence={"module":270,"record_offset":10528,"parent_record_offset":10492,"ancestor_record_offset":9324,"inlinee":14476,"source_line":32,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::length::Length::fluid
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/length.rs
//     evidence={"module":270,"record_offset":10992,"parent_record_offset":10408,"ancestor_record_offset":9324,"inlinee":14492,"source_line":51,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::widget::text::impl$2::from
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/widget/text.rs
//     evidence={"module":270,"record_offset":10492,"parent_record_offset":10456,"ancestor_record_offset":9324,"inlinee":14475,"source_line":279,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::button::Button::new
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/button.rs
//     evidence={"module":270,"record_offset":10408,"parent_record_offset":10372,"ancestor_record_offset":9324,"inlinee":14490,"source_line":74,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::column::impl$3::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/column.rs
//     evidence={"module":270,"record_offset":19396,"parent_record_offset":19360,"ancestor_record_offset":9324,"inlinee":14486,"source_line":333,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::Container::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":270,"record_offset":19740,"parent_record_offset":19704,"ancestor_record_offset":9324,"inlinee":14503,"source_line":50,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::container::impl$2::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/container.rs
//     evidence={"module":270,"record_offset":20200,"parent_record_offset":20164,"ancestor_record_offset":9324,"inlinee":14509,"source_line":313,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::row::Row::extend
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/row.rs
//     evidence={"module":270,"record_offset":14136,"parent_record_offset":14104,"ancestor_record_offset":9324,"inlinee":14750,"source_line":136,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] iced_widget::scrollable::impl$6::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/widget/src/scrollable.rs
//     evidence={"module":270,"record_offset":19824,"parent_record_offset":19788,"ancestor_record_offset":9324,"inlinee":14505,"source_line":420,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::dialog::MessageDialog::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":270,"record_offset":1568,"rva":4137376,"va":5372846496}
// 编译实例 RVAs: 0x3f21a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::dialog::MessageDialog::set_level
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":270,"record_offset":1680,"rva":4137456,"va":5372846576}
// 编译实例 RVAs: 0x3f21f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::dialog::MessageDialog::set_title
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":270,"record_offset":1800,"rva":4137536,"va":5372846656}
// 编译实例 RVAs: 0x3f2240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1285150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846656,"raw_hex":"e8e263ecff","size":5,"soft_fail":false,"target_va":5371561776,"va":5372846921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-97028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846656,"raw_hex":"e8fc84feff","size":5,"soft_fail":false,"target_va":5372749696,"va":5372846719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-97068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846656,"raw_hex":"e8d484feff","size":5,"soft_fail":false,"target_va":5372749712,"va":5372846775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30746603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846656,"raw_hex":"e8eb27d501","size":5,"soft_fail":false,"target_va":5403593360,"va":5372846752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30131261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846656,"raw_hex":"e83dc4cb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372846878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4680256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846656,"raw_hex":"e8406a4700","size":5,"soft_fail":false,"target_va":5377527120,"va":5372846859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":270,"record_offset":1932,"parent_record_offset":1800,"ancestor_record_offset":1800,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":2496,"parent_record_offset":1800,"ancestor_record_offset":1800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":1800,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":1800,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":1800,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":2252,"parent_record_offset":2216,"ancestor_record_offset":1800,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":2616,"parent_record_offset":2560,"ancestor_record_offset":1800,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":2588,"parent_record_offset":2560,"ancestor_record_offset":1800,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":2216,"parent_record_offset":2188,"ancestor_record_offset":1800,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":2188,"parent_record_offset":2160,"ancestor_record_offset":1800,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":2368,"parent_record_offset":2188,"ancestor_record_offset":1800,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":2560,"parent_record_offset":2536,"ancestor_record_offset":1800,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":2128,"parent_record_offset":2100,"ancestor_record_offset":1800,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":2100,"parent_record_offset":2072,"ancestor_record_offset":1800,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":2044,"parent_record_offset":2016,"ancestor_record_offset":1800,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":2072,"parent_record_offset":2044,"ancestor_record_offset":1800,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":2016,"parent_record_offset":1988,"ancestor_record_offset":1800,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":270,"record_offset":1988,"parent_record_offset":1960,"ancestor_record_offset":1800,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":270,"record_offset":1960,"parent_record_offset":1932,"ancestor_record_offset":1800,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":2160,"parent_record_offset":2128,"ancestor_record_offset":1800,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":270,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":1800,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":270,"record_offset":2408,"parent_record_offset":2128,"ancestor_record_offset":1800,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":2516,"parent_record_offset":2496,"ancestor_record_offset":1800,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":1800,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::dialog::MessageDialog::set_description
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":270,"record_offset":2692,"rva":4137824,"va":5372846944}
// 编译实例 RVAs: 0x3f2360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1285438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846944,"raw_hex":"e8c262ecff","size":5,"soft_fail":false,"target_va":5371561776,"va":5372847209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-97316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846944,"raw_hex":"e8dc83feff","size":5,"soft_fail":false,"target_va":5372749696,"va":5372847007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-97357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846944,"raw_hex":"e8b383feff","size":5,"soft_fail":false,"target_va":5372749712,"va":5372847064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dialog.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30746315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846944,"raw_hex":"e8cb26d501","size":5,"soft_fail":false,"target_va":5403593360,"va":5372847040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30130971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846944,"raw_hex":"e81bc3cb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5372847168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4679966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372846944,"raw_hex":"e81e694700","size":5,"soft_fail":false,"target_va":5377527120,"va":5372847149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":270,"record_offset":2828,"parent_record_offset":2692,"ancestor_record_offset":2692,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":3392,"parent_record_offset":2692,"ancestor_record_offset":2692,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":2692,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":3196,"parent_record_offset":3172,"ancestor_record_offset":2692,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":3536,"parent_record_offset":3512,"ancestor_record_offset":2692,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":3148,"parent_record_offset":3112,"ancestor_record_offset":2692,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":270,"record_offset":3512,"parent_record_offset":3456,"ancestor_record_offset":2692,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":3484,"parent_record_offset":3456,"ancestor_record_offset":2692,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":3112,"parent_record_offset":3084,"ancestor_record_offset":2692,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":3084,"parent_record_offset":3056,"ancestor_record_offset":2692,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":3264,"parent_record_offset":3084,"ancestor_record_offset":2692,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":270,"record_offset":3456,"parent_record_offset":3432,"ancestor_record_offset":2692,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":3024,"parent_record_offset":2996,"ancestor_record_offset":2692,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":2996,"parent_record_offset":2968,"ancestor_record_offset":2692,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":2692,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":2692,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":270,"record_offset":2912,"parent_record_offset":2884,"ancestor_record_offset":2692,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":270,"record_offset":2884,"parent_record_offset":2856,"ancestor_record_offset":2692,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":270,"record_offset":2856,"parent_record_offset":2828,"ancestor_record_offset":2692,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":270,"record_offset":3056,"parent_record_offset":3024,"ancestor_record_offset":2692,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":270,"record_offset":3328,"parent_record_offset":3304,"ancestor_record_offset":2692,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":270,"record_offset":3304,"parent_record_offset":3024,"ancestor_record_offset":2692,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":3412,"parent_record_offset":3392,"ancestor_record_offset":2692,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":270,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":2692,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00270.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::dialog::impl$17::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12E5D
// 首条位置: {"module":2013,"inlinee":77405}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::dialog::impl$2::program_outcome
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x39A9
// 首条位置: {"module":786,"inlinee":14761}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":786,"record_offset":5544,"parent_record_offset":5516,"ancestor_record_offset":5516,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::dialog::Controls::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x399B
// 首条位置: {"module":270,"inlinee":14747}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::dialog::impl$4::update
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x39AB
// 首条位置: {"module":2011,"inlinee":14763}
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
// 完整结构及原始类型记录: dialog.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_iced::dialog::Controls
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_iced::dialog::Controls>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_iced::dialog::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State<tg_iced::dialog::Controls>", "constructor": "iced_runtime::program::state::State", "arguments": ["tg_iced::dialog::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::program::state::State<tg_iced::dialog::Controls> >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::program::state::State<tg_iced::dialog::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls>", "constructor": "tg_iced::gui_runner::run_gui_with_options::closure_env$1", "arguments": ["tg_iced::dialog::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::EventLoop::pump_events", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>", "constructor": "tg_iced::gui_runner::run_gui", "arguments": ["tg_iced::dialog::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >", "tuple$<enum2$<winit::event::Event<tuple$<> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >", "constructor": "winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0", "arguments": ["tuple$<>", "tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$0<tg_iced::dialog::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$0", "arguments": ["tg_iced::dialog::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_iced::dialog::Controls> >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::program::state::impl$0::update::closure_env$0<tg_iced::dialog::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_iced::dialog::Controls> > >", "constructor": "core::iter::adapters::cloned::Cloned", "arguments": ["core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_iced::dialog::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<iced_core::event::Event>,core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_iced::dialog::Controls> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<iced_core::event::Event>", "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::Iter<enum2$<iced_core::event::Event> >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::program::state::impl$0::update::closure_env$0<tg_iced::dialog::Controls> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::new<tg_iced::dialog::Controls>", "constructor": "iced_runtime::program::state::State::new", "arguments": ["tg_iced::dialog::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::State::update<tg_iced::dialog::Controls>", "constructor": "iced_runtime::program::state::State::update", "arguments": ["tg_iced::dialog::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::program::state::impl$0::update::closure_env$1<tg_iced::dialog::Controls>", "constructor": "iced_runtime::program::state::impl$0::update::closure_env$1", "arguments": ["tg_iced::dialog::Controls"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_iced::dialog::Message,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_iced::dialog::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<tg_iced::dialog::Message,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<tg_iced::dialog::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<tg_iced::dialog::Message,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_iced::dialog::Message,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_iced::dialog::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["tg_iced::dialog::Message", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_iced::dialog::Message,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_iced::dialog::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_iced::dialog::Message
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 149
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::action::Action<tg_iced::dialog::Message>", "constructor": "iced_runtime::command::action::Action", "arguments": ["tg_iced::dialog::Message"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<iced_runtime::command::action::Action<tg_iced::dialog::Message> >", "constructor": "enum2$", "arguments": ["iced_runtime::command::action::Action<tg_iced::dialog::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<tg_iced::dialog::Message> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<iced_runtime::command::action::Action<tg_iced::dialog::Message> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<tg_iced::dialog::Message> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<iced_runtime::command::action::Action<tg_iced::dialog::Message> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Item,tg_iced::dialog::Message>", "constructor": "assoc$", "arguments": ["Item", "tg_iced::dialog::Message"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_core::stream::Stream<assoc$<Item,tg_iced::dialog::Message> >", "constructor": "futures_core::stream::Stream", "arguments": ["assoc$<Item,tg_iced::dialog::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<futures_core::stream::Stream<assoc$<Item,tg_iced::dialog::Message> >,core::marker::Send>", "constructor": "dyn$", "arguments": ["futures_core::stream::Stream<assoc$<Item,tg_iced::dialog::Message> >", "core::marker::Send"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,tg_iced::dialog::Message> >,core::marker::Send>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<futures_core::stream::Stream<assoc$<Item,tg_iced::dialog::Message> >,core::marker::Send>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,tg_iced::dialog::Message> >,core::marker::Send>,alloc::alloc::Global> >", "constructor": "core::pin::Pin", "arguments": ["alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,tg_iced::dialog::Message> >,core::marker::Send>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,tg_iced::dialog::Message> >,core::marker::Send>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::pin::Pin<alloc::boxed::Box<dyn$<futures_core::stream::Stream<assoc$<Item,tg_iced::dialog::Message> >,core::marker::Send>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::Row", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::Button<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::Button", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::button::Button<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::button::Button<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::Column", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::column::Column<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::column::Column<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::Group<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::Group", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::group::Group<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::group::Group<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::Container<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::Container", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::container::Container<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::container::Container<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::Scrollable<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::Scrollable", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::scrollable::Scrollable<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::scrollable::Scrollable<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::Nested<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::Nested", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::overlay::nested::Nested<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::overlay::nested::Nested<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::user_interface::UserInterface<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::user_interface::UserInterface<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::element::Element", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::element::Element", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,1>", "constructor": "core::array::iter::IntoIter", "arguments": ["iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::iter::IntoIter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,1> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::iter::IntoIter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::iter::IntoIter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "core::array::iter::IntoIter", "arguments": ["iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::iter::IntoIter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::iter::IntoIter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command<tg_iced::dialog::Message>", "constructor": "iced_runtime::command::Command", "arguments": ["tg_iced::dialog::Message"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_runtime::command::Command<tg_iced::dialog::Message> >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_runtime::command::Command<tg_iced::dialog::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<tg_iced::dialog::Message> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<iced_runtime::command::action::Action<tg_iced::dialog::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::size<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::size", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::size<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::size", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::size<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::size", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::size<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::size", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::size<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::size", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::size<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::size", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::mouse_interaction<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::mouse_interaction", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::diff<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::diff", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::state<iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::state", "arguments": ["iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::operate<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::operate", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::overlay<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::overlay", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::children<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::children", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::tag<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::tag", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::draw<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::draw", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::state<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::state", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::text::impl$1::layout<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::text::impl$1::layout", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::on_event<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::on_event", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_core::widget::text::Text<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>", "constructor": "array$", "arguments": ["iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::Row::with_children<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::row::Row::with_children", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::mouse_interaction<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::mouse_interaction", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::diff<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::diff", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::draw<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::draw", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::layout<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::layout", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate::closure$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate::closure$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::overlay<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::overlay", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::children<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::children", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::on_event<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::on_event", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "tuple$", "arguments": ["iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnMut::call_mut<iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >),tuple$<iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::ops::function::FnMut::call_mut", "arguments": ["iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > (*)(iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >)", "tuple$<iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::row::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::row::impl$2::operate::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::operation::Operation<tg_iced::dialog::Message>", "constructor": "iced_core::widget::operation::Operation", "arguments": ["tg_iced::dialog::Message"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> >", "constructor": "dyn$", "arguments": ["iced_core::widget::operation::Operation<tg_iced::dialog::Message>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > >", "constructor": "ref_mut$", "arguments": ["dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > >", "constructor": "tuple$", "arguments": ["ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::row::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::row::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::row::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::row::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::row::Row<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::Column::with_children<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,array$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2> >", "constructor": "iced_widget::column::Column::with_children", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "array$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::mouse_interaction<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::mouse_interaction", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::diff<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::diff", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::draw<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::draw", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::layout<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::layout", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::overlay<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::overlay", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::children<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::children", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::on_event<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::on_event", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::column::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::column::impl$2::operate::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::column::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::column::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::column::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::column::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::tag<iced_widget::column::Column<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::tag", "arguments": ["iced_widget::column::Column<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::column::Column<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::column::Column<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::notify_on_scroll<tg_iced::dialog::Message>", "constructor": "iced_widget::scrollable::notify_on_scroll", "arguments": ["tg_iced::dialog::Message"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::mouse_interaction<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::mouse_interaction", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::tag<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::tag", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::diff<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::diff", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::draw<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::draw", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::state<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::state", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::layout<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::layout", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate::closure$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate::closure$2", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::overlay<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::overlay", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::children<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::children", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::on_event<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::on_event", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::operate::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::operate::closure_env$2", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::scrollable::impl$5::operate::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::scrollable::impl$5::operate::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::scrollable::impl$5::operate::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::scrollable::impl$5::operate::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::scrollable::Scrollable<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::scrollable::Scrollable<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::Iter", "arguments": ["iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$1", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::impl$0::diff_children::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<iced_core::widget::tree::Tree,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["iced_core::widget::tree::Tree", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::IterMut<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::overlay::group::impl$2::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::layout::node::Node,core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::layout::node::Node", "core::iter::adapters::map::Map<core::slice::iter::IterMut<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::overlay::group::impl$2::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::impl$0::update::closure_env$2", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::event::Status,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::event::Status", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::group::impl$2::overlay::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::group::impl$2::overlay::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "ref$", "arguments": ["iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::widget::tree::Tree,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::widget::tree::Tree", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::Tree (*)(ref$<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::IterMut<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::slice::iter::IterMut", "arguments": ["iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::IterMut<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "core::slice::iter::IterMut<iced_core::widget::tree::Tree>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >", "iced_core::overlay::from_children::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::IterMut<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,core::slice::iter::IterMut<iced_core::widget::tree::Tree> >,core::iter::adapters::map::Map<core::slice::iter::Iter<iced_core::layout::node::Node>,iced_core::layout::impl$0::children::closure_env$0> >,iced_core::overlay::from_children::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tg_iced::dialog::Message,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tg_iced::dialog::Message", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_iced::dialog::Message,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_iced::dialog::Controls> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<tg_iced::dialog::Message,alloc::alloc::Global>", "iced_runtime::program::state::impl$0::update::closure_env$1<tg_iced::dialog::Controls>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::command::Command::batch<tg_iced::dialog::Message,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_iced::dialog::Message,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_iced::dialog::Controls> > >", "constructor": "iced_runtime::command::Command::batch", "arguments": ["tg_iced::dialog::Message", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_iced::dialog::Message,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_iced::dialog::Controls> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::mouse_interaction<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::mouse_interaction", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::tag<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::tag", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::diff<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::diff", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::draw<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::draw", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::state<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::state", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::layout<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::layout", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate::closure$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate::closure$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::overlay<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::overlay", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::children<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::children", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::on_event<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::on_event", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::operate::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::button::impl$1::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::button::impl$1::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::button::impl$1::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::button::impl$1::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::push<tg_iced::dialog::Message,alloc::alloc::Global>", "constructor": "alloc::vec::Vec::push", "arguments": ["tg_iced::dialog::Message", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::button::Button<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::button::Button<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::mouse_interaction<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::mouse_interaction", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::tag<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::tag", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::diff<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::diff", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::draw<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::draw", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::state<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::state", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::layout<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::layout", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate::closure$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate::closure$1", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::overlay<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::overlay", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::children<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::children", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::on_event<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::on_event", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::operate::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::operate::closure_env$1", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_widget::container::impl$1::operate::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_widget::container::impl$1::operate::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_widget::container::impl$1::operate::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_widget::container::impl$1::operate::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::Widget::size_hint<iced_widget::container::Container<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::widget::Widget::size_hint", "arguments": ["iced_widget::container::Container<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<tg_iced::dialog::Message,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["tg_iced::dialog::Message", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<tg_iced::dialog::Message,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["tg_iced::dialog::Message", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["iced_core::overlay::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::impl$0::diff_children::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::widget::tree::impl$0::diff_children::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::widget::tree::Tree::diff_children_custom<iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::widget::tree::impl$0::diff_children::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_core::widget::tree::impl$0::diff_children::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::widget::tree::Tree::diff_children_custom", "arguments": ["iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::widget::tree::impl$0::diff_children::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_core::widget::tree::impl$0::diff_children::closure_env$1<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::draw::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::draw::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure$1<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure$1", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure_env$0", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_graphics::renderer::impl$1::with_layer<iced_tiny_skia::backend::Backend,iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_graphics::renderer::impl$1::with_layer", "arguments": ["iced_tiny_skia::backend::Backend", "iced_widget::scrollable::draw::closure_env$0<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::draw::closure_env$2", "arguments": ["enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::draw::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_graphics::renderer::impl$1::with_layer<iced_tiny_skia::backend::Backend,iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_graphics::renderer::impl$1::with_layer", "arguments": ["iced_tiny_skia::backend::Backend", "iced_widget::scrollable::draw::closure_env$2<enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::draw::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::button::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::button::impl$1::layout::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_core::layout::padded::closure_env$0", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::positioned<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::positioned", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::button::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "iced_core::layout::padded::closure_env$0<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::button::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::container::impl$1::layout::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::layout::closure_env$0", "arguments": ["iced_widget::container::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::container::layout::closure_env$1", "arguments": ["iced_widget::container::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::positioned<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_core::padding::Padding,iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >,iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::positioned", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_core::padding::Padding", "iced_widget::container::layout::closure_env$0<iced_widget::container::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "iced_widget::container::layout::closure_env$1<iced_widget::container::impl$1::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::impl$5::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_widget::scrollable::impl$5::layout::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_widget::scrollable::layout::closure_env$0", "arguments": ["iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_widget::scrollable::impl$5::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::contained<enum2$<iced_core::length::Length>,enum2$<iced_core::length::Length>,iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "iced_core::layout::contained", "arguments": ["enum2$<iced_core::length::Length>", "enum2$<iced_core::length::Length>", "iced_widget::scrollable::layout::closure_env$0<iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_widget::scrollable::impl$5::layout::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::layout::flex::resolve<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::layout::flex::resolve", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::from_children<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::from_children", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["iced_core::event::Status", "alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<iced_core::event::Status,core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["iced_core::event::Status", "core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >", "iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<iced_core::event::Status,alloc::vec::impl$19::extend_trusted::closure_env$0<iced_core::event::Status,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<enum2$<iced_core::event::Event> > >,alloc::vec::into_iter::IntoIter<iced_core::event::Status,alloc::alloc::Global> >,iced_runtime::user_interface::impl$0::update::closure_env$2<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::into_cache<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::into_cache", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::draw<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::draw", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::build<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>,iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "iced_runtime::user_interface::UserInterface::build", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>", "iced_core::element::Element<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::user_interface::UserInterface::update<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::user_interface::UserInterface::update", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::mouse_interaction<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::mouse_interaction", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::draw<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::draw", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::layout<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::layout", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::is_over<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::is_over", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate::closure$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate::closure$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::overlay<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::overlay", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::on_event<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::on_event", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_core::overlay::group::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_core::overlay::group::impl$2::operate::closure_env$0", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<iced_core::overlay::group::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >,tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["iced_core::overlay::group::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "tuple$<ref_mut$<dyn$<iced_core::widget::operation::Operation<tg_iced::dialog::Message> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<iced_core::overlay::group::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["iced_core::overlay::group::impl$2::operate::closure_env$0<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::mouse_interaction::recurse", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::draw::recurse<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::draw::recurse", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::layout::recurse<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::layout::recurse", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::is_over::recurse<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::is_over::recurse", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "iced_runtime::overlay::nested::impl$0::on_event::recurse<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >", "constructor": "iced_runtime::overlay::nested::impl$0::on_event::recurse", "arguments": ["tg_iced::dialog::Message", "enum2$<iced_style::theme::Theme>", "iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_iced::dialog::MessageDialog
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_iced::dialog::MessageDialog>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_iced::dialog::MessageDialog"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dialog.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dialog.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tg_iced::dialog::Controls::new
// 信息 ID: c8b7d3f349aa53ed4998fef3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::dialog::MessageDialog::show; module=270, inline_record=8948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::dialog::MessageDialog::new
// 信息 ID: 4adf19d64a0c9c68e0b6fcfc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_config::get_cli_args @ 0x325a4d -> 0x3f21a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::dialog::MessageDialog::set_description
// 信息 ID: cd24e1ee10d7c2696bad4bd9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_config::get_cli_args @ 0x325b1e -> 0x3f2360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::dialog::MessageDialog::set_level
// 信息 ID: c49541439d28f297e083236b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_config::get_cli_args @ 0x325a63 -> 0x3f21f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::dialog::MessageDialog::set_title
// 信息 ID: 9e12727bed86f1fc1b667e69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_config::get_cli_args @ 0x325a83 -> 0x3f2240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::dialog::MessageDialog::show
// 信息 ID: b752f3cc8863211b2c4e4a3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_config::get_cli_args @ 0x325b2a -> 0x3f2480 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::dialog::impl$17::clone
// 信息 ID: fc376f9f99be54ade788e65e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_widget::button::impl$1::on_event<tg_iced::dialog::Message,enum2$<iced_style::theme::Theme>,iced_graphics::renderer::Renderer<iced_tiny_skia::backend::Backend> >; module=2013, inline_record=8844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::dialog::impl$2::program_outcome
// 信息 ID: aa16024801982f80794a901c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>; module=786, inline_record=5516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::dialog::impl$4::update
// 信息 ID: b90460aea929d656a52164f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] iced_runtime::command::Command::batch<tg_iced::dialog::Message,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tg_iced::dialog::Message,alloc::alloc::Global>,iced_runtime::program::state::impl$0::update::closure_env$1<tg_iced::dialog::Controls> > >; module=2012, inline_record=1068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::dialog::impl$4::view
// 信息 ID: aa1c0eb1a9d76cbeac9ab5bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] iced_runtime::program::state::State::new<tg_iced::dialog::Controls> @ 0x131c021 -> 0x3f3550 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_runtime::program::state::State::update<tg_iced::dialog::Controls> @ 0x131c296 -> 0x3f3550 (direct_call); 同地址别名未裁定。
// [静态直接引用] iced_runtime::program::state::State::update<tg_iced::dialog::Controls> @ 0x131c5d2 -> 0x3f3550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

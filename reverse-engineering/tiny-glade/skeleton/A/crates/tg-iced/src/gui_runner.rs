// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tg-iced/src/gui_runner.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: fa2eadd8f9a7f284779d2213b7a1c9848423121c628505993043a0633cdb330f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gui_runner.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 140, 202, 729, 779, 786, 1855, 1995, 3170, 3236, 3244
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use iced_core; // namespace candidate
//   package identities: []
// use iced_graphics; // namespace candidate
//   package identities: []
// use iced_runtime; // namespace candidate
//   package identities: []
// use iced_style; // namespace candidate
//   package identities: []
// use iced_tiny_skia; // namespace candidate
//   package identities: []
// use iced_winit; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tg_gpu_selector; // namespace candidate
//   package identities: []
// use tg_iced; // namespace candidate
//   package identities: []
// use tg_integrity_report; // namespace candidate
//   package identities: []
// use winit; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB276
// 首条位置: {"module":729,"inlinee":45686}
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
// 首条位置: {"module":779,"record_offset":12128,"rva":9393120,"va":5378102240}
// 编译实例 RVAs: 0x8f53e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5196794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e806b4b0ff","size":5,"soft_fail":false,"target_va":5372905968,"va":5378102757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10469667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e823c19f00","size":5,"soft_fail":false,"target_va":5388572480,"va":5378102808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5834985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e817f7a6ff","size":5,"soft_fail":false,"target_va":5372274416,"va":5378109396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036afd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5813114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8864ca7ff","size":5,"soft_fail":false,"target_va":5372293072,"va":5378106181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370de0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5788059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e865aea7ff","size":5,"soft_fail":false,"target_va":5372317152,"va":5378105206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140384450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5712681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8d7d4a8ff","size":5,"soft_fail":false,"target_va":5372396624,"va":5378109300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5352969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8f751aeff","size":5,"soft_fail":false,"target_va":5372749696,"va":5378102660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5355538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8ee47aeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5378105245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408efdb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e86a9bffff","size":5,"soft_fail":false,"target_va":5378080176,"va":5378105921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f08c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8de9dffff","size":5,"soft_fail":false,"target_va":5378083008,"va":5378108125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f29f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e88bbcffff","size":5,"soft_fail":false,"target_va":5378091504,"va":5378108768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f9bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10498736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8b032a000","size":5,"soft_fail":false,"target_va":5388606400,"va":5378107659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t25490519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e857f48401","size":5,"soft_fail":false,"target_va":5403593360,"va":5378102836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24871275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e86b817b01","size":5,"soft_fail":false,"target_va":5402978144,"va":5378106864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-6451426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e81e8f9dff","size":5,"soft_fail":false,"target_va":5371654736,"va":5378106157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24872099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8a3847b01","size":5,"soft_fail":false,"target_va":5402978944,"va":5378106840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..collections..vec_deque..VecDeque$LT$alloc..vec..Vec$LT$iced_graphics..primitive..Primitive$LT$iced_tiny_skia..primitive..Custom$GT$$GT$$GT$$GT$::hb58e91c63e9ee5fd (.llvm.18404438595659946405)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-22603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8b5a7ffff","size":5,"soft_fail":false,"target_va":5378083632,"va":5378106230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-17195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8d5bcffff","size":5,"soft_fail":false,"target_va":5378089056,"va":5378106246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..program..state..State$LT$tg_gpu_selector..gpu_selection..Controls$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-27102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e82296ffff","size":5,"soft_fail":false,"target_va":5378079072,"va":5378106169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_tiny_skia..window..compositor..Surface$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8deacffff","size":5,"soft_fail":false,"target_va":5378087824,"va":5378109101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_gpu_selector..gpu_selection..Controls$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e87aa9ffff","size":5,"soft_fail":false,"target_va":5378087488,"va":5378109633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e860ceffff","size":5,"soft_fail":false,"target_va":5378093104,"va":5378105803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5148134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e81a72b1ff","size":5,"soft_fail":false,"target_va":5372954416,"va":5378102545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10479063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8d7e59f00","size":5,"soft_fail":false,"target_va":5388581952,"va":5378102884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10477478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8a6df9f00","size":5,"soft_fail":false,"target_va":5388582512,"va":5378105029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5237401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e86715b0ff","size":5,"soft_fail":false,"target_va":5372865200,"va":5378102596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5238838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8ca0fb0ff","size":5,"soft_fail":false,"target_va":5372864528,"va":5378103361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24872850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e892877b01","size":5,"soft_fail":false,"target_va":5402975328,"va":5378102473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24873181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8dd887b01","size":5,"soft_fail":false,"target_va":5402975696,"va":5378102510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10486667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e88b03a000","size":5,"soft_fail":false,"target_va":5388589936,"va":5378103264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10488526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e8ce0aa000","size":5,"soft_fail":false,"target_va":5388591488,"va":5378102957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4849264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378102240,"raw_hex":"e89001b6ff","size":5,"soft_fail":false,"target_va":5373253168,"va":5378102427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*33183297(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378102240,"raw_hex":"ff154156fa01","size":6,"soft_fail":false,"va":5378102369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":779,"record_offset":12288,"parent_record_offset":12128,"ancestor_record_offset":12128,"inlinee":45686,"source_line":28,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::gui_runner::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":779,"record_offset":12312,"parent_record_offset":12128,"ancestor_record_offset":12128,"inlinee":12801,"source_line":46,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":13696,"parent_record_offset":13672,"ancestor_record_offset":12128,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":12128,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":14852,"parent_record_offset":14828,"ancestor_record_offset":12128,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":13640,"parent_record_offset":13604,"ancestor_record_offset":12128,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":13672,"parent_record_offset":13640,"ancestor_record_offset":12128,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":14828,"parent_record_offset":14736,"ancestor_record_offset":12128,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":779,"record_offset":13604,"parent_record_offset":13568,"ancestor_record_offset":12128,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":779,"record_offset":17712,"parent_record_offset":17684,"ancestor_record_offset":12128,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":779,"record_offset":14764,"parent_record_offset":14736,"ancestor_record_offset":12128,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":779,"record_offset":14736,"parent_record_offset":14712,"ancestor_record_offset":12128,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":779,"record_offset":17012,"parent_record_offset":16976,"ancestor_record_offset":12128,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":779,"record_offset":13444,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":779,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":12128,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":779,"record_offset":13816,"parent_record_offset":13792,"ancestor_record_offset":12128,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":779,"record_offset":15272,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":779,"record_offset":14636,"parent_record_offset":14604,"ancestor_record_offset":12128,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":779,"record_offset":15500,"parent_record_offset":15460,"ancestor_record_offset":12128,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":779,"record_offset":15460,"parent_record_offset":15432,"ancestor_record_offset":12128,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":779,"record_offset":15724,"parent_record_offset":15700,"ancestor_record_offset":12128,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":779,"record_offset":15328,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":779,"record_offset":15572,"parent_record_offset":15548,"ancestor_record_offset":12128,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":779,"record_offset":17740,"parent_record_offset":17712,"ancestor_record_offset":12128,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":779,"record_offset":12792,"parent_record_offset":12624,"ancestor_record_offset":12128,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":779,"record_offset":12752,"parent_record_offset":12728,"ancestor_record_offset":12128,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":779,"record_offset":15548,"parent_record_offset":15524,"ancestor_record_offset":12128,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":779,"record_offset":15524,"parent_record_offset":15500,"ancestor_record_offset":12128,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":779,"record_offset":12704,"parent_record_offset":12680,"ancestor_record_offset":12128,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":779,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":12128,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":779,"record_offset":12680,"parent_record_offset":12624,"ancestor_record_offset":12128,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":779,"record_offset":14792,"parent_record_offset":14764,"ancestor_record_offset":12128,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":779,"record_offset":14428,"parent_record_offset":14404,"ancestor_record_offset":12128,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":779,"record_offset":14340,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":779,"record_offset":14404,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":779,"record_offset":12836,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":779,"record_offset":15620,"parent_record_offset":15596,"ancestor_record_offset":12128,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":779,"record_offset":13012,"parent_record_offset":12984,"ancestor_record_offset":12128,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":779,"record_offset":15296,"parent_record_offset":15272,"ancestor_record_offset":12128,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":779,"record_offset":12728,"parent_record_offset":12704,"ancestor_record_offset":12128,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":779,"record_offset":15400,"parent_record_offset":15328,"ancestor_record_offset":12128,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":779,"record_offset":17044,"parent_record_offset":17012,"ancestor_record_offset":12128,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":779,"record_offset":15596,"parent_record_offset":15572,"ancestor_record_offset":12128,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":779,"record_offset":15368,"parent_record_offset":15328,"ancestor_record_offset":12128,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":779,"record_offset":13748,"parent_record_offset":13720,"ancestor_record_offset":12128,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":779,"record_offset":13956,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":13236,"parent_record_offset":13064,"ancestor_record_offset":12128,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":13324,"parent_record_offset":13296,"ancestor_record_offset":12128,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":13472,"parent_record_offset":13444,"ancestor_record_offset":12128,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":13852,"parent_record_offset":13816,"ancestor_record_offset":12128,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":13168,"parent_record_offset":13140,"ancestor_record_offset":12128,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":13496,"parent_record_offset":13472,"ancestor_record_offset":12128,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":12128,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":12128,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":13876,"parent_record_offset":13852,"ancestor_record_offset":12128,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":13348,"parent_record_offset":13324,"ancestor_record_offset":12128,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":13908,"parent_record_offset":13816,"ancestor_record_offset":12128,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/mouse/cursor.rs
//     evidence={"module":779,"record_offset":14452,"parent_record_offset":14428,"ancestor_record_offset":12128,"inlinee":12795,"source_line":7,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/command.rs
//     evidence={"module":779,"record_offset":15076,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":4466,"source_line":104,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/style/src/theme.rs
//     evidence={"module":779,"record_offset":14488,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":11172,"source_line":123,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":779,"record_offset":13532,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":7811,"source_line":31,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":779,"record_offset":13568,"parent_record_offset":13532,"ancestor_record_offset":12128,"inlinee":7680,"source_line":139,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":779,"record_offset":13140,"parent_record_offset":13116,"ancestor_record_offset":12128,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":779,"record_offset":13116,"parent_record_offset":13092,"ancestor_record_offset":12128,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":779,"record_offset":13036,"parent_record_offset":13012,"ancestor_record_offset":12128,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":779,"record_offset":13064,"parent_record_offset":13036,"ancestor_record_offset":12128,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":779,"record_offset":13296,"parent_record_offset":13036,"ancestor_record_offset":12128,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":779,"record_offset":13092,"parent_record_offset":13064,"ancestor_record_offset":12128,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::gpu_selection::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/gpu_selection.rs
//     evidence={"module":779,"record_offset":14276,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":46038,"source_line":57,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":779,"record_offset":12860,"parent_record_offset":12836,"ancestor_record_offset":12128,"inlinee":12792,"source_line":59,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":779,"record_offset":14372,"parent_record_offset":14340,"ancestor_record_offset":12128,"inlinee":12793,"source_line":194,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform/pump_events.rs
//     evidence={"module":779,"record_offset":14312,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":12641,"source_line":183,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":779,"record_offset":12624,"parent_record_offset":12596,"ancestor_record_offset":12128,"inlinee":10542,"source_line":215,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":779,"record_offset":14096,"parent_record_offset":14072,"ancestor_record_offset":12128,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":779,"record_offset":12568,"parent_record_offset":12540,"ancestor_record_offset":12128,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":779,"record_offset":12984,"parent_record_offset":12960,"ancestor_record_offset":12128,"inlinee":8250,"source_line":452,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":779,"record_offset":14144,"parent_record_offset":14120,"ancestor_record_offset":12128,"inlinee":10528,"source_line":143,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":12540,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":12912,"parent_record_offset":12860,"ancestor_record_offset":12128,"inlinee":8246,"source_line":568,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":14072,"parent_record_offset":12312,"ancestor_record_offset":12128,"inlinee":10696,"source_line":940,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":12960,"parent_record_offset":12936,"ancestor_record_offset":12128,"inlinee":8249,"source_line":569,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":12596,"parent_record_offset":12568,"ancestor_record_offset":12128,"inlinee":11163,"source_line":747,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":14120,"parent_record_offset":14096,"ancestor_record_offset":12128,"inlinee":10698,"source_line":942,"archive":"_pdb/A/modules/00779.jsonl.gz"}
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
// 首条位置: {"module":779,"record_offset":3416,"rva":9385504,"va":5378094624}
// 编译实例 RVAs: 0x8f3620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5189171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8cdd1b0ff","size":5,"soft_fail":false,"target_va":5372905968,"va":5378095134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10477290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8eade9f00","size":5,"soft_fail":false,"target_va":5388572480,"va":5378095185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5827577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e80714a7ff","size":5,"soft_fail":false,"target_va":5372274416,"va":5378101988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036afd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5805538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e81e6aa7ff","size":5,"soft_fail":false,"target_va":5372293072,"va":5378098605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370de0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5780435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e82dcca7ff","size":5,"soft_fail":false,"target_va":5372317152,"va":5378097582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140384450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5705273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8c7f1a8ff","size":5,"soft_fail":false,"target_va":5372396624,"va":5378101892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5345349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8bb6faeff","size":5,"soft_fail":false,"target_va":5372749696,"va":5378095040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5347914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8b665aeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5378097621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408efdb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e80ab9ffff","size":5,"soft_fail":false,"target_va":5378080176,"va":5378098337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f08c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8debaffff","size":5,"soft_fail":false,"target_va":5378083008,"va":5378100701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f29f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e872d9ffff","size":5,"soft_fail":false,"target_va":5378091504,"va":5378101369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f74e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10497973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8b52fa000","size":5,"soft_fail":false,"target_va":5388596448,"va":5378098470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f9bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10506521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e81951a000","size":5,"soft_fail":false,"target_va":5388606400,"va":5378099874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24878708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8749e7b01","size":5,"soft_fail":false,"target_va":5402978144,"va":5378099431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-6443850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8b6ac9dff","size":5,"soft_fail":false,"target_va":5371654736,"va":5378098581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24879532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8aca17b01","size":5,"soft_fail":false,"target_va":5402978944,"va":5378099407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..collections..vec_deque..VecDeque$LT$alloc..vec..Vec$LT$iced_graphics..primitive..Primitive$LT$iced_tiny_skia..primitive..Custom$GT$$GT$$GT$$GT$::hb58e91c63e9ee5fd (.llvm.18404438595659946405)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-15027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e84dc5ffff","size":5,"soft_fail":false,"target_va":5378083632,"va":5378098654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-9619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e86ddaffff","size":5,"soft_fail":false,"target_va":5378089056,"va":5378098670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..program..state..State$LT$tg_gpu_selector..vulkan_config..Controls$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-18966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8eab5ffff","size":5,"soft_fail":false,"target_va":5378079632,"va":5378098593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_tiny_skia..window..compositor..Surface$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8cec9ffff","size":5,"soft_fail":false,"target_va":5378087824,"va":5378101693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$tg_gpu_selector..vulkan_config..Controls$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8bac6ffff","size":5,"soft_fail":false,"target_va":5378087568,"va":5378102225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_gpu_selector::gpu_selection::Message> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e80cecffff","size":5,"soft_fail":false,"target_va":5378093104,"va":5378098207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5140514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8de8fb1ff","size":5,"soft_fail":false,"target_va":5372954416,"va":5378094925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10486055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e82701a000","size":5,"soft_fail":false,"target_va":5388581376,"va":5378095316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10488526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8ce0aa000","size":5,"soft_fail":false,"target_va":5388585936,"va":5378097405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5229781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e82b33b0ff","size":5,"soft_fail":false,"target_va":5372865200,"va":5378094976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5231226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8862db0ff","size":5,"soft_fail":false,"target_va":5372864528,"va":5378095749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24880470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e856a57b01","size":5,"soft_fail":false,"target_va":5402975328,"va":5378094853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24880801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8a1a67b01","size":5,"soft_fail":false,"target_va":5402975696,"va":5378094890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10494826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e86a23a000","size":5,"soft_fail":false,"target_va":5388590512,"va":5378095681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10496094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e85e28a000","size":5,"soft_fail":false,"target_va":5388591488,"va":5378095389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4841644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378094624,"raw_hex":"e8541fb6ff","size":5,"soft_fail":false,"target_va":5373253168,"va":5378094807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*33190917(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378094624,"raw_hex":"ff150574fa01","size":6,"soft_fail":false,"va":5378094749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":779,"record_offset":3576,"parent_record_offset":3416,"ancestor_record_offset":3416,"inlinee":45686,"source_line":28,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::gui_runner::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":779,"record_offset":3600,"parent_record_offset":3416,"ancestor_record_offset":3416,"inlinee":12801,"source_line":46,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":4984,"parent_record_offset":4960,"ancestor_record_offset":3416,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":3416,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":3416,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":4928,"parent_record_offset":4892,"ancestor_record_offset":3416,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":4960,"parent_record_offset":4928,"ancestor_record_offset":3416,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":779,"record_offset":6064,"parent_record_offset":5972,"ancestor_record_offset":3416,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":779,"record_offset":4892,"parent_record_offset":4856,"ancestor_record_offset":3416,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":779,"record_offset":8328,"parent_record_offset":8300,"ancestor_record_offset":3416,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":779,"record_offset":6000,"parent_record_offset":5972,"ancestor_record_offset":3416,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":779,"record_offset":5972,"parent_record_offset":5948,"ancestor_record_offset":3416,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":779,"record_offset":7628,"parent_record_offset":7592,"ancestor_record_offset":3416,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":779,"record_offset":4732,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":779,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":3416,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":779,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":3416,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":779,"record_offset":6508,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":779,"record_offset":5884,"parent_record_offset":5856,"ancestor_record_offset":3416,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":779,"record_offset":6736,"parent_record_offset":6696,"ancestor_record_offset":3416,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":779,"record_offset":6696,"parent_record_offset":6668,"ancestor_record_offset":3416,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":779,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":3416,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":779,"record_offset":6564,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":779,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":3416,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":779,"record_offset":8356,"parent_record_offset":8328,"ancestor_record_offset":3416,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":779,"record_offset":4080,"parent_record_offset":3912,"ancestor_record_offset":3416,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":779,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":3416,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":779,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":3416,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":779,"record_offset":6760,"parent_record_offset":6736,"ancestor_record_offset":3416,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":779,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":3416,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":779,"record_offset":8380,"parent_record_offset":8356,"ancestor_record_offset":3416,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":779,"record_offset":3968,"parent_record_offset":3912,"ancestor_record_offset":3416,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":779,"record_offset":6028,"parent_record_offset":6000,"ancestor_record_offset":3416,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":779,"record_offset":5680,"parent_record_offset":5656,"ancestor_record_offset":3416,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":779,"record_offset":5592,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":779,"record_offset":5656,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":779,"record_offset":4124,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":779,"record_offset":7440,"parent_record_offset":7416,"ancestor_record_offset":3416,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":779,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":3416,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":779,"record_offset":4300,"parent_record_offset":4272,"ancestor_record_offset":3416,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":779,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":3416,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":779,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":3416,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":779,"record_offset":6636,"parent_record_offset":6564,"ancestor_record_offset":3416,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":779,"record_offset":7660,"parent_record_offset":7628,"ancestor_record_offset":3416,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":779,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":3416,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":779,"record_offset":6604,"parent_record_offset":6564,"ancestor_record_offset":3416,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":779,"record_offset":5036,"parent_record_offset":5008,"ancestor_record_offset":3416,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":779,"record_offset":5244,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":4524,"parent_record_offset":4352,"ancestor_record_offset":3416,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":4612,"parent_record_offset":4584,"ancestor_record_offset":3416,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":4760,"parent_record_offset":4732,"ancestor_record_offset":3416,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":5140,"parent_record_offset":5104,"ancestor_record_offset":3416,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":4456,"parent_record_offset":4428,"ancestor_record_offset":3416,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":3416,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":3416,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":4548,"parent_record_offset":4524,"ancestor_record_offset":3416,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":5164,"parent_record_offset":5140,"ancestor_record_offset":3416,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":3416,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":779,"record_offset":5196,"parent_record_offset":5104,"ancestor_record_offset":3416,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/mouse/cursor.rs
//     evidence={"module":779,"record_offset":5704,"parent_record_offset":5680,"ancestor_record_offset":3416,"inlinee":12795,"source_line":7,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/command.rs
//     evidence={"module":779,"record_offset":6312,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":4466,"source_line":104,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/style/src/theme.rs
//     evidence={"module":779,"record_offset":5740,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":11172,"source_line":123,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":779,"record_offset":4820,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":7811,"source_line":31,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":779,"record_offset":4856,"parent_record_offset":4820,"ancestor_record_offset":3416,"inlinee":7680,"source_line":139,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":779,"record_offset":7500,"parent_record_offset":7468,"ancestor_record_offset":3416,"inlinee":10594,"source_line":87,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":779,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":3416,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":779,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":3416,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":779,"record_offset":4324,"parent_record_offset":4300,"ancestor_record_offset":3416,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":779,"record_offset":4352,"parent_record_offset":4324,"ancestor_record_offset":3416,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":779,"record_offset":4584,"parent_record_offset":4324,"ancestor_record_offset":3416,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":779,"record_offset":4380,"parent_record_offset":4352,"ancestor_record_offset":3416,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::vulkan_config::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":779,"record_offset":7416,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":46034,"source_line":47,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::vulkan_config::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":779,"record_offset":7468,"parent_record_offset":7440,"ancestor_record_offset":3416,"inlinee":46036,"source_line":16,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":779,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":3416,"inlinee":12792,"source_line":59,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":779,"record_offset":5624,"parent_record_offset":5592,"ancestor_record_offset":3416,"inlinee":12793,"source_line":194,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform/pump_events.rs
//     evidence={"module":779,"record_offset":5564,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":12641,"source_line":183,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":779,"record_offset":3912,"parent_record_offset":3884,"ancestor_record_offset":3416,"inlinee":10542,"source_line":215,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":779,"record_offset":5384,"parent_record_offset":5360,"ancestor_record_offset":3416,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":779,"record_offset":3856,"parent_record_offset":3828,"ancestor_record_offset":3416,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":779,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":3416,"inlinee":8250,"source_line":452,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":779,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":3416,"inlinee":10528,"source_line":143,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":3828,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":4200,"parent_record_offset":4148,"ancestor_record_offset":3416,"inlinee":8246,"source_line":568,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":5360,"parent_record_offset":3600,"ancestor_record_offset":3416,"inlinee":10696,"source_line":940,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":3416,"inlinee":8249,"source_line":569,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":3884,"parent_record_offset":3856,"ancestor_record_offset":3416,"inlinee":11163,"source_line":747,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":779,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":3416,"inlinee":10698,"source_line":942,"archive":"_pdb/A/modules/00779.jsonl.gz"}
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
// 首条位置: {"module":786,"record_offset":3384,"rva":9459296,"va":5378168416}
// 编译实例 RVAs: 0x905660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5262970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e886b1afff","size":5,"soft_fail":false,"target_va":5372905968,"va":5378168933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10568819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e87344a100","size":5,"soft_fail":false,"target_va":5388737808,"va":5378168984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6614070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8ca139bff","size":5,"soft_fail":false,"target_va":5371561776,"va":5378175841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5901193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e877f4a5ff","size":5,"soft_fail":false,"target_va":5372274416,"va":5378175604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036afd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5879338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8d649a6ff","size":5,"soft_fail":false,"target_va":5372293072,"va":5378172405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370de0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5854283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8b5aba6ff","size":5,"soft_fail":false,"target_va":5372317152,"va":5378171430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140384450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5778889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e837d2a7ff","size":5,"soft_fail":false,"target_va":5372396624,"va":5378175508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5419145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8774fadff","size":5,"soft_fail":false,"target_va":5372749696,"va":5378168836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5421762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e83e45adff","size":5,"soft_fail":false,"target_va":5372749712,"va":5378171469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14131d0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10577152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e80065a100","size":5,"soft_fail":false,"target_va":5388751024,"va":5378173867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24805068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8cc7e7a01","size":5,"soft_fail":false,"target_va":5402978144,"va":5378173071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-6517650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e86e8c9cff","size":5,"soft_fail":false,"target_va":5371654736,"va":5378172381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24805892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e804827a01","size":5,"soft_fail":false,"target_va":5402978944,"va":5378173047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..collections..vec_deque..VecDeque$LT$alloc..vec..Vec$LT$iced_graphics..primitive..Primitive$LT$iced_tiny_skia..primitive..Custom$GT$$GT$$GT$$GT$::h5c3ea81a9da75184 (.llvm.8624472363653721913)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-51019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8b538ffff","size":5,"soft_fail":false,"target_va":5378121440,"va":5378172454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$iced_runtime..command..action..Action$LT$tg_iced..dialog..Message$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-53602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e89e2effff","size":5,"soft_fail":false,"target_va":5378120736,"va":5378174333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-44379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8a552ffff","size":5,"soft_fail":false,"target_va":5378128096,"va":5378172470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..Command$LT$tg_iced..dialog..Message$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-49525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e88b3effff","size":5,"soft_fail":false,"target_va":5378125456,"va":5378174976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$tg_iced..dialog..Message$GT$$GT$::haef27fd213e640b1 (.llvm.8624472363653721913)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-45878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8ca4cffff","size":5,"soft_fail":false,"target_va":5378126272,"va":5378172145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..program..state..State$LT$tg_iced..dialog..Controls$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-46606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8f249ffff","size":5,"soft_fail":false,"target_va":5378125792,"va":5378172393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_tiny_skia..window..compositor..Surface$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-51170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e81e38ffff","size":5,"soft_fail":false,"target_va":5378124144,"va":5378175309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<tg_iced::dialog::Message> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8f0ebffff","size":5,"soft_fail":false,"target_va":5378166896,"va":5378172027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5214310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e89a6fb0ff","size":5,"soft_fail":false,"target_va":5372954416,"va":5378168721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10577602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8c266a100","size":5,"soft_fail":false,"target_va":5388746704,"va":5378169097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10576006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e88660a100","size":5,"soft_fail":false,"target_va":5388747264,"va":5378171253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5303577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8e712afff","size":5,"soft_fail":false,"target_va":5372865200,"va":5378168772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5305062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e81a0dafff","size":5,"soft_fail":false,"target_va":5372864528,"va":5378169585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24806674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e812857a01","size":5,"soft_fail":false,"target_va":5402975328,"va":5378168649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24807005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e85d867a01","size":5,"soft_fail":false,"target_va":5402975696,"va":5378168686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-32247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e80982ffff","size":5,"soft_fail":false,"target_va":5378137248,"va":5378169490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-25111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e8e99dffff","size":5,"soft_fail":false,"target_va":5378144064,"va":5378169170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4915440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378168416,"raw_hex":"e810ffb4ff","size":5,"soft_fail":false,"target_va":5373253168,"va":5378168603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*33117121(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378168416,"raw_hex":"ff15c153f901","size":6,"soft_fail":false,"va":5378168545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":786,"record_offset":3528,"parent_record_offset":3384,"ancestor_record_offset":3384,"inlinee":45686,"source_line":28,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::gui_runner::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":786,"record_offset":3552,"parent_record_offset":3384,"ancestor_record_offset":3384,"inlinee":12801,"source_line":46,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":786,"record_offset":4936,"parent_record_offset":4912,"ancestor_record_offset":3384,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":786,"record_offset":4960,"parent_record_offset":4936,"ancestor_record_offset":3384,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":786,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":3384,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":786,"record_offset":4880,"parent_record_offset":4844,"ancestor_record_offset":3384,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":786,"record_offset":4912,"parent_record_offset":4880,"ancestor_record_offset":3384,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":786,"record_offset":6096,"parent_record_offset":6004,"ancestor_record_offset":3384,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":786,"record_offset":4844,"parent_record_offset":4808,"ancestor_record_offset":3384,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":786,"record_offset":8980,"parent_record_offset":8952,"ancestor_record_offset":3384,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":786,"record_offset":6032,"parent_record_offset":6004,"ancestor_record_offset":3384,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":786,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":3384,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":786,"record_offset":8280,"parent_record_offset":8244,"ancestor_record_offset":3384,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":786,"record_offset":4684,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":786,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":3384,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":786,"record_offset":5056,"parent_record_offset":5032,"ancestor_record_offset":3384,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":786,"record_offset":6540,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":786,"record_offset":5904,"parent_record_offset":5872,"ancestor_record_offset":3384,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":786,"record_offset":6768,"parent_record_offset":6728,"ancestor_record_offset":3384,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":786,"record_offset":6728,"parent_record_offset":6700,"ancestor_record_offset":3384,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":786,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":3384,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":786,"record_offset":6596,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":786,"record_offset":6840,"parent_record_offset":6816,"ancestor_record_offset":3384,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":786,"record_offset":9008,"parent_record_offset":8980,"ancestor_record_offset":3384,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":786,"record_offset":4032,"parent_record_offset":3864,"ancestor_record_offset":3384,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":786,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":3384,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":786,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":3384,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":786,"record_offset":6792,"parent_record_offset":6768,"ancestor_record_offset":3384,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":786,"record_offset":3944,"parent_record_offset":3920,"ancestor_record_offset":3384,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":786,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":3384,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":786,"record_offset":3920,"parent_record_offset":3864,"ancestor_record_offset":3384,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":786,"record_offset":6060,"parent_record_offset":6032,"ancestor_record_offset":3384,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":786,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":3384,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":786,"record_offset":5608,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":786,"record_offset":5672,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":786,"record_offset":4076,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":786,"record_offset":5544,"parent_record_offset":5516,"ancestor_record_offset":3384,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":786,"record_offset":6888,"parent_record_offset":6864,"ancestor_record_offset":3384,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":786,"record_offset":4252,"parent_record_offset":4224,"ancestor_record_offset":3384,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":786,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":3384,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":786,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":3384,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":786,"record_offset":6668,"parent_record_offset":6596,"ancestor_record_offset":3384,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":786,"record_offset":8312,"parent_record_offset":8280,"ancestor_record_offset":3384,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":786,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":3384,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":786,"record_offset":6636,"parent_record_offset":6596,"ancestor_record_offset":3384,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":786,"record_offset":4988,"parent_record_offset":4960,"ancestor_record_offset":3384,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":786,"record_offset":5196,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":786,"record_offset":4476,"parent_record_offset":4304,"ancestor_record_offset":3384,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":786,"record_offset":4564,"parent_record_offset":4536,"ancestor_record_offset":3384,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":786,"record_offset":4712,"parent_record_offset":4684,"ancestor_record_offset":3384,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":786,"record_offset":5092,"parent_record_offset":5056,"ancestor_record_offset":3384,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":786,"record_offset":4408,"parent_record_offset":4380,"ancestor_record_offset":3384,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":786,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":3384,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":786,"record_offset":4432,"parent_record_offset":4408,"ancestor_record_offset":3384,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":786,"record_offset":4500,"parent_record_offset":4476,"ancestor_record_offset":3384,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":786,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":3384,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":786,"record_offset":4588,"parent_record_offset":4564,"ancestor_record_offset":3384,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":786,"record_offset":5148,"parent_record_offset":5056,"ancestor_record_offset":3384,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/mouse/cursor.rs
//     evidence={"module":786,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":3384,"inlinee":12795,"source_line":7,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/command.rs
//     evidence={"module":786,"record_offset":6344,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":4466,"source_line":104,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/style/src/theme.rs
//     evidence={"module":786,"record_offset":5756,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":11172,"source_line":123,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":786,"record_offset":4772,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":7811,"source_line":31,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":786,"record_offset":4808,"parent_record_offset":4772,"ancestor_record_offset":3384,"inlinee":7680,"source_line":139,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":786,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":3384,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":786,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":3384,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":786,"record_offset":4276,"parent_record_offset":4252,"ancestor_record_offset":3384,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":786,"record_offset":4304,"parent_record_offset":4276,"ancestor_record_offset":3384,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":786,"record_offset":4536,"parent_record_offset":4276,"ancestor_record_offset":3384,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":786,"record_offset":4332,"parent_record_offset":4304,"ancestor_record_offset":3384,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::dialog::impl$2::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/dialog.rs
//     evidence={"module":786,"record_offset":5516,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":14761,"source_line":147,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":786,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":3384,"inlinee":12792,"source_line":59,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":786,"record_offset":5640,"parent_record_offset":5608,"ancestor_record_offset":3384,"inlinee":12793,"source_line":194,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform/pump_events.rs
//     evidence={"module":786,"record_offset":5580,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":12641,"source_line":183,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":786,"record_offset":3864,"parent_record_offset":3836,"ancestor_record_offset":3384,"inlinee":10542,"source_line":215,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":786,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":3384,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":786,"record_offset":3808,"parent_record_offset":3780,"ancestor_record_offset":3384,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":786,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":3384,"inlinee":8250,"source_line":452,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":786,"record_offset":5384,"parent_record_offset":5360,"ancestor_record_offset":3384,"inlinee":10528,"source_line":143,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":786,"record_offset":3780,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":786,"record_offset":4152,"parent_record_offset":4100,"ancestor_record_offset":3384,"inlinee":8246,"source_line":568,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":786,"record_offset":5312,"parent_record_offset":3552,"ancestor_record_offset":3384,"inlinee":10696,"source_line":940,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":786,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3384,"inlinee":8249,"source_line":569,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":786,"record_offset":3836,"parent_record_offset":3808,"ancestor_record_offset":3384,"inlinee":11163,"source_line":747,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":786,"record_offset":5360,"parent_record_offset":5336,"ancestor_record_offset":3384,"inlinee":10698,"source_line":942,"archive":"_pdb/A/modules/00786.jsonl.gz"}
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
// 首条位置: {"module":729,"record_offset":3420,"rva":9202592,"va":5377911712}
// 编译实例 RVAs: 0x8c6ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5006262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e84a9cb3ff","size":5,"soft_fail":false,"target_va":5372905968,"va":5377912225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10366919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8c72f9e00","size":5,"soft_fail":false,"target_va":5388279200,"va":5377912276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400056b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9187814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e81ace73ff","size":5,"soft_fail":false,"target_va":5368731312,"va":5377919121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5644473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e847dfa9ff","size":5,"soft_fail":false,"target_va":5372274416,"va":5377918884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036afd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5622650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e88634aaff","size":5,"soft_fail":false,"target_va":5372293072,"va":5377915717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370de0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5597579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e87596aaff","size":5,"soft_fail":false,"target_va":5372317152,"va":5377914726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140384450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5522169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e807bdabff","size":5,"soft_fail":false,"target_va":5372396624,"va":5377918788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5162437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e83b3ab1ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5377912128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5165058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8fe2fb1ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5377914765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412ae6c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10380768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8e0659e00","size":5,"soft_fail":false,"target_va":5388297920,"va":5377917147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25061783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e897697e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5377916356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-6260962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e81e77a0ff","size":5,"soft_fail":false,"target_va":5371654736,"va":5377915693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25062607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8cf6c7e01","size":5,"soft_fail":false,"target_va":5402978944,"va":5377916332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..collections..vec_deque..VecDeque$LT$alloc..vec..Vec$LT$iced_graphics..primitive..Primitive$LT$iced_tiny_skia..primitive..Custom$GT$$GT$$GT$$GT$::h2cf44eaf4c6a4ff1 (.llvm.12305981749508188794)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t10301717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e815319d00","size":5,"soft_fail":false,"target_va":5388217488,"va":5377915766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$iced_runtime..command..action..Action$LT$tg_integrity_report..controls..Message$GT$$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10299582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8be289d00","size":5,"soft_fail":false,"target_va":5388217200,"va":5377917613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_graphics..renderer..Renderer$LT$iced_tiny_skia..backend..Backend$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10303621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e885389d00","size":5,"soft_fail":false,"target_va":5388219408,"va":5377915782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..Command$LT$tg_integrity_report..controls..Message$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10303595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e86b389d00","size":5,"soft_fail":false,"target_va":5388221856,"va":5377918256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$tg_integrity_report..controls..Message$GT$$GT$::hf6ec3c170776898e (.llvm.12305981749508188794)
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t10299210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e84a279d00","size":5,"soft_fail":false,"target_va":5388214688,"va":5377915473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..program..state..State$LT$tg_integrity_report..controls..Controls$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t10298546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8b2249d00","size":5,"soft_fail":false,"target_va":5388214256,"va":5377915705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_tiny_skia..window..compositor..Surface$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10300654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8ee2c9d00","size":5,"soft_fail":false,"target_va":5388219248,"va":5377918589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<tg_integrity_report::controls::Message> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8d9ebffff","size":5,"soft_fail":false,"target_va":5377910192,"va":5377915346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4957602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e85e5ab4ff","size":5,"soft_fail":false,"target_va":5372954416,"va":5377912013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10375713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e821529e00","size":5,"soft_fail":false,"target_va":5388288096,"va":5377912378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10374118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8e64b9e00","size":5,"soft_fail":false,"target_va":5388288656,"va":5377914533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5046869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8abfdb2ff","size":5,"soft_fail":false,"target_va":5372865200,"va":5377912064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5048342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8eaf7b2ff","size":5,"soft_fail":false,"target_va":5372864528,"va":5377912865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25063383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8d76f7e01","size":5,"soft_fail":false,"target_va":5402975328,"va":5377911940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t25063714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e822717e01","size":5,"soft_fail":false,"target_va":5402975696,"va":5377911977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10379867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e85b629e00","size":5,"soft_fail":false,"target_va":5388292640,"va":5377912768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10381960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8886a9e00","size":5,"soft_fail":false,"target_va":5388294416,"va":5377912451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4658731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377911712,"raw_hex":"e8d5e9b8ff","size":5,"soft_fail":false,"target_va":5373253168,"va":5377911894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*33373832(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5377911712,"raw_hex":"ff15883efd01","size":6,"soft_fail":false,"va":5377911834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tg_iced::gui_runner::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":729,"record_offset":3580,"parent_record_offset":3420,"ancestor_record_offset":3420,"inlinee":45686,"source_line":28,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::gui_runner::run_gui_with_options
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":729,"record_offset":3604,"parent_record_offset":3420,"ancestor_record_offset":3420,"inlinee":12801,"source_line":46,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":3420,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":3420,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":6152,"parent_record_offset":6128,"ancestor_record_offset":3420,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":4920,"parent_record_offset":4884,"ancestor_record_offset":3420,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":4952,"parent_record_offset":4920,"ancestor_record_offset":3420,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":6128,"parent_record_offset":6036,"ancestor_record_offset":3420,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":729,"record_offset":4884,"parent_record_offset":4848,"ancestor_record_offset":3420,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":729,"record_offset":9012,"parent_record_offset":8984,"ancestor_record_offset":3420,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":729,"record_offset":6064,"parent_record_offset":6036,"ancestor_record_offset":3420,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":729,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":3420,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":729,"record_offset":8312,"parent_record_offset":8276,"ancestor_record_offset":3420,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":729,"record_offset":4724,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":729,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3420,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":729,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":3420,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":729,"record_offset":6572,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":729,"record_offset":5936,"parent_record_offset":5904,"ancestor_record_offset":3420,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":729,"record_offset":6800,"parent_record_offset":6760,"ancestor_record_offset":3420,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":729,"record_offset":6760,"parent_record_offset":6732,"ancestor_record_offset":3420,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":729,"record_offset":7024,"parent_record_offset":7000,"ancestor_record_offset":3420,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":729,"record_offset":6628,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":729,"record_offset":6872,"parent_record_offset":6848,"ancestor_record_offset":3420,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":729,"record_offset":9040,"parent_record_offset":9012,"ancestor_record_offset":3420,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":729,"record_offset":4072,"parent_record_offset":3904,"ancestor_record_offset":3420,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":729,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":3420,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":729,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":3420,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":729,"record_offset":6824,"parent_record_offset":6800,"ancestor_record_offset":3420,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":729,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3420,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":729,"record_offset":9064,"parent_record_offset":9040,"ancestor_record_offset":3420,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":729,"record_offset":3960,"parent_record_offset":3904,"ancestor_record_offset":3420,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":729,"record_offset":6092,"parent_record_offset":6064,"ancestor_record_offset":3420,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":729,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":3420,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":729,"record_offset":5640,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":729,"record_offset":5704,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":729,"record_offset":4116,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":729,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":3420,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":729,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":3420,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":729,"record_offset":4292,"parent_record_offset":4264,"ancestor_record_offset":3420,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":729,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":3420,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":729,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3420,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":729,"record_offset":6700,"parent_record_offset":6628,"ancestor_record_offset":3420,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":729,"record_offset":8344,"parent_record_offset":8312,"ancestor_record_offset":3420,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":729,"record_offset":6896,"parent_record_offset":6872,"ancestor_record_offset":3420,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":729,"record_offset":6668,"parent_record_offset":6628,"ancestor_record_offset":3420,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":729,"record_offset":5028,"parent_record_offset":5000,"ancestor_record_offset":3420,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":729,"record_offset":5236,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4516,"parent_record_offset":4344,"ancestor_record_offset":3420,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4604,"parent_record_offset":4576,"ancestor_record_offset":3420,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4752,"parent_record_offset":4724,"ancestor_record_offset":3420,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":5132,"parent_record_offset":5096,"ancestor_record_offset":3420,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4448,"parent_record_offset":4420,"ancestor_record_offset":3420,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":3420,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":3420,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":3420,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":5156,"parent_record_offset":5132,"ancestor_record_offset":3420,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4628,"parent_record_offset":4604,"ancestor_record_offset":3420,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":5188,"parent_record_offset":5096,"ancestor_record_offset":3420,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/mouse/cursor.rs
//     evidence={"module":729,"record_offset":5752,"parent_record_offset":5728,"ancestor_record_offset":3420,"inlinee":12795,"source_line":7,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/command.rs
//     evidence={"module":729,"record_offset":6376,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":4466,"source_line":104,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/style/src/theme.rs
//     evidence={"module":729,"record_offset":5788,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":11172,"source_line":123,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":729,"record_offset":4812,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":7811,"source_line":31,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":729,"record_offset":4848,"parent_record_offset":4812,"ancestor_record_offset":3420,"inlinee":7680,"source_line":139,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":729,"record_offset":4420,"parent_record_offset":4396,"ancestor_record_offset":3420,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":729,"record_offset":4396,"parent_record_offset":4372,"ancestor_record_offset":3420,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":729,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":3420,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":729,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":3420,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":729,"record_offset":4576,"parent_record_offset":4316,"ancestor_record_offset":3420,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":729,"record_offset":4372,"parent_record_offset":4344,"ancestor_record_offset":3420,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":729,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":3420,"inlinee":12792,"source_line":59,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":729,"record_offset":5672,"parent_record_offset":5640,"ancestor_record_offset":3420,"inlinee":12793,"source_line":194,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] tg_integrity_report::controls::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-integrity-report/src/controls.rs
//     evidence={"module":729,"record_offset":5556,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":45354,"source_line":34,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform/pump_events.rs
//     evidence={"module":729,"record_offset":5612,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":12641,"source_line":183,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":729,"record_offset":3904,"parent_record_offset":3876,"ancestor_record_offset":3420,"inlinee":10542,"source_line":215,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":729,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":3420,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":729,"record_offset":3848,"parent_record_offset":3820,"ancestor_record_offset":3420,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":729,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":3420,"inlinee":8250,"source_line":452,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":729,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":3420,"inlinee":10528,"source_line":143,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":3820,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":4192,"parent_record_offset":4140,"ancestor_record_offset":3420,"inlinee":8246,"source_line":568,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":5352,"parent_record_offset":3604,"ancestor_record_offset":3420,"inlinee":10696,"source_line":940,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":3420,"inlinee":8249,"source_line":569,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":3420,"inlinee":11163,"source_line":747,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":3420,"inlinee":10698,"source_line":942,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui_with_options
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3201
// 首条位置: {"module":729,"inlinee":12801}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":729,"record_offset":4724,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":729,"record_offset":4696,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":729,"record_offset":6572,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":729,"record_offset":6628,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":729,"record_offset":5640,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":729,"record_offset":5704,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":729,"record_offset":4116,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=60
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":729,"record_offset":5904,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":729,"record_offset":5236,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/command.rs
//     evidence={"module":729,"record_offset":6376,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":4466,"source_line":104,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/style/src/theme.rs
//     evidence={"module":729,"record_offset":5788,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":11172,"source_line":123,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":729,"record_offset":4812,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":7811,"source_line":31,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] tg_gpu_selector::gpu_selection::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/gpu_selection.rs
//     evidence={"module":779,"record_offset":14276,"parent_record_offset":12312,"ancestor_record_offset":12312,"inlinee":46038,"source_line":57,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_direct] tg_gpu_selector::vulkan_config::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":779,"record_offset":7416,"parent_record_offset":3600,"ancestor_record_offset":3600,"inlinee":46034,"source_line":47,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_direct] tg_iced::dialog::impl$2::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/dialog.rs
//     evidence={"module":786,"record_offset":5516,"parent_record_offset":3552,"ancestor_record_offset":3552,"inlinee":14761,"source_line":147,"archive":"_pdb/A/modules/00786.jsonl.gz"}
//   [pdb_inline_direct] tg_integrity_report::controls::impl$0::program_outcome
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-integrity-report/src/controls.rs
//     evidence={"module":729,"record_offset":5556,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":45354,"source_line":34,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform/pump_events.rs
//     evidence={"module":729,"record_offset":5612,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":12641,"source_line":183,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":3820,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":5352,"parent_record_offset":3604,"ancestor_record_offset":3604,"inlinee":10696,"source_line":940,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":3604,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":3604,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":6152,"parent_record_offset":6128,"ancestor_record_offset":3604,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":4920,"parent_record_offset":4884,"ancestor_record_offset":3604,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":4952,"parent_record_offset":4920,"ancestor_record_offset":3604,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":729,"record_offset":6128,"parent_record_offset":6036,"ancestor_record_offset":3604,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":729,"record_offset":4884,"parent_record_offset":4848,"ancestor_record_offset":3604,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":729,"record_offset":9012,"parent_record_offset":8984,"ancestor_record_offset":3604,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":729,"record_offset":6064,"parent_record_offset":6036,"ancestor_record_offset":3604,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":729,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":3604,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":729,"record_offset":8312,"parent_record_offset":8276,"ancestor_record_offset":3604,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":729,"record_offset":5456,"parent_record_offset":5424,"ancestor_record_offset":3604,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":729,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3604,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":729,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":3604,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":729,"record_offset":5936,"parent_record_offset":5904,"ancestor_record_offset":3604,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":729,"record_offset":6800,"parent_record_offset":6760,"ancestor_record_offset":3604,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":729,"record_offset":6760,"parent_record_offset":6732,"ancestor_record_offset":3604,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":729,"record_offset":7024,"parent_record_offset":7000,"ancestor_record_offset":3604,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":729,"record_offset":6872,"parent_record_offset":6848,"ancestor_record_offset":3604,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":729,"record_offset":9040,"parent_record_offset":9012,"ancestor_record_offset":3604,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":729,"record_offset":4072,"parent_record_offset":3904,"ancestor_record_offset":3604,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":729,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":3604,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":729,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":3604,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":729,"record_offset":6824,"parent_record_offset":6800,"ancestor_record_offset":3604,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":729,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3604,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":729,"record_offset":9064,"parent_record_offset":9040,"ancestor_record_offset":3604,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":729,"record_offset":3960,"parent_record_offset":3904,"ancestor_record_offset":3604,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":729,"record_offset":6092,"parent_record_offset":6064,"ancestor_record_offset":3604,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":729,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":3604,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":729,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":3604,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":729,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":3604,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=168
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":729,"record_offset":4292,"parent_record_offset":4264,"ancestor_record_offset":3604,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":729,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":3604,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":729,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3604,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":729,"record_offset":6700,"parent_record_offset":6628,"ancestor_record_offset":3604,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":729,"record_offset":8344,"parent_record_offset":8312,"ancestor_record_offset":3604,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":729,"record_offset":6896,"parent_record_offset":6872,"ancestor_record_offset":3604,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":729,"record_offset":6668,"parent_record_offset":6628,"ancestor_record_offset":3604,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":729,"record_offset":5028,"parent_record_offset":5000,"ancestor_record_offset":3604,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4516,"parent_record_offset":4344,"ancestor_record_offset":3604,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4604,"parent_record_offset":4576,"ancestor_record_offset":3604,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4752,"parent_record_offset":4724,"ancestor_record_offset":3604,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":5132,"parent_record_offset":5096,"ancestor_record_offset":3604,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4448,"parent_record_offset":4420,"ancestor_record_offset":3604,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":3604,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":3604,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":3604,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":5156,"parent_record_offset":5132,"ancestor_record_offset":3604,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":4628,"parent_record_offset":4604,"ancestor_record_offset":3604,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":729,"record_offset":5188,"parent_record_offset":5096,"ancestor_record_offset":3604,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/mouse/cursor.rs
//     evidence={"module":729,"record_offset":5752,"parent_record_offset":5728,"ancestor_record_offset":3604,"inlinee":12795,"source_line":7,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":729,"record_offset":4848,"parent_record_offset":4812,"ancestor_record_offset":3604,"inlinee":7680,"source_line":139,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":779,"record_offset":7500,"parent_record_offset":7468,"ancestor_record_offset":3600,"inlinee":10594,"source_line":87,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":729,"record_offset":4420,"parent_record_offset":4396,"ancestor_record_offset":3604,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":729,"record_offset":4396,"parent_record_offset":4372,"ancestor_record_offset":3604,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":729,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":3604,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":729,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":3604,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":729,"record_offset":4576,"parent_record_offset":4316,"ancestor_record_offset":3604,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":729,"record_offset":4372,"parent_record_offset":4344,"ancestor_record_offset":3604,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] tg_gpu_selector::vulkan_config::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-gpu-selector/src/vulkan_config.rs
//     evidence={"module":779,"record_offset":7468,"parent_record_offset":7440,"ancestor_record_offset":3600,"inlinee":46036,"source_line":16,"archive":"_pdb/A/modules/00779.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":729,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":3604,"inlinee":12792,"source_line":59,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":729,"record_offset":5672,"parent_record_offset":5640,"ancestor_record_offset":3604,"inlinee":12793,"source_line":194,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":729,"record_offset":3904,"parent_record_offset":3876,"ancestor_record_offset":3604,"inlinee":10542,"source_line":215,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":729,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":3604,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":729,"record_offset":3848,"parent_record_offset":3820,"ancestor_record_offset":3604,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":729,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":3604,"inlinee":8250,"source_line":452,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":729,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":3604,"inlinee":10528,"source_line":143,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":4192,"parent_record_offset":4140,"ancestor_record_offset":3604,"inlinee":8246,"source_line":568,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":3604,"inlinee":8249,"source_line":569,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":3604,"inlinee":11163,"source_line":747,"archive":"_pdb/A/modules/00729.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":729,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":3604,"inlinee":10698,"source_line":942,"archive":"_pdb/A/modules/00729.jsonl.gz"}
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
// 首条位置: {"module":202,"record_offset":7384,"rva":3194496,"va":5371903616}
// 编译实例 RVAs: 0x30be80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_renderer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1001822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e85e490f00","size":5,"soft_fail":false,"target_va":5372905968,"va":5371904141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$iced_tiny_skia..window..compositor..Compositor$u20$as$u20$iced_graphics..compositor..Compositor$GT$::create_surface
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2703605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e80bbfd6ff","size":5,"soft_fail":false,"target_va":5369200592,"va":5371904192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140323b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t91747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e863660100","size":5,"soft_fail":false,"target_va":5372001104,"va":5371909352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140364590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t356030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8be6e0500","size":5,"soft_fail":false,"target_va":5372265872,"va":5371909837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t363287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8178b0500","size":5,"soft_fail":false,"target_va":5372274416,"va":5371911124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403693e0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t378085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8e5c40500","size":5,"soft_fail":false,"target_va":5372285920,"va":5371907830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036afd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t385286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e806e10500","size":5,"soft_fail":false,"target_va":5372293072,"va":5371907781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370de0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t410557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8bd430600","size":5,"soft_fail":false,"target_va":5372317152,"va":5371906590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140374040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t418682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e87a630600","size":5,"soft_fail":false,"target_va":5372330048,"va":5371911361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t479118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e88e4f0700","size":5,"soft_fail":false,"target_va":5372389952,"va":5371910829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140384450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t485591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8d7680700","size":5,"soft_fail":false,"target_va":5372396624,"va":5371911028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038f610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t531618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8a21c0800","size":5,"soft_fail":false,"target_va":5372442128,"va":5371910505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403936f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t550962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e832680800","size":5,"soft_fail":false,"target_va":5372458736,"va":5371907769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140394040
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t553802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e84a730800","size":5,"soft_fail":false,"target_va":5372461120,"va":5371907313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140394b10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t556037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8057c0800","size":5,"soft_fail":false,"target_va":5372463888,"va":5371907846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t845644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e84ce70c00","size":5,"soft_fail":false,"target_va":5372749696,"va":5371904047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t843070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e83edd0c00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371906637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403e0d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t868135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8273f0d00","size":5,"soft_fail":false,"target_va":5372775696,"va":5371907556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403e0d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t868335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8ef3f0d00","size":5,"soft_fail":false,"target_va":5372775728,"va":5371907388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gui_runner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31689135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8af89e301","size":5,"soft_fail":false,"target_va":5403593360,"va":5371904220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31069699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e80316da01","size":5,"soft_fail":false,"target_va":5402978144,"va":5371908440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-602243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e87dcff6ff","size":5,"soft_fail":false,"target_va":5371305232,"va":5371907470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-253026","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e89e23fcff","size":5,"soft_fail":false,"target_va":5371654736,"va":5371907757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31070523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e83b19da01","size":5,"soft_fail":false,"target_va":5402978944,"va":5371908416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$iced_runtime..command..action..Action$LT$crash_reporter_gui..Message$GT$$GT$::h12d7ab0c53151d27 (.llvm.5540620226090063491)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3185271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e88965cfff","size":5,"soft_fail":false,"target_va":5368723968,"va":5371909234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<iced_runtime::command::action::Action<enum2$<crash_reporter_gui::Message> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e81cecffff","size":5,"soft_fail":false,"target_va":5371902096,"va":5371907183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_graphics::viewport::Viewport::with_physical_size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1050479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e86f071000","size":5,"soft_fail":false,"target_va":5372954416,"va":5371903932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t92795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e87b6a0100","size":5,"soft_fail":false,"target_va":5371997072,"va":5371904272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_runtime::program::state::State$LT$P$GT$::update
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t91202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e842640100","size":5,"soft_fail":false,"target_va":5371997632,"va":5371906425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::clipboard::Clipboard::connect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t961212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8bcaa0e00","size":5,"soft_fail":false,"target_va":5372865200,"va":5371903983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] iced_winit::conversion::cursor_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t959779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e823a50e00","size":5,"soft_fail":false,"target_va":5372864528,"va":5371904744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31071469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8ed1cda01","size":5,"soft_fail":false,"target_va":5402975328,"va":5371903854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31071794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8321eda01","size":5,"soft_fail":false,"target_va":5402975696,"va":5371903897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::pump_events
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t287279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e82f620400","size":5,"soft_fail":false,"target_va":5372191952,"va":5371904668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t293442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8427a0400","size":5,"soft_fail":false,"target_va":5372197792,"va":5371904345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1349358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371903616,"raw_hex":"e8ee961400","size":5,"soft_fail":false,"target_va":5373253168,"va":5371903805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*39381942(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5371903616,"raw_hex":"ff15b6eb5802","size":6,"soft_fail":false,"va":5371903724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":8508,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":8480,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":10284,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":202,"record_offset":10340,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":202,"record_offset":9368,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":202,"record_offset":9432,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":202,"record_offset":7896,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":9632,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":202,"record_offset":9020,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::command::Command::actions
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/command.rs
//     evidence={"module":202,"record_offset":10088,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":4466,"source_line":104,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::queue_message
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/program/state.rs
//     evidence={"module":202,"record_offset":11192,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":12796,"source_line":69,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/style/src/theme.rs
//     evidence={"module":202,"record_offset":9516,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":11172,"source_line":123,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::window::compositor::impl$0::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":202,"record_offset":8596,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":7811,"source_line":31,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] winit::platform::pump_events::impl$0::pump_events
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform/pump_events.rs
//     evidence={"module":202,"record_offset":9340,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":12641,"source_line":183,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":202,"record_offset":7600,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":202,"record_offset":9136,"parent_record_offset":7384,"ancestor_record_offset":7384,"inlinee":10696,"source_line":940,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":8760,"parent_record_offset":8736,"ancestor_record_offset":7384,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":7384,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":9864,"parent_record_offset":9840,"ancestor_record_offset":7384,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":8704,"parent_record_offset":8668,"ancestor_record_offset":7384,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":8736,"parent_record_offset":8704,"ancestor_record_offset":7384,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":202,"record_offset":9840,"parent_record_offset":9748,"ancestor_record_offset":7384,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":202,"record_offset":8668,"parent_record_offset":8632,"ancestor_record_offset":7384,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":202,"record_offset":12900,"parent_record_offset":12872,"ancestor_record_offset":7384,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":202,"record_offset":9776,"parent_record_offset":9748,"ancestor_record_offset":7384,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":202,"record_offset":11300,"parent_record_offset":11268,"ancestor_record_offset":7384,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":202,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":7384,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":12200,"parent_record_offset":12164,"ancestor_record_offset":7384,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":9240,"parent_record_offset":9208,"ancestor_record_offset":7384,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":7948,"parent_record_offset":7924,"ancestor_record_offset":7384,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":8880,"parent_record_offset":8856,"ancestor_record_offset":7384,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":11268,"parent_record_offset":11220,"ancestor_record_offset":7384,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":11220,"parent_record_offset":11192,"ancestor_record_offset":7384,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":202,"record_offset":9660,"parent_record_offset":9632,"ancestor_record_offset":7384,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":202,"record_offset":10512,"parent_record_offset":10472,"ancestor_record_offset":7384,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":202,"record_offset":10472,"parent_record_offset":10444,"ancestor_record_offset":7384,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":202,"record_offset":10736,"parent_record_offset":10712,"ancestor_record_offset":7384,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":202,"record_offset":10584,"parent_record_offset":10560,"ancestor_record_offset":7384,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":202,"record_offset":12928,"parent_record_offset":12900,"ancestor_record_offset":7384,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":202,"record_offset":7852,"parent_record_offset":7684,"ancestor_record_offset":7384,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":202,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":7384,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":202,"record_offset":10560,"parent_record_offset":10536,"ancestor_record_offset":7384,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":202,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":7384,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":202,"record_offset":7764,"parent_record_offset":7740,"ancestor_record_offset":7384,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":202,"record_offset":12952,"parent_record_offset":12928,"ancestor_record_offset":7384,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":202,"record_offset":7740,"parent_record_offset":7684,"ancestor_record_offset":7384,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":202,"record_offset":9804,"parent_record_offset":9776,"ancestor_record_offset":7384,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":202,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":7384,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":202,"record_offset":10632,"parent_record_offset":10608,"ancestor_record_offset":7384,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":8076,"parent_record_offset":8048,"ancestor_record_offset":7384,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":202,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":7384,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":202,"record_offset":7788,"parent_record_offset":7764,"ancestor_record_offset":7384,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":202,"record_offset":10412,"parent_record_offset":10340,"ancestor_record_offset":7384,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":202,"record_offset":12232,"parent_record_offset":12200,"ancestor_record_offset":7384,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":202,"record_offset":10608,"parent_record_offset":10584,"ancestor_record_offset":7384,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":202,"record_offset":10380,"parent_record_offset":10340,"ancestor_record_offset":7384,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":8812,"parent_record_offset":8784,"ancestor_record_offset":7384,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":11332,"parent_record_offset":11220,"ancestor_record_offset":7384,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8300,"parent_record_offset":8128,"ancestor_record_offset":7384,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8388,"parent_record_offset":8360,"ancestor_record_offset":7384,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8536,"parent_record_offset":8508,"ancestor_record_offset":7384,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8916,"parent_record_offset":8880,"ancestor_record_offset":7384,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8232,"parent_record_offset":8204,"ancestor_record_offset":7384,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8560,"parent_record_offset":8536,"ancestor_record_offset":7384,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8256,"parent_record_offset":8232,"ancestor_record_offset":7384,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":7384,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8940,"parent_record_offset":8916,"ancestor_record_offset":7384,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":7384,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8972,"parent_record_offset":8880,"ancestor_record_offset":7384,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::cursor::Cursor::Available::constructor$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/mouse/cursor.rs
//     evidence={"module":202,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":7384,"inlinee":12795,"source_line":7,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":202,"record_offset":8632,"parent_record_offset":8596,"ancestor_record_offset":7384,"inlinee":7680,"source_line":139,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":202,"record_offset":8204,"parent_record_offset":8180,"ancestor_record_offset":7384,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":202,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":7384,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":202,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":7384,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":202,"record_offset":8128,"parent_record_offset":8100,"ancestor_record_offset":7384,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":202,"record_offset":8360,"parent_record_offset":8100,"ancestor_record_offset":7384,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":202,"record_offset":8156,"parent_record_offset":8128,"ancestor_record_offset":7384,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":202,"record_offset":7924,"parent_record_offset":7896,"ancestor_record_offset":7384,"inlinee":12792,"source_line":59,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":202,"record_offset":9400,"parent_record_offset":9368,"ancestor_record_offset":7384,"inlinee":12793,"source_line":194,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":202,"record_offset":7684,"parent_record_offset":7656,"ancestor_record_offset":7384,"inlinee":10542,"source_line":215,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":202,"record_offset":9160,"parent_record_offset":9136,"ancestor_record_offset":7384,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":202,"record_offset":7628,"parent_record_offset":7600,"ancestor_record_offset":7384,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":202,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":7384,"inlinee":8250,"source_line":452,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":202,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":7384,"inlinee":10528,"source_line":143,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":202,"record_offset":7976,"parent_record_offset":7924,"ancestor_record_offset":7384,"inlinee":8246,"source_line":568,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":202,"record_offset":8024,"parent_record_offset":8000,"ancestor_record_offset":7384,"inlinee":8249,"source_line":569,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":202,"record_offset":7656,"parent_record_offset":7628,"ancestor_record_offset":7384,"inlinee":11163,"source_line":747,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":202,"record_offset":9184,"parent_record_offset":9160,"ancestor_record_offset":7384,"inlinee":10698,"source_line":942,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui_with_options::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31F8
// 首条位置: {"module":202,"inlinee":12792}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":202,"record_offset":7948,"parent_record_offset":7924,"ancestor_record_offset":7924,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::scale_factor
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":202,"record_offset":7976,"parent_record_offset":7924,"ancestor_record_offset":7924,"inlinee":8246,"source_line":568,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":202,"record_offset":8076,"parent_record_offset":8048,"ancestor_record_offset":7924,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8300,"parent_record_offset":8128,"ancestor_record_offset":7924,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8388,"parent_record_offset":8360,"ancestor_record_offset":7924,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8232,"parent_record_offset":8204,"ancestor_record_offset":7924,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8256,"parent_record_offset":8232,"ancestor_record_offset":7924,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":7924,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":202,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":7924,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":202,"record_offset":8204,"parent_record_offset":8180,"ancestor_record_offset":7924,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":202,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":7924,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":202,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":7924,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":202,"record_offset":8128,"parent_record_offset":8100,"ancestor_record_offset":7924,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":202,"record_offset":8360,"parent_record_offset":8100,"ancestor_record_offset":7924,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":202,"record_offset":8156,"parent_record_offset":8128,"ancestor_record_offset":7924,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":202,"record_offset":8000,"parent_record_offset":7976,"ancestor_record_offset":7924,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::scale_factor
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":202,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":7924,"inlinee":8250,"source_line":452,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::scale_factor::closure$0
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":202,"record_offset":8024,"parent_record_offset":8000,"ancestor_record_offset":7924,"inlinee":8249,"source_line":569,"archive":"_pdb/A/modules/00202.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui_with_options::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B95
// 首条位置: {"module":140,"inlinee":11157}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":140,"record_offset":11488,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":140,"record_offset":11432,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":140,"record_offset":10688,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":140,"record_offset":11396,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::debug::Debug::overlay
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/debug/null.rs
//     evidence={"module":140,"record_offset":11252,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":11174,"source_line":44,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] iced_runtime::program::state::State::queue_event
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/runtime/src/program/state.rs
//     evidence={"module":140,"record_offset":11076,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":11168,"source_line":62,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] iced_tiny_skia::window::compositor::impl$0::present
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":140,"record_offset":11308,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":7812,"source_line":98,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] iced_winit::conversion::mouse_interaction
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/winit/src/conversion.rs
//     evidence={"module":140,"record_offset":11516,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":11178,"source_line":393,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::exit
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":140,"record_offset":10880,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":11164,"source_line":363,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::inner_size
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":140,"record_offset":10396,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":11161,"source_line":746,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::request_redraw
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":140,"record_offset":9208,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":11158,"source_line":599,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_cursor_icon
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":140,"record_offset":11544,"parent_record_offset":8980,"ancestor_record_offset":8980,"inlinee":11179,"source_line":1344,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":140,"record_offset":11184,"parent_record_offset":11152,"ancestor_record_offset":8980,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":140,"record_offset":9392,"parent_record_offset":9368,"ancestor_record_offset":8980,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":140,"record_offset":9368,"parent_record_offset":9324,"ancestor_record_offset":8980,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":140,"record_offset":11152,"parent_record_offset":11104,"ancestor_record_offset":8980,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":140,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":8980,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":140,"record_offset":11104,"parent_record_offset":11076,"ancestor_record_offset":8980,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":140,"record_offset":10976,"parent_record_offset":10952,"ancestor_record_offset":8980,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":140,"record_offset":10952,"parent_record_offset":10928,"ancestor_record_offset":8980,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":140,"record_offset":10644,"parent_record_offset":10480,"ancestor_record_offset":8980,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":140,"record_offset":10604,"parent_record_offset":10580,"ancestor_record_offset":8980,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":140,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":8980,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":140,"record_offset":11000,"parent_record_offset":10976,"ancestor_record_offset":8980,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":140,"record_offset":10532,"parent_record_offset":10480,"ancestor_record_offset":8980,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":140,"record_offset":9960,"parent_record_offset":9324,"ancestor_record_offset":8980,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":140,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":8980,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":140,"record_offset":9416,"parent_record_offset":9392,"ancestor_record_offset":8980,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":140,"record_offset":11024,"parent_record_offset":11000,"ancestor_record_offset":8980,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":140,"record_offset":9568,"parent_record_offset":9508,"ancestor_record_offset":8980,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":140,"record_offset":9916,"parent_record_offset":9324,"ancestor_record_offset":8980,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":9836,"parent_record_offset":9808,"ancestor_record_offset":8980,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":10196,"parent_record_offset":10020,"ancestor_record_offset":8980,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":9508,"parent_record_offset":9484,"ancestor_record_offset":8980,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":10284,"parent_record_offset":10256,"ancestor_record_offset":8980,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":9740,"parent_record_offset":9712,"ancestor_record_offset":8980,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":9540,"parent_record_offset":9508,"ancestor_record_offset":8980,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":9764,"parent_record_offset":9740,"ancestor_record_offset":8980,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":10220,"parent_record_offset":10196,"ancestor_record_offset":8980,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":140,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":8980,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] iced_core::mouse::interaction::impl$3::eq
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/core/src/mouse/interaction.rs
//     evidence={"module":140,"record_offset":11456,"parent_record_offset":11432,"ancestor_record_offset":8980,"inlinee":11176,"source_line":2,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] iced_graphics::renderer::Renderer::with_primitives
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/graphics/src/renderer.rs
//     evidence={"module":140,"record_offset":11332,"parent_record_offset":11308,"ancestor_record_offset":8980,"inlinee":7682,"source_line":52,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/style/src/theme.rs
//     evidence={"module":140,"record_offset":10756,"parent_record_offset":10728,"ancestor_record_offset":8980,"inlinee":11172,"source_line":123,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] iced_tiny_skia::window::compositor::impl$0::present::closure$0
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/tiny_skia/src/window/compositor.rs
//     evidence={"module":140,"record_offset":11356,"parent_record_offset":11332,"ancestor_record_offset":8980,"inlinee":7684,"source_line":106,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":140,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":8980,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":140,"record_offset":9688,"parent_record_offset":9664,"ancestor_record_offset":8980,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":140,"record_offset":9452,"parent_record_offset":9324,"ancestor_record_offset":8980,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":140,"record_offset":9604,"parent_record_offset":9452,"ancestor_record_offset":8980,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":140,"record_offset":9988,"parent_record_offset":9960,"ancestor_record_offset":8980,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":140,"record_offset":10020,"parent_record_offset":9988,"ancestor_record_offset":8980,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":140,"record_offset":9808,"parent_record_offset":9632,"ancestor_record_offset":8980,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":140,"record_offset":9632,"parent_record_offset":9604,"ancestor_record_offset":8980,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":140,"record_offset":9484,"parent_record_offset":9452,"ancestor_record_offset":8980,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":140,"record_offset":10256,"parent_record_offset":9988,"ancestor_record_offset":8980,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":140,"record_offset":9664,"parent_record_offset":9632,"ancestor_record_offset":8980,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] tg_iced::gui_runner::run_gui_with_options::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-iced/src/gui_runner.rs
//     evidence={"module":140,"record_offset":10728,"parent_record_offset":10688,"ancestor_record_offset":8980,"inlinee":11170,"source_line":131,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::exit
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/event_loop.rs
//     evidence={"module":140,"record_offset":10904,"parent_record_offset":10880,"ancestor_record_offset":8980,"inlinee":11165,"source_line":525,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::runner::EventLoopRunner::set_exit_code
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/event_loop/runner.rs
//     evidence={"module":140,"record_offset":10928,"parent_record_offset":10904,"ancestor_record_offset":8980,"inlinee":11166,"source_line":158,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":140,"record_offset":10352,"parent_record_offset":9324,"ancestor_record_offset":8980,"inlinee":8244,"source_line":344,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::inner_size
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":140,"record_offset":10480,"parent_record_offset":10452,"ancestor_record_offset":8980,"inlinee":10542,"source_line":215,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":140,"record_offset":9248,"parent_record_offset":9208,"ancestor_record_offset":8980,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":140,"record_offset":10424,"parent_record_offset":10396,"ancestor_record_offset":8980,"inlinee":8247,"source_line":113,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":140,"record_offset":9324,"parent_record_offset":9288,"ancestor_record_offset":8980,"inlinee":8233,"source_line":160,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_cursor_icon
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":140,"record_offset":11616,"parent_record_offset":11592,"ancestor_record_offset":8980,"inlinee":11182,"source_line":397,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$10::request_redraw::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":140,"record_offset":9288,"parent_record_offset":9248,"ancestor_record_offset":8980,"inlinee":11160,"source_line":600,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$11::inner_size::closure$0
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":140,"record_offset":10452,"parent_record_offset":10424,"ancestor_record_offset":8980,"inlinee":11163,"source_line":747,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$13::set_cursor_icon::closure$0
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":140,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":8980,"inlinee":11181,"source_line":1346,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui_with_options::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2BA2
// 首条位置: {"module":140,"inlinee":11170}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] iced_style::theme::Theme::palette
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/iced-8189c10ee55bf96d/ea4f048/style/src/theme.rs
//     evidence={"module":140,"record_offset":10756,"parent_record_offset":10728,"ancestor_record_offset":10728,"inlinee":11172,"source_line":123,"archive":"_pdb/A/modules/00140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_iced::gui_runner::run_gui_with_options::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31F9
// 首条位置: {"module":202,"inlinee":12793}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::impl$0::default
// 信息 ID: c103efca0448edcd0d91963c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>; module=729, inline_record=3580; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=779, inline_record=3576; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=779, inline_record=12288; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>; module=786, inline_record=3528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>
// 信息 ID: adcbd9bb7ae2d125c4cd881e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner::run_gui -> ["tg_gpu_selector::gpu_selection::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_gpu_selector::run_gpu_selection_gui @ 0x3f1eb1 -> 0x8f53e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>
// 信息 ID: f9d357fe26f464b4604200fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner::run_gui -> ["tg_gpu_selector::vulkan_config::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_gpu_selector::run_vulkan_configuration_gui @ 0x3f1f7a -> 0x8f3620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>
// 信息 ID: d5d826b4b36b4d035ae3a871
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner::run_gui -> ["tg_iced::dialog::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_iced::dialog::MessageDialog::show @ 0x3f2cef -> 0x905660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>
// 信息 ID: 61987f28e6d5d867354b9869
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner::run_gui -> ["tg_integrity_report::controls::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_integrity_report::run_integrity_report_gui @ 0x3eedf4 -> 0x8c6ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options
// 信息 ID: c08fd12ff22fb95fee44e9d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>; module=729, inline_record=3604; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=779, inline_record=3600; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=779, inline_record=12312; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>; module=786, inline_record=3552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options::closure$0
// 信息 ID: cb4756d6f293b8a75957e5c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_iced::gui_runner::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls>; module=202, inline_record=7924; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>; module=729, inline_record=4140; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=779, inline_record=4148; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=779, inline_record=12860; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>; module=786, inline_record=4100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options::closure$1
// 信息 ID: 85fab8ad6241d9de28d7e11f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_iced::gui_runner::run_gui_with_options", "captures": "unknown"}
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::vulkan_config::Controls> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tg_iced::gui_runner::run_gui_with_options::closure_env$1 <- core::ops::function::FnOnce::call_once<winit::platform_impl::platform::event_loop::impl$3::pump_events::closure_env$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >,tuple$<enum2$<winit::event::Event<tuple$<> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 18 条。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; module=140, inline_record=8980; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >; module=1995, inline_record=3200; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >; module=3170, inline_record=2664; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >; module=3236, inline_record=3052; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::vulkan_config::Controls> >; module=3236, inline_record=5276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options::closure$1::closure$0
// 信息 ID: 934f3a16bee8d5e7df6096cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tg_iced::gui_runner::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<crash_reporter_gui::Controls> >; module=140, inline_record=10728; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_iced::dialog::Controls> >; module=1995, inline_record=4948; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_integrity_report::controls::Controls> >; module=3170, inline_record=4412; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::gpu_selection::Controls> >; module=3236, inline_record=3632; 不是独立调用边。
// [PDB 内联宿主] winit::platform_impl::platform::event_loop::impl$3::pump_events::closure$0<tuple$<>,tg_iced::gui_runner::run_gui_with_options::closure_env$1<tg_gpu_selector::vulkan_config::Controls> >; module=3236, inline_record=5856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options::closure$2
// 信息 ID: babf001e30fd9be5096eb0db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tg_iced::gui_runner::run_gui_with_options", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls>; module=202, inline_record=9400; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_integrity_report::controls::Controls>; module=729, inline_record=5672; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::vulkan_config::Controls>; module=779, inline_record=5624; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_gpu_selector::gpu_selection::Controls>; module=779, inline_record=14372; 不是独立调用边。
// [PDB 内联宿主] tg_iced::gui_runner::run_gui<tg_iced::dialog::Controls>; module=786, inline_record=5640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_iced::gui_runner::run_gui_with_options<crash_reporter_gui::Controls>
// 信息 ID: 88fdc740aea3e82c033db43f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_iced::gui_runner::run_gui_with_options -> ["crash_reporter_gui::Controls"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_reporter_gui::run_crash_reporter_gui<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6> @ 0xd7682 -> 0x30be80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

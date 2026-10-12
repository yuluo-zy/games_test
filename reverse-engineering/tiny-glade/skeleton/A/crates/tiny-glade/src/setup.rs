// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/setup.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 05c4a7168d2726e463edd6d05aa719465894b6d83253993a0246d80a6b927bbb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: setup.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 21, 73, 76, 127, 158, 182
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"PDB_registry_path_only","version":"0.25.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"PDB_registry_path_only","version":"1.19.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// use winit; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::setup_window
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":76,"record_offset":6072,"rva":966400,"va":5369675520}
// 编译实例 RVAs: 0xebf00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..collections..vec_deque..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::try_fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2243997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e89d3d2200","size":5,"soft_fail":false,"target_va":5371920944,"va":5369676942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t3587659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e84bbe3600","size":5,"soft_fail":false,"target_va":5373265456,"va":5369677792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t3588088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8f8bf3600","size":5,"soft_fail":false,"target_va":5373265408,"va":5369677315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3575705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8998f3600","size":5,"soft_fail":false,"target_va":5373252112,"va":5369676402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d84b0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-86500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e81caefeff","size":5,"soft_fail":false,"target_va":5369595056,"va":5369681551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402eb4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2092848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e830ef1f00","size":5,"soft_fail":false,"target_va":5371770016,"va":5369677163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2568355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8a3302700","size":5,"soft_fail":false,"target_va":5372249392,"va":5369681032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=5
//     target_aliases=89 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2636679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8873b2800","size":5,"soft_fail":false,"target_va":5372314208,"va":5369677524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140375190
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2654446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8ee802800","size":5,"soft_fail":false,"target_va":5372334480,"va":5369680029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140377f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2664886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8b6a92800","size":5,"soft_fail":false,"target_va":5372346176,"va":5369681285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3073464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8b8e52e00","size":5,"soft_fail":false,"target_va":5372749696,"va":5369676227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3073127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e867e42e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5369676580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140455590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3576087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e817913600","size":5,"soft_fail":false,"target_va":5373253008,"va":5369676916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33916931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e803880502","size":5,"soft_fail":false,"target_va":5403593360,"va":5369676424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33298463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e81f18fc01","size":5,"soft_fail":false,"target_va":5402978144,"va":5369679676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1981377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8c13b1e00","size":5,"soft_fail":false,"target_va":5371660480,"va":5369679098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33298873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8b919fc01","size":5,"soft_fail":false,"target_va":5402978688,"va":5369679810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33299066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e87a1afc01","size":5,"soft_fail":false,"target_va":5402978912,"va":5369679841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8bce9ffff","size":5,"soft_fail":false,"target_va":5369673936,"va":5369679631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<image::dynimage::DynamicImage>,enum2$<image::error::ImageError> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e84be9ffff","size":5,"soft_fail":false,"target_va":5369673376,"va":5369679184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e831ebffff","size":5,"soft_fail":false,"target_va":5369673824,"va":5369679146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33304114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8322efc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5369676009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e841faffff","size":5,"soft_fail":false,"target_va":5369674128,"va":5369675594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33304980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e89431fc01","size":5,"soft_fail":false,"target_va":5402980656,"va":5369675671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba8
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6370863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e82f366100","size":5,"soft_fail":false,"target_va":5376049776,"va":5369678908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::reader::Reader$LT$R$GT$::decode
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1614220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e88ca11800","size":5,"soft_fail":false,"target_va":5371293040,"va":5369678815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::reader::Reader$LT$std..io..buffered..bufreader..BufReader$LT$std..fs..File$GT$$GT$::open_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6399661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8ada66100","size":5,"soft_fail":false,"target_va":5376078320,"va":5369678654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t7676874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8ca237500","size":5,"soft_fail":false,"target_va":5377353776,"va":5369676897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6680731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e89bf06500","size":5,"soft_fail":false,"target_va":5376356912,"va":5369676176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6684618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8caff6500","size":5,"soft_fail":false,"target_va":5376360560,"va":5369675937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6680565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8f5ef6500","size":5,"soft_fail":false,"target_va":5376356144,"va":5369675574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33295857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8f10dfc01","size":5,"soft_fail":false,"target_va":5402975328,"va":5369679466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33296056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8b80efc01","size":5,"soft_fail":false,"target_va":5402975504,"va":5369679443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33296138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e80a0ffc01","size":5,"soft_fail":false,"target_va":5402975696,"va":5369679553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3d9350ee38791ca5 (.llvm.8305701337189838128)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2949206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e856002d00","size":5,"soft_fail":false,"target_va":5372625168,"va":5369675957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::setup::find_best_fullscreen_video_mode
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8350f0000","size":5,"soft_fail":false,"target_va":5369681776,"va":5369677878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::icon::Icon::from_rgba
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3572791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e837843600","size":5,"soft_fail":false,"target_va":5373251792,"va":5369678996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::VideoMode::size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3587706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e87abe3600","size":5,"soft_fail":false,"target_va":5373265648,"va":5369677937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t3588370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e812c13600","size":5,"soft_fail":false,"target_va":5373265840,"va":5369677465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::primary_monitor
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3588687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e84fc23600","size":5,"soft_fail":false,"target_va":5373265808,"va":5369677116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::init
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1028334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369675520,"raw_hex":"e8eeb00f00","size":5,"soft_fail":false,"target_va":5370707680,"va":5369679341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*41606370(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5369675520,"raw_hex":"ff15e2dc7a02","size":6,"soft_fail":false,"va":5369679576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":76,"record_offset":13696,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":76,"record_offset":11916,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":76,"record_offset":13724,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":76,"record_offset":12740,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8210,"source_line":1547,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":76,"record_offset":10460,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":76,"record_offset":10432,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":76,"record_offset":12172,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":76,"record_offset":11372,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":76,"record_offset":11660,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":17020,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":15332,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::into_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":76,"record_offset":15304,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8680,"source_line":688,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":76,"record_offset":15184,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":7596,"source_line":1083,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":76,"record_offset":13752,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":76,"record_offset":13612,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":76,"record_offset":7896,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":76,"record_offset":6308,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":76,"record_offset":6884,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::setup::impl$1::from_winit
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":76,"record_offset":13292,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":6079,"source_line":206,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::dpi::impl$18::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":76,"record_offset":14652,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8219,"source_line":302,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":76,"record_offset":15728,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8226,"source_line":305,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":76,"record_offset":12208,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8211,"source_line":125,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":76,"record_offset":12372,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8207,"source_line":118,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::build
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":76,"record_offset":15756,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8227,"source_line":508,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":76,"record_offset":8964,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8190,"source_line":205,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_fullscreen
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":76,"record_offset":13780,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8679,"source_line":318,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":76,"record_offset":12140,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8196,"source_line":222,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_position
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":76,"record_offset":12960,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8220,"source_line":274,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":76,"record_offset":9592,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8195,"source_line":307,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_visible
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":76,"record_offset":10404,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8201,"source_line":340,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_window_icon
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":76,"record_offset":15424,"parent_record_offset":6072,"ancestor_record_offset":6072,"inlinee":8681,"source_line":406,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":9368,"parent_record_offset":9344,"ancestor_record_offset":6072,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":9392,"parent_record_offset":9368,"ancestor_record_offset":6072,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":10352,"parent_record_offset":10328,"ancestor_record_offset":6072,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":9344,"parent_record_offset":9316,"ancestor_record_offset":6072,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":10328,"parent_record_offset":10272,"ancestor_record_offset":6072,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":76,"record_offset":13964,"parent_record_offset":13940,"ancestor_record_offset":6072,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":10300,"parent_record_offset":10272,"ancestor_record_offset":6072,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":9316,"parent_record_offset":9288,"ancestor_record_offset":6072,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":9288,"parent_record_offset":9260,"ancestor_record_offset":6072,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":9464,"parent_record_offset":9288,"ancestor_record_offset":6072,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":10272,"parent_record_offset":10244,"ancestor_record_offset":6072,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":9224,"parent_record_offset":9196,"ancestor_record_offset":6072,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":9196,"parent_record_offset":9168,"ancestor_record_offset":6072,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":9140,"parent_record_offset":9112,"ancestor_record_offset":6072,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":9168,"parent_record_offset":9140,"ancestor_record_offset":6072,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":9112,"parent_record_offset":9084,"ancestor_record_offset":6072,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":76,"record_offset":9084,"parent_record_offset":9048,"ancestor_record_offset":6072,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":76,"record_offset":9664,"parent_record_offset":9636,"ancestor_record_offset":6072,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":76,"record_offset":15984,"parent_record_offset":15960,"ancestor_record_offset":6072,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":76,"record_offset":15960,"parent_record_offset":15920,"ancestor_record_offset":6072,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":76,"record_offset":15584,"parent_record_offset":15556,"ancestor_record_offset":6072,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":76,"record_offset":9260,"parent_record_offset":9224,"ancestor_record_offset":6072,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":76,"record_offset":8316,"parent_record_offset":8288,"ancestor_record_offset":6072,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":76,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":6072,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":76,"record_offset":8344,"parent_record_offset":8316,"ancestor_record_offset":6072,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":76,"record_offset":8256,"parent_record_offset":8224,"ancestor_record_offset":6072,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":76,"record_offset":8288,"parent_record_offset":8256,"ancestor_record_offset":6072,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":76,"record_offset":8460,"parent_record_offset":8288,"ancestor_record_offset":6072,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":76,"record_offset":8572,"parent_record_offset":8548,"ancestor_record_offset":6072,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":76,"record_offset":10616,"parent_record_offset":10492,"ancestor_record_offset":6072,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":76,"record_offset":9636,"parent_record_offset":9592,"ancestor_record_offset":6072,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$77::from
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":76,"record_offset":13128,"parent_record_offset":13104,"ancestor_record_offset":6072,"inlinee":8217,"source_line":76,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":76,"record_offset":12012,"parent_record_offset":11988,"ancestor_record_offset":6072,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":76,"record_offset":11540,"parent_record_offset":11400,"ancestor_record_offset":6072,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":76,"record_offset":10644,"parent_record_offset":10492,"ancestor_record_offset":6072,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":76,"record_offset":9524,"parent_record_offset":9500,"ancestor_record_offset":6072,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":76,"record_offset":10780,"parent_record_offset":10756,"ancestor_record_offset":6072,"inlinee":6221,"source_line":116,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":76,"record_offset":10756,"parent_record_offset":10492,"ancestor_record_offset":6072,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":76,"record_offset":8392,"parent_record_offset":8368,"ancestor_record_offset":6072,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":76,"record_offset":10976,"parent_record_offset":10944,"ancestor_record_offset":6072,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":76,"record_offset":6520,"parent_record_offset":6424,"ancestor_record_offset":6072,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":76,"record_offset":8148,"parent_record_offset":8120,"ancestor_record_offset":6072,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":76,"record_offset":7480,"parent_record_offset":7020,"ancestor_record_offset":6072,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":76,"record_offset":8196,"parent_record_offset":8016,"ancestor_record_offset":6072,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":76,"record_offset":6832,"parent_record_offset":6804,"ancestor_record_offset":6072,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":76,"record_offset":9500,"parent_record_offset":9224,"ancestor_record_offset":6072,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":76,"record_offset":8524,"parent_record_offset":8224,"ancestor_record_offset":6072,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":76,"record_offset":16008,"parent_record_offset":15984,"ancestor_record_offset":6072,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":76,"record_offset":7256,"parent_record_offset":7232,"ancestor_record_offset":6072,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":76,"record_offset":7232,"parent_record_offset":7208,"ancestor_record_offset":6072,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":76,"record_offset":7440,"parent_record_offset":7408,"ancestor_record_offset":6072,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":76,"record_offset":9420,"parent_record_offset":9392,"ancestor_record_offset":6072,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":76,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":6072,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":8924,"parent_record_offset":7976,"ancestor_record_offset":6072,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":16164,"parent_record_offset":16104,"ancestor_record_offset":6072,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":12272,"parent_record_offset":12236,"ancestor_record_offset":6072,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":12456,"parent_record_offset":12408,"ancestor_record_offset":6072,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":12308,"parent_record_offset":12236,"ancestor_record_offset":6072,"inlinee":8213,"source_line":1407,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":15236,"parent_record_offset":15184,"ancestor_record_offset":6072,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":15272,"parent_record_offset":15184,"ancestor_record_offset":6072,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":76,"record_offset":6456,"parent_record_offset":6424,"ancestor_record_offset":6072,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":76,"record_offset":6484,"parent_record_offset":6456,"ancestor_record_offset":6072,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":76,"record_offset":7208,"parent_record_offset":7184,"ancestor_record_offset":6072,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":76,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":6072,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":76,"record_offset":7184,"parent_record_offset":7076,"ancestor_record_offset":6072,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":76,"record_offset":6424,"parent_record_offset":6396,"ancestor_record_offset":6072,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":76,"record_offset":6916,"parent_record_offset":6884,"ancestor_record_offset":6072,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":76,"record_offset":7048,"parent_record_offset":7020,"ancestor_record_offset":6072,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":76,"record_offset":7020,"parent_record_offset":6988,"ancestor_record_offset":6072,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":76,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":6072,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":76,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":6072,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":76,"record_offset":6940,"parent_record_offset":6916,"ancestor_record_offset":6072,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":76,"record_offset":7512,"parent_record_offset":6988,"ancestor_record_offset":6072,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":76,"record_offset":6620,"parent_record_offset":6588,"ancestor_record_offset":6072,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":76,"record_offset":6780,"parent_record_offset":6620,"ancestor_record_offset":6072,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":76,"record_offset":6588,"parent_record_offset":6564,"ancestor_record_offset":6072,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":76,"record_offset":6340,"parent_record_offset":6308,"ancestor_record_offset":6072,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":76,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":6072,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":76,"record_offset":6396,"parent_record_offset":6364,"ancestor_record_offset":6072,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":76,"record_offset":6364,"parent_record_offset":6340,"ancestor_record_offset":6072,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":76,"record_offset":7076,"parent_record_offset":7048,"ancestor_record_offset":6072,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":76,"record_offset":7324,"parent_record_offset":7076,"ancestor_record_offset":6072,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":76,"record_offset":7296,"parent_record_offset":7076,"ancestor_record_offset":6072,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":76,"record_offset":7128,"parent_record_offset":7076,"ancestor_record_offset":6072,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":16432,"parent_record_offset":16404,"ancestor_record_offset":6072,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":16792,"parent_record_offset":16616,"ancestor_record_offset":6072,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":16104,"parent_record_offset":16076,"ancestor_record_offset":6072,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":16880,"parent_record_offset":16852,"ancestor_record_offset":6072,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":15620,"parent_record_offset":15584,"ancestor_record_offset":6072,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":6072,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":16136,"parent_record_offset":16104,"ancestor_record_offset":6072,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":6072,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":16816,"parent_record_offset":16792,"ancestor_record_offset":6072,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":15644,"parent_record_offset":15620,"ancestor_record_offset":6072,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":16904,"parent_record_offset":16880,"ancestor_record_offset":6072,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":15676,"parent_record_offset":15584,"ancestor_record_offset":6072,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] image::io::reader::Reader::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/io/reader.rs
//     evidence={"module":76,"record_offset":15208,"parent_record_offset":15184,"ancestor_record_offset":6072,"inlinee":7590,"source_line":293,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":76,"record_offset":10672,"parent_record_offset":10492,"ancestor_record_offset":6072,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":76,"record_offset":10532,"parent_record_offset":10492,"ancestor_record_offset":6072,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":76,"record_offset":7936,"parent_record_offset":7896,"ancestor_record_offset":6072,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":76,"record_offset":8060,"parent_record_offset":8016,"ancestor_record_offset":6072,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":76,"record_offset":8224,"parent_record_offset":8016,"ancestor_record_offset":6072,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":76,"record_offset":8496,"parent_record_offset":8224,"ancestor_record_offset":6072,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":76,"record_offset":16308,"parent_record_offset":16284,"ancestor_record_offset":6072,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":76,"record_offset":16284,"parent_record_offset":16260,"ancestor_record_offset":6072,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":76,"record_offset":16044,"parent_record_offset":15920,"ancestor_record_offset":6072,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":76,"record_offset":16200,"parent_record_offset":16044,"ancestor_record_offset":6072,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":76,"record_offset":16584,"parent_record_offset":16556,"ancestor_record_offset":6072,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":76,"record_offset":16616,"parent_record_offset":16584,"ancestor_record_offset":6072,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":76,"record_offset":16404,"parent_record_offset":16228,"ancestor_record_offset":6072,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":76,"record_offset":16228,"parent_record_offset":16200,"ancestor_record_offset":6072,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":76,"record_offset":16076,"parent_record_offset":16044,"ancestor_record_offset":6072,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":76,"record_offset":16852,"parent_record_offset":16584,"ancestor_record_offset":6072,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":76,"record_offset":8088,"parent_record_offset":8060,"ancestor_record_offset":6072,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":76,"record_offset":8120,"parent_record_offset":8088,"ancestor_record_offset":6072,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":76,"record_offset":8016,"parent_record_offset":7976,"ancestor_record_offset":6072,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":76,"record_offset":7976,"parent_record_offset":7936,"ancestor_record_offset":6072,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":76,"record_offset":16260,"parent_record_offset":16228,"ancestor_record_offset":6072,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::setup::setup_window::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":76,"record_offset":10492,"parent_record_offset":10460,"ancestor_record_offset":6072,"inlinee":8673,"source_line":23,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::setup::setup_window::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":76,"record_offset":11400,"parent_record_offset":11372,"ancestor_record_offset":6072,"inlinee":8676,"source_line":37,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalPosition::cast
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":76,"record_offset":13044,"parent_record_offset":13020,"ancestor_record_offset":6072,"inlinee":8224,"source_line":281,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":76,"record_offset":11964,"parent_record_offset":11940,"ancestor_record_offset":6072,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$19::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":76,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":6072,"inlinee":8215,"source_line":308,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$30::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":76,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":6072,"inlinee":8678,"source_line":460,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":76,"record_offset":12536,"parent_record_offset":12508,"ancestor_record_offset":6072,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$36::from
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":76,"record_offset":13020,"parent_record_offset":12996,"ancestor_record_offset":6072,"inlinee":8222,"source_line":573,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":76,"record_offset":10700,"parent_record_offset":10492,"ancestor_record_offset":6072,"inlinee":8674,"source_line":313,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":76,"record_offset":11596,"parent_record_offset":11400,"ancestor_record_offset":6072,"inlinee":8203,"source_line":329,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":76,"record_offset":13324,"parent_record_offset":13292,"ancestor_record_offset":6072,"inlinee":6081,"source_line":51,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/event_loop.rs
//     evidence={"module":76,"record_offset":10724,"parent_record_offset":10700,"ancestor_record_offset":6072,"inlinee":8675,"source_line":490,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/event_loop.rs
//     evidence={"module":76,"record_offset":11620,"parent_record_offset":11596,"ancestor_record_offset":6072,"inlinee":8205,"source_line":494,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":76,"record_offset":12236,"parent_record_offset":12208,"ancestor_record_offset":6072,"inlinee":8212,"source_line":211,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":76,"record_offset":12408,"parent_record_offset":12372,"ancestor_record_offset":6072,"inlinee":8209,"source_line":185,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":76,"record_offset":16948,"parent_record_offset":15920,"ancestor_record_offset":6072,"inlinee":8244,"source_line":344,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":76,"record_offset":15852,"parent_record_offset":15756,"ancestor_record_offset":6072,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":76,"record_offset":15816,"parent_record_offset":15756,"ancestor_record_offset":6072,"inlinee":8229,"source_line":96,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":76,"record_offset":15920,"parent_record_offset":15888,"ancestor_record_offset":6072,"inlinee":8233,"source_line":160,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$30::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":76,"record_offset":9000,"parent_record_offset":8964,"ancestor_record_offset":6072,"inlinee":8192,"source_line":116,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":76,"record_offset":9048,"parent_record_offset":9000,"ancestor_record_offset":6072,"inlinee":8194,"source_line":162,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$9::build::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":76,"record_offset":15888,"parent_record_offset":15852,"ancestor_record_offset":6072,"inlinee":8232,"source_line":514,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::setup_window::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21E1
// 首条位置: {"module":76,"inlinee":8673}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":76,"record_offset":10616,"parent_record_offset":10492,"ancestor_record_offset":10492,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":76,"record_offset":10644,"parent_record_offset":10492,"ancestor_record_offset":10492,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":76,"record_offset":10756,"parent_record_offset":10492,"ancestor_record_offset":10492,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":76,"record_offset":10812,"parent_record_offset":10492,"ancestor_record_offset":10492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":76,"record_offset":10672,"parent_record_offset":10492,"ancestor_record_offset":10492,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":76,"record_offset":10532,"parent_record_offset":10492,"ancestor_record_offset":10492,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":76,"record_offset":10700,"parent_record_offset":10492,"ancestor_record_offset":10492,"inlinee":8674,"source_line":313,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":11036,"parent_record_offset":11012,"ancestor_record_offset":10492,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":11012,"parent_record_offset":10912,"ancestor_record_offset":10492,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":10944,"parent_record_offset":10912,"ancestor_record_offset":10492,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":10492,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":76,"record_offset":10780,"parent_record_offset":10756,"ancestor_record_offset":10492,"inlinee":6221,"source_line":116,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":76,"record_offset":10976,"parent_record_offset":10944,"ancestor_record_offset":10492,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":76,"record_offset":10836,"parent_record_offset":10812,"ancestor_record_offset":10492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":10492,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":10492,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::available_monitors
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/event_loop.rs
//     evidence={"module":76,"record_offset":10724,"parent_record_offset":10700,"ancestor_record_offset":10492,"inlinee":8675,"source_line":490,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::setup_window::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1832
// 首条位置: {"module":21,"inlinee":6194}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":21,"record_offset":30656,"parent_record_offset":30516,"ancestor_record_offset":30516,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":21,"record_offset":31208,"parent_record_offset":30516,"ancestor_record_offset":30516,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":21,"record_offset":30684,"parent_record_offset":30516,"ancestor_record_offset":30516,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":31264,"parent_record_offset":30516,"ancestor_record_offset":30516,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::render_config::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":21,"record_offset":30740,"parent_record_offset":30516,"ancestor_record_offset":30516,"inlinee":6196,"source_line":29,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":21,"record_offset":30712,"parent_record_offset":30516,"ancestor_record_offset":30516,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":21,"record_offset":30572,"parent_record_offset":30516,"ancestor_record_offset":30516,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":31436,"parent_record_offset":31412,"ancestor_record_offset":30516,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":31412,"parent_record_offset":31388,"ancestor_record_offset":30516,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":31388,"parent_record_offset":31364,"ancestor_record_offset":30516,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":21,"record_offset":30840,"parent_record_offset":30808,"ancestor_record_offset":30516,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":21,"record_offset":30868,"parent_record_offset":30840,"ancestor_record_offset":30516,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":21,"record_offset":30980,"parent_record_offset":30740,"ancestor_record_offset":30516,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":21,"record_offset":31004,"parent_record_offset":30980,"ancestor_record_offset":30516,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":21,"record_offset":30780,"parent_record_offset":30740,"ancestor_record_offset":30516,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":21,"record_offset":30808,"parent_record_offset":30780,"ancestor_record_offset":30516,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":31288,"parent_record_offset":31264,"ancestor_record_offset":30516,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":21,"record_offset":30896,"parent_record_offset":30868,"ancestor_record_offset":30516,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":21,"record_offset":30924,"parent_record_offset":30896,"ancestor_record_offset":30516,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":21,"record_offset":30596,"parent_record_offset":30572,"ancestor_record_offset":30516,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":21,"record_offset":30620,"parent_record_offset":30596,"ancestor_record_offset":30516,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::setup_window::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x21E4
// 首条位置: {"module":76,"inlinee":8676}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":76,"record_offset":11512,"parent_record_offset":11400,"ancestor_record_offset":11400,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":76,"record_offset":11540,"parent_record_offset":11400,"ancestor_record_offset":11400,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":76,"record_offset":11568,"parent_record_offset":11400,"ancestor_record_offset":11400,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":76,"record_offset":11428,"parent_record_offset":11400,"ancestor_record_offset":11400,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":76,"record_offset":11596,"parent_record_offset":11400,"ancestor_record_offset":11400,"inlinee":8203,"source_line":329,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":11452,"parent_record_offset":11428,"ancestor_record_offset":11400,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":11400,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::event_loop::EventLoopWindowTarget::primary_monitor
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/event_loop.rs
//     evidence={"module":76,"record_offset":11620,"parent_record_offset":11596,"ancestor_record_offset":11400,"inlinee":8205,"source_line":494,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::find_best_fullscreen_video_mode
// 记录形式: compiled_procedure; instances=1
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":76,"record_offset":17260,"rva":972656,"va":5369681776}
// 编译实例 RVAs: 0xed770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14002b480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-798281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8b7d1f3ff","size":5,"soft_fail":false,"target_va":5368886400,"va":5369684676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007ad60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-471427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e87dcef8ff","size":5,"soft_fail":false,"target_va":5369212256,"va":5369683678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-69060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e83cf2feff","size":5,"soft_fail":false,"target_va":5369614224,"va":5369683279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e5980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e80e7dffff","size":5,"soft_fail":false,"target_va":5369649536,"va":5369683053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e6b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8ba8cffff","size":5,"soft_fail":false,"target_va":5369654128,"va":5369683633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8e8ccffff","size":5,"soft_fail":false,"target_va":5369673744,"va":5369686819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c73f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1938224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e830931d00","size":5,"soft_fail":false,"target_va":5371622384,"va":5369684155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402eb900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2085822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8bed31f00","size":5,"soft_fail":false,"target_va":5371771136,"va":5369685309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f8220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2139527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e887a52000","size":5,"soft_fail":false,"target_va":5371822624,"va":5369683092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2561598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e83e162700","size":5,"soft_fail":false,"target_va":5372249392,"va":5369687789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14036eb20
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2622319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e86f032800","size":5,"soft_fail":false,"target_va":5372308256,"va":5369685932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=6
//     target_aliases=89 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2631253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e855262800","size":5,"soft_fail":false,"target_va":5372314208,"va":5369682950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382c20
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2702536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8c83c2900","size":5,"soft_fail":false,"target_va":5372390432,"va":5369687891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140385940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2714428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e83c6b2900","size":5,"soft_fail":false,"target_va":5372401984,"va":5369687551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140394640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2775948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e88c5b2a00","size":5,"soft_fail":false,"target_va":5372462656,"va":5369686703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3067631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8efce2e00","size":5,"soft_fail":false,"target_va":5372749696,"va":5369682060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3067556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8a4ce2e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5369682151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5489045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e895c15300","size":5,"soft_fail":false,"target_va":5375171056,"va":5369682006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33909874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8726c0502","size":5,"soft_fail":false,"target_va":5403593344,"va":5369683465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33911262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8de710502","size":5,"soft_fail":false,"target_va":5403593360,"va":5369682093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33292267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8ebfffb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5369685872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::map::BTreeMap$LT$K$C$V$C$A$GT$::insert
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-636724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8cc48f6ff","size":5,"soft_fail":false,"target_va":5369046000,"va":5369682719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7841091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e843a57700","size":5,"soft_fail":false,"target_va":5377527120,"va":5369686024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33292924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e87c02fc01","size":5,"soft_fail":false,"target_va":5402978880,"va":5369685951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33293239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8b703fc01","size":5,"soft_fail":false,"target_va":5402979152,"va":5369685908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33296130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8020ffc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5369683993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t7670861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e84d0c7500","size":5,"soft_fail":false,"target_va":5377353776,"va":5369682910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] once_cell::imp::OnceCell$LT$T$GT$::initialize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t33238139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e87b2cfb01","size":5,"soft_fail":false,"target_va":5402921280,"va":5369683136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33292120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e858fffb01","size":5,"soft_fail":false,"target_va":5402975328,"va":5369683203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7833926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e846897700","size":5,"soft_fail":false,"target_va":5377515952,"va":5369682021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33292330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e82a00fc01","size":5,"soft_fail":false,"target_va":5402975504,"va":5369683169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33290884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e884fafb01","size":5,"soft_fail":false,"target_va":5402975696,"va":5369684807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::VideoMode::size
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t3581228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e82ca53600","size":5,"soft_fail":false,"target_va":5373265648,"va":5369684415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t3583972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e8e4af3600","size":5,"soft_fail":false,"target_va":5373265840,"va":5369681863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::decode_wide
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3558925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369681776,"raw_hex":"e80d4e3600","size":5,"soft_fail":false,"target_va":5373240928,"va":5369681998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*41603099(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5369681776,"raw_hex":"ff151bd07a02","size":6,"soft_fail":false,"va":5369682535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":20404,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":8700,"source_line":207,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":76,"record_offset":23632,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":76,"record_offset":20460,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":76,"record_offset":24476,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":76,"record_offset":26720,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":76,"record_offset":22432,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":76,"record_offset":22880,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":76,"record_offset":20320,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":76,"record_offset":24532,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":8708,"source_line":3276,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":76,"record_offset":26812,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":76,"record_offset":22936,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":21184,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":76,"record_offset":22740,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":76,"record_offset":25944,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":8711,"source_line":296,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":76,"record_offset":22908,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":76,"record_offset":22348,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":76,"record_offset":20488,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":6933,"source_line":1375,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":76,"record_offset":21224,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":8702,"source_line":670,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":76,"record_offset":24156,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":76,"record_offset":20716,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":76,"record_offset":17492,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":8682,"source_line":112,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":76,"record_offset":22460,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":8211,"source_line":125,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::refresh_rate_millihertz
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":76,"record_offset":24212,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":8706,"source_line":137,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":76,"record_offset":25824,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":8207,"source_line":118,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::video_modes
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":76,"record_offset":19396,"parent_record_offset":17260,"ancestor_record_offset":17260,"inlinee":8686,"source_line":164,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":18196,"parent_record_offset":18172,"ancestor_record_offset":17260,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":18220,"parent_record_offset":18196,"ancestor_record_offset":17260,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":18564,"parent_record_offset":18540,"ancestor_record_offset":17260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":19904,"parent_record_offset":19876,"ancestor_record_offset":17260,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":18172,"parent_record_offset":18124,"ancestor_record_offset":17260,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":76,"record_offset":18540,"parent_record_offset":18516,"ancestor_record_offset":17260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":76,"record_offset":19876,"parent_record_offset":19436,"ancestor_record_offset":17260,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":76,"record_offset":25276,"parent_record_offset":25252,"ancestor_record_offset":17260,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":76,"record_offset":20112,"parent_record_offset":20088,"ancestor_record_offset":17260,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::BTreeSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\set.rs
//     evidence={"module":76,"record_offset":20060,"parent_record_offset":19436,"ancestor_record_offset":17260,"inlinee":8697,"source_line":902,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::BTreeSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\set.rs
//     evidence={"module":76,"record_offset":19588,"parent_record_offset":19436,"ancestor_record_offset":17260,"inlinee":8689,"source_line":341,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\set.rs
//     evidence={"module":76,"record_offset":20088,"parent_record_offset":19436,"ancestor_record_offset":17260,"inlinee":8699,"source_line":1252,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":18808,"parent_record_offset":18772,"ancestor_record_offset":17260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":18124,"parent_record_offset":18084,"ancestor_record_offset":17260,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":18084,"parent_record_offset":18044,"ancestor_record_offset":17260,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":18292,"parent_record_offset":18084,"ancestor_record_offset":17260,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":76,"record_offset":18516,"parent_record_offset":18492,"ancestor_record_offset":17260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":18000,"parent_record_offset":17960,"ancestor_record_offset":17260,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":17960,"parent_record_offset":17920,"ancestor_record_offset":17260,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":17880,"parent_record_offset":17840,"ancestor_record_offset":17260,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":17920,"parent_record_offset":17880,"ancestor_record_offset":17260,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":17840,"parent_record_offset":17800,"ancestor_record_offset":17260,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":76,"record_offset":20428,"parent_record_offset":20404,"ancestor_record_offset":17260,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":76,"record_offset":17800,"parent_record_offset":17760,"ancestor_record_offset":17260,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$38::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":76,"record_offset":17760,"parent_record_offset":17544,"ancestor_record_offset":17260,"inlinee":8685,"source_line":2649,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":76,"record_offset":18044,"parent_record_offset":18000,"ancestor_record_offset":17260,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":76,"record_offset":20344,"parent_record_offset":20320,"ancestor_record_offset":17260,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":76,"record_offset":23168,"parent_record_offset":23140,"ancestor_record_offset":17260,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":76,"record_offset":26872,"parent_record_offset":26848,"ancestor_record_offset":17260,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":76,"record_offset":27120,"parent_record_offset":27096,"ancestor_record_offset":17260,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":76,"record_offset":24760,"parent_record_offset":24724,"ancestor_record_offset":17260,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":76,"record_offset":19844,"parent_record_offset":19820,"ancestor_record_offset":17260,"inlinee":8696,"source_line":1469,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":76,"record_offset":22144,"parent_record_offset":22116,"ancestor_record_offset":17260,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":76,"record_offset":21504,"parent_record_offset":21480,"ancestor_record_offset":17260,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":76,"record_offset":21588,"parent_record_offset":21564,"ancestor_record_offset":17260,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":76,"record_offset":20144,"parent_record_offset":19436,"ancestor_record_offset":17260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":76,"record_offset":18356,"parent_record_offset":18332,"ancestor_record_offset":17260,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":76,"record_offset":19780,"parent_record_offset":19756,"ancestor_record_offset":17260,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":76,"record_offset":20280,"parent_record_offset":20256,"ancestor_record_offset":17260,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":76,"record_offset":24724,"parent_record_offset":24676,"ancestor_record_offset":17260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":76,"record_offset":20368,"parent_record_offset":20344,"ancestor_record_offset":17260,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":76,"record_offset":20256,"parent_record_offset":19436,"ancestor_record_offset":17260,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":76,"record_offset":24624,"parent_record_offset":24532,"ancestor_record_offset":17260,"inlinee":8709,"source_line":3310,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by_key::key::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":76,"record_offset":24924,"parent_record_offset":24888,"ancestor_record_offset":17260,"inlinee":6136,"source_line":3283,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":76,"record_offset":24676,"parent_record_offset":24624,"ancestor_record_offset":17260,"inlinee":8710,"source_line":2618,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":76,"record_offset":19732,"parent_record_offset":19708,"ancestor_record_offset":17260,"inlinee":8691,"source_line":402,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":76,"record_offset":19708,"parent_record_offset":19436,"ancestor_record_offset":17260,"inlinee":8690,"source_line":651,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":76,"record_offset":23528,"parent_record_offset":23500,"ancestor_record_offset":17260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$3::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":76,"record_offset":25116,"parent_record_offset":24956,"ancestor_record_offset":17260,"inlinee":6140,"source_line":3203,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":76,"record_offset":21704,"parent_record_offset":21680,"ancestor_record_offset":17260,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":76,"record_offset":24888,"parent_record_offset":24852,"ancestor_record_offset":17260,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":76,"record_offset":24852,"parent_record_offset":24724,"ancestor_record_offset":17260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":76,"record_offset":25388,"parent_record_offset":24676,"ancestor_record_offset":17260,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":76,"record_offset":18332,"parent_record_offset":18000,"ancestor_record_offset":17260,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":76,"record_offset":18416,"parent_record_offset":17544,"ancestor_record_offset":17260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":76,"record_offset":22788,"parent_record_offset":22764,"ancestor_record_offset":17260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":76,"record_offset":22048,"parent_record_offset":22024,"ancestor_record_offset":17260,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":76,"record_offset":21940,"parent_record_offset":21916,"ancestor_record_offset":17260,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":76,"record_offset":19756,"parent_record_offset":19732,"ancestor_record_offset":17260,"inlinee":8692,"source_line":1505,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":76,"record_offset":24820,"parent_record_offset":24760,"ancestor_record_offset":17260,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":76,"record_offset":24792,"parent_record_offset":24760,"ancestor_record_offset":17260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":76,"record_offset":18248,"parent_record_offset":18220,"ancestor_record_offset":17260,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":20836,"parent_record_offset":20776,"ancestor_record_offset":17260,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":22528,"parent_record_offset":22488,"ancestor_record_offset":17260,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":24288,"parent_record_offset":24244,"ancestor_record_offset":17260,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":17632,"parent_record_offset":17544,"ancestor_record_offset":17260,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":76,"record_offset":22564,"parent_record_offset":22488,"ancestor_record_offset":17260,"inlinee":8213,"source_line":1407,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":76,"record_offset":22228,"parent_record_offset":22200,"ancestor_record_offset":17260,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":76,"record_offset":22256,"parent_record_offset":22228,"ancestor_record_offset":17260,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":76,"record_offset":22764,"parent_record_offset":22740,"ancestor_record_offset":17260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":76,"record_offset":25972,"parent_record_offset":25944,"ancestor_record_offset":17260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":76,"record_offset":22172,"parent_record_offset":22144,"ancestor_record_offset":17260,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":21104,"parent_record_offset":21076,"ancestor_record_offset":17260,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":25668,"parent_record_offset":25492,"ancestor_record_offset":17260,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":20636,"parent_record_offset":20612,"ancestor_record_offset":17260,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":20776,"parent_record_offset":20748,"ancestor_record_offset":17260,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":25756,"parent_record_offset":25728,"ancestor_record_offset":17260,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":19648,"parent_record_offset":19616,"ancestor_record_offset":17260,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":20808,"parent_record_offset":20776,"ancestor_record_offset":17260,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":19672,"parent_record_offset":19648,"ancestor_record_offset":17260,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":25692,"parent_record_offset":25668,"ancestor_record_offset":17260,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":76,"record_offset":25780,"parent_record_offset":25756,"ancestor_record_offset":17260,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":76,"record_offset":22116,"parent_record_offset":22088,"ancestor_record_offset":17260,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":76,"record_offset":21272,"parent_record_offset":21248,"ancestor_record_offset":17260,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":76,"record_offset":21296,"parent_record_offset":21272,"ancestor_record_offset":17260,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":76,"record_offset":22088,"parent_record_offset":21868,"ancestor_record_offset":17260,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":76,"record_offset":21328,"parent_record_offset":21296,"ancestor_record_offset":17260,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":76,"record_offset":22000,"parent_record_offset":21976,"ancestor_record_offset":17260,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":76,"record_offset":21976,"parent_record_offset":21868,"ancestor_record_offset":17260,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":76,"record_offset":21916,"parent_record_offset":21892,"ancestor_record_offset":17260,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":76,"record_offset":21836,"parent_record_offset":21404,"ancestor_record_offset":17260,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":76,"record_offset":21892,"parent_record_offset":21868,"ancestor_record_offset":17260,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":76,"record_offset":21380,"parent_record_offset":21356,"ancestor_record_offset":17260,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":76,"record_offset":21356,"parent_record_offset":21296,"ancestor_record_offset":17260,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":76,"record_offset":21404,"parent_record_offset":21380,"ancestor_record_offset":17260,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":76,"record_offset":21648,"parent_record_offset":21620,"ancestor_record_offset":17260,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":76,"record_offset":21680,"parent_record_offset":21648,"ancestor_record_offset":17260,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":76,"record_offset":21620,"parent_record_offset":21404,"ancestor_record_offset":17260,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":76,"record_offset":21452,"parent_record_offset":21404,"ancestor_record_offset":17260,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":76,"record_offset":21868,"parent_record_offset":21404,"ancestor_record_offset":17260,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":76,"record_offset":21480,"parent_record_offset":21404,"ancestor_record_offset":17260,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":76,"record_offset":21564,"parent_record_offset":21404,"ancestor_record_offset":17260,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":76,"record_offset":21744,"parent_record_offset":21404,"ancestor_record_offset":17260,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":76,"record_offset":21248,"parent_record_offset":21224,"ancestor_record_offset":17260,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":76,"record_offset":24180,"parent_record_offset":24156,"ancestor_record_offset":17260,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":76,"record_offset":20172,"parent_record_offset":19436,"ancestor_record_offset":17260,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":76,"record_offset":19616,"parent_record_offset":19436,"ancestor_record_offset":17260,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/imp_std.rs
//     evidence={"module":76,"record_offset":20612,"parent_record_offset":20588,"ancestor_record_offset":17260,"inlinee":6941,"source_line":49,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":76,"record_offset":20512,"parent_record_offset":20488,"ancestor_record_offset":17260,"inlinee":6935,"source_line":1308,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":76,"record_offset":20588,"parent_record_offset":20560,"ancestor_record_offset":17260,"inlinee":6939,"source_line":963,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":76,"record_offset":20536,"parent_record_offset":20512,"ancestor_record_offset":17260,"inlinee":6937,"source_line":1115,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.19.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/once_cell-1.19.0/src/lib.rs
//     evidence={"module":76,"record_offset":20560,"parent_record_offset":20536,"ancestor_record_offset":17260,"inlinee":6938,"source_line":1152,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":76,"record_offset":17732,"parent_record_offset":17544,"ancestor_record_offset":17260,"inlinee":6742,"source_line":865,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":76,"record_offset":17700,"parent_record_offset":17676,"ancestor_record_offset":17260,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":76,"record_offset":17676,"parent_record_offset":17544,"ancestor_record_offset":17260,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":76,"record_offset":20980,"parent_record_offset":20956,"ancestor_record_offset":17260,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":76,"record_offset":20956,"parent_record_offset":20932,"ancestor_record_offset":17260,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":76,"record_offset":20872,"parent_record_offset":20716,"ancestor_record_offset":17260,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":76,"record_offset":25460,"parent_record_offset":25436,"ancestor_record_offset":17260,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":76,"record_offset":25492,"parent_record_offset":25460,"ancestor_record_offset":17260,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":76,"record_offset":21076,"parent_record_offset":20900,"ancestor_record_offset":17260,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":76,"record_offset":20900,"parent_record_offset":20872,"ancestor_record_offset":17260,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":76,"record_offset":20748,"parent_record_offset":20716,"ancestor_record_offset":17260,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":76,"record_offset":25728,"parent_record_offset":25460,"ancestor_record_offset":17260,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":76,"record_offset":20932,"parent_record_offset":20900,"ancestor_record_offset":17260,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::setup::find_best_fullscreen_video_mode::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":76,"record_offset":26048,"parent_record_offset":25944,"ancestor_record_offset":17260,"inlinee":8712,"source_line":146,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::setup::find_best_fullscreen_video_mode::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/setup.rs
//     evidence={"module":76,"record_offset":24956,"parent_record_offset":24924,"ancestor_record_offset":17260,"inlinee":6138,"source_line":163,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$73::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":76,"record_offset":26104,"parent_record_offset":26048,"ancestor_record_offset":17260,"inlinee":8714,"source_line":410,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":76,"record_offset":25028,"parent_record_offset":24956,"ancestor_record_offset":17260,"inlinee":6081,"source_line":51,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":76,"record_offset":17544,"parent_record_offset":17492,"ancestor_record_offset":17260,"inlinee":8683,"source_line":165,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":76,"record_offset":22488,"parent_record_offset":22460,"ancestor_record_offset":17260,"inlinee":8212,"source_line":211,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::refresh_rate_millihertz
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":76,"record_offset":24244,"parent_record_offset":24212,"ancestor_record_offset":17260,"inlinee":8707,"source_line":194,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":76,"record_offset":25860,"parent_record_offset":25824,"ancestor_record_offset":17260,"inlinee":8209,"source_line":185,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::video_modes
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":76,"record_offset":19436,"parent_record_offset":19396,"ancestor_record_offset":17260,"inlinee":8687,"source_line":229,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::util::has_flag
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/util.rs
//     evidence={"module":76,"record_offset":19820,"parent_record_offset":19436,"ancestor_record_offset":17260,"inlinee":8694,"source_line":53,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::find_best_fullscreen_video_mode::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2208
// 首条位置: {"module":76,"inlinee":8712}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] winit::dpi::impl$73::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":76,"record_offset":26104,"parent_record_offset":26048,"ancestor_record_offset":26048,"inlinee":8714,"source_line":410,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":76,"record_offset":26076,"parent_record_offset":26048,"ancestor_record_offset":26048,"inlinee":6081,"source_line":51,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":76,"record_offset":26128,"parent_record_offset":26104,"ancestor_record_offset":26048,"inlinee":8696,"source_line":1469,"archive":"_pdb/A/modules/00076.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::find_best_fullscreen_video_mode::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17FA
// 首条位置: {"module":21,"inlinee":6138}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$3::abs
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":21,"record_offset":21276,"parent_record_offset":21136,"ancestor_record_offset":21136,"inlinee":6140,"source_line":3203,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":21,"record_offset":21188,"parent_record_offset":21136,"ancestor_record_offset":21136,"inlinee":6081,"source_line":51,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::impl$0::from_winit
// 记录形式: compiled_procedure; instances=1
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":182,"record_offset":45388,"rva":3060896,"va":5371770016}
// 编译实例 RVAs: 0x2eb4a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1495050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e80ad01600","size":5,"soft_fail":false,"target_va":5373265456,"va":5371770401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1494935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e897cf1600","size":5,"soft_fail":false,"target_va":5373265408,"va":5371770468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=3
//     target_aliases=89 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t543818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e84a4c0800","size":5,"soft_fail":false,"target_va":5372314208,"va":5371770385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t979460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e804f20e00","size":5,"soft_fail":false,"target_va":5372749696,"va":5371770231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t979368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e8a8f10e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371770339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3400874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e8aae43300","size":5,"soft_fail":false,"target_va":5375171056,"va":5371770177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: setup.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31823068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e8dc94e501","size":5,"soft_fail":false,"target_va":5403593360,"va":5371770287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31207883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e8cb31dc01","size":5,"soft_fail":false,"target_va":5402978144,"va":5371770256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5756399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e8efd55700","size":5,"soft_fail":false,"target_va":5377527120,"va":5371770716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31209471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e8ff37dc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371770652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5745755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e85bac5700","size":5,"soft_fail":false,"target_va":5377515952,"va":5371770192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1495762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e8d2d21600","size":5,"soft_fail":false,"target_va":5373265840,"va":5371770073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::decode_wide
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1470754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371770016,"raw_hex":"e822711600","size":5,"soft_fail":false,"target_va":5373240928,"va":5371770169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":182,"record_offset":47524,"parent_record_offset":45388,"ancestor_record_offset":45388,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":182,"record_offset":48112,"parent_record_offset":45388,"ancestor_record_offset":45388,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":182,"record_offset":45528,"parent_record_offset":45388,"ancestor_record_offset":45388,"inlinee":8682,"source_line":112,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":182,"record_offset":47372,"parent_record_offset":45388,"ancestor_record_offset":45388,"inlinee":8211,"source_line":125,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":182,"record_offset":47736,"parent_record_offset":45388,"ancestor_record_offset":45388,"inlinee":8207,"source_line":118,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":182,"record_offset":46168,"parent_record_offset":46144,"ancestor_record_offset":45388,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":182,"record_offset":46192,"parent_record_offset":46168,"ancestor_record_offset":45388,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":182,"record_offset":46540,"parent_record_offset":46516,"ancestor_record_offset":45388,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":182,"record_offset":46144,"parent_record_offset":46100,"ancestor_record_offset":45388,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":182,"record_offset":46516,"parent_record_offset":46492,"ancestor_record_offset":45388,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":182,"record_offset":46784,"parent_record_offset":46748,"ancestor_record_offset":45388,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":182,"record_offset":46100,"parent_record_offset":46064,"ancestor_record_offset":45388,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":182,"record_offset":46064,"parent_record_offset":46028,"ancestor_record_offset":45388,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":182,"record_offset":46264,"parent_record_offset":46064,"ancestor_record_offset":45388,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":182,"record_offset":46492,"parent_record_offset":46468,"ancestor_record_offset":45388,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":182,"record_offset":45988,"parent_record_offset":45952,"ancestor_record_offset":45388,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":182,"record_offset":45952,"parent_record_offset":45916,"ancestor_record_offset":45388,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":182,"record_offset":45880,"parent_record_offset":45844,"ancestor_record_offset":45388,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":182,"record_offset":45916,"parent_record_offset":45880,"ancestor_record_offset":45388,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":182,"record_offset":45844,"parent_record_offset":45808,"ancestor_record_offset":45388,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":182,"record_offset":45808,"parent_record_offset":45772,"ancestor_record_offset":45388,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$38::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":182,"record_offset":45772,"parent_record_offset":45568,"ancestor_record_offset":45388,"inlinee":8685,"source_line":2649,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":182,"record_offset":46028,"parent_record_offset":45988,"ancestor_record_offset":45388,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":182,"record_offset":47612,"parent_record_offset":47584,"ancestor_record_offset":45388,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$77::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":182,"record_offset":47636,"parent_record_offset":47612,"ancestor_record_offset":45388,"inlinee":8217,"source_line":76,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":182,"record_offset":47984,"parent_record_offset":47960,"ancestor_record_offset":45388,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":182,"record_offset":46332,"parent_record_offset":46308,"ancestor_record_offset":45388,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":182,"record_offset":46308,"parent_record_offset":45988,"ancestor_record_offset":45388,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":182,"record_offset":46392,"parent_record_offset":45568,"ancestor_record_offset":45388,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":182,"record_offset":46220,"parent_record_offset":46192,"ancestor_record_offset":45388,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":182,"record_offset":47428,"parent_record_offset":47396,"ancestor_record_offset":45388,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":182,"record_offset":45644,"parent_record_offset":45568,"ancestor_record_offset":45388,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":182,"record_offset":47460,"parent_record_offset":47396,"ancestor_record_offset":45388,"inlinee":8213,"source_line":1407,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":182,"record_offset":45744,"parent_record_offset":45568,"ancestor_record_offset":45388,"inlinee":6742,"source_line":865,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":182,"record_offset":45712,"parent_record_offset":45688,"ancestor_record_offset":45388,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":182,"record_offset":45688,"parent_record_offset":45568,"ancestor_record_offset":45388,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":182,"record_offset":47584,"parent_record_offset":47552,"ancestor_record_offset":45388,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$19::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":182,"record_offset":47552,"parent_record_offset":47524,"ancestor_record_offset":45388,"inlinee":8215,"source_line":308,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":182,"record_offset":47904,"parent_record_offset":47872,"ancestor_record_offset":45388,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":182,"record_offset":45568,"parent_record_offset":45528,"ancestor_record_offset":45388,"inlinee":8683,"source_line":165,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::position
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":182,"record_offset":47396,"parent_record_offset":47372,"ancestor_record_offset":45388,"inlinee":8212,"source_line":211,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":182,"record_offset":47776,"parent_record_offset":47736,"ancestor_record_offset":45388,"inlinee":8209,"source_line":185,"archive":"_pdb/A/modules/00182.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::setup::impl$1::from_winit
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=8
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17BF
// 首条位置: {"module":21,"inlinee":6079}
// 编译实例 RVAs: 0x2eb900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1494236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371771136,"raw_hex":"e8dccc1600","size":5,"soft_fail":false,"target_va":5373265408,"va":5371771167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::VideoMode::size
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1494490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371771136,"raw_hex":"e8dacd1600","size":5,"soft_fail":false,"target_va":5373265648,"va":5371771153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":73,"record_offset":73600,"parent_record_offset":73544,"ancestor_record_offset":73544,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::VideoMode::size
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":73,"record_offset":73572,"parent_record_offset":73544,"ancestor_record_offset":73544,"inlinee":6081,"source_line":51,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":73,"record_offset":73676,"parent_record_offset":73652,"ancestor_record_offset":73544,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$82::from
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":73,"record_offset":73700,"parent_record_offset":73676,"ancestor_record_offset":73544,"inlinee":6087,"source_line":76,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=11
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":73,"record_offset":73652,"parent_record_offset":73624,"ancestor_record_offset":73544,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":73,"record_offset":73624,"parent_record_offset":73600,"ancestor_record_offset":73544,"inlinee":6083,"source_line":466,"archive":"_pdb/A/modules/00073.jsonl.gz"}
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
// 完整结构及原始类型记录: setup.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: setup.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: setup.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 14; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::find_best_fullscreen_video_mode
// 信息 ID: 2cdcbe8333f1e42084bc8386
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::setup_window @ 0xec836 -> 0xed770 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24c75c -> 0xed770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::find_best_fullscreen_video_mode::closure$0
// 信息 ID: d1aa7d9ed8a3440aa43bffc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::setup::find_best_fullscreen_video_mode", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::setup::find_best_fullscreen_video_mode; module=76, inline_record=26048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::find_best_fullscreen_video_mode::closure$1
// 信息 ID: f33610ab590af130d40ac740
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d96c03f858954aa9834a31e1: core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::setup::find_best_fullscreen_video_mode", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::setup::find_best_fullscreen_video_mode::closure_env$1 <- core::iter::adapters::map::impl$2::fold<tuple$<i64,winit::monitor::VideoMode>,alloc::vec::into_iter::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<winit::monitor::VideoMode,i64,tiny_glade::setup::find_best_fullscreen_video_mode::closure_env$1>,tuple$<i64,winit::monitor::VideoMode>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,winit::monitor::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::setup::find_best_fullscreen_video_mode::closure_env$1 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<winit::monitor::VideoMode,i64,tiny_glade::setup::find_best_fullscreen_video_mode::closure_env$1> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<i64,winit::monitor::VideoMode>,alloc::vec::into_iter::IntoIter<winit::monitor::VideoMode,alloc::alloc::Global>,core::iter::traits::iterator::Iterator::min_by_key::key::closure_env$0<winit::monitor::VideoMode,i64,tiny_glade::setup::find_best_fullscreen_video_mode::closure_env$1>,tuple$<i64,winit::monitor::VideoMode>,core::iter::traits::iterator::Iterator::min_by::fold::closure_env$0<tuple$<i64,winit::monitor::VideoMode>,core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)> >; module=21, inline_record=21136; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::setup::find_best_fullscreen_video_mode; module=76, inline_record=24956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::impl$0::from_winit
// 信息 ID: c8632e7746197837b32fcb3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::setup::setup_window::closure$0::closure_env$0> > > @ 0x2bf2d -> 0x2eb4a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::game_loop::impl$0::apply_setting_changes::closure_env$0> > > @ 0x2c3df -> 0x2eb4a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0xec56b -> 0x2eb4a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::game_loop::GameLoop::extract_available_monitors_info @ 0x24fc6b -> 0x2eb4a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::impl$1::from_winit
// 信息 ID: e96370fe3f1f9337e96997f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d79336355d1adce860522d53: country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0xee53d -> 0x2eb900 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::render::render_config::VideoMode,core::iter::adapters::map::Map<core::slice::iter::Iter<winit::monitor::VideoMode>,country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)> >; module=73, inline_record=73544; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::setup::setup_window; module=76, inline_record=13292; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=127, inline_record=22132; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::setup_window
// 信息 ID: f0386cd37498b1fefe41ca1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c57e3 -> 0xebf00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::setup_window::closure$0
// 信息 ID: 3ed7d79bb4ea2ce0604e35a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::setup::setup_window", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::setup::setup_window; module=76, inline_record=10492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::setup_window::closure$0::closure$0
// 信息 ID: 74ffbdb2291eefda99ab008a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::setup::setup_window", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::setup::setup_window::closure$0::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::setup::setup_window::closure$0::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::setup::setup_window::closure$0::closure_env$0 <- alloc::collections::vec_deque::into_iter::impl$2::try_fold<winit::platform_impl::platform::monitor::MonitorHandle,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::setup::setup_window::closure$0::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<>,winit::platform_impl::platform::monitor::MonitorHandle>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::event_loop::impl$9::available_monitors::closure_env$0<tuple$<> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::setup::setup_window::closure$0::closure_env$0> > >; module=21, inline_record=30516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::setup::setup_window::closure$1
// 信息 ID: edd0226b7eccb3ee06a2b26c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::setup::setup_window", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::setup::setup_window; module=76, inline_record=11400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

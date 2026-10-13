// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/integrity_report_main.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 36e3c4695703c9af5da2fba56b944ee0a0f822b469fa79381a46b06ca9b958e5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: integrity_report_main.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 200
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// use winit; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::integrity_report_main::verify_package_integrity_and_show_report
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":200,"record_offset":11580,"rva":3178224,"va":5371887344}
// 编译实例 RVAs: 0x307ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1375753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e809fe1400","size":5,"soft_fail":false,"target_va":5373265408,"va":5371889650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..PlatformSpecificWindowBuilderAttributes$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1362721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e821cb1400","size":5,"soft_fail":false,"target_va":5373252112,"va":5371889386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400056b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3160148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e8acc7cfff","size":5,"soft_fail":false,"target_va":5368731312,"va":5371891455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d84b0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2296144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e8b0f6dcff","size":5,"soft_fail":false,"target_va":5369595056,"va":5371891195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t357799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e8a7750500","size":5,"soft_fail":false,"target_va":5372249392,"va":5371891588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t383728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e8f0da0500","size":5,"soft_fail":false,"target_va":5372274416,"va":5371890683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t423335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e8a7750600","size":5,"soft_fail":false,"target_va":5372314208,"va":5371890868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403706c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t423968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e820780600","size":5,"soft_fail":false,"target_va":5372315328,"va":5371891355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140375190
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t443360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e8e0c30600","size":5,"soft_fail":false,"target_va":5372334480,"va":5371891115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140377f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t455216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e830f20600","size":5,"soft_fail":false,"target_va":5372346176,"va":5371890955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140383620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t502684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e89cab0700","size":5,"soft_fail":false,"target_va":5372392992,"va":5371890303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t860501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e855210d00","size":5,"soft_fail":false,"target_va":5372749696,"va":5371889190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t860600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e8b8210d00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371889107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: integrity_report_main.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31703944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e888c3e301","size":5,"soft_fail":false,"target_va":5403593360,"va":5371889411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31087698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e8525cda01","size":5,"soft_fail":false,"target_va":5402978144,"va":5371890441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31089709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e82d64da01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371890414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t5466154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e82a685300","size":5,"soft_fail":false,"target_va":5377353776,"va":5371887617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31085351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e82753da01","size":5,"soft_fail":false,"target_va":5402975328,"va":5371889972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31085550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e8ee53da01","size":5,"soft_fail":false,"target_va":5402975504,"va":5371889949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31085632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e84054da01","size":5,"soft_fail":false,"target_va":5402975696,"va":5371890059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5556277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e835c85400","size":5,"soft_fail":false,"target_va":5377443760,"va":5371887478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5555554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e862c55400","size":5,"soft_fail":false,"target_va":5377442944,"va":5371887385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_integrity_report::run_integrity_report_gui
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t942930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e852630e00","size":5,"soft_fail":false,"target_va":5372833216,"va":5371890281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_package_integrity::verify_package_manifest
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1252278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e8b61b1300","size":5,"soft_fail":false,"target_va":5373139696,"va":5371887413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t304974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e84ea70400","size":5,"soft_fail":false,"target_va":5372193920,"va":5371888941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::init
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1182174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371887344,"raw_hex":"e822f6edff","size":5,"soft_fail":false,"target_va":5370707680,"va":5371889849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5371887344,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5371889078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":200,"record_offset":16760,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":200,"record_offset":12472,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":200,"record_offset":11884,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":200,"record_offset":12416,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":200,"record_offset":11912,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":200,"record_offset":12784,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":200,"record_offset":16692,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":200,"record_offset":13676,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":200,"record_offset":11940,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":200,"record_offset":11800,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::EventLoop::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":200,"record_offset":13036,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":8182,"source_line":200,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] winit::event_loop::impl$8::deref
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":200,"record_offset":15400,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":8226,"source_line":305,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::build
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":200,"record_offset":15428,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":8227,"source_line":508,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":200,"record_offset":13776,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":8190,"source_line":205,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_inner_size
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":200,"record_offset":15216,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":8196,"source_line":222,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] winit::window::WindowBuilder::with_title
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":200,"record_offset":14404,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":8195,"source_line":307,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":11580,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":11580,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":13476,"parent_record_offset":13452,"ancestor_record_offset":11580,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":16832,"parent_record_offset":16796,"ancestor_record_offset":11580,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":14156,"parent_record_offset":14128,"ancestor_record_offset":11580,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":13452,"parent_record_offset":13368,"ancestor_record_offset":11580,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":200,"record_offset":16796,"parent_record_offset":16760,"ancestor_record_offset":11580,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":200,"record_offset":13368,"parent_record_offset":13340,"ancestor_record_offset":11580,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":15112,"parent_record_offset":15084,"ancestor_record_offset":11580,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":14128,"parent_record_offset":14100,"ancestor_record_offset":11580,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":14100,"parent_record_offset":14072,"ancestor_record_offset":11580,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":14276,"parent_record_offset":14100,"ancestor_record_offset":11580,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":15084,"parent_record_offset":15056,"ancestor_record_offset":11580,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":200,"record_offset":14036,"parent_record_offset":14008,"ancestor_record_offset":11580,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":200,"record_offset":14008,"parent_record_offset":13980,"ancestor_record_offset":11580,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":200,"record_offset":13952,"parent_record_offset":13924,"ancestor_record_offset":11580,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":200,"record_offset":13980,"parent_record_offset":13952,"ancestor_record_offset":11580,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":200,"record_offset":13924,"parent_record_offset":13896,"ancestor_record_offset":11580,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":200,"record_offset":13896,"parent_record_offset":13860,"ancestor_record_offset":11580,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":200,"record_offset":14476,"parent_record_offset":14448,"ancestor_record_offset":11580,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":200,"record_offset":15656,"parent_record_offset":15632,"ancestor_record_offset":11580,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":200,"record_offset":15632,"parent_record_offset":15592,"ancestor_record_offset":11580,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":200,"record_offset":14072,"parent_record_offset":14036,"ancestor_record_offset":11580,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":200,"record_offset":13396,"parent_record_offset":13368,"ancestor_record_offset":11580,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":200,"record_offset":14448,"parent_record_offset":14404,"ancestor_record_offset":11580,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":200,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":11580,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":200,"record_offset":13420,"parent_record_offset":13396,"ancestor_record_offset":11580,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":200,"record_offset":14312,"parent_record_offset":14036,"ancestor_record_offset":11580,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":200,"record_offset":13248,"parent_record_offset":13036,"ancestor_record_offset":11580,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":200,"record_offset":15680,"parent_record_offset":15656,"ancestor_record_offset":11580,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":200,"record_offset":14232,"parent_record_offset":14204,"ancestor_record_offset":11580,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":200,"record_offset":15836,"parent_record_offset":15776,"ancestor_record_offset":11580,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":200,"record_offset":16184,"parent_record_offset":15592,"ancestor_record_offset":11580,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":200,"record_offset":13212,"parent_record_offset":13128,"ancestor_record_offset":11580,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":200,"record_offset":16660,"parent_record_offset":15428,"ancestor_record_offset":11580,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":16104,"parent_record_offset":16076,"ancestor_record_offset":11580,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":16464,"parent_record_offset":16288,"ancestor_record_offset":11580,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":13156,"parent_record_offset":13128,"ancestor_record_offset":11580,"inlinee":8187,"source_line":669,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":15776,"parent_record_offset":15748,"ancestor_record_offset":11580,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":16552,"parent_record_offset":16524,"ancestor_record_offset":11580,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":11580,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":15808,"parent_record_offset":15776,"ancestor_record_offset":11580,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":11848,"parent_record_offset":11824,"ancestor_record_offset":11580,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":16488,"parent_record_offset":16464,"ancestor_record_offset":11580,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":13180,"parent_record_offset":13156,"ancestor_record_offset":11580,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":200,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":11580,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":200,"record_offset":15956,"parent_record_offset":15932,"ancestor_record_offset":11580,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":200,"record_offset":15716,"parent_record_offset":15592,"ancestor_record_offset":11580,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":200,"record_offset":15872,"parent_record_offset":15716,"ancestor_record_offset":11580,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":200,"record_offset":16256,"parent_record_offset":16228,"ancestor_record_offset":11580,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":200,"record_offset":16288,"parent_record_offset":16256,"ancestor_record_offset":11580,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":200,"record_offset":16076,"parent_record_offset":15900,"ancestor_record_offset":11580,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":200,"record_offset":15900,"parent_record_offset":15872,"ancestor_record_offset":11580,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":200,"record_offset":15748,"parent_record_offset":15716,"ancestor_record_offset":11580,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":200,"record_offset":16524,"parent_record_offset":16256,"ancestor_record_offset":11580,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":200,"record_offset":15932,"parent_record_offset":15900,"ancestor_record_offset":11580,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::PhysicalSize::cast
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":200,"record_offset":15300,"parent_record_offset":15276,"ancestor_record_offset":11580,"inlinee":8200,"source_line":439,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":200,"record_offset":15328,"parent_record_offset":15300,"ancestor_record_offset":11580,"inlinee":6085,"source_line":109,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$33::from
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/dpi.rs
//     evidence={"module":200,"record_offset":15276,"parent_record_offset":15252,"ancestor_record_offset":11580,"inlinee":8198,"source_line":531,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::build
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":200,"record_offset":13128,"parent_record_offset":13036,"ancestor_record_offset":11580,"inlinee":8186,"source_line":120,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":200,"record_offset":13072,"parent_record_offset":13036,"ancestor_record_offset":11580,"inlinee":8184,"source_line":69,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::event_loop::EventLoopBuilder::with_user_event
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/event_loop.rs
//     evidence={"module":200,"record_offset":13096,"parent_record_offset":13072,"ancestor_record_offset":11580,"inlinee":8185,"source_line":80,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::hwnd
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":200,"record_offset":16620,"parent_record_offset":15592,"ancestor_record_offset":11580,"inlinee":8244,"source_line":344,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":200,"record_offset":15524,"parent_record_offset":15428,"ancestor_record_offset":11580,"inlinee":8230,"source_line":108,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":200,"record_offset":15488,"parent_record_offset":15428,"ancestor_record_offset":11580,"inlinee":8229,"source_line":96,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::request_redraw
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/window.rs
//     evidence={"module":200,"record_offset":15592,"parent_record_offset":15560,"ancestor_record_offset":11580,"inlinee":8233,"source_line":160,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$30::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":200,"record_offset":13812,"parent_record_offset":13776,"ancestor_record_offset":11580,"inlinee":8192,"source_line":116,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$6::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":200,"record_offset":13860,"parent_record_offset":13812,"ancestor_record_offset":11580,"inlinee":8194,"source_line":162,"archive":"_pdb/A/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$9::build::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/window.rs
//     evidence={"module":200,"record_offset":15560,"parent_record_offset":15524,"ancestor_record_offset":11580,"inlinee":8232,"source_line":514,"archive":"_pdb/A/modules/00200.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]

// ------------------------------------------------------------------
// 函数: tiny_glade::integrity_report_main::verify_package_integrity_and_show_report
// 信息 ID: ad4d8ca86c348aa99fe37cf3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x39a1b6 -> 0x307ef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

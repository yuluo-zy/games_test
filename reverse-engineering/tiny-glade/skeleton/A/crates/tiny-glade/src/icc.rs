// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/icc.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 1b8d882b44a8b98d5c49af200ae609155fd46c977aa529947fdeac5575793cb7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: icc.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 191
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use winit; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::icc::get_monitor_icc_profile
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":191,"record_offset":3340,"rva":3098928,"va":5371808048}
// 编译实例 RVAs: 0x2f4930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t503658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e86aaf0700","size":5,"soft_fail":false,"target_va":5372314208,"va":5371810545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t941406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e85e5d0e00","size":5,"soft_fail":false,"target_va":5372749696,"va":5371808285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t941338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e81a5d0e00","size":5,"soft_fail":false,"target_va":5372749712,"va":5371808369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t940987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e8bb5b0e00","size":5,"soft_fail":false,"target_va":5372749744,"va":5371808752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3362820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e804503300","size":5,"soft_fail":false,"target_va":5375171056,"va":5371808231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144650
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31783533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e86dfae401","size":5,"soft_fail":false,"target_va":5403592272,"va":5371808734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144660
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31783164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e8fcf8e401","size":5,"soft_fail":false,"target_va":5403592288,"va":5371809119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144670
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31783489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e841fae401","size":5,"soft_fail":false,"target_va":5403592304,"va":5371808810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31785037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e84d00e501","size":5,"soft_fail":false,"target_va":5403593360,"va":5371808318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31168460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e8cc97db01","size":5,"soft_fail":false,"target_va":5402978144,"va":5371809679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::ffi::c_str::CString::_from_vec_unchecked
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5719669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e875465700","size":5,"soft_fail":false,"target_va":5377528368,"va":5371808694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::ffi::c_str::CString::from_vec_with_nul
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5720246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e8b6485700","size":5,"soft_fail":false,"target_va":5377529152,"va":5371808901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::ffi::c_str::CString::into_string
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5719901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e85d475700","size":5,"soft_fail":false,"target_va":5377528848,"va":5371808942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5717422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e8ae3d5700","size":5,"soft_fail":false,"target_va":5377527120,"va":5371809693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31136101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e86519db01","size":5,"soft_fail":false,"target_va":5402945184,"va":5371809078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h89ec121691a064e6 (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31139440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e87026db01","size":5,"soft_fail":false,"target_va":5402948816,"va":5371809371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::hd1db36e2d07f0ff7 (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31139930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e85a28db01","size":5,"soft_fail":false,"target_va":5402949472,"va":5371809537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3078291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e86d07d1ff","size":5,"soft_fail":false,"target_va":5368732208,"va":5371810494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31171498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e8aaa3db01","size":5,"soft_fail":false,"target_va":5402980128,"va":5371808625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5781071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e84f365800","size":5,"soft_fail":false,"target_va":5377589536,"va":5371808460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5586303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e87f3d5500","size":5,"soft_fail":false,"target_va":5377395552,"va":5371809244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5707701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e8b5175700","size":5,"soft_fail":false,"target_va":5377515952,"va":5371808246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1457735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e8473e1600","size":5,"soft_fail":false,"target_va":5373265840,"va":5371808100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::decode_wide
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1432704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371808048,"raw_hex":"e880dc1500","size":5,"soft_fail":false,"target_va":5373240928,"va":5371808219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":191,"record_offset":5384,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":9051,"source_line":263,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":191,"record_offset":6172,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":8655,"source_line":1192,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":191,"record_offset":5932,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":191,"record_offset":6200,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":191,"record_offset":6828,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":6856,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":5692,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":6800,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":7168,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/monitor.rs
//     evidence={"module":191,"record_offset":3500,"parent_record_offset":3340,"ancestor_record_offset":3340,"inlinee":8682,"source_line":112,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":3340,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":4208,"parent_record_offset":4184,"ancestor_record_offset":3340,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":3340,"inlinee":6014,"source_line":171,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":3340,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":4160,"parent_record_offset":4112,"ancestor_record_offset":3340,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":6044,"parent_record_offset":6016,"ancestor_record_offset":3340,"inlinee":6013,"source_line":247,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":191,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":3340,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":191,"record_offset":6692,"parent_record_offset":6668,"ancestor_record_offset":3340,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$1::new::impl$0::spec_new_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":191,"record_offset":5424,"parent_record_offset":5384,"ancestor_record_offset":3340,"inlinee":7950,"source_line":269,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":191,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":3340,"inlinee":8650,"source_line":703,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":4796,"parent_record_offset":4760,"ancestor_record_offset":3340,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":4112,"parent_record_offset":4072,"ancestor_record_offset":3340,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":4072,"parent_record_offset":4032,"ancestor_record_offset":3340,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":5988,"parent_record_offset":5960,"ancestor_record_offset":3340,"inlinee":6012,"source_line":169,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":4280,"parent_record_offset":4072,"ancestor_record_offset":3340,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":191,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":3340,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":191,"record_offset":3988,"parent_record_offset":3948,"ancestor_record_offset":3340,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":191,"record_offset":3948,"parent_record_offset":3908,"ancestor_record_offset":3340,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":191,"record_offset":3868,"parent_record_offset":3828,"ancestor_record_offset":3340,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":191,"record_offset":3908,"parent_record_offset":3868,"ancestor_record_offset":3340,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":191,"record_offset":3828,"parent_record_offset":3788,"ancestor_record_offset":3340,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":191,"record_offset":3788,"parent_record_offset":3748,"ancestor_record_offset":3340,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$38::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":191,"record_offset":3748,"parent_record_offset":3548,"ancestor_record_offset":3340,"inlinee":8685,"source_line":2649,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":191,"record_offset":4032,"parent_record_offset":3988,"ancestor_record_offset":3340,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":191,"record_offset":5960,"parent_record_offset":5932,"ancestor_record_offset":3340,"inlinee":6032,"source_line":50,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":191,"record_offset":6276,"parent_record_offset":6240,"ancestor_record_offset":3340,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":191,"record_offset":6240,"parent_record_offset":6200,"ancestor_record_offset":3340,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":191,"record_offset":4344,"parent_record_offset":4320,"ancestor_record_offset":3340,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":191,"record_offset":4320,"parent_record_offset":3988,"ancestor_record_offset":3340,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":4404,"parent_record_offset":3548,"ancestor_record_offset":3340,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":191,"record_offset":4236,"parent_record_offset":4208,"ancestor_record_offset":3340,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":191,"record_offset":3624,"parent_record_offset":3548,"ancestor_record_offset":3340,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\memchr.rs
//     evidence={"module":191,"record_offset":5468,"parent_record_offset":5424,"ancestor_record_offset":3340,"inlinee":5066,"source_line":27,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\memchr.rs
//     evidence={"module":191,"record_offset":5500,"parent_record_offset":5468,"ancestor_record_offset":3340,"inlinee":5067,"source_line":38,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":3720,"parent_record_offset":3548,"ancestor_record_offset":3340,"inlinee":6742,"source_line":865,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":3340,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":191,"record_offset":3664,"parent_record_offset":3548,"ancestor_record_offset":3340,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00191.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/winit-0d8106a45b12a3fb/26ba5f0/src/platform_impl/windows/monitor.rs
//     evidence={"module":191,"record_offset":3548,"parent_record_offset":3500,"ancestor_record_offset":3340,"inlinee":8683,"source_line":165,"archive":"_pdb/A/modules/00191.jsonl.gz"}
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
// 完整结构及原始类型记录: icc.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: icc.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: icc.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::icc::get_monitor_icc_profile
// 信息 ID: 8599c0f8fb8e877d830974af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::event_loop::run_event_loop @ 0x30f108 -> 0x2f4930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

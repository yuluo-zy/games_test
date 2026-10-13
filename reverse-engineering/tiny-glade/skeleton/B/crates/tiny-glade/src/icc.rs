// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/icc.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1b8d882b44a8b98d5c49af200ae609155fd46c977aa529947fdeac5575793cb7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: icc.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 157
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use winit; // namespace candidate
//   package identities: [{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::icc::get_monitor_icc_profile
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":157,"record_offset":10208,"rva":3851264,"va":5372560384}
// 编译实例 RVAs: 0x3ac400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2338207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e89fad2300","size":5,"soft_fail":false,"target_va":5374900736,"va":5372562524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2520882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e832772600","size":5,"soft_fail":false,"target_va":5375081488,"va":5372560601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2520819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e8f3762600","size":5,"soft_fail":false,"target_va":5375081504,"va":5372560680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2520674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e862762600","size":5,"soft_fail":false,"target_va":5375081536,"va":5372560857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10267181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e82daa9c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5372560542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38754640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e850594f02","size":5,"soft_fail":false,"target_va":5411315744,"va":5372561099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38757318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e8c6634f02","size":5,"soft_fail":false,"target_va":5411318720,"va":5372561397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38760591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e88f704f02","size":5,"soft_fail":false,"target_va":5411322208,"va":5372561612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923470
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39284395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e8ab6e5702","size":5,"soft_fail":false,"target_va":5411845232,"va":5372560832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923480
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39284195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e8e36d5702","size":5,"soft_fail":false,"target_va":5411845248,"va":5372561048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923490
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39284348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e87c6e5702","size":5,"soft_fail":false,"target_va":5411845264,"va":5372560911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: icc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39286528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e800775702","size":5,"soft_fail":false,"target_va":5411847168,"va":5372560635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::ffi::c_str::CString::_from_vec_unchecked
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10277793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e8a1d39c00","size":5,"soft_fail":false,"target_va":5382838608,"va":5372560810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::ffi::c_str::CString::from_vec_with_nul
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10278536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e888d69c00","size":5,"soft_fail":false,"target_va":5382839520,"va":5372560979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::ffi::c_str::CString::into_string
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10278045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e89dd49c00","size":5,"soft_fail":false,"target_va":5382839056,"va":5372561006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38979154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e852c65202","size":5,"soft_fail":false,"target_va":5411540963,"va":5372561804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38981150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e81ece5202","size":5,"soft_fail":false,"target_va":5411542880,"va":5372561725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10339527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e8c7c49d00","size":5,"soft_fail":false,"target_va":5382900288,"va":5372560756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10133369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e8799f9a00","size":5,"soft_fail":false,"target_va":5382694656,"va":5372561282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10268302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e88eae9c00","size":5,"soft_fail":false,"target_va":5382828864,"va":5372560557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3061471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e8dfb62e00","size":5,"soft_fail":false,"target_va":5375621904,"va":5372560428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::util::decode_wide
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t3053381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372560384,"raw_hex":"e845972e00","size":5,"soft_fail":false,"target_va":5375613920,"va":5372560534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":157,"record_offset":12412,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":7851,"source_line":257,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":12984,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":4915,"source_line":1524,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":12736,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":13012,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":157,"record_offset":13676,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":13704,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":14288,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":13648,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":14044,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":157,"record_offset":10368,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":6014,"source_line":112,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":11188,"parent_record_offset":11164,"ancestor_record_offset":10208,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":11212,"parent_record_offset":11188,"ancestor_record_offset":10208,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":12928,"parent_record_offset":12904,"ancestor_record_offset":10208,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":11524,"parent_record_offset":11500,"ancestor_record_offset":10208,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":11164,"parent_record_offset":10980,"ancestor_record_offset":10208,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":12880,"parent_record_offset":12852,"ancestor_record_offset":10208,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":157,"record_offset":11500,"parent_record_offset":11476,"ancestor_record_offset":10208,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":157,"record_offset":13540,"parent_record_offset":13516,"ancestor_record_offset":10208,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$1::new::impl$0::spec_new_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":157,"record_offset":12444,"parent_record_offset":12412,"ancestor_record_offset":10208,"inlinee":6455,"source_line":263,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":157,"record_offset":13488,"parent_record_offset":13464,"ancestor_record_offset":10208,"inlinee":7853,"source_line":696,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":10912,"parent_record_offset":10880,"ancestor_record_offset":10208,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":12792,"parent_record_offset":12764,"ancestor_record_offset":10208,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":11780,"parent_record_offset":11748,"ancestor_record_offset":10208,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":10208,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":10980,"parent_record_offset":10944,"ancestor_record_offset":10208,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":10944,"parent_record_offset":10912,"ancestor_record_offset":10208,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":12820,"parent_record_offset":12792,"ancestor_record_offset":10208,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":11452,"parent_record_offset":11428,"ancestor_record_offset":10208,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":157,"record_offset":11024,"parent_record_offset":10980,"ancestor_record_offset":10208,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":10840,"parent_record_offset":10808,"ancestor_record_offset":10208,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":10808,"parent_record_offset":10776,"ancestor_record_offset":10208,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":10744,"parent_record_offset":10712,"ancestor_record_offset":10208,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":10776,"parent_record_offset":10744,"ancestor_record_offset":10208,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":157,"record_offset":10712,"parent_record_offset":10680,"ancestor_record_offset":10208,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":157,"record_offset":10680,"parent_record_offset":10648,"ancestor_record_offset":10208,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":157,"record_offset":10616,"parent_record_offset":10416,"ancestor_record_offset":10208,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$41::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":157,"record_offset":10648,"parent_record_offset":10616,"ancestor_record_offset":10208,"inlinee":6020,"source_line":2904,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":157,"record_offset":10880,"parent_record_offset":10840,"ancestor_record_offset":10208,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":157,"record_offset":12764,"parent_record_offset":12736,"ancestor_record_offset":10208,"inlinee":5369,"source_line":53,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":157,"record_offset":13092,"parent_record_offset":13052,"ancestor_record_offset":10208,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":157,"record_offset":13052,"parent_record_offset":13012,"ancestor_record_offset":10208,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":11048,"parent_record_offset":11024,"ancestor_record_offset":10208,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":157,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":10208,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":11288,"parent_record_offset":11264,"ancestor_record_offset":10208,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":157,"record_offset":11120,"parent_record_offset":11096,"ancestor_record_offset":10208,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":157,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":10208,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":157,"record_offset":11264,"parent_record_offset":10840,"ancestor_record_offset":10208,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":157,"record_offset":11352,"parent_record_offset":10416,"ancestor_record_offset":10208,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":157,"record_offset":10492,"parent_record_offset":10416,"ancestor_record_offset":10208,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\memchr.rs
//     evidence={"module":157,"record_offset":12484,"parent_record_offset":12444,"ancestor_record_offset":10208,"inlinee":6428,"source_line":26,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\memchr.rs
//     evidence={"module":157,"record_offset":12516,"parent_record_offset":12484,"ancestor_record_offset":10208,"inlinee":6429,"source_line":36,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":10588,"parent_record_offset":10416,"ancestor_record_offset":10208,"inlinee":6016,"source_line":906,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":10208,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":157,"record_offset":10532,"parent_record_offset":10416,"ancestor_record_offset":10208,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00157.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":157,"record_offset":10416,"parent_record_offset":10368,"ancestor_record_offset":10208,"inlinee":6015,"source_line":165,"archive":"_pdb/B/modules/00157.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::icc::get_monitor_icc_profile
// 信息 ID: bd918b466fa4bab6143dbc7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::event_loop::run_event_loop @ 0x26f613 -> 0x3ac400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

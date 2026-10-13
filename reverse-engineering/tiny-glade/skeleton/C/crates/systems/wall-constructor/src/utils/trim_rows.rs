// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/utils/trim_rows.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 3bf0f8ebd9d0630d4f6be7df1c1abb79f2b563b719488195fd531368eb7a136c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: trim_rows.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 428, 1455, 1537
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::trim_rows::trim_rows
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":428,"record_offset":1780,"rva":8661088,"va":5377370208}
// 编译实例 RVAs: 0x842860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1692773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e89b2be6ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377371456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-1693344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e86029e6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377372043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: trim_rows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8649256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e8d8057cff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377371443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141497bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trim_rows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12931237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e8a550c500","size":5,"soft_fail":false,"target_va":5390302144,"va":5377370902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bec0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: trim_rows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39686894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e8ee925d02","size":5,"soft_fail":false,"target_va":5417057984,"va":5377371085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38999053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e80d145302","size":5,"soft_fail":false,"target_va":5416370864,"va":5377371806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39248453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e845e25602","size":5,"soft_fail":false,"target_va":5416620794,"va":5377372336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39249133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e8ede45602","size":5,"soft_fail":false,"target_va":5416621408,"va":5377372270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e84b0b0000","size":5,"soft_fail":false,"target_va":5377373232,"va":5377370336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39248997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e865e45602","size":5,"soft_fail":false,"target_va":5416621376,"va":5377372374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6593109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e8559a6400","size":5,"soft_fail":false,"target_va":5383963792,"va":5377370678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6601613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e88dbb6400","size":5,"soft_fail":false,"target_va":5383972224,"va":5377370606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6592808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e828996400","size":5,"soft_fail":false,"target_va":5383964912,"va":5377372099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6592505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e8f9976400","size":5,"soft_fail":false,"target_va":5383962816,"va":5377370306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39324723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e8330c5802","size":5,"soft_fail":false,"target_va":5416697008,"va":5377372280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12397992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e8a82dbd00","size":5,"soft_fail":false,"target_va":5389771136,"va":5377373139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39247033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377370208,"raw_hex":"e8b9dc5602","size":5,"soft_fail":false,"target_va":5416619360,"va":5377372322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":6644,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":6382,"source_line":2922,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":4192,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":5900,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":5040,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":5380,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":428,"record_offset":4800,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":428,"record_offset":5800,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":428,"record_offset":6120,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":428,"record_offset":7996,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":428,"record_offset":6368,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":6824,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":428,"record_offset":4220,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":428,"record_offset":4676,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":428,"record_offset":4304,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":428,"record_offset":3384,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":428,"record_offset":1984,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":428,"record_offset":2372,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":428,"record_offset":4744,"parent_record_offset":1780,"ancestor_record_offset":1780,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":1780,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":1780,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":1780,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":7036,"parent_record_offset":7012,"ancestor_record_offset":1780,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":1780,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":7060,"parent_record_offset":7036,"ancestor_record_offset":1780,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":5668,"parent_record_offset":5520,"ancestor_record_offset":1780,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":428,"record_offset":6988,"parent_record_offset":6896,"ancestor_record_offset":1780,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":5996,"parent_record_offset":5964,"ancestor_record_offset":1780,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":5460,"parent_record_offset":5420,"ancestor_record_offset":1780,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":6924,"parent_record_offset":6896,"ancestor_record_offset":1780,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":6896,"parent_record_offset":6872,"ancestor_record_offset":1780,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":1780,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":1780,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":5520,"parent_record_offset":5488,"ancestor_record_offset":1780,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":5488,"parent_record_offset":5460,"ancestor_record_offset":1780,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":6872,"parent_record_offset":6848,"ancestor_record_offset":1780,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":5556,"parent_record_offset":5520,"ancestor_record_offset":1780,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":5964,"parent_record_offset":5928,"ancestor_record_offset":1780,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":6708,"parent_record_offset":6644,"ancestor_record_offset":1780,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":5928,"parent_record_offset":5900,"ancestor_record_offset":1780,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":1780,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":5344,"parent_record_offset":5064,"ancestor_record_offset":1780,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":5420,"parent_record_offset":5380,"ancestor_record_offset":1780,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":428,"record_offset":8120,"parent_record_offset":8024,"ancestor_record_offset":1780,"inlinee":5049,"source_line":155,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\drain.rs
//     evidence={"module":428,"record_offset":8212,"parent_record_offset":8144,"ancestor_record_offset":1780,"inlinee":5060,"source_line":156,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":5120,"parent_record_offset":5064,"ancestor_record_offset":1780,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":428,"record_offset":5580,"parent_record_offset":5556,"ancestor_record_offset":1780,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":428,"record_offset":3712,"parent_record_offset":3684,"ancestor_record_offset":1780,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":428,"record_offset":3740,"parent_record_offset":3712,"ancestor_record_offset":1780,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":428,"record_offset":3660,"parent_record_offset":3632,"ancestor_record_offset":1780,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":428,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":1780,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":428,"record_offset":3800,"parent_record_offset":3684,"ancestor_record_offset":1780,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":428,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":1780,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":428,"record_offset":6432,"parent_record_offset":6396,"ancestor_record_offset":1780,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":428,"record_offset":6460,"parent_record_offset":6432,"ancestor_record_offset":1780,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":428,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":1780,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":428,"record_offset":5176,"parent_record_offset":5064,"ancestor_record_offset":1780,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":428,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":1780,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":428,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":1780,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":428,"record_offset":4516,"parent_record_offset":4492,"ancestor_record_offset":1780,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":428,"record_offset":6256,"parent_record_offset":6120,"ancestor_record_offset":1780,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":428,"record_offset":6164,"parent_record_offset":6120,"ancestor_record_offset":1780,"inlinee":26654,"source_line":93,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":428,"record_offset":8024,"parent_record_offset":7996,"ancestor_record_offset":1780,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":428,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":1780,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":428,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":1780,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":428,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":1780,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":428,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":1780,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":428,"record_offset":6956,"parent_record_offset":6924,"ancestor_record_offset":1780,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":428,"record_offset":3532,"parent_record_offset":3492,"ancestor_record_offset":1780,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":428,"record_offset":6396,"parent_record_offset":6368,"ancestor_record_offset":1780,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":428,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":1780,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":428,"record_offset":6224,"parent_record_offset":6164,"ancestor_record_offset":1780,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":428,"record_offset":2320,"parent_record_offset":2292,"ancestor_record_offset":1780,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":428,"record_offset":4148,"parent_record_offset":3492,"ancestor_record_offset":1780,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":5316,"parent_record_offset":5064,"ancestor_record_offset":1780,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":3864,"parent_record_offset":3632,"ancestor_record_offset":1780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":428,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":1780,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":428,"record_offset":4412,"parent_record_offset":4344,"ancestor_record_offset":1780,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":428,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":1780,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":428,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":1780,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":428,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":1780,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":1780,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":6084,"parent_record_offset":5928,"ancestor_record_offset":1780,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":428,"record_offset":3076,"parent_record_offset":3048,"ancestor_record_offset":1780,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":428,"record_offset":3048,"parent_record_offset":3020,"ancestor_record_offset":1780,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":428,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":1780,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":428,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":1780,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":428,"record_offset":2744,"parent_record_offset":2720,"ancestor_record_offset":1780,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":428,"record_offset":3104,"parent_record_offset":3076,"ancestor_record_offset":1780,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":428,"record_offset":4344,"parent_record_offset":4304,"ancestor_record_offset":1780,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":428,"record_offset":6192,"parent_record_offset":6164,"ancestor_record_offset":1780,"inlinee":7948,"source_line":229,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":428,"record_offset":2696,"parent_record_offset":2580,"ancestor_record_offset":1780,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":428,"record_offset":8056,"parent_record_offset":8024,"ancestor_record_offset":1780,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":428,"record_offset":2112,"parent_record_offset":2088,"ancestor_record_offset":1780,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":428,"record_offset":2408,"parent_record_offset":2372,"ancestor_record_offset":1780,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":428,"record_offset":2548,"parent_record_offset":2516,"ancestor_record_offset":1780,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":428,"record_offset":2516,"parent_record_offset":2480,"ancestor_record_offset":1780,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":428,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":1780,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":428,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":1780,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":428,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":1780,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":428,"record_offset":3020,"parent_record_offset":2480,"ancestor_record_offset":1780,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":428,"record_offset":2088,"parent_record_offset":2056,"ancestor_record_offset":1780,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":428,"record_offset":2056,"parent_record_offset":2028,"ancestor_record_offset":1780,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":428,"record_offset":2212,"parent_record_offset":2180,"ancestor_record_offset":1780,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":428,"record_offset":2268,"parent_record_offset":2212,"ancestor_record_offset":1780,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":428,"record_offset":2180,"parent_record_offset":2152,"ancestor_record_offset":1780,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":428,"record_offset":2028,"parent_record_offset":1984,"ancestor_record_offset":1780,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":428,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":1780,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":428,"record_offset":2580,"parent_record_offset":2548,"ancestor_record_offset":1780,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":428,"record_offset":2864,"parent_record_offset":2580,"ancestor_record_offset":1780,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":428,"record_offset":2836,"parent_record_offset":2580,"ancestor_record_offset":1780,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":428,"record_offset":2640,"parent_record_offset":2580,"ancestor_record_offset":1780,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":428,"record_offset":4444,"parent_record_offset":4304,"ancestor_record_offset":1780,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":428,"record_offset":3420,"parent_record_offset":3384,"ancestor_record_offset":1780,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":428,"record_offset":7180,"parent_record_offset":7148,"ancestor_record_offset":1780,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":428,"record_offset":7632,"parent_record_offset":7428,"ancestor_record_offset":1780,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":428,"record_offset":3560,"parent_record_offset":3532,"ancestor_record_offset":1780,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":428,"record_offset":3632,"parent_record_offset":3492,"ancestor_record_offset":1780,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":428,"record_offset":3836,"parent_record_offset":3632,"ancestor_record_offset":1780,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":428,"record_offset":3588,"parent_record_offset":3560,"ancestor_record_offset":1780,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":428,"record_offset":3492,"parent_record_offset":3456,"ancestor_record_offset":1780,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":428,"record_offset":3456,"parent_record_offset":3420,"ancestor_record_offset":1780,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::trim_rows::trim_rows::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/trim_rows.rs
//     evidence={"module":428,"record_offset":5144,"parent_record_offset":5120,"ancestor_record_offset":1780,"inlinee":26653,"source_line":88,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::trim_rows::trim_rows::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x681D
// 首条位置: {"module":428,"inlinee":26653}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::trim_rows::impl$0::above
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6822
// 首条位置: {"module":1537,"inlinee":26658}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1537,"record_offset":656,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1537,"record_offset":900,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1537,"record_offset":872,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1537,"record_offset":540,"parent_record_offset":492,"ancestor_record_offset":492,"inlinee":26651,"source_line":157,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1537,"record_offset":748,"parent_record_offset":716,"ancestor_record_offset":492,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1537,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":492,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1537,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":492,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1537,"record_offset":716,"parent_record_offset":680,"ancestor_record_offset":492,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1537,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":492,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1537,"record_offset":620,"parent_record_offset":540,"ancestor_record_offset":492,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1537,"record_offset":836,"parent_record_offset":680,"ancestor_record_offset":492,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1537,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":492,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1537,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":492,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01537.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::trim_rows::impl$0::above_to_below
// 记录形式: compiled_procedure; instances=1
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":428,"record_offset":280,"rva":8660640,"va":5377369760}
// 编译实例 RVAs: 0x8426a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39251684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377369760,"raw_hex":"e8e4ee5602","size":5,"soft_fail":false,"target_va":5416621600,"va":5377369911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39252304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377369760,"raw_hex":"e850f15602","size":5,"soft_fail":false,"target_va":5416622208,"va":5377369899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39251834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377369760,"raw_hex":"e87aef5602","size":5,"soft_fail":false,"target_va":5416621762,"va":5377369923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5911736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377369760,"raw_hex":"e8b8345a00","size":5,"soft_fail":false,"target_va":5383281568,"va":5377369827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":428,"record_offset":912,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":428,"record_offset":884,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":428,"record_offset":576,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26649,"source_line":845,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":428,"record_offset":444,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26648,"source_line":1292,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":632,"parent_record_offset":576,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":428,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":428,"record_offset":536,"parent_record_offset":516,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":428,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":428,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":280,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":428,"record_offset":660,"parent_record_offset":632,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":428,"record_offset":688,"parent_record_offset":660,"ancestor_record_offset":280,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":428,"record_offset":604,"parent_record_offset":576,"ancestor_record_offset":280,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":428,"record_offset":728,"parent_record_offset":576,"ancestor_record_offset":280,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":428,"record_offset":852,"parent_record_offset":576,"ancestor_record_offset":280,"inlinee":26650,"source_line":136,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::trim_rows::impl$0::below_to_above
// 记录形式: compiled_procedure; instances=1
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":428,"record_offset":952,"rva":8660832,"va":5377369952}
// 编译实例 RVAs: 0x842760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trim_rows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38980304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377369952,"raw_hex":"e8d0ca5202","size":5,"soft_fail":false,"target_va":5416350384,"va":5377370075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39252043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377369952,"raw_hex":"e84bf05602","size":5,"soft_fail":false,"target_va":5416622208,"va":5377370160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39251585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377369952,"raw_hex":"e881ee5602","size":5,"soft_fail":false,"target_va":5416621762,"va":5377370172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5911517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377369952,"raw_hex":"e8dd335a00","size":5,"soft_fail":false,"target_va":5383281568,"va":5377370046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":1560,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":428,"record_offset":1252,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":26649,"source_line":845,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":428,"record_offset":1112,"parent_record_offset":952,"ancestor_record_offset":952,"inlinee":26648,"source_line":1292,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":1652,"parent_record_offset":1620,"ancestor_record_offset":952,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":952,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":428,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":952,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":1620,"parent_record_offset":1584,"ancestor_record_offset":952,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":952,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":428,"record_offset":1308,"parent_record_offset":1252,"ancestor_record_offset":952,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":428,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":952,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":428,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":952,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":428,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":952,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":428,"record_offset":1140,"parent_record_offset":1112,"ancestor_record_offset":952,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":428,"record_offset":1740,"parent_record_offset":1584,"ancestor_record_offset":952,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":428,"record_offset":1336,"parent_record_offset":1308,"ancestor_record_offset":952,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":428,"record_offset":1364,"parent_record_offset":1336,"ancestor_record_offset":952,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":428,"record_offset":1280,"parent_record_offset":1252,"ancestor_record_offset":952,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":428,"record_offset":1404,"parent_record_offset":1252,"ancestor_record_offset":952,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00428.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":428,"record_offset":1528,"parent_record_offset":1252,"ancestor_record_offset":952,"inlinee":26650,"source_line":136,"archive":"_pdb/C/modules/00428.jsonl.gz"}
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
// 完整结构及原始类型记录: trim_rows.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::utils::trim_rows::RowTrimmerSink
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink>", "constructor": "utils::geometry::split_heightmap::split_heightmap_by_threshold", "arguments": ["system_wall_constructor::utils::trim_rows::RowTrimmerSink"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: trim_rows.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: trim_rows.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::trim_rows::impl$0::above
// 信息 ID: a2379c8cddc8b243bd383a3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink>; module=1537, inline_record=492; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink>; module=1537, inline_record=1112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::trim_rows::impl$0::above_to_below
// 信息 ID: 306c9e4fce40dc33fbe7cb68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink> @ 0x1497d29 -> 0x8426a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::trim_rows::impl$0::below_to_above
// 信息 ID: 07dcd8baf134d35b3d8071cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::geometry::split_heightmap::split_heightmap_by_threshold<system_wall_constructor::utils::trim_rows::RowTrimmerSink> @ 0x1497d6e -> 0x842760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::trim_rows::trim_rows
// 信息 ID: aaf4f29da6f2bd026ac615da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x679261 -> 0x842860 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3> @ 0x1448bef -> 0x842860 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x144c5fe -> 0x842860 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x144fc81 -> 0x842860 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> @ 0x1453672 -> 0x842860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::trim_rows::trim_rows::closure$0
// 信息 ID: 9f22af8a0a3d115a7e026728
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::trim_rows::trim_rows", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::trim_rows::trim_rows; module=428, inline_record=5144; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::trim_rows::trim_rows; module=428, inline_record=5256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

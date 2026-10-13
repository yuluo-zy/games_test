// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/panic_reporter/reverse_backtrace.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: de671c37930b420f5963b766db75daa462a71337f7da4124c0e56296f8ff1be5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: reverse_backtrace.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 149, 160
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use backtrace; // namespace candidate
//   package identities: [{"name":"backtrace","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.71"}]
// use core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":160,"record_offset":248,"rva":4894592,"va":5373603712}
// 编译实例 RVAs: 0x4aaf80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$backtrace..symbolize..SymbolName$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10388045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373603712,"raw_hex":"e84d829e00","size":5,"soft_fail":false,"target_va":5383992416,"va":5373604366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t2074167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373603712,"raw_hex":"e837a61f00","size":5,"soft_fail":false,"target_va":5375678704,"va":5373604532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4265910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373603712,"raw_hex":"e84ae8beff","size":5,"soft_fail":false,"target_va":5369338848,"va":5373604753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e6f0
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4246666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373603712,"raw_hex":"e87633bfff","size":5,"soft_fail":false,"target_va":5369358064,"va":5373604725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140325300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1596671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373603712,"raw_hex":"e801a3e7ff","size":5,"soft_fail":false,"target_va":5372007168,"va":5373603834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404ac2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: reverse_backtrace.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373603712,"raw_hex":"e8050d0000","size":5,"soft_fail":false,"target_va":5373608672,"va":5373605334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::capture::BacktraceSymbol::name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10385382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373603712,"raw_hex":"e8e6779e00","size":5,"soft_fail":false,"target_va":5383989616,"va":5373604229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::print::BacktraceFrameFmt::backtrace_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10391193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373603712,"raw_hex":"e8998e9e00","size":5,"soft_fail":false,"target_va":5383995888,"va":5373604690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t43016721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373603712,"raw_hex":"e811629002","size":5,"soft_fail":false,"target_va":5416621632,"va":5373604906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t43017358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373603712,"raw_hex":"e88e649002","size":5,"soft_fail":false,"target_va":5416622320,"va":5373604957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::current_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11107586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373603712,"raw_hex":"e8027da900","size":5,"soft_fail":false,"target_va":5384711376,"va":5373603785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":160,"record_offset":2492,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] backtrace::capture::BacktraceFrame::symbols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.71/src/capture.rs
//     evidence={"module":160,"record_offset":960,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14778,"source_line":309,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] backtrace::print::BacktraceFmt::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.71/src/print.rs
//     evidence={"module":160,"record_offset":728,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14776,"source_line":44,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":160,"record_offset":840,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":160,"record_offset":1100,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":160,"record_offset":2180,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":160,"record_offset":756,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":160,"record_offset":428,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":160,"record_offset":2960,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":160,"record_offset":1820,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":1664,"parent_record_offset":1608,"ancestor_record_offset":248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":160,"record_offset":1636,"parent_record_offset":1608,"ancestor_record_offset":248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":160,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":160,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":160,"record_offset":1420,"parent_record_offset":1368,"ancestor_record_offset":248,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":160,"record_offset":1332,"parent_record_offset":1268,"ancestor_record_offset":248,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":160,"record_offset":1368,"parent_record_offset":1332,"ancestor_record_offset":248,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] backtrace::print::impl$2::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.71/src/print.rs
//     evidence={"module":160,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":248,"inlinee":14780,"source_line":308,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":160,"record_offset":1904,"parent_record_offset":1876,"ancestor_record_offset":248,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":160,"record_offset":1448,"parent_record_offset":1368,"ancestor_record_offset":248,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::typed_swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":160,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":248,"inlinee":5746,"source_line":2574,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":160,"record_offset":484,"parent_record_offset":456,"ancestor_record_offset":248,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":160,"record_offset":2604,"parent_record_offset":2564,"ancestor_record_offset":248,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":160,"record_offset":1232,"parent_record_offset":1136,"ancestor_record_offset":248,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":160,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":248,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":160,"record_offset":1300,"parent_record_offset":1268,"ancestor_record_offset":248,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":160,"record_offset":1512,"parent_record_offset":1368,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":160,"record_offset":804,"parent_record_offset":780,"ancestor_record_offset":248,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":160,"record_offset":924,"parent_record_offset":868,"ancestor_record_offset":248,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":160,"record_offset":896,"parent_record_offset":868,"ancestor_record_offset":248,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_chunk
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":160,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":248,"inlinee":5753,"source_line":1432,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":160,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":248,"inlinee":5747,"source_line":1363,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":160,"record_offset":560,"parent_record_offset":532,"ancestor_record_offset":248,"inlinee":5749,"source_line":2434,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":160,"record_offset":588,"parent_record_offset":560,"ancestor_record_offset":248,"inlinee":5750,"source_line":1440,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap_nonoverlapping_bytes::swap_nonoverlapping_chunks
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":160,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":248,"inlinee":5752,"source_line":1442,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":160,"record_offset":1476,"parent_record_offset":1368,"ancestor_record_offset":248,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":160,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":248,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":160,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":248,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":160,"record_offset":456,"parent_record_offset":428,"ancestor_record_offset":248,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":160,"record_offset":1876,"parent_record_offset":1848,"ancestor_record_offset":248,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":160,"record_offset":2636,"parent_record_offset":2564,"ancestor_record_offset":248,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":160,"record_offset":2528,"parent_record_offset":2492,"ancestor_record_offset":248,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":160,"record_offset":2564,"parent_record_offset":2528,"ancestor_record_offset":248,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":160,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":248,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":160,"record_offset":1136,"parent_record_offset":1100,"ancestor_record_offset":248,"inlinee":6772,"source_line":360,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":160,"record_offset":868,"parent_record_offset":840,"ancestor_record_offset":248,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":160,"record_offset":1848,"parent_record_offset":1820,"ancestor_record_offset":248,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/reverse_backtrace.rs
//     evidence={"module":160,"record_offset":1268,"parent_record_offset":1232,"ancestor_record_offset":248,"inlinee":14414,"source_line":36,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x39CC
// 首条位置: {"module":160,"inlinee":14796}
// 编译实例 RVAs: 0x4ab600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Display as core::fmt::Display>::fmt
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11155512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373605376,"raw_hex":"e83838aa00","size":5,"soft_fail":false,"target_va":5384760992,"va":5373605475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_strip_prefix
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11049094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373605376,"raw_hex":"e88698a800","size":5,"soft_fail":false,"target_va":5384654544,"va":5373605445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2073174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373605376,"raw_hex":"e856a21f00","size":5,"soft_fail":false,"target_va":5375678704,"va":5373605525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::types::BytesOrWideString::into_path_buf
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10387241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373605376,"raw_hex":"e8297f9e00","size":5,"soft_fail":false,"target_va":5383992656,"va":5373605410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":160,"record_offset":4216,"parent_record_offset":4000,"ancestor_record_offset":4000,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::strip_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":160,"record_offset":4160,"parent_record_offset":4000,"ancestor_record_offset":4000,"inlinee":11153,"source_line":2710,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":4464,"parent_record_offset":4440,"ancestor_record_offset":4000,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":4488,"parent_record_offset":4464,"ancestor_record_offset":4000,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":4512,"parent_record_offset":4488,"ancestor_record_offset":4000,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":4440,"parent_record_offset":4384,"ancestor_record_offset":4000,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":160,"record_offset":4412,"parent_record_offset":4384,"ancestor_record_offset":4000,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":160,"record_offset":4384,"parent_record_offset":4360,"ancestor_record_offset":4000,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":160,"record_offset":4360,"parent_record_offset":4336,"ancestor_record_offset":4000,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":160,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":4000,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":160,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":4000,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x384E
// 首条位置: {"module":149,"inlinee":14414}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":160,"record_offset":1332,"parent_record_offset":1268,"ancestor_record_offset":1268,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":160,"record_offset":1300,"parent_record_offset":1268,"ancestor_record_offset":1268,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":1268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":1268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":160,"record_offset":1664,"parent_record_offset":1608,"ancestor_record_offset":1268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":160,"record_offset":1636,"parent_record_offset":1608,"ancestor_record_offset":1268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":160,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":1268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":160,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":1268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":160,"record_offset":1420,"parent_record_offset":1368,"ancestor_record_offset":1268,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":160,"record_offset":1368,"parent_record_offset":1332,"ancestor_record_offset":1268,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":160,"record_offset":1448,"parent_record_offset":1368,"ancestor_record_offset":1268,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":160,"record_offset":1512,"parent_record_offset":1368,"ancestor_record_offset":1268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00160.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":160,"record_offset":1476,"parent_record_offset":1368,"ancestor_record_offset":1268,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00160.jsonl.gz"}
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
// 完整结构及原始类型记录: reverse_backtrace.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: reverse_backtrace.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: reverse_backtrace.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt
// 信息 ID: 705184e2c86da0b7181d737f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$0
// 信息 ID: c4ca472494149c2e51d1cc47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0,tuple$<ref_mut$<core::fmt::Formatter>,enum2$<backtrace::types::BytesOrWideString> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_env$0,tuple$<ref_mut$<core::fmt::Formatter>,enum2$<backtrace::types::BytesOrWideString> > >; module=160, inline_record=9548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure$1
// 信息 ID: 050a949546b7dd6de881378d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt; module=160, inline_record=1268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

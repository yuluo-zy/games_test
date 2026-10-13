// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 96958044bb0545e14eafde65ff443f8e7f4f20145d8d43265ba2ec3abb86e84a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sentry_reporter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 19, 65, 81, 117, 228, 235
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use raw_cpuid; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.11.0"},{"name":"raw-cpuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"11.1.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"}]
// use sentry_backtrace; // namespace candidate
//   package identities: [{"name":"sentry-backtrace","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.31.7"}]
// use sentry_core; // namespace candidate
//   package identities: [{"name":"sentry-core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.31.7"}]
// use sentry_types; // namespace candidate
//   package identities: [{"name":"sentry-types","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.31.7"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":65,"record_offset":988,"rva":2209712,"va":5370918832}
// 编译实例 RVAs: 0x21b7b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t13740263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e8e7a8d100","size":5,"soft_fail":false,"target_va":5384659264,"va":5370918996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t13737631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e89f9ed100","size":5,"soft_fail":false,"target_va":5384656912,"va":5370919276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t13738089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e869a0d100","size":5,"soft_fail":false,"target_va":5384657024,"va":5370918930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4759521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e8e19f4800","size":5,"soft_fail":false,"target_va":5375678688,"va":5370919162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t4759630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e84ea04800","size":5,"soft_fail":false,"target_va":5375678704,"va":5370919069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2196956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e8247adeff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370919143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fad30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4059439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e82ff13d00","size":5,"soft_fail":false,"target_va":5374979376,"va":5370919932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fb380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4060964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e824f73d00","size":5,"soft_fail":false,"target_va":5374980992,"va":5370920023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d66e40
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45397494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e8f6b5b402","size":5,"soft_fail":false,"target_va":5416316480,"va":5370918981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d672b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45398590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e83ebab402","size":5,"soft_fail":false,"target_va":5416317616,"va":5370919021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d69060
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45405565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e87dd5b402","size":5,"soft_fail":false,"target_va":5416325216,"va":5370919646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46137863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e80702c002","size":5,"soft_fail":false,"target_va":5417057056,"va":5370919188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t45701673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e8295ab902","size":5,"soft_fail":false,"target_va":5416620794,"va":5370919116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13911121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e85144d400","size":5,"soft_fail":false,"target_va":5384830080,"va":5370918954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13831928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e8f80ed300","size":5,"soft_fail":false,"target_va":5384751936,"va":5370920003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13805088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e820a6d200","size":5,"soft_fail":false,"target_va":5384724912,"va":5370919819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t45406048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370918832,"raw_hex":"e860d7b402","size":5,"soft_fail":false,"target_va":5416325280,"va":5370919227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":65,"record_offset":1864,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":65,"record_offset":4620,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":65,"record_offset":1224,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":65,"record_offset":1380,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":1496,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":65,"record_offset":1188,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":65,"record_offset":4588,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":65,"record_offset":1468,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":65,"record_offset":1296,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":6783,"source_line":913,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":65,"record_offset":4648,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":65,"record_offset":4496,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":10149,"source_line":3261,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::sentry_reporter::extension_to_content_type
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":65,"record_offset":2844,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":10148,"source_line":175,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":988,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":2412,"parent_record_offset":2388,"ancestor_record_offset":988,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":988,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":988,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":988,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":988,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":2364,"parent_record_offset":2196,"ancestor_record_offset":988,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":1720,"parent_record_offset":1664,"ancestor_record_offset":988,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":2108,"parent_record_offset":2068,"ancestor_record_offset":988,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":1692,"parent_record_offset":1664,"ancestor_record_offset":988,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":988,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":2196,"parent_record_offset":2148,"ancestor_record_offset":988,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":2148,"parent_record_offset":2108,"ancestor_record_offset":988,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":988,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":2252,"parent_record_offset":2196,"ancestor_record_offset":988,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":1944,"parent_record_offset":1904,"ancestor_record_offset":988,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":1984,"parent_record_offset":1944,"ancestor_record_offset":988,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":2024,"parent_record_offset":1984,"ancestor_record_offset":988,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":1904,"parent_record_offset":1864,"ancestor_record_offset":988,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":65,"record_offset":2068,"parent_record_offset":2024,"ancestor_record_offset":988,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":65,"record_offset":1264,"parent_record_offset":1224,"ancestor_record_offset":988,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":65,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":988,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":65,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":988,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":65,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":988,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":65,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":988,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":65,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":988,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":65,"record_offset":2492,"parent_record_offset":2024,"ancestor_record_offset":988,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":2516,"parent_record_offset":2492,"ancestor_record_offset":988,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":988,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":65,"record_offset":4544,"parent_record_offset":4520,"ancestor_record_offset":988,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":65,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":988,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":65,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":988,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":65,"record_offset":2900,"parent_record_offset":2844,"ancestor_record_offset":988,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::wtf8::Wtf8::as_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\wtf8.rs
//     evidence={"module":65,"record_offset":1344,"parent_record_offset":1320,"ancestor_record_offset":988,"inlinee":6786,"source_line":274,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":65,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":988,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":65,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":988,"inlinee":6784,"source_line":255,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":65,"record_offset":1408,"parent_record_offset":1380,"ancestor_record_offset":988,"inlinee":10145,"source_line":142,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":65,"record_offset":2784,"parent_record_offset":2756,"ancestor_record_offset":988,"inlinee":10146,"source_line":149,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":65,"record_offset":12044,"rva":2214320,"va":5370923440}
// 编译实例 RVAs: 0x21c9b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4754069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8958a4800","size":5,"soft_fail":false,"target_va":5375678688,"va":5370924614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t4754853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8a58d4800","size":5,"soft_fail":false,"target_va":5375678704,"va":5370923846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2202407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8d964deff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370924594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14008e3b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1634414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8920fe7ff","size":5,"soft_fail":false,"target_va":5369291696,"va":5370926105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1585143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e809d0e7ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370923986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ad50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1580798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e802e1e7ff","size":5,"soft_fail":false,"target_va":5369343312,"va":5370924105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a3590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1548370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8ae5fe8ff","size":5,"soft_fail":false,"target_va":5369378192,"va":5370926557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400aa8e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1516099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8bddde8ff","size":5,"soft_fail":false,"target_va":5369407712,"va":5370923806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0b70
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1491225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8e73ee9ff","size":5,"soft_fail":false,"target_va":5369432944,"va":5370924164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bad80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1450076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8a4dfe9ff","size":5,"soft_fail":false,"target_va":5369474432,"va":5370924503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c6960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1404085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e84b93eaff","size":5,"soft_fail":false,"target_va":5369522528,"va":5370926608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140103f10
//     owner=unresolved_namespace; count=7
//     target_aliases=3 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1152493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8136aeeff","size":5,"soft_fail":false,"target_va":5369773840,"va":5370926328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140133330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-957950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e80262f1ff","size":5,"soft_fail":false,"target_va":5369967408,"va":5370925353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401395b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-931081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8f7caf1ff","size":5,"soft_fail":false,"target_va":5369992624,"va":5370923700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t903902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8deca0d00","size":5,"soft_fail":false,"target_va":5371829152,"va":5370925245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140304500
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t947279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e84f740e00","size":5,"soft_fail":false,"target_va":5371872512,"va":5370925228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4128561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e831ff3e00","size":5,"soft_fail":false,"target_va":5375052640,"va":5370924074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46132921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8b9eebf02","size":5,"soft_fail":false,"target_va":5417057056,"va":5370924130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13870517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8b5a5d300","size":5,"soft_fail":false,"target_va":5384794864,"va":5370924342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t45695318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e85641b902","size":5,"soft_fail":false,"target_va":5416620794,"va":5370925471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-774554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8662ef4ff","size":5,"soft_fail":false,"target_va":5370151808,"va":5370926357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e8911e0000","size":5,"soft_fail":false,"target_va":5370933696,"va":5370925866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13827329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370923440,"raw_hex":"e801fdd200","size":5,"soft_fail":false,"target_va":5384751264,"va":5370923930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":65,"record_offset":12248,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":65,"record_offset":12304,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":65,"record_offset":15828,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":65,"record_offset":13572,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":65,"record_offset":15292,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":12516,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":65,"record_offset":14416,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":65,"record_offset":14508,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":65,"record_offset":13504,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::add_breadcrumb
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":65,"record_offset":12432,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":10155,"source_line":109,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::with_scope
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":65,"record_offset":15912,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":10162,"source_line":183,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":65,"record_offset":13448,"parent_record_offset":12044,"ancestor_record_offset":12044,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":15640,"parent_record_offset":15616,"ancestor_record_offset":12044,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":15664,"parent_record_offset":15640,"ancestor_record_offset":12044,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":12044,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":12044,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":15688,"parent_record_offset":15664,"ancestor_record_offset":12044,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":13064,"parent_record_offset":13040,"ancestor_record_offset":12044,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":15616,"parent_record_offset":15588,"ancestor_record_offset":12044,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":12992,"parent_record_offset":12904,"ancestor_record_offset":12044,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":65,"record_offset":14576,"parent_record_offset":14548,"ancestor_record_offset":12044,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":65,"record_offset":14624,"parent_record_offset":14600,"ancestor_record_offset":12044,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":15528,"parent_record_offset":15500,"ancestor_record_offset":12044,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":12932,"parent_record_offset":12904,"ancestor_record_offset":12044,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":12904,"parent_record_offset":12880,"ancestor_record_offset":12044,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":15588,"parent_record_offset":15556,"ancestor_record_offset":12044,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":15556,"parent_record_offset":15528,"ancestor_record_offset":12044,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":12880,"parent_record_offset":12856,"ancestor_record_offset":12044,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":15408,"parent_record_offset":15380,"ancestor_record_offset":12044,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":15436,"parent_record_offset":15408,"ancestor_record_offset":12044,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":15464,"parent_record_offset":15436,"ancestor_record_offset":12044,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":15380,"parent_record_offset":15352,"ancestor_record_offset":12044,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":65,"record_offset":15352,"parent_record_offset":15324,"ancestor_record_offset":12044,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":65,"record_offset":15500,"parent_record_offset":15464,"ancestor_record_offset":12044,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":65,"record_offset":15852,"parent_record_offset":15828,"ancestor_record_offset":12044,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":65,"record_offset":12580,"parent_record_offset":12544,"ancestor_record_offset":12044,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":65,"record_offset":12544,"parent_record_offset":12516,"ancestor_record_offset":12044,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":65,"record_offset":12832,"parent_record_offset":12808,"ancestor_record_offset":12044,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":65,"record_offset":12652,"parent_record_offset":12628,"ancestor_record_offset":12044,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":65,"record_offset":13472,"parent_record_offset":13448,"ancestor_record_offset":12044,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":65,"record_offset":14660,"parent_record_offset":14548,"ancestor_record_offset":12044,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":65,"record_offset":15876,"parent_record_offset":15852,"ancestor_record_offset":12044,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":65,"record_offset":12628,"parent_record_offset":12604,"ancestor_record_offset":12044,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":65,"record_offset":12604,"parent_record_offset":12580,"ancestor_record_offset":12044,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":65,"record_offset":12960,"parent_record_offset":12932,"ancestor_record_offset":12044,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":65,"record_offset":14600,"parent_record_offset":14576,"ancestor_record_offset":12044,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":65,"record_offset":13784,"parent_record_offset":13608,"ancestor_record_offset":12044,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":65,"record_offset":15744,"parent_record_offset":15464,"ancestor_record_offset":12044,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":65,"record_offset":12724,"parent_record_offset":12700,"ancestor_record_offset":12044,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":15768,"parent_record_offset":15744,"ancestor_record_offset":12044,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":12776,"parent_record_offset":12544,"ancestor_record_offset":12044,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":65,"record_offset":12272,"parent_record_offset":12248,"ancestor_record_offset":12044,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":65,"record_offset":12700,"parent_record_offset":12676,"ancestor_record_offset":12044,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":65,"record_offset":12676,"parent_record_offset":12652,"ancestor_record_offset":12044,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":65,"record_offset":12372,"parent_record_offset":12304,"ancestor_record_offset":12044,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":65,"record_offset":12340,"parent_record_offset":12304,"ancestor_record_offset":12044,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":12396,"parent_record_offset":12372,"ancestor_record_offset":12044,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":65,"record_offset":13720,"parent_record_offset":13608,"ancestor_record_offset":12044,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::from_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":65,"record_offset":13660,"parent_record_offset":13608,"ancestor_record_offset":12044,"inlinee":10120,"source_line":119,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/options.rs
//     evidence={"module":65,"record_offset":13684,"parent_record_offset":13660,"ancestor_record_offset":12044,"inlinee":9932,"source_line":59,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":65,"record_offset":12480,"parent_record_offset":12456,"ancestor_record_offset":12044,"inlinee":7527,"source_line":145,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":65,"record_offset":12456,"parent_record_offset":12432,"ancestor_record_offset":12044,"inlinee":7526,"source_line":51,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":65,"record_offset":13608,"parent_record_offset":13572,"ancestor_record_offset":12044,"inlinee":10156,"source_line":30,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":65,"record_offset":14448,"parent_record_offset":14416,"ancestor_record_offset":12044,"inlinee":10160,"source_line":49,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":65,"record_offset":14548,"parent_record_offset":14508,"ancestor_record_offset":12044,"inlinee":10159,"source_line":50,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":65,"record_offset":15324,"parent_record_offset":15292,"ancestor_record_offset":12044,"inlinee":10161,"source_line":52,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27AC
// 首条位置: {"module":65,"inlinee":10156}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":65,"record_offset":13784,"parent_record_offset":13608,"ancestor_record_offset":13608,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":13812,"parent_record_offset":13608,"ancestor_record_offset":13608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":65,"record_offset":13720,"parent_record_offset":13608,"ancestor_record_offset":13608,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":65,"record_offset":13660,"parent_record_offset":13608,"ancestor_record_offset":13608,"inlinee":10120,"source_line":119,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":14016,"parent_record_offset":13992,"ancestor_record_offset":13608,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":13608,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":14064,"parent_record_offset":14040,"ancestor_record_offset":13608,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":13992,"parent_record_offset":13936,"ancestor_record_offset":13608,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":13964,"parent_record_offset":13936,"ancestor_record_offset":13608,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":13936,"parent_record_offset":13912,"ancestor_record_offset":13608,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":13912,"parent_record_offset":13888,"ancestor_record_offset":13608,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":13752,"parent_record_offset":13720,"ancestor_record_offset":13608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/options.rs
//     evidence={"module":65,"record_offset":13684,"parent_record_offset":13660,"ancestor_record_offset":13608,"inlinee":9932,"source_line":59,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27B0
// 首条位置: {"module":65,"inlinee":10160}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":14448,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27AF
// 首条位置: {"module":65,"inlinee":10159}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":65,"record_offset":14576,"parent_record_offset":14548,"ancestor_record_offset":14548,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":65,"record_offset":14660,"parent_record_offset":14548,"ancestor_record_offset":14548,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":14688,"parent_record_offset":14548,"ancestor_record_offset":14548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":14888,"parent_record_offset":14864,"ancestor_record_offset":14548,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":14912,"parent_record_offset":14888,"ancestor_record_offset":14548,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":14548,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":14864,"parent_record_offset":14808,"ancestor_record_offset":14548,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":65,"record_offset":14624,"parent_record_offset":14600,"ancestor_record_offset":14548,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":14836,"parent_record_offset":14808,"ancestor_record_offset":14548,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":14808,"parent_record_offset":14784,"ancestor_record_offset":14548,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":14784,"parent_record_offset":14760,"ancestor_record_offset":14548,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":65,"record_offset":14600,"parent_record_offset":14576,"ancestor_record_offset":14548,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":14712,"parent_record_offset":14688,"ancestor_record_offset":14548,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27B1
// 首条位置: {"module":65,"inlinee":10161}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":65,"record_offset":15352,"parent_record_offset":15324,"ancestor_record_offset":15324,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":15640,"parent_record_offset":15616,"ancestor_record_offset":15324,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":15664,"parent_record_offset":15640,"ancestor_record_offset":15324,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":15688,"parent_record_offset":15664,"ancestor_record_offset":15324,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":15616,"parent_record_offset":15588,"ancestor_record_offset":15324,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":15528,"parent_record_offset":15500,"ancestor_record_offset":15324,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":15588,"parent_record_offset":15556,"ancestor_record_offset":15324,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":15556,"parent_record_offset":15528,"ancestor_record_offset":15324,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":15408,"parent_record_offset":15380,"ancestor_record_offset":15324,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":15436,"parent_record_offset":15408,"ancestor_record_offset":15324,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":15464,"parent_record_offset":15436,"ancestor_record_offset":15324,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":15380,"parent_record_offset":15352,"ancestor_record_offset":15324,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":65,"record_offset":15500,"parent_record_offset":15464,"ancestor_record_offset":15324,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":65,"record_offset":15744,"parent_record_offset":15464,"ancestor_record_offset":15324,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":15768,"parent_record_offset":15744,"ancestor_record_offset":15324,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B15
// 首条位置: {"module":81,"inlinee":11029}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2A1E
// 首条位置: {"module":81,"inlinee":10782}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":81,"record_offset":43768,"parent_record_offset":43176,"ancestor_record_offset":43176,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":81,"record_offset":43236,"parent_record_offset":43176,"ancestor_record_offset":43176,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":81,"record_offset":43852,"parent_record_offset":43176,"ancestor_record_offset":43176,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":81,"record_offset":43976,"parent_record_offset":43176,"ancestor_record_offset":43176,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::add_breadcrumb
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":81,"record_offset":43880,"parent_record_offset":43176,"ancestor_record_offset":43176,"inlinee":10155,"source_line":109,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":43580,"parent_record_offset":43556,"ancestor_record_offset":43176,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":43604,"parent_record_offset":43580,"ancestor_record_offset":43176,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":44176,"parent_record_offset":44152,"ancestor_record_offset":43176,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":44200,"parent_record_offset":44176,"ancestor_record_offset":43176,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":43628,"parent_record_offset":43604,"ancestor_record_offset":43176,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":44224,"parent_record_offset":44200,"ancestor_record_offset":43176,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":43556,"parent_record_offset":43528,"ancestor_record_offset":43176,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":81,"record_offset":44152,"parent_record_offset":44096,"ancestor_record_offset":43176,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":81,"record_offset":43816,"parent_record_offset":43792,"ancestor_record_offset":43176,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":81,"record_offset":43468,"parent_record_offset":43440,"ancestor_record_offset":43176,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":81,"record_offset":44124,"parent_record_offset":44096,"ancestor_record_offset":43176,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":81,"record_offset":44096,"parent_record_offset":44072,"ancestor_record_offset":43176,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":81,"record_offset":43528,"parent_record_offset":43496,"ancestor_record_offset":43176,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":81,"record_offset":43496,"parent_record_offset":43468,"ancestor_record_offset":43176,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":81,"record_offset":44072,"parent_record_offset":44048,"ancestor_record_offset":43176,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":81,"record_offset":43348,"parent_record_offset":43320,"ancestor_record_offset":43176,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":81,"record_offset":43376,"parent_record_offset":43348,"ancestor_record_offset":43176,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":81,"record_offset":43404,"parent_record_offset":43376,"ancestor_record_offset":43176,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":81,"record_offset":43320,"parent_record_offset":43292,"ancestor_record_offset":43176,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":81,"record_offset":43292,"parent_record_offset":43264,"ancestor_record_offset":43176,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":81,"record_offset":43264,"parent_record_offset":43236,"ancestor_record_offset":43176,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":81,"record_offset":43440,"parent_record_offset":43404,"ancestor_record_offset":43176,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":81,"record_offset":43792,"parent_record_offset":43768,"ancestor_record_offset":43176,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":81,"record_offset":43684,"parent_record_offset":43404,"ancestor_record_offset":43176,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":81,"record_offset":43708,"parent_record_offset":43684,"ancestor_record_offset":43176,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":81,"record_offset":44000,"parent_record_offset":43976,"ancestor_record_offset":43176,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":81,"record_offset":43936,"parent_record_offset":43908,"ancestor_record_offset":43176,"inlinee":7527,"source_line":145,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":81,"record_offset":43908,"parent_record_offset":43880,"ancestor_record_offset":43176,"inlinee":7526,"source_line":51,"archive":"_pdb/C/modules/00081.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30EF
// 首条位置: {"module":117,"inlinee":12527}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":117,"record_offset":26416,"parent_record_offset":24780,"ancestor_record_offset":24780,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":117,"record_offset":25204,"parent_record_offset":24780,"ancestor_record_offset":24780,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":117,"record_offset":24904,"parent_record_offset":24780,"ancestor_record_offset":24780,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":117,"record_offset":24960,"parent_record_offset":24780,"ancestor_record_offset":24780,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":117,"record_offset":25132,"parent_record_offset":24780,"ancestor_record_offset":24780,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] raw_cpuid::CpuId::get_feature_info
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":117,"record_offset":26184,"parent_record_offset":24780,"ancestor_record_offset":24780,"inlinee":5875,"source_line":391,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] raw_cpuid::CpuId::get_processor_brand_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":117,"record_offset":25808,"parent_record_offset":24780,"ancestor_record_offset":24780,"inlinee":5876,"source_line":820,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_direct] raw_cpuid::CpuId::get_vendor_info
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":117,"record_offset":25668,"parent_record_offset":24780,"ancestor_record_offset":24780,"inlinee":5874,"source_line":374,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":25552,"parent_record_offset":25528,"ancestor_record_offset":24780,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":25576,"parent_record_offset":25552,"ancestor_record_offset":24780,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":25600,"parent_record_offset":25576,"ancestor_record_offset":24780,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":117,"record_offset":25528,"parent_record_offset":25472,"ancestor_record_offset":24780,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":117,"record_offset":26464,"parent_record_offset":26440,"ancestor_record_offset":24780,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":117,"record_offset":25500,"parent_record_offset":25472,"ancestor_record_offset":24780,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":117,"record_offset":25472,"parent_record_offset":25448,"ancestor_record_offset":24780,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":117,"record_offset":25448,"parent_record_offset":25424,"ancestor_record_offset":24780,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":117,"record_offset":25284,"parent_record_offset":25236,"ancestor_record_offset":24780,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":117,"record_offset":25236,"parent_record_offset":25204,"ancestor_record_offset":24780,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::cpuid::__cpuid_count
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\cpuid.rs
//     evidence={"module":117,"record_offset":25764,"parent_record_offset":25740,"ancestor_record_offset":24780,"inlinee":5864,"source_line":61,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":117,"record_offset":25312,"parent_record_offset":25236,"ancestor_record_offset":24780,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":117,"record_offset":26440,"parent_record_offset":26416,"ancestor_record_offset":24780,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":25172,"parent_record_offset":25132,"ancestor_record_offset":24780,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":117,"record_offset":24928,"parent_record_offset":24904,"ancestor_record_offset":24780,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":117,"record_offset":25044,"parent_record_offset":24960,"ancestor_record_offset":24780,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":117,"record_offset":25072,"parent_record_offset":24960,"ancestor_record_offset":24780,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":117,"record_offset":25008,"parent_record_offset":24960,"ancestor_record_offset":24780,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":117,"record_offset":25096,"parent_record_offset":25072,"ancestor_record_offset":24780,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":117,"record_offset":25340,"parent_record_offset":25236,"ancestor_record_offset":24780,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::CpuIdReader::cpuid1
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":117,"record_offset":25692,"parent_record_offset":25668,"ancestor_record_offset":24780,"inlinee":5858,"source_line":175,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::FeatureInfoFlags::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":117,"record_offset":26348,"parent_record_offset":26184,"ancestor_record_offset":24780,"inlinee":5866,"source_line":663,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::_::InternalBitFlags::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.11.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.11.0/src/lib.rs
//     evidence={"module":117,"record_offset":26376,"parent_record_offset":26348,"ancestor_record_offset":24780,"inlinee":5868,"source_line":663,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::native_cpuid::cpuid_count
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":117,"record_offset":25740,"parent_record_offset":25716,"ancestor_record_offset":24780,"inlinee":5862,"source_line":85,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::native_cpuid::impl$0::cpuid2
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":117,"record_offset":25716,"parent_record_offset":25692,"ancestor_record_offset":24780,"inlinee":5860,"source_line":103,"archive":"_pdb/C/modules/00117.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x198B
// 首条位置: {"module":19,"inlinee":6539}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":65,"record_offset":6096,"rva":47616160,"va":5416325280}
// 编译实例 RVAs: 0x2d690a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-40646661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416325280,"raw_hex":"e8fbc793fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5416325360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d69060
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416325280,"raw_hex":"e88affffff","size":5,"soft_fail":false,"target_va":5416325216,"va":5416325329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":6260,"parent_record_offset":6096,"ancestor_record_offset":6096,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":6424,"parent_record_offset":6400,"ancestor_record_offset":6096,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":6448,"parent_record_offset":6424,"ancestor_record_offset":6096,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":6096,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":6400,"parent_record_offset":6344,"ancestor_record_offset":6096,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":6372,"parent_record_offset":6344,"ancestor_record_offset":6096,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":6096,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":6320,"parent_record_offset":6300,"ancestor_record_offset":6096,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":6280,"parent_record_offset":6260,"ancestor_record_offset":6096,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27A1
// 首条位置: {"module":65,"inlinee":10145}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":65,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":1408,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27A2
// 首条位置: {"module":65,"inlinee":10146}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":65,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":2784,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43BE
// 首条位置: {"module":235,"inlinee":17342}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x43C3
// 首条位置: {"module":235,"inlinee":17347}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::extension_to_content_type
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27A4
// 首条位置: {"module":65,"inlinee":10148}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":65,"record_offset":2900,"parent_record_offset":2844,"ancestor_record_offset":2844,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":65,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":2844,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":65,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":2844,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":65,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":2844,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_message
// 记录形式: compiled_procedure; instances=1
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":65,"record_offset":536,"rva":2209568,"va":5370918688}
// 编译实例 RVAs: 0x21b720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5370918688,"raw_hex":"ff5718","size":3,"soft_fail":false,"va":5370918709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::any::impl$5::downcast_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":65,"record_offset":688,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":10138,"source_line":379,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":65,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":536,"inlinee":10140,"source_line":228,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":65,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":536,"inlinee":10141,"source_line":196,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":65,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":536,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":65,"record_offset":788,"parent_record_offset":760,"ancestor_record_offset":536,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_sentry_event
// 记录形式: compiled_procedure; instances=1
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":65,"record_offset":6808,"rva":2211360,"va":5370920480}
// 编译实例 RVAs: 0x21be20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$sentry_types..protocol..v7..Event$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5205988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e8e46f4f00","size":5,"soft_fail":false,"target_va":5376127312,"va":5370921319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4758138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e87a9a4800","size":5,"soft_fail":false,"target_va":5375678688,"va":5370920545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t4756367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e88f934800","size":5,"soft_fail":false,"target_va":5375678704,"va":5370922332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2198343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e8b974deff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370920530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007df10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1698417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e88f15e6ff","size":5,"soft_fail":false,"target_va":5369224976,"va":5370923388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a8660
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1523414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e82ac1e8ff","size":5,"soft_fail":false,"target_va":5369398880,"va":5370922289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a87f0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1523167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e821c2e8ff","size":5,"soft_fail":false,"target_va":5369399280,"va":5370922442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400aad60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1513559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e8a9e7e8ff","size":5,"soft_fail":false,"target_va":5369408864,"va":5370922418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t46136224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e8a0fbbf02","size":5,"soft_fail":false,"target_va":5417057056,"va":5370920827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t45698284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e8ec4cb902","size":5,"soft_fail":false,"target_va":5416620769,"va":5370922480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t45700027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e8bb53b902","size":5,"soft_fail":false,"target_va":5416620794,"va":5370920762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] sentry_backtrace::process::backtrace_to_stacktrace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"sentry-backtrace","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5128515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370920480,"raw_hex":"e843414e00","size":5,"soft_fail":false,"target_va":5376049376,"va":5370920856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":65,"record_offset":6980,"parent_record_offset":6808,"ancestor_record_offset":6808,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":65,"record_offset":8224,"parent_record_offset":6808,"ancestor_record_offset":6808,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":65,"record_offset":7160,"parent_record_offset":6808,"ancestor_record_offset":6808,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":8908,"parent_record_offset":6808,"ancestor_record_offset":6808,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_direct] sentry_types::protocol::v7::impl$172::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":65,"record_offset":8824,"parent_record_offset":6808,"ancestor_record_offset":6808,"inlinee":10152,"source_line":619,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":6808,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":6808,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":9188,"parent_record_offset":9164,"ancestor_record_offset":6808,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":9212,"parent_record_offset":9188,"ancestor_record_offset":6808,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":6808,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":6808,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":7040,"parent_record_offset":7008,"ancestor_record_offset":6808,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":65,"record_offset":9164,"parent_record_offset":9076,"ancestor_record_offset":6808,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":65,"record_offset":7008,"parent_record_offset":6980,"ancestor_record_offset":6808,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":65,"record_offset":10232,"parent_record_offset":10208,"ancestor_record_offset":6808,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":7392,"parent_record_offset":7364,"ancestor_record_offset":6808,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":9104,"parent_record_offset":9076,"ancestor_record_offset":6808,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":6808,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":7452,"parent_record_offset":7420,"ancestor_record_offset":6808,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":7420,"parent_record_offset":7392,"ancestor_record_offset":6808,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":6808,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":65,"record_offset":8508,"parent_record_offset":8464,"ancestor_record_offset":6808,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":7272,"parent_record_offset":7244,"ancestor_record_offset":6808,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":7300,"parent_record_offset":7272,"ancestor_record_offset":6808,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":7328,"parent_record_offset":7300,"ancestor_record_offset":6808,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":65,"record_offset":7244,"parent_record_offset":7216,"ancestor_record_offset":6808,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":65,"record_offset":7216,"parent_record_offset":7188,"ancestor_record_offset":6808,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":65,"record_offset":7188,"parent_record_offset":7160,"ancestor_record_offset":6808,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":65,"record_offset":7364,"parent_record_offset":7328,"ancestor_record_offset":6808,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":65,"record_offset":8964,"parent_record_offset":8936,"ancestor_record_offset":6808,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":65,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":6808,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":65,"record_offset":8580,"parent_record_offset":8556,"ancestor_record_offset":6808,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":65,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":6808,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":65,"record_offset":9132,"parent_record_offset":9104,"ancestor_record_offset":6808,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":65,"record_offset":7608,"parent_record_offset":7328,"ancestor_record_offset":6808,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":7632,"parent_record_offset":7608,"ancestor_record_offset":6808,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":65,"record_offset":8936,"parent_record_offset":8908,"ancestor_record_offset":6808,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00065.jsonl.gz"}
//   [pdb_inline_transitive] sentry_types::protocol::v7::impl$2::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":65,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":6808,"inlinee":10154,"source_line":84,"archive":"_pdb/C/modules/00065.jsonl.gz"}
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
// 完整结构及原始类型记录: sentry_reporter.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sentry_reporter.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sentry_reporter.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 18; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnShapeMoved>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnShapeMoved>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnShapeMoved>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnRectangleEdited>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnRectangleEdited>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ReanchorDecorators>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnFreehandWallEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnFreehandWallEdited>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnFreehandWallEdited>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnCircleRotated>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnCircleRotated>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::OnCircleRotated>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment
// 信息 ID: c06e569e4b479bb877b6f41d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,8>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4> > @ 0x30469c -> 0x21b7b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$0
// 信息 ID: ef3dee163a8b3f4150a7c637
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment @ 0x21b93b -> 0x2d690a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$1
// 信息 ID: 77e3f2e8170fe8696f18c051
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment; module=65, inline_record=1408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$2
// 信息 ID: 07150031429f4d06c5ba35bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment; module=65, inline_record=2784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$3
// 信息 ID: b1f39b7b6c276b94a358d464
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$3 <- anyhow::context::impl$0::with_context<std::fs::Metadata,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::Metadata,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$3>; module=235, inline_record=21588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$4
// 信息 ID: 97e4e8696767ffcdca4bbad7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$4 <- anyhow::context::impl$0::with_context<alloc::vec::Vec<u8,alloc::alloc::Global>,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::vec::Vec<u8,alloc::alloc::Global>,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$4>; module=235, inline_record=24868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::extension_to_content_type
// 信息 ID: ffd866b774473faeea7f4a99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment; module=65, inline_record=2844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_message
// 信息 ID: db54577bef47b53ce1d850f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0x1851b9 -> 0x21b720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_sentry_event
// 信息 ID: c64f567bb9bde37991fab472
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > @ 0x5e8364 -> 0x21be20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry
// 信息 ID: 05ce02527c38b4db6fedd894
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$6,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > @ 0x5e84d4 -> 0x21c9b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$0
// 信息 ID: 7712ca879fc204f9a72dc7ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry; module=65, inline_record=13608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$1
// 信息 ID: 812f2f4b40eee64991aa6c5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry; module=65, inline_record=14448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$2
// 信息 ID: 33c8414528d47dbd4d3ac6ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry; module=65, inline_record=14548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$3
// 信息 ID: 46ed3a05e9ff7646d9d8a528
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry; module=65, inline_record=15324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$4
// 信息 ID: 37bd6bb8da5793b5eef5255b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,8>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,8>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,8>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4> >; module=81, inline_record=214692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$5
// 信息 ID: 8aeaba33d129a8d3a40c214f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5 <- alloc::vec::Vec::retain<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,alloc::alloc::Global,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5 <- alloc::vec::impl$1::retain::closure$0<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,alloc::alloc::Global,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,alloc::alloc::Global,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5>; module=81, inline_record=43176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$6
// 信息 ID: 393a0cb0dff90fca77a5e378
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6 <- std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>,uuid::Uuid>,uuid::Uuid>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6 <- core::ptr::drop_in_place<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6 <- core::ptr::drop_in_place<sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>,uuid::Uuid> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6 <- sentry_core::hub::Hub::with_scope<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6 <- sentry_core::hub_impl::HubImpl::with_mut<sentry_core::hub_impl::impl$3::with_current_scope_mut::closure_env$0<sentry_core::hub::impl$0::configure_scope::closure_env$1<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tuple$<> >,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] sentry_core::hub::Hub::with_scope<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>; module=117, inline_record=24780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$7
// 信息 ID: afaae7d8d1653cc7c0d5eb1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7 <- std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>,uuid::Uuid>,uuid::Uuid>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7 <- core::ptr::drop_in_place<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7 <- core::ptr::drop_in_place<sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>,uuid::Uuid> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7 <- sentry_core::hub::Hub::with_scope<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>,uuid::Uuid>,uuid::Uuid>; module=19, inline_record=125996; 不是独立调用边。
// [PDB 内联宿主] std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>,uuid::Uuid>,uuid::Uuid>; module=19, inline_record=126180; 不是独立调用边。
// [PDB 内联宿主] sentry_core::hub::Hub::with_scope<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>; module=117, inline_record=26816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

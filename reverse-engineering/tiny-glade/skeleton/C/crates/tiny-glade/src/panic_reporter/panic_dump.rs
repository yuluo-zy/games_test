// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/panic_reporter/panic_dump.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: b51e89b8dcc31909c5b231ba2b69d55e51497e286e939532c5a02d90a2025776
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: panic_dump.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 39
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_log_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":26628,"rva":1673856,"va":5370382976}
// 编译实例 RVAs: 0x198a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14276110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370382976,"raw_hex":"e80ed6d900","size":5,"soft_fail":false,"target_va":5384659264,"va":5370383149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14273207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370382976,"raw_hex":"e8b7cad900","size":5,"soft_fail":false,"target_va":5384656240,"va":5370383028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t5295573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370382976,"raw_hex":"e8d5cd5000","size":5,"soft_fail":false,"target_va":5375678704,"va":5370383126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1044549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370382976,"raw_hex":"e8bb0ff0ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370383392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d678c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45935949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370382976,"raw_hex":"e84dedbc02","size":5,"soft_fail":false,"target_va":5416319168,"va":5370383214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::tmp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12756433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370382976,"raw_hex":"e8d1a5c200","size":5,"soft_fail":false,"target_va":5383139472,"va":5370383034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14340628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370382976,"raw_hex":"e814d2da00","size":5,"soft_fail":false,"target_va":5384723728,"va":5370383095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":39,"record_offset":27616,"parent_record_offset":26628,"ancestor_record_offset":26628,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":28164,"parent_record_offset":26628,"ancestor_record_offset":26628,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":27728,"parent_record_offset":26628,"ancestor_record_offset":26628,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":26844,"parent_record_offset":26628,"ancestor_record_offset":26628,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":39,"record_offset":26784,"parent_record_offset":26628,"ancestor_record_offset":26628,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":27124,"parent_record_offset":27100,"ancestor_record_offset":26628,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":27148,"parent_record_offset":27124,"ancestor_record_offset":26628,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":27172,"parent_record_offset":27148,"ancestor_record_offset":26628,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":27100,"parent_record_offset":27048,"ancestor_record_offset":26628,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":27072,"parent_record_offset":27048,"ancestor_record_offset":26628,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":27048,"parent_record_offset":27024,"ancestor_record_offset":26628,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":27024,"parent_record_offset":27004,"ancestor_record_offset":26628,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":39,"record_offset":27688,"parent_record_offset":27648,"ancestor_record_offset":26628,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":39,"record_offset":27648,"parent_record_offset":27616,"ancestor_record_offset":26628,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":39,"record_offset":26876,"parent_record_offset":26844,"ancestor_record_offset":26628,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":26904,"parent_record_offset":26844,"ancestor_record_offset":26628,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":21644,"rva":1672784,"va":5370381904}
// 编译实例 RVAs: 0x198650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14274249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370381904,"raw_hex":"e8c9ced900","size":5,"soft_fail":false,"target_va":5384656240,"va":5370381986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5296640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370381904,"raw_hex":"e800d25000","size":5,"soft_fail":false,"target_va":5375678688,"va":5370382043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t5296566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370381904,"raw_hex":"e8b6d15000","size":5,"soft_fail":false,"target_va":5375678704,"va":5370382133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1659841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370381904,"raw_hex":"e83face6ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370382028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14019fa50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370381904,"raw_hex":"e83f730000","size":5,"soft_fail":false,"target_va":5370411600,"va":5370382092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d66e40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45934208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370381904,"raw_hex":"e880e6bc02","size":5,"soft_fail":false,"target_va":5416316480,"va":5370382267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d68480
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45939877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370381904,"raw_hex":"e8a5fcbc02","size":5,"soft_fail":false,"target_va":5416322176,"va":5370382294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d69060
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45942606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370381904,"raw_hex":"e84e07bd02","size":5,"soft_fail":false,"target_va":5416325216,"va":5370382605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t46238163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370381904,"raw_hex":"e8d389c102","size":5,"soft_fail":false,"target_va":5416620794,"va":5370382626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14370057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370381904,"raw_hex":"e80945db00","size":5,"soft_fail":false,"target_va":5384752464,"va":5370382402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$1
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t45942640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370381904,"raw_hex":"e87007bd02","size":5,"soft_fail":false,"target_va":5416325088,"va":5370382443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":21788,"parent_record_offset":21644,"ancestor_record_offset":21644,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":21816,"parent_record_offset":21644,"ancestor_record_offset":21644,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":21908,"parent_record_offset":21644,"ancestor_record_offset":21644,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22376,"parent_record_offset":22352,"ancestor_record_offset":21644,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22400,"parent_record_offset":22376,"ancestor_record_offset":21644,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22752,"parent_record_offset":22728,"ancestor_record_offset":21644,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22776,"parent_record_offset":22752,"ancestor_record_offset":21644,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22424,"parent_record_offset":22400,"ancestor_record_offset":21644,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22800,"parent_record_offset":22776,"ancestor_record_offset":21644,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22352,"parent_record_offset":22324,"ancestor_record_offset":21644,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22728,"parent_record_offset":22672,"ancestor_record_offset":21644,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22264,"parent_record_offset":22236,"ancestor_record_offset":21644,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22700,"parent_record_offset":22672,"ancestor_record_offset":21644,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22672,"parent_record_offset":22648,"ancestor_record_offset":21644,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22324,"parent_record_offset":22292,"ancestor_record_offset":21644,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22292,"parent_record_offset":22264,"ancestor_record_offset":21644,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22648,"parent_record_offset":22624,"ancestor_record_offset":21644,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":22208,"parent_record_offset":22144,"ancestor_record_offset":21644,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":22236,"parent_record_offset":22208,"ancestor_record_offset":21644,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":39,"record_offset":21872,"parent_record_offset":21848,"ancestor_record_offset":21644,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":39,"record_offset":24632,"parent_record_offset":24600,"ancestor_record_offset":21644,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":22600,"parent_record_offset":22144,"ancestor_record_offset":21644,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":25232,"parent_record_offset":21948,"ancestor_record_offset":21644,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":24176,"parent_record_offset":21948,"ancestor_record_offset":21644,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":39,"record_offset":22568,"parent_record_offset":22484,"ancestor_record_offset":21644,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":22512,"parent_record_offset":22484,"ancestor_record_offset":21644,"inlinee":8447,"source_line":28,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":22536,"parent_record_offset":22512,"ancestor_record_offset":21644,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_string
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":22092,"parent_record_offset":21948,"ancestor_record_offset":21644,"inlinee":8444,"source_line":2245,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":22144,"parent_record_offset":22092,"ancestor_record_offset":21644,"inlinee":8445,"source_line":2213,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":22484,"parent_record_offset":22144,"ancestor_record_offset":21644,"inlinee":8446,"source_line":2177,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":24600,"parent_record_offset":21948,"ancestor_record_offset":21644,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":39,"record_offset":22064,"parent_record_offset":21948,"ancestor_record_offset":21644,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":39,"record_offset":21848,"parent_record_offset":21816,"ancestor_record_offset":21644,"inlinee":8450,"source_line":29,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":39,"record_offset":21948,"parent_record_offset":21908,"ancestor_record_offset":21644,"inlinee":8443,"source_line":30,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2102
// 首条位置: {"module":39,"inlinee":8450}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":39,"record_offset":21872,"parent_record_offset":21848,"ancestor_record_offset":21848,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20FB
// 首条位置: {"module":39,"inlinee":8443}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::__private::format_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":39,"record_offset":24148,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":7509,"source_line":687,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":24232,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":25232,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":24176,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":22092,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":8444,"source_line":2245,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":24600,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":39,"record_offset":22064,"parent_record_offset":21948,"ancestor_record_offset":21948,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22376,"parent_record_offset":22352,"ancestor_record_offset":21948,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22400,"parent_record_offset":22376,"ancestor_record_offset":21948,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22752,"parent_record_offset":22728,"ancestor_record_offset":21948,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22776,"parent_record_offset":22752,"ancestor_record_offset":21948,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22424,"parent_record_offset":22400,"ancestor_record_offset":21948,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22800,"parent_record_offset":22776,"ancestor_record_offset":21948,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22352,"parent_record_offset":22324,"ancestor_record_offset":21948,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":22728,"parent_record_offset":22672,"ancestor_record_offset":21948,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22264,"parent_record_offset":22236,"ancestor_record_offset":21948,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22700,"parent_record_offset":22672,"ancestor_record_offset":21948,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22672,"parent_record_offset":22648,"ancestor_record_offset":21948,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22324,"parent_record_offset":22292,"ancestor_record_offset":21948,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22292,"parent_record_offset":22264,"ancestor_record_offset":21948,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":22648,"parent_record_offset":22624,"ancestor_record_offset":21948,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":22208,"parent_record_offset":22144,"ancestor_record_offset":21948,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":22236,"parent_record_offset":22208,"ancestor_record_offset":21948,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":39,"record_offset":24632,"parent_record_offset":24600,"ancestor_record_offset":21948,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":22600,"parent_record_offset":22144,"ancestor_record_offset":21948,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":39,"record_offset":22568,"parent_record_offset":22484,"ancestor_record_offset":21948,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":22512,"parent_record_offset":22484,"ancestor_record_offset":21948,"inlinee":8447,"source_line":28,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":22536,"parent_record_offset":22512,"ancestor_record_offset":21948,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":22144,"parent_record_offset":22092,"ancestor_record_offset":21948,"inlinee":8445,"source_line":2213,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":39,"record_offset":22484,"parent_record_offset":22144,"ancestor_record_offset":21948,"inlinee":8446,"source_line":2177,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":26476,"rva":47615968,"va":5416325088}
// 编译实例 RVAs: 0x2d68fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-46986311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416325088,"raw_hex":"e8b90b33fd","size":5,"soft_fail":false,"target_va":5369338848,"va":5416325154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d69060
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t70","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416325088,"raw_hex":"e846000000","size":5,"soft_fail":false,"target_va":5416325216,"va":5416325141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":35144,"rva":1676144,"va":5370385264}
// 编译实例 RVAs: 0x199370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14409246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e81ededb00","size":5,"soft_fail":false,"target_va":5384795984,"va":5370386733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14318058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e8ea79da00","size":5,"soft_fail":false,"target_va":5384703536,"va":5370385473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14270869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e895c1d900","size":5,"soft_fail":false,"target_va":5384656240,"va":5370385366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5292542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e8fec15000","size":5,"soft_fail":false,"target_va":5375678688,"va":5370386141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t5292580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e824c25000","size":5,"soft_fail":false,"target_va":5375678704,"va":5370386119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1663939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e83d9ce6ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5370386126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1046726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e83a07f0ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370385569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b0540
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-956651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e81567f1ff","size":5,"soft_fail":false,"target_va":5369431360,"va":5370388006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401a2170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e8488b0000","size":5,"soft_fail":false,"target_va":5370421616,"va":5370385955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402fb580
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1448859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e89b1b1600","size":5,"soft_fail":false,"target_va":5371835776,"va":5370386912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403012e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1473908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e8747d1600","size":5,"soft_fail":false,"target_va":5371859680,"va":5370385767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e7ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8710495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e85fe98400","size":5,"soft_fail":false,"target_va":5379096304,"va":5370385804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c71d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11372499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e8d387ad00","size":5,"soft_fail":false,"target_va":5381758240,"va":5370385736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14409324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e86cdedb00","size":5,"soft_fail":false,"target_va":5384794864,"va":5370385535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t46234345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e8e97ac102","size":5,"soft_fail":false,"target_va":5416620794,"va":5370386444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14238013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e83d41d900","size":5,"soft_fail":false,"target_va":5384624080,"va":5370386062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46234267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e89b7ac102","size":5,"soft_fail":false,"target_va":5416621600,"va":5370387328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::iter_resolved_snapshots
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10165428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e8b41c9b00","size":5,"soft_fail":false,"target_va":5380551184,"va":5370385751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::HistoryIdentity::generate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10159251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e893049b00","size":5,"soft_fail":false,"target_va":5380544880,"va":5370385624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::weather::Weather::fog_save_state
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11450348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e8ecb7ae00","size":5,"soft_fail":false,"target_va":5381836000,"va":5370385647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::weather::Weather::precipitation_save_state
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11450371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e803b8ae00","size":5,"soft_fail":false,"target_va":5381836032,"va":5370385656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::store_parameters
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8932855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370385264,"raw_hex":"e8f74d8800","size":5,"soft_fail":false,"target_va":5379318560,"va":5370385700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*55774965(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5370385264,"raw_hex":"ff15f50e5303","size":6,"soft_fail":false,"va":5370386205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":35288,"parent_record_offset":35144,"ancestor_record_offset":35144,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":35316,"parent_record_offset":35144,"ancestor_record_offset":35144,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":35344,"parent_record_offset":35144,"ancestor_record_offset":35144,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36968,"parent_record_offset":36944,"ancestor_record_offset":35144,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36992,"parent_record_offset":36968,"ancestor_record_offset":35144,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36576,"parent_record_offset":36552,"ancestor_record_offset":35144,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36600,"parent_record_offset":36576,"ancestor_record_offset":35144,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":37016,"parent_record_offset":36992,"ancestor_record_offset":35144,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36624,"parent_record_offset":36600,"ancestor_record_offset":35144,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36944,"parent_record_offset":36916,"ancestor_record_offset":35144,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36552,"parent_record_offset":36464,"ancestor_record_offset":35144,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":39,"record_offset":35844,"parent_record_offset":35816,"ancestor_record_offset":35144,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":39,"record_offset":35892,"parent_record_offset":35868,"ancestor_record_offset":35144,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36856,"parent_record_offset":36828,"ancestor_record_offset":35144,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36492,"parent_record_offset":36464,"ancestor_record_offset":35144,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36464,"parent_record_offset":36440,"ancestor_record_offset":35144,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36916,"parent_record_offset":36884,"ancestor_record_offset":35144,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36884,"parent_record_offset":36856,"ancestor_record_offset":35144,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36440,"parent_record_offset":36416,"ancestor_record_offset":35144,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":39696,"parent_record_offset":39648,"ancestor_record_offset":35144,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":36736,"parent_record_offset":36708,"ancestor_record_offset":35144,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":36764,"parent_record_offset":36736,"ancestor_record_offset":35144,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":39472,"parent_record_offset":39400,"ancestor_record_offset":35144,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":36792,"parent_record_offset":36764,"ancestor_record_offset":35144,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":36708,"parent_record_offset":36680,"ancestor_record_offset":35144,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":39,"record_offset":36680,"parent_record_offset":35380,"ancestor_record_offset":35144,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":39444,"parent_record_offset":39400,"ancestor_record_offset":35144,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":36828,"parent_record_offset":36792,"ancestor_record_offset":35144,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":39364,"parent_record_offset":39300,"ancestor_record_offset":35144,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":36076,"parent_record_offset":36048,"ancestor_record_offset":35144,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":39400,"parent_record_offset":39364,"ancestor_record_offset":35144,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":39,"record_offset":39720,"parent_record_offset":39696,"ancestor_record_offset":35144,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":39,"record_offset":35724,"parent_record_offset":35576,"ancestor_record_offset":35144,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":39,"record_offset":35928,"parent_record_offset":35816,"ancestor_record_offset":35144,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":39,"record_offset":39768,"parent_record_offset":39744,"ancestor_record_offset":35144,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":39,"record_offset":39980,"parent_record_offset":39936,"ancestor_record_offset":35144,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":39,"record_offset":39936,"parent_record_offset":39472,"ancestor_record_offset":35144,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":39,"record_offset":36048,"parent_record_offset":36020,"ancestor_record_offset":35144,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":39,"record_offset":40072,"parent_record_offset":39472,"ancestor_record_offset":35144,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":38296,"parent_record_offset":38268,"ancestor_record_offset":35144,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":38324,"parent_record_offset":38296,"ancestor_record_offset":35144,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":36520,"parent_record_offset":36492,"ancestor_record_offset":35144,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":39124,"parent_record_offset":35380,"ancestor_record_offset":35144,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":35868,"parent_record_offset":35844,"ancestor_record_offset":35144,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":39216,"parent_record_offset":39172,"ancestor_record_offset":35144,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":40116,"parent_record_offset":39300,"ancestor_record_offset":35144,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":39,"record_offset":37072,"parent_record_offset":36792,"ancestor_record_offset":35144,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":37096,"parent_record_offset":37072,"ancestor_record_offset":35144,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=52
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":36300,"parent_record_offset":36160,"ancestor_record_offset":35144,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":38360,"parent_record_offset":38236,"ancestor_record_offset":35144,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":40032,"parent_record_offset":40008,"ancestor_record_offset":35144,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":35784,"parent_record_offset":35380,"ancestor_record_offset":35144,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":35964,"parent_record_offset":35380,"ancestor_record_offset":35144,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":36364,"parent_record_offset":35380,"ancestor_record_offset":35144,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":40008,"parent_record_offset":39980,"ancestor_record_offset":35144,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":39,"record_offset":40180,"parent_record_offset":40144,"ancestor_record_offset":35144,"inlinee":8473,"source_line":24,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::history_chain::HistoryEditChain::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":39,"record_offset":35992,"parent_record_offset":35380,"ancestor_record_offset":35144,"inlinee":8459,"source_line":402,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":39,"record_offset":40144,"parent_record_offset":40116,"ancestor_record_offset":35144,"inlinee":8471,"source_line":27,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":39,"record_offset":40224,"parent_record_offset":40144,"ancestor_record_offset":35144,"inlinee":8475,"source_line":11,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":39,"record_offset":40316,"parent_record_offset":40288,"ancestor_record_offset":35144,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":39,"record_offset":39300,"parent_record_offset":39252,"ancestor_record_offset":35144,"inlinee":8466,"source_line":176,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":39,"record_offset":39252,"parent_record_offset":39172,"ancestor_record_offset":35144,"inlinee":8464,"source_line":278,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":39,"record_offset":40288,"parent_record_offset":40260,"ancestor_record_offset":35144,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":38236,"parent_record_offset":38200,"ancestor_record_offset":35144,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":38168,"parent_record_offset":38140,"ancestor_record_offset":35144,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":38200,"parent_record_offset":38168,"ancestor_record_offset":35144,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":38268,"parent_record_offset":38236,"ancestor_record_offset":35144,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":38140,"parent_record_offset":38112,"ancestor_record_offset":35144,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::collect_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":39,"record_offset":36020,"parent_record_offset":35380,"ancestor_record_offset":35144,"inlinee":8460,"source_line":2348,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":35576,"parent_record_offset":35380,"ancestor_record_offset":35144,"inlinee":7543,"source_line":638,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":35600,"parent_record_offset":35576,"ancestor_record_offset":35144,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":35752,"parent_record_offset":35576,"ancestor_record_offset":35144,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":39,"record_offset":37244,"parent_record_offset":37220,"ancestor_record_offset":35144,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":39,"record_offset":35548,"parent_record_offset":35380,"ancestor_record_offset":35144,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":39,"record_offset":35664,"parent_record_offset":35600,"ancestor_record_offset":35144,"inlinee":7545,"source_line":227,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":39,"record_offset":35692,"parent_record_offset":35600,"ancestor_record_offset":35144,"inlinee":7546,"source_line":224,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":39,"record_offset":35636,"parent_record_offset":35600,"ancestor_record_offset":35144,"inlinee":7544,"source_line":218,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":39,"record_offset":35380,"parent_record_offset":35344,"ancestor_record_offset":35144,"inlinee":8455,"source_line":65,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":39,"record_offset":35816,"parent_record_offset":35784,"ancestor_record_offset":35144,"inlinee":8457,"source_line":70,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":39,"record_offset":36160,"parent_record_offset":36116,"ancestor_record_offset":35144,"inlinee":8461,"source_line":102,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":39,"record_offset":39172,"parent_record_offset":39124,"ancestor_record_offset":35144,"inlinee":8462,"source_line":106,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2107
// 首条位置: {"module":39,"inlinee":8455}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":39,"record_offset":36680,"parent_record_offset":35380,"ancestor_record_offset":35380,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":39124,"parent_record_offset":35380,"ancestor_record_offset":35380,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":36392,"parent_record_offset":35380,"ancestor_record_offset":35380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":35784,"parent_record_offset":35380,"ancestor_record_offset":35380,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":35964,"parent_record_offset":35380,"ancestor_record_offset":35380,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":36364,"parent_record_offset":35380,"ancestor_record_offset":35380,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::history_chain::HistoryEditChain::is_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":39,"record_offset":35992,"parent_record_offset":35380,"ancestor_record_offset":35380,"inlinee":8459,"source_line":402,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::collect_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":39,"record_offset":36020,"parent_record_offset":35380,"ancestor_record_offset":35380,"inlinee":8460,"source_line":2348,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":35576,"parent_record_offset":35380,"ancestor_record_offset":35380,"inlinee":7543,"source_line":638,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":39,"record_offset":35548,"parent_record_offset":35380,"ancestor_record_offset":35380,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36968,"parent_record_offset":36944,"ancestor_record_offset":35380,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36992,"parent_record_offset":36968,"ancestor_record_offset":35380,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36576,"parent_record_offset":36552,"ancestor_record_offset":35380,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36600,"parent_record_offset":36576,"ancestor_record_offset":35380,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":37016,"parent_record_offset":36992,"ancestor_record_offset":35380,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36624,"parent_record_offset":36600,"ancestor_record_offset":35380,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36944,"parent_record_offset":36916,"ancestor_record_offset":35380,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":36552,"parent_record_offset":36464,"ancestor_record_offset":35380,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":39,"record_offset":35844,"parent_record_offset":35816,"ancestor_record_offset":35380,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":39,"record_offset":35892,"parent_record_offset":35868,"ancestor_record_offset":35380,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36856,"parent_record_offset":36828,"ancestor_record_offset":35380,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36492,"parent_record_offset":36464,"ancestor_record_offset":35380,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36464,"parent_record_offset":36440,"ancestor_record_offset":35380,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36916,"parent_record_offset":36884,"ancestor_record_offset":35380,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36884,"parent_record_offset":36856,"ancestor_record_offset":35380,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":36440,"parent_record_offset":36416,"ancestor_record_offset":35380,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":39696,"parent_record_offset":39648,"ancestor_record_offset":35380,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":36736,"parent_record_offset":36708,"ancestor_record_offset":35380,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":36764,"parent_record_offset":36736,"ancestor_record_offset":35380,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":39472,"parent_record_offset":39400,"ancestor_record_offset":35380,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":36792,"parent_record_offset":36764,"ancestor_record_offset":35380,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":36708,"parent_record_offset":36680,"ancestor_record_offset":35380,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":39444,"parent_record_offset":39400,"ancestor_record_offset":35380,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":36828,"parent_record_offset":36792,"ancestor_record_offset":35380,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":39364,"parent_record_offset":39300,"ancestor_record_offset":35380,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":36076,"parent_record_offset":36048,"ancestor_record_offset":35380,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":39400,"parent_record_offset":39364,"ancestor_record_offset":35380,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":39,"record_offset":39720,"parent_record_offset":39696,"ancestor_record_offset":35380,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":39,"record_offset":35724,"parent_record_offset":35576,"ancestor_record_offset":35380,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":39,"record_offset":35928,"parent_record_offset":35816,"ancestor_record_offset":35380,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":39,"record_offset":39768,"parent_record_offset":39744,"ancestor_record_offset":35380,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":39,"record_offset":39980,"parent_record_offset":39936,"ancestor_record_offset":35380,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":39,"record_offset":39936,"parent_record_offset":39472,"ancestor_record_offset":35380,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":39,"record_offset":36048,"parent_record_offset":36020,"ancestor_record_offset":35380,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":39,"record_offset":40072,"parent_record_offset":39472,"ancestor_record_offset":35380,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":38296,"parent_record_offset":38268,"ancestor_record_offset":35380,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":38324,"parent_record_offset":38296,"ancestor_record_offset":35380,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":36520,"parent_record_offset":36492,"ancestor_record_offset":35380,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":35868,"parent_record_offset":35844,"ancestor_record_offset":35380,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":39216,"parent_record_offset":39172,"ancestor_record_offset":35380,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":40116,"parent_record_offset":39300,"ancestor_record_offset":35380,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":39,"record_offset":37072,"parent_record_offset":36792,"ancestor_record_offset":35380,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":37096,"parent_record_offset":37072,"ancestor_record_offset":35380,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":36300,"parent_record_offset":36160,"ancestor_record_offset":35380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":39,"record_offset":38360,"parent_record_offset":38236,"ancestor_record_offset":35380,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":40032,"parent_record_offset":40008,"ancestor_record_offset":35380,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":40008,"parent_record_offset":39980,"ancestor_record_offset":35380,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":39,"record_offset":40180,"parent_record_offset":40144,"ancestor_record_offset":35380,"inlinee":8473,"source_line":24,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":39,"record_offset":40144,"parent_record_offset":40116,"ancestor_record_offset":35380,"inlinee":8471,"source_line":27,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":39,"record_offset":40224,"parent_record_offset":40144,"ancestor_record_offset":35380,"inlinee":8475,"source_line":11,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":39,"record_offset":40316,"parent_record_offset":40288,"ancestor_record_offset":35380,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":39,"record_offset":39300,"parent_record_offset":39252,"ancestor_record_offset":35380,"inlinee":8466,"source_line":176,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":39,"record_offset":39252,"parent_record_offset":39172,"ancestor_record_offset":35380,"inlinee":8464,"source_line":278,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":39,"record_offset":40288,"parent_record_offset":40260,"ancestor_record_offset":35380,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":38236,"parent_record_offset":38200,"ancestor_record_offset":35380,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":38168,"parent_record_offset":38140,"ancestor_record_offset":35380,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":38200,"parent_record_offset":38168,"ancestor_record_offset":35380,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":38268,"parent_record_offset":38236,"ancestor_record_offset":35380,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":39,"record_offset":38140,"parent_record_offset":38112,"ancestor_record_offset":35380,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":35600,"parent_record_offset":35576,"ancestor_record_offset":35380,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":35752,"parent_record_offset":35576,"ancestor_record_offset":35380,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":39,"record_offset":37244,"parent_record_offset":37220,"ancestor_record_offset":35380,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":39,"record_offset":35664,"parent_record_offset":35600,"ancestor_record_offset":35380,"inlinee":7545,"source_line":227,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":39,"record_offset":35692,"parent_record_offset":35600,"ancestor_record_offset":35380,"inlinee":7546,"source_line":224,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":39,"record_offset":35636,"parent_record_offset":35600,"ancestor_record_offset":35380,"inlinee":7544,"source_line":218,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":39,"record_offset":35816,"parent_record_offset":35784,"ancestor_record_offset":35380,"inlinee":8457,"source_line":70,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":39,"record_offset":36160,"parent_record_offset":36116,"ancestor_record_offset":35380,"inlinee":8461,"source_line":102,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/panic_dump.rs
//     evidence={"module":39,"record_offset":39172,"parent_record_offset":39124,"ancestor_record_offset":35380,"inlinee":8462,"source_line":106,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2109
// 首条位置: {"module":39,"inlinee":8457}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":39,"record_offset":35844,"parent_record_offset":35816,"ancestor_record_offset":35816,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":39,"record_offset":35928,"parent_record_offset":35816,"ancestor_record_offset":35816,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":39,"record_offset":35892,"parent_record_offset":35868,"ancestor_record_offset":35816,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":35868,"parent_record_offset":35844,"ancestor_record_offset":35816,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x210D
// 首条位置: {"module":39,"inlinee":8461}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":39,"record_offset":36188,"parent_record_offset":36160,"ancestor_record_offset":36160,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":39,"record_offset":36272,"parent_record_offset":36160,"ancestor_record_offset":36160,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":36300,"parent_record_offset":36160,"ancestor_record_offset":36160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":39,"record_offset":36236,"parent_record_offset":36212,"ancestor_record_offset":36160,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":36212,"parent_record_offset":36188,"ancestor_record_offset":36160,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x210E
// 首条位置: {"module":39,"inlinee":8462}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":39216,"parent_record_offset":39172,"ancestor_record_offset":39172,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$31::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":39,"record_offset":39252,"parent_record_offset":39172,"ancestor_record_offset":39172,"inlinee":8464,"source_line":278,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":39832,"parent_record_offset":39808,"ancestor_record_offset":39172,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":39856,"parent_record_offset":39832,"ancestor_record_offset":39172,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":40504,"parent_record_offset":40480,"ancestor_record_offset":39172,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":40528,"parent_record_offset":40504,"ancestor_record_offset":39172,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":39880,"parent_record_offset":39856,"ancestor_record_offset":39172,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":40552,"parent_record_offset":40528,"ancestor_record_offset":39172,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":39808,"parent_record_offset":39648,"ancestor_record_offset":39172,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":40480,"parent_record_offset":40424,"ancestor_record_offset":39172,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":39568,"parent_record_offset":39532,"ancestor_record_offset":39172,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":40452,"parent_record_offset":40424,"ancestor_record_offset":39172,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":40424,"parent_record_offset":40400,"ancestor_record_offset":39172,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":39648,"parent_record_offset":39604,"ancestor_record_offset":39172,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":39604,"parent_record_offset":39568,"ancestor_record_offset":39172,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":40400,"parent_record_offset":40376,"ancestor_record_offset":39172,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":39696,"parent_record_offset":39648,"ancestor_record_offset":39172,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":39,"record_offset":39472,"parent_record_offset":39400,"ancestor_record_offset":39172,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":39444,"parent_record_offset":39400,"ancestor_record_offset":39172,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":39532,"parent_record_offset":39472,"ancestor_record_offset":39172,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":39364,"parent_record_offset":39300,"ancestor_record_offset":39172,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":39,"record_offset":39400,"parent_record_offset":39364,"ancestor_record_offset":39172,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":39,"record_offset":39720,"parent_record_offset":39696,"ancestor_record_offset":39172,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":39,"record_offset":39768,"parent_record_offset":39744,"ancestor_record_offset":39172,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":39,"record_offset":39980,"parent_record_offset":39936,"ancestor_record_offset":39172,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":39,"record_offset":39936,"parent_record_offset":39472,"ancestor_record_offset":39172,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":39,"record_offset":40072,"parent_record_offset":39472,"ancestor_record_offset":39172,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":39,"record_offset":39744,"parent_record_offset":39720,"ancestor_record_offset":39172,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":40116,"parent_record_offset":39300,"ancestor_record_offset":39172,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":40352,"parent_record_offset":39300,"ancestor_record_offset":39172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":39,"record_offset":40032,"parent_record_offset":40008,"ancestor_record_offset":39172,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":39,"record_offset":40008,"parent_record_offset":39980,"ancestor_record_offset":39172,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":39,"record_offset":40180,"parent_record_offset":40144,"ancestor_record_offset":39172,"inlinee":8473,"source_line":24,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":39,"record_offset":40144,"parent_record_offset":40116,"ancestor_record_offset":39172,"inlinee":8471,"source_line":27,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":39,"record_offset":40224,"parent_record_offset":40144,"ancestor_record_offset":39172,"inlinee":8475,"source_line":11,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":39,"record_offset":40316,"parent_record_offset":40288,"ancestor_record_offset":39172,"inlinee":8479,"source_line":122,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":39,"record_offset":39300,"parent_record_offset":39252,"ancestor_record_offset":39172,"inlinee":8466,"source_line":176,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::import::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session/import.rs
//     evidence={"module":39,"record_offset":40288,"parent_record_offset":40260,"ancestor_record_offset":39172,"inlinee":8477,"source_line":17,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_settings_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":30852,"rva":1674992,"va":5370384112}
// 编译实例 RVAs: 0x198ef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14274931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384112,"raw_hex":"e873d1d900","size":5,"soft_fail":false,"target_va":5384659264,"va":5370384328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14272054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384112,"raw_hex":"e836c6d900","size":5,"soft_fail":false,"target_va":5384656240,"va":5370384181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14272814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384112,"raw_hex":"e82ec9d900","size":5,"soft_fail":false,"target_va":5384657024,"va":5370384205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t5294393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384112,"raw_hex":"e839c95000","size":5,"soft_fail":false,"target_va":5375678704,"va":5370384306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1045749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384112,"raw_hex":"e80b0bf0ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370384592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d678c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45934771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384112,"raw_hex":"e8b3e8bc02","size":5,"soft_fail":false,"target_va":5416319168,"va":5370384392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46237111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384112,"raw_hex":"e8b785c102","size":5,"soft_fail":false,"target_va":5416621600,"va":5370384484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12755413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384112,"raw_hex":"e8d5a1c200","size":5,"soft_fail":false,"target_va":5383139568,"va":5370384150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14339448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384112,"raw_hex":"e878cdda00","size":5,"soft_fail":false,"target_va":5384723728,"va":5370384275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":39,"record_offset":31940,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":31076,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":32488,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":32052,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":31140,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":39,"record_offset":31112,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::settings_file_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/app_config.rs
//     evidence={"module":39,"record_offset":31020,"parent_record_offset":30852,"ancestor_record_offset":30852,"inlinee":8453,"source_line":207,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":31448,"parent_record_offset":31424,"ancestor_record_offset":30852,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":31472,"parent_record_offset":31448,"ancestor_record_offset":30852,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":31496,"parent_record_offset":31472,"ancestor_record_offset":30852,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":31424,"parent_record_offset":31368,"ancestor_record_offset":30852,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":31396,"parent_record_offset":31368,"ancestor_record_offset":30852,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":31368,"parent_record_offset":31344,"ancestor_record_offset":30852,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":31344,"parent_record_offset":31320,"ancestor_record_offset":30852,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":39,"record_offset":32012,"parent_record_offset":31972,"ancestor_record_offset":30852,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":39,"record_offset":31972,"parent_record_offset":31940,"ancestor_record_offset":30852,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":39,"record_offset":31172,"parent_record_offset":31140,"ancestor_record_offset":30852,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":31200,"parent_record_offset":31140,"ancestor_record_offset":30852,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":39,"record_offset":31044,"parent_record_offset":31020,"ancestor_record_offset":30852,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_profile_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":28536,"rva":1674368,"va":5370383488}
// 编译实例 RVAs: 0x198c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14275582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370383488,"raw_hex":"e8fed3d900","size":5,"soft_fail":false,"target_va":5384659264,"va":5370383677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14272644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370383488,"raw_hex":"e884c8d900","size":5,"soft_fail":false,"target_va":5384656240,"va":5370383591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14273465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370383488,"raw_hex":"e8b9cbd900","size":5,"soft_fail":false,"target_va":5384657024,"va":5370383554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t5295044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370383488,"raw_hex":"e8c4cb5000","size":5,"soft_fail":false,"target_va":5375678704,"va":5370383655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1045109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370383488,"raw_hex":"e88b0df0ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370383952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d678c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45935422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370383488,"raw_hex":"e83eebbc02","size":5,"soft_fail":false,"target_va":5416319168,"va":5370383741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46237762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370383488,"raw_hex":"e84288c102","size":5,"soft_fail":false,"target_va":5416621600,"va":5370383833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::game_profile::profile_file_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8978225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370383488,"raw_hex":"e831ff8800","size":5,"soft_fail":false,"target_va":5379361760,"va":5370383530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14340099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370383488,"raw_hex":"e803d0da00","size":5,"soft_fail":false,"target_va":5384723728,"va":5370383624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":39,"record_offset":29564,"parent_record_offset":28536,"ancestor_record_offset":28536,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":39,"record_offset":28700,"parent_record_offset":28536,"ancestor_record_offset":28536,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":30112,"parent_record_offset":28536,"ancestor_record_offset":28536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":29676,"parent_record_offset":28536,"ancestor_record_offset":28536,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":28764,"parent_record_offset":28536,"ancestor_record_offset":28536,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":39,"record_offset":28736,"parent_record_offset":28536,"ancestor_record_offset":28536,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":29072,"parent_record_offset":29048,"ancestor_record_offset":28536,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":29096,"parent_record_offset":29072,"ancestor_record_offset":28536,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":29120,"parent_record_offset":29096,"ancestor_record_offset":28536,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":29048,"parent_record_offset":28992,"ancestor_record_offset":28536,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":29020,"parent_record_offset":28992,"ancestor_record_offset":28536,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":28992,"parent_record_offset":28968,"ancestor_record_offset":28536,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":28968,"parent_record_offset":28944,"ancestor_record_offset":28536,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":39,"record_offset":29636,"parent_record_offset":29596,"ancestor_record_offset":28536,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":39,"record_offset":29596,"parent_record_offset":29564,"ancestor_record_offset":28536,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":39,"record_offset":28796,"parent_record_offset":28764,"ancestor_record_offset":28536,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":28824,"parent_record_offset":28764,"ancestor_record_offset":28536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_dump::dump_gpu_crash_dump_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":39,"record_offset":33228,"rva":1675632,"va":5370384752}
// 编译实例 RVAs: 0x199170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14274334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384752,"raw_hex":"e81ecfd900","size":5,"soft_fail":false,"target_va":5384659264,"va":5370384925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14271431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384752,"raw_hex":"e8c7c3d900","size":5,"soft_fail":false,"target_va":5384656240,"va":5370384804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t5293797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384752,"raw_hex":"e8e5c65000","size":5,"soft_fail":false,"target_va":5375678704,"va":5370384902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1046325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384752,"raw_hex":"e8cb08f0ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5370385168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d678c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: panic_dump.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t45934173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384752,"raw_hex":"e85de6bc02","size":5,"soft_fail":false,"target_va":5416319168,"va":5370384990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::tmp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12754657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384752,"raw_hex":"e8e19ec200","size":5,"soft_fail":false,"target_va":5383139472,"va":5370384810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14338852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370384752,"raw_hex":"e824cbda00","size":5,"soft_fail":false,"target_va":5384723728,"va":5370384871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":39,"record_offset":34224,"parent_record_offset":33228,"ancestor_record_offset":33228,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":34772,"parent_record_offset":33228,"ancestor_record_offset":33228,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":39,"record_offset":34336,"parent_record_offset":33228,"ancestor_record_offset":33228,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::fs::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":39,"record_offset":33452,"parent_record_offset":33228,"ancestor_record_offset":33228,"inlinee":8451,"source_line":2880,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":39,"record_offset":33392,"parent_record_offset":33228,"ancestor_record_offset":33228,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":33732,"parent_record_offset":33708,"ancestor_record_offset":33228,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":33756,"parent_record_offset":33732,"ancestor_record_offset":33228,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":33780,"parent_record_offset":33756,"ancestor_record_offset":33228,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":39,"record_offset":33708,"parent_record_offset":33656,"ancestor_record_offset":33228,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":33680,"parent_record_offset":33656,"ancestor_record_offset":33228,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":33656,"parent_record_offset":33632,"ancestor_record_offset":33228,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":39,"record_offset":33632,"parent_record_offset":33612,"ancestor_record_offset":33228,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":39,"record_offset":34296,"parent_record_offset":34256,"ancestor_record_offset":33228,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":39,"record_offset":34256,"parent_record_offset":34224,"ancestor_record_offset":33228,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":39,"record_offset":33484,"parent_record_offset":33452,"ancestor_record_offset":33228,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":39,"record_offset":33512,"parent_record_offset":33452,"ancestor_record_offset":33228,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00039.jsonl.gz"}
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
// 完整结构及原始类型记录: panic_dump.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: panic_dump.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: panic_dump.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_gpu_crash_dump_file
// 信息 ID: 7380f1301c9f734101c95651
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x185cb5 -> 0x199170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs
// 信息 ID: 7a24b63fca25815bb8f11aba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x18575d -> 0x198650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$0
// 信息 ID: 5a5de45995abb3290314b779
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_inputs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_inputs; module=39, inline_record=21848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1
// 信息 ID: 05cc73238f88b62561a1a532
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_inputs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_inputs; module=39, inline_record=21948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_inputs::closure$1::closure$1
// 信息 ID: 8ea6721c404a342cd87e6a8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_inputs", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_dump::dump_inputs @ 0x19886b -> 0x2d68fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_dump::dump_inputs @ 0x1988b3 -> 0x2d68fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_log_file
// 信息 ID: 08942a767d56df3364d2738c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x185913 -> 0x198a80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_profile_file
// 信息 ID: 2e77b797be9ed00dcc4d9233
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x185b7f -> 0x198c80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save
// 信息 ID: 84f70830d29dfa6c0a5c7893
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x185630 -> 0x199370 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0
// 信息 ID: 968d51f820fe42e1cd07ec69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_save", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=39, inline_record=35380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$0
// 信息 ID: 42b32b3e9a359cb26ce98eea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_save", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=39, inline_record=35816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$1
// 信息 ID: 30fa03acbbf93d340b93c3d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_save", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=39, inline_record=36160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_save::closure$0::closure$2
// 信息 ID: c570553a5473f6b0f06bbc14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::panic_dump::dump_save", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::panic_dump::dump_save; module=39, inline_record=39172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::panic_dump::dump_settings_file
// 信息 ID: 337c2e075415d297fc1346d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x185a49 -> 0x198ef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

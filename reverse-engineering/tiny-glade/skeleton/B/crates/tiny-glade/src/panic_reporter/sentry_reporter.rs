// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6b1be22e8845813cce01a338e09efa71bb0536797bc319603d7f93c275b6251e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sentry_reporter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 20, 21, 104, 125, 184, 190
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use raw_cpuid; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.9.0"},{"name":"raw-cpuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"11.1.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
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
// 首条位置: {"module":190,"record_offset":30892,"rva":4934992,"va":5373644112}
// 编译实例 RVAs: 0x4b4d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140312020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1716908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e854cde5ff","size":5,"soft_fail":false,"target_va":5371928608,"va":5373645511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140312180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1716419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e83dcfe5ff","size":5,"soft_fail":false,"target_va":5371928960,"va":5373645374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1436927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e8ffec1500","size":5,"soft_fail":false,"target_va":5375081488,"va":5373644556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1437206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e816ee1500","size":5,"soft_fail":false,"target_va":5375081504,"va":5373644293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9060369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e811408a00","size":5,"soft_fail":false,"target_va":5382704752,"va":5373644378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9183376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e890208c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5373644347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2020
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37671473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e831d23e02","size":5,"soft_fail":false,"target_va":5411315744,"va":5373644266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3890
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37677512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e8c8e93e02","size":5,"soft_fail":false,"target_va":5411322000,"va":5373644483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38202566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e8c6ec4602","size":5,"soft_fail":false,"target_va":5411847168,"va":5373644597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37895250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e8523c4202","size":5,"soft_fail":false,"target_va":5411540963,"va":5373645708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9050192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e850188a00","size":5,"soft_fail":false,"target_va":5382694656,"va":5373644459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9059319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e8f73b8a00","size":5,"soft_fail":false,"target_va":5382704816,"va":5373645492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9099349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e855d88a00","size":5,"soft_fail":false,"target_va":5382743968,"va":5373644614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9097144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e8b8cf8a00","size":5,"soft_fail":false,"target_va":5382741504,"va":5373644355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9131325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373644112,"raw_hex":"e83d558b00","size":5,"soft_fail":false,"target_va":5382776592,"va":5373645262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":190,"record_offset":32048,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":190,"record_offset":31824,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":12564,"source_line":91,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":190,"record_offset":35204,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":190,"record_offset":31932,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":5145,"source_line":1277,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":33896,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":31096,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":35172,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":31656,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":190,"record_offset":31900,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":190,"record_offset":35232,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":190,"record_offset":35072,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":14362,"source_line":2913,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":190,"record_offset":31684,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::sentry_reporter::extension_to_content_type
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":190,"record_offset":33004,"parent_record_offset":30892,"ancestor_record_offset":30892,"inlinee":14361,"source_line":176,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":32512,"parent_record_offset":32488,"ancestor_record_offset":30892,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":32536,"parent_record_offset":32512,"ancestor_record_offset":30892,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":31368,"parent_record_offset":31344,"ancestor_record_offset":30892,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":32488,"parent_record_offset":32312,"ancestor_record_offset":30892,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":31344,"parent_record_offset":31288,"ancestor_record_offset":30892,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":32252,"parent_record_offset":32224,"ancestor_record_offset":30892,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":31316,"parent_record_offset":31288,"ancestor_record_offset":30892,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":31288,"parent_record_offset":31264,"ancestor_record_offset":30892,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":32312,"parent_record_offset":32280,"ancestor_record_offset":30892,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":32280,"parent_record_offset":32252,"ancestor_record_offset":30892,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":31264,"parent_record_offset":31240,"ancestor_record_offset":30892,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":32348,"parent_record_offset":32312,"ancestor_record_offset":30892,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":32188,"parent_record_offset":32160,"ancestor_record_offset":30892,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":32160,"parent_record_offset":32132,"ancestor_record_offset":30892,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":32104,"parent_record_offset":32076,"ancestor_record_offset":30892,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":32132,"parent_record_offset":32104,"ancestor_record_offset":30892,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":32076,"parent_record_offset":32048,"ancestor_record_offset":30892,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":190,"record_offset":32224,"parent_record_offset":32188,"ancestor_record_offset":30892,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_display
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":190,"record_offset":31868,"parent_record_offset":31824,"ancestor_record_offset":30892,"inlinee":12565,"source_line":143,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":190,"record_offset":32372,"parent_record_offset":32348,"ancestor_record_offset":30892,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":190,"record_offset":32396,"parent_record_offset":32372,"ancestor_record_offset":30892,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":190,"record_offset":33072,"parent_record_offset":33048,"ancestor_record_offset":30892,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":190,"record_offset":35256,"parent_record_offset":35232,"ancestor_record_offset":30892,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":190,"record_offset":31984,"parent_record_offset":31960,"ancestor_record_offset":30892,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":190,"record_offset":31164,"parent_record_offset":31136,"ancestor_record_offset":30892,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":190,"record_offset":32612,"parent_record_offset":32588,"ancestor_record_offset":30892,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":190,"record_offset":32444,"parent_record_offset":32420,"ancestor_record_offset":30892,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":190,"record_offset":32420,"parent_record_offset":32396,"ancestor_record_offset":30892,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":190,"record_offset":32588,"parent_record_offset":32188,"ancestor_record_offset":30892,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":31192,"parent_record_offset":31136,"ancestor_record_offset":30892,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":35128,"parent_record_offset":35100,"ancestor_record_offset":30892,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":190,"record_offset":33096,"parent_record_offset":33072,"ancestor_record_offset":30892,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":190,"record_offset":33120,"parent_record_offset":33096,"ancestor_record_offset":30892,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":190,"record_offset":33048,"parent_record_offset":33004,"ancestor_record_offset":30892,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":190,"record_offset":31732,"parent_record_offset":31708,"ancestor_record_offset":30892,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":190,"record_offset":31780,"parent_record_offset":31756,"ancestor_record_offset":30892,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":190,"record_offset":31756,"parent_record_offset":31732,"ancestor_record_offset":30892,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":190,"record_offset":35100,"parent_record_offset":35072,"ancestor_record_offset":30892,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":190,"record_offset":31708,"parent_record_offset":31684,"ancestor_record_offset":30892,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":190,"record_offset":35304,"parent_record_offset":35280,"ancestor_record_offset":30892,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":190,"record_offset":35280,"parent_record_offset":35256,"ancestor_record_offset":30892,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":190,"record_offset":31136,"parent_record_offset":31096,"ancestor_record_offset":30892,"inlinee":14357,"source_line":137,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":190,"record_offset":31960,"parent_record_offset":31932,"ancestor_record_offset":30892,"inlinee":14358,"source_line":143,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":190,"record_offset":32944,"parent_record_offset":32916,"ancestor_record_offset":30892,"inlinee":14359,"source_line":150,"archive":"_pdb/B/modules/00190.jsonl.gz"}
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
// 首条位置: {"module":190,"record_offset":24576,"rva":4931120,"va":5373640240}
// 编译实例 RVAs: 0x4b3e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a83b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4244347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8853cbfff","size":5,"soft_fail":false,"target_va":5369398192,"va":5373642534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a8950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4240873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8174abfff","size":5,"soft_fail":false,"target_va":5369399632,"va":5373640500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b5990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2092333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8d312e0ff","size":5,"soft_fail":false,"target_va":5371550096,"va":5373642424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bb170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2069818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8c66ae0ff","size":5,"soft_fail":false,"target_va":5371572592,"va":5373642405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t457297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e851fa0600","size":5,"soft_fail":false,"target_va":5374098304,"va":5373641002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dddb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1217276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8fc921200","size":5,"soft_fail":false,"target_va":5374860720,"va":5373643439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e87d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1263247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e88f461300","size":5,"soft_fail":false,"target_va":5374904272,"va":5373641020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ef7d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1289054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e85eab1300","size":5,"soft_fail":false,"target_va":5374932944,"va":5373643885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f4970
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1313213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8bd091400","size":5,"soft_fail":false,"target_va":5374953840,"va":5373640622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060de30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1413498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e87a911500","size":5,"soft_fail":false,"target_va":5375057456,"va":5373643953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1439689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8c9f71500","size":5,"soft_fail":false,"target_va":5375081488,"va":5373641794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1440838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e846fc1500","size":5,"soft_fail":false,"target_va":5375081504,"va":5373640661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9186989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8ad2e8c00","size":5,"soft_fail":false,"target_va":5382827728,"va":5373640734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38206093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e88dfa4602","size":5,"soft_fail":false,"target_va":5411847168,"va":5373641070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9198801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8d15c8c00","size":5,"soft_fail":false,"target_va":5382840208,"va":5373641402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37898215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8e7474202","size":5,"soft_fail":false,"target_va":5411540963,"va":5373642743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-4562909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e82360baff","size":5,"soft_fail":false,"target_va":5369080752,"va":5373643656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4564762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8e658baff","size":5,"soft_fail":false,"target_va":5369078928,"va":5373643685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-13736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e858caffff","size":5,"soft_fail":false,"target_va":5373629632,"va":5373643363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9064690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373640240,"raw_hex":"e8f2508a00","size":5,"soft_fail":false,"target_va":5382705472,"va":5373640777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5373640240,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5373640860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":190,"record_offset":24780,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":190,"record_offset":24836,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":190,"record_offset":30240,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":190,"record_offset":26984,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":190,"record_offset":29700,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":5145,"source_line":1277,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":25056,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":27708,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":29028,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":26100,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::add_breadcrumb
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":190,"record_offset":24972,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":10240,"source_line":109,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::with_scope
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":190,"record_offset":30324,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":10239,"source_line":183,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":190,"record_offset":25876,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":30076,"parent_record_offset":30052,"ancestor_record_offset":24576,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":30100,"parent_record_offset":30076,"ancestor_record_offset":24576,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":25556,"parent_record_offset":25532,"ancestor_record_offset":24576,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":30052,"parent_record_offset":30024,"ancestor_record_offset":24576,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":25532,"parent_record_offset":25444,"ancestor_record_offset":24576,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":190,"record_offset":26488,"parent_record_offset":26460,"ancestor_record_offset":24576,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":190,"record_offset":29096,"parent_record_offset":29068,"ancestor_record_offset":24576,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":190,"record_offset":29144,"parent_record_offset":29120,"ancestor_record_offset":24576,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":29964,"parent_record_offset":29936,"ancestor_record_offset":24576,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":25472,"parent_record_offset":25444,"ancestor_record_offset":24576,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":25444,"parent_record_offset":25420,"ancestor_record_offset":24576,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":30024,"parent_record_offset":29992,"ancestor_record_offset":24576,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":29992,"parent_record_offset":29964,"ancestor_record_offset":24576,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":25420,"parent_record_offset":25396,"ancestor_record_offset":24576,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":29900,"parent_record_offset":29872,"ancestor_record_offset":24576,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":29872,"parent_record_offset":29844,"ancestor_record_offset":24576,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":29816,"parent_record_offset":29788,"ancestor_record_offset":24576,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":29844,"parent_record_offset":29816,"ancestor_record_offset":24576,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":29788,"parent_record_offset":29760,"ancestor_record_offset":24576,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":190,"record_offset":29760,"parent_record_offset":29732,"ancestor_record_offset":24576,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":190,"record_offset":29936,"parent_record_offset":29900,"ancestor_record_offset":24576,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":190,"record_offset":30264,"parent_record_offset":30240,"ancestor_record_offset":24576,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":190,"record_offset":25120,"parent_record_offset":25084,"ancestor_record_offset":24576,"inlinee":4929,"source_line":112,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":190,"record_offset":25084,"parent_record_offset":25056,"ancestor_record_offset":24576,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":190,"record_offset":25372,"parent_record_offset":25348,"ancestor_record_offset":24576,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":190,"record_offset":25192,"parent_record_offset":25168,"ancestor_record_offset":24576,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":190,"record_offset":26520,"parent_record_offset":26488,"ancestor_record_offset":24576,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":190,"record_offset":25900,"parent_record_offset":25876,"ancestor_record_offset":24576,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":190,"record_offset":29180,"parent_record_offset":29068,"ancestor_record_offset":24576,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":190,"record_offset":30176,"parent_record_offset":30152,"ancestor_record_offset":24576,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":190,"record_offset":30288,"parent_record_offset":30264,"ancestor_record_offset":24576,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":190,"record_offset":25168,"parent_record_offset":25144,"ancestor_record_offset":24576,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":190,"record_offset":25144,"parent_record_offset":25120,"ancestor_record_offset":24576,"inlinee":4931,"source_line":155,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":190,"record_offset":26544,"parent_record_offset":26520,"ancestor_record_offset":24576,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":190,"record_offset":25500,"parent_record_offset":25472,"ancestor_record_offset":24576,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":190,"record_offset":29120,"parent_record_offset":29096,"ancestor_record_offset":24576,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":190,"record_offset":27192,"parent_record_offset":27020,"ancestor_record_offset":24576,"inlinee":6199,"source_line":2499,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":190,"record_offset":30152,"parent_record_offset":29900,"ancestor_record_offset":24576,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":190,"record_offset":25264,"parent_record_offset":25240,"ancestor_record_offset":24576,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=60
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":25316,"parent_record_offset":25084,"ancestor_record_offset":24576,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":190,"record_offset":24804,"parent_record_offset":24780,"ancestor_record_offset":24576,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":190,"record_offset":25240,"parent_record_offset":25216,"ancestor_record_offset":24576,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":190,"record_offset":26288,"parent_record_offset":26260,"ancestor_record_offset":24576,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":190,"record_offset":26336,"parent_record_offset":26312,"ancestor_record_offset":24576,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":190,"record_offset":26312,"parent_record_offset":26288,"ancestor_record_offset":24576,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":190,"record_offset":24912,"parent_record_offset":24836,"ancestor_record_offset":24576,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":190,"record_offset":25216,"parent_record_offset":25192,"ancestor_record_offset":24576,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":190,"record_offset":24880,"parent_record_offset":24836,"ancestor_record_offset":24576,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":24936,"parent_record_offset":24912,"ancestor_record_offset":24576,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":27124,"parent_record_offset":27020,"ancestor_record_offset":24576,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":190,"record_offset":27064,"parent_record_offset":27020,"ancestor_record_offset":24576,"inlinee":13006,"source_line":108,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":190,"record_offset":27088,"parent_record_offset":27064,"ancestor_record_offset":24576,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":190,"record_offset":25020,"parent_record_offset":24996,"ancestor_record_offset":24576,"inlinee":10229,"source_line":145,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":190,"record_offset":24996,"parent_record_offset":24972,"ancestor_record_offset":24576,"inlinee":10230,"source_line":51,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":190,"record_offset":25996,"parent_record_offset":25972,"ancestor_record_offset":24576,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":190,"record_offset":26044,"parent_record_offset":26020,"ancestor_record_offset":24576,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":190,"record_offset":26020,"parent_record_offset":25996,"ancestor_record_offset":24576,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":190,"record_offset":26260,"parent_record_offset":26228,"ancestor_record_offset":24576,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":190,"record_offset":26228,"parent_record_offset":26200,"ancestor_record_offset":24576,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":190,"record_offset":25972,"parent_record_offset":25948,"ancestor_record_offset":24576,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":190,"record_offset":25948,"parent_record_offset":25924,"ancestor_record_offset":24576,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":190,"record_offset":25924,"parent_record_offset":25900,"ancestor_record_offset":24576,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":190,"record_offset":27020,"parent_record_offset":26984,"ancestor_record_offset":24576,"inlinee":14353,"source_line":30,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":190,"record_offset":27744,"parent_record_offset":27708,"ancestor_record_offset":24576,"inlinee":14355,"source_line":49,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":190,"record_offset":29068,"parent_record_offset":29028,"ancestor_record_offset":24576,"inlinee":14354,"source_line":50,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":190,"record_offset":29732,"parent_record_offset":29700,"ancestor_record_offset":24576,"inlinee":14356,"source_line":52,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3811
// 首条位置: {"module":190,"inlinee":14353}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$40::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":190,"record_offset":27192,"parent_record_offset":27020,"ancestor_record_offset":27020,"inlinee":6199,"source_line":2499,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":27220,"parent_record_offset":27020,"ancestor_record_offset":27020,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":190,"record_offset":27124,"parent_record_offset":27020,"ancestor_record_offset":27020,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":190,"record_offset":27064,"parent_record_offset":27020,"ancestor_record_offset":27020,"inlinee":13006,"source_line":108,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":27420,"parent_record_offset":27396,"ancestor_record_offset":27020,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":27396,"parent_record_offset":27332,"ancestor_record_offset":27020,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":27364,"parent_record_offset":27332,"ancestor_record_offset":27020,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":27332,"parent_record_offset":27304,"ancestor_record_offset":27020,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":27304,"parent_record_offset":27276,"ancestor_record_offset":27020,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":27160,"parent_record_offset":27124,"ancestor_record_offset":27020,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":190,"record_offset":27088,"parent_record_offset":27064,"ancestor_record_offset":27020,"inlinee":12914,"source_line":51,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3813
// 首条位置: {"module":190,"inlinee":14355}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":27776,"parent_record_offset":27744,"ancestor_record_offset":27744,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":28112,"parent_record_offset":28088,"ancestor_record_offset":27744,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":28088,"parent_record_offset":28064,"ancestor_record_offset":27744,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":28580,"parent_record_offset":28548,"ancestor_record_offset":27744,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":28064,"parent_record_offset":28040,"ancestor_record_offset":27744,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":28040,"parent_record_offset":28016,"ancestor_record_offset":27744,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":27808,"parent_record_offset":27776,"ancestor_record_offset":27744,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3812
// 首条位置: {"module":190,"inlinee":14354}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":190,"record_offset":29096,"parent_record_offset":29068,"ancestor_record_offset":29068,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":190,"record_offset":29180,"parent_record_offset":29068,"ancestor_record_offset":29068,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":29208,"parent_record_offset":29068,"ancestor_record_offset":29068,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":29408,"parent_record_offset":29384,"ancestor_record_offset":29068,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":29384,"parent_record_offset":29328,"ancestor_record_offset":29068,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":190,"record_offset":29144,"parent_record_offset":29120,"ancestor_record_offset":29068,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":29356,"parent_record_offset":29328,"ancestor_record_offset":29068,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":29328,"parent_record_offset":29304,"ancestor_record_offset":29068,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":29304,"parent_record_offset":29280,"ancestor_record_offset":29068,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":190,"record_offset":29120,"parent_record_offset":29096,"ancestor_record_offset":29068,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":29232,"parent_record_offset":29208,"ancestor_record_offset":29068,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3814
// 首条位置: {"module":190,"inlinee":14356}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":190,"record_offset":29760,"parent_record_offset":29732,"ancestor_record_offset":29732,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":30076,"parent_record_offset":30052,"ancestor_record_offset":29732,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":30100,"parent_record_offset":30076,"ancestor_record_offset":29732,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":30052,"parent_record_offset":30024,"ancestor_record_offset":29732,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":29964,"parent_record_offset":29936,"ancestor_record_offset":29732,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":30024,"parent_record_offset":29992,"ancestor_record_offset":29732,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":29992,"parent_record_offset":29964,"ancestor_record_offset":29732,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":29900,"parent_record_offset":29872,"ancestor_record_offset":29732,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":29872,"parent_record_offset":29844,"ancestor_record_offset":29732,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":29816,"parent_record_offset":29788,"ancestor_record_offset":29732,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":29844,"parent_record_offset":29816,"ancestor_record_offset":29732,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":29788,"parent_record_offset":29760,"ancestor_record_offset":29732,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":190,"record_offset":29936,"parent_record_offset":29900,"ancestor_record_offset":29732,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":190,"record_offset":30176,"parent_record_offset":30152,"ancestor_record_offset":29732,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":190,"record_offset":30152,"parent_record_offset":29900,"ancestor_record_offset":29732,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E5F
// 首条位置: {"module":104,"inlinee":11871}
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
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2DB5
// 首条位置: {"module":104,"inlinee":11701}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":104,"record_offset":25172,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":104,"record_offset":24636,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":104,"record_offset":25256,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":104,"record_offset":25380,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::add_breadcrumb
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":104,"record_offset":25284,"parent_record_offset":24576,"ancestor_record_offset":24576,"inlinee":10240,"source_line":109,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":104,"record_offset":25008,"parent_record_offset":24984,"ancestor_record_offset":24576,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":104,"record_offset":25032,"parent_record_offset":25008,"ancestor_record_offset":24576,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":104,"record_offset":25580,"parent_record_offset":25556,"ancestor_record_offset":24576,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":104,"record_offset":24984,"parent_record_offset":24956,"ancestor_record_offset":24576,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":104,"record_offset":25556,"parent_record_offset":25500,"ancestor_record_offset":24576,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":104,"record_offset":25220,"parent_record_offset":25196,"ancestor_record_offset":24576,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":104,"record_offset":24896,"parent_record_offset":24868,"ancestor_record_offset":24576,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":104,"record_offset":25528,"parent_record_offset":25500,"ancestor_record_offset":24576,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":104,"record_offset":25500,"parent_record_offset":25476,"ancestor_record_offset":24576,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":104,"record_offset":24956,"parent_record_offset":24924,"ancestor_record_offset":24576,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":104,"record_offset":24924,"parent_record_offset":24896,"ancestor_record_offset":24576,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":104,"record_offset":25476,"parent_record_offset":25452,"ancestor_record_offset":24576,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":104,"record_offset":24832,"parent_record_offset":24804,"ancestor_record_offset":24576,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":104,"record_offset":24804,"parent_record_offset":24776,"ancestor_record_offset":24576,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":104,"record_offset":24748,"parent_record_offset":24720,"ancestor_record_offset":24576,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":104,"record_offset":24776,"parent_record_offset":24748,"ancestor_record_offset":24576,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":104,"record_offset":24720,"parent_record_offset":24692,"ancestor_record_offset":24576,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":104,"record_offset":24692,"parent_record_offset":24664,"ancestor_record_offset":24576,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":104,"record_offset":24664,"parent_record_offset":24636,"ancestor_record_offset":24576,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":104,"record_offset":24868,"parent_record_offset":24832,"ancestor_record_offset":24576,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":104,"record_offset":25108,"parent_record_offset":25084,"ancestor_record_offset":24576,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":104,"record_offset":25196,"parent_record_offset":25172,"ancestor_record_offset":24576,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":104,"record_offset":25084,"parent_record_offset":24832,"ancestor_record_offset":24576,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":104,"record_offset":25404,"parent_record_offset":25380,"ancestor_record_offset":24576,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub_impl.rs
//     evidence={"module":104,"record_offset":25340,"parent_record_offset":25312,"ancestor_record_offset":24576,"inlinee":10229,"source_line":145,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":104,"record_offset":25312,"parent_record_offset":25284,"ancestor_record_offset":24576,"inlinee":10230,"source_line":51,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B78
// 首条位置: {"module":20,"inlinee":7032}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":20,"record_offset":2952,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":20,"record_offset":1768,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":20,"record_offset":1484,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":20,"record_offset":1540,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":20,"record_offset":1696,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_direct] raw_cpuid::CpuId::get_feature_info
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":20,"record_offset":2692,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":7044,"source_line":391,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_direct] raw_cpuid::CpuId::get_processor_brand_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":20,"record_offset":2316,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":7043,"source_line":820,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_direct] raw_cpuid::CpuId::get_vendor_info
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":20,"record_offset":2176,"parent_record_offset":1364,"ancestor_record_offset":1364,"inlinee":7034,"source_line":374,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":20,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1364,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":20,"record_offset":2092,"parent_record_offset":2036,"ancestor_record_offset":1364,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":20,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":1364,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":20,"record_offset":2064,"parent_record_offset":2036,"ancestor_record_offset":1364,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":20,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":1364,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":20,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":1364,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":20,"record_offset":1848,"parent_record_offset":1800,"ancestor_record_offset":1364,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":20,"record_offset":1800,"parent_record_offset":1768,"ancestor_record_offset":1364,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::cpuid::__cpuid_count
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\cpuid.rs
//     evidence={"module":20,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":1364,"inlinee":7042,"source_line":54,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":20,"record_offset":1876,"parent_record_offset":1800,"ancestor_record_offset":1364,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":20,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":1364,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":20,"record_offset":1736,"parent_record_offset":1696,"ancestor_record_offset":1364,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":20,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":1364,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":20,"record_offset":1608,"parent_record_offset":1540,"ancestor_record_offset":1364,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":20,"record_offset":1636,"parent_record_offset":1540,"ancestor_record_offset":1364,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":20,"record_offset":1576,"parent_record_offset":1540,"ancestor_record_offset":1364,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":20,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":1364,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":20,"record_offset":1904,"parent_record_offset":1800,"ancestor_record_offset":1364,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::CpuId::leaf_is_supported
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":20,"record_offset":2728,"parent_record_offset":2692,"ancestor_record_offset":1364,"inlinee":7045,"source_line":353,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::CpuIdReader::cpuid1
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":20,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":1364,"inlinee":7036,"source_line":175,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::FeatureInfoFlags::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":20,"record_offset":2884,"parent_record_offset":2692,"ancestor_record_offset":1364,"inlinee":7047,"source_line":657,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::_::InternalBitFlags::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":20,"record_offset":2912,"parent_record_offset":2884,"ancestor_record_offset":1364,"inlinee":7049,"source_line":657,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::native_cpuid::cpuid_count
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":20,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":1364,"inlinee":7040,"source_line":85,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::native_cpuid::impl$0::cpuid2
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":20,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":1364,"inlinee":7038,"source_line":103,"archive":"_pdb/B/modules/00020.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B8F
// 首条位置: {"module":20,"inlinee":7055}
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
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 137
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3815
// 首条位置: {"module":190,"inlinee":14357}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":190,"record_offset":31164,"parent_record_offset":31136,"ancestor_record_offset":31136,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":31192,"parent_record_offset":31136,"ancestor_record_offset":31136,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":31368,"parent_record_offset":31344,"ancestor_record_offset":31136,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":31344,"parent_record_offset":31288,"ancestor_record_offset":31136,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":31316,"parent_record_offset":31288,"ancestor_record_offset":31136,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":31288,"parent_record_offset":31264,"ancestor_record_offset":31136,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":31264,"parent_record_offset":31240,"ancestor_record_offset":31136,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":31216,"parent_record_offset":31192,"ancestor_record_offset":31136,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3816
// 首条位置: {"module":190,"inlinee":14358}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":190,"record_offset":31984,"parent_record_offset":31960,"ancestor_record_offset":31960,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3817
// 首条位置: {"module":190,"inlinee":14359}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":190,"record_offset":32968,"parent_record_offset":32944,"ancestor_record_offset":32944,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x309A
// 首条位置: {"module":125,"inlinee":12442}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":125,"record_offset":21828,"parent_record_offset":21808,"ancestor_record_offset":21808,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3099
// 首条位置: {"module":125,"inlinee":12441}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":125,"record_offset":21404,"parent_record_offset":21384,"ancestor_record_offset":21384,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::extension_to_content_type
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3819
// 首条位置: {"module":190,"inlinee":14361}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":190,"record_offset":33836,"parent_record_offset":33004,"ancestor_record_offset":33004,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":190,"record_offset":33048,"parent_record_offset":33004,"ancestor_record_offset":33004,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":190,"record_offset":33072,"parent_record_offset":33048,"ancestor_record_offset":33004,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":190,"record_offset":33096,"parent_record_offset":33072,"ancestor_record_offset":33004,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":190,"record_offset":33120,"parent_record_offset":33096,"ancestor_record_offset":33004,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_message
// 记录形式: compiled_procedure; instances=1
// 源码行: 189
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":190,"record_offset":36680,"rva":4936896,"va":5373646016}
// 编译实例 RVAs: 0x4b54c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5373646016,"raw_hex":"ff5718","size":3,"soft_fail":false,"va":5373646032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::any::impl$5::downcast_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":190,"record_offset":36832,"parent_record_offset":36680,"ancestor_record_offset":36680,"inlinee":14364,"source_line":376,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":190,"record_offset":36852,"parent_record_offset":36832,"ancestor_record_offset":36680,"inlinee":10550,"source_line":225,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":190,"record_offset":36872,"parent_record_offset":36852,"ancestor_record_offset":36680,"inlinee":10551,"source_line":193,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":190,"record_offset":36900,"parent_record_offset":36872,"ancestor_record_offset":36680,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":190,"record_offset":36948,"parent_record_offset":36924,"ancestor_record_offset":36680,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":190,"record_offset":36924,"parent_record_offset":36900,"ancestor_record_offset":36680,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_sentry_event
// 记录形式: compiled_procedure; instances=1
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":190,"record_offset":37164,"rva":4937040,"va":5373646160}
// 编译实例 RVAs: 0x4b5550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$sentry_types..protocol..v7..Event$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1822113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373646160,"raw_hex":"e8a1cd1b00","size":5,"soft_fail":false,"target_va":5375469104,"va":5373646986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cfaf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1153567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373646160,"raw_hex":"e81f9a1100","size":5,"soft_fail":false,"target_va":5374802672,"va":5373649100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f3380
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1300266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373646160,"raw_hex":"e82ad71300","size":5,"soft_fail":false,"target_va":5374948224,"va":5373647953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f3520
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1300531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373646160,"raw_hex":"e833d81300","size":5,"soft_fail":false,"target_va":5374948640,"va":5373648104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f4de0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1306890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373646160,"raw_hex":"e80af11300","size":5,"soft_fail":false,"target_va":5374954976,"va":5373648081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1435256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373646160,"raw_hex":"e878e61500","size":5,"soft_fail":false,"target_va":5375081488,"va":5373646227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1433504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373646160,"raw_hex":"e8a0df1500","size":5,"soft_fail":false,"target_va":5375081504,"va":5373647995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38200661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373646160,"raw_hex":"e855e54602","size":5,"soft_fail":false,"target_va":5411847168,"va":5373646502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37892845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373646160,"raw_hex":"e8ed324202","size":5,"soft_fail":false,"target_va":5411540992,"va":5373648142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37892794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373646160,"raw_hex":"e8ba324202","size":5,"soft_fail":false,"target_va":5411540963,"va":5373648164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] sentry_backtrace::process::backtrace_to_stacktrace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"sentry-backtrace","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1744159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373646160,"raw_hex":"e81f9d1a00","size":5,"soft_fail":false,"target_va":5375390688,"va":5373646524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":37340,"parent_record_offset":37164,"ancestor_record_offset":37164,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":190,"record_offset":40340,"parent_record_offset":37164,"ancestor_record_offset":37164,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":190,"record_offset":38560,"parent_record_offset":37164,"ancestor_record_offset":37164,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":190,"record_offset":37488,"parent_record_offset":37164,"ancestor_record_offset":37164,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":39264,"parent_record_offset":37164,"ancestor_record_offset":37164,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_direct] sentry_types::protocol::v7::impl$170::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":190,"record_offset":39180,"parent_record_offset":37164,"ancestor_record_offset":37164,"inlinee":8120,"source_line":619,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":37396,"parent_record_offset":37372,"ancestor_record_offset":37164,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":37420,"parent_record_offset":37396,"ancestor_record_offset":37164,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":39544,"parent_record_offset":39520,"ancestor_record_offset":37164,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":37372,"parent_record_offset":37340,"ancestor_record_offset":37164,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":190,"record_offset":39520,"parent_record_offset":39432,"ancestor_record_offset":37164,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":37748,"parent_record_offset":37720,"ancestor_record_offset":37164,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":39460,"parent_record_offset":39432,"ancestor_record_offset":37164,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":39432,"parent_record_offset":39408,"ancestor_record_offset":37164,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":37808,"parent_record_offset":37776,"ancestor_record_offset":37164,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":37776,"parent_record_offset":37748,"ancestor_record_offset":37164,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":39408,"parent_record_offset":39384,"ancestor_record_offset":37164,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":190,"record_offset":38860,"parent_record_offset":38824,"ancestor_record_offset":37164,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":37684,"parent_record_offset":37656,"ancestor_record_offset":37164,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":37656,"parent_record_offset":37628,"ancestor_record_offset":37164,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":37600,"parent_record_offset":37572,"ancestor_record_offset":37164,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":37628,"parent_record_offset":37600,"ancestor_record_offset":37164,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":190,"record_offset":37572,"parent_record_offset":37544,"ancestor_record_offset":37164,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":190,"record_offset":37544,"parent_record_offset":37516,"ancestor_record_offset":37164,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":190,"record_offset":37516,"parent_record_offset":37488,"ancestor_record_offset":37164,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":190,"record_offset":37720,"parent_record_offset":37684,"ancestor_record_offset":37164,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":190,"record_offset":39320,"parent_record_offset":39292,"ancestor_record_offset":37164,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":190,"record_offset":38884,"parent_record_offset":38860,"ancestor_record_offset":37164,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":190,"record_offset":38908,"parent_record_offset":38884,"ancestor_record_offset":37164,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":190,"record_offset":37960,"parent_record_offset":37936,"ancestor_record_offset":37164,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":190,"record_offset":38956,"parent_record_offset":38932,"ancestor_record_offset":37164,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":190,"record_offset":38932,"parent_record_offset":38908,"ancestor_record_offset":37164,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":190,"record_offset":39488,"parent_record_offset":39460,"ancestor_record_offset":37164,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":190,"record_offset":37936,"parent_record_offset":37684,"ancestor_record_offset":37164,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":39292,"parent_record_offset":39264,"ancestor_record_offset":37164,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":190,"record_offset":37444,"parent_record_offset":37420,"ancestor_record_offset":37164,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00190.jsonl.gz"}
//   [pdb_inline_transitive] sentry_types::protocol::v7::impl$2::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":190,"record_offset":39232,"parent_record_offset":39208,"ancestor_record_offset":37164,"inlinee":8122,"source_line":84,"archive":"_pdb/B/modules/00190.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment
// 信息 ID: b9b01e82309496f7e5f761e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,9>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4> > @ 0x2bb303 -> 0x4b4d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$0
// 信息 ID: bcc0b30fd52db86af22d1285
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment; module=190, inline_record=31136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$1
// 信息 ID: d40c440eb0a47657dca60fb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment; module=190, inline_record=31960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$2
// 信息 ID: 2aabfc8d0d98051c1ec50e12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment; module=190, inline_record=32944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$3
// 信息 ID: 5a104abb87be6b86c6a3914a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$3 <- anyhow::context::impl$0::with_context<std::fs::Metadata,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::Metadata,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$3>; module=125, inline_record=21808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$4
// 信息 ID: 65366ebed6e1cdbbaaf87051
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$4 <- anyhow::context::impl$0::with_context<alloc::vec::Vec<u8,alloc::alloc::Global>,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::vec::Vec<u8,alloc::alloc::Global>,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$4>; module=125, inline_record=21384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::extension_to_content_type
// 信息 ID: fc4bcd59f4bb6c301991bdbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment; module=190, inline_record=33004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_message
// 信息 ID: 4374490d165703cda103f619
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdb8cd -> 0x4b54c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_sentry_event
// 信息 ID: 9c9232ac5e7b296a8d3b7255
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > @ 0xd8eaa -> 0x4b5550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry
// 信息 ID: eac8edf2c0580ad85f5b8eeb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > @ 0xd90f6 -> 0x4b3e30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$0
// 信息 ID: c37be01244bccadadb7037be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry; module=190, inline_record=27020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$1
// 信息 ID: 425ed38b540d936d0a4f7cfb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry; module=190, inline_record=27744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$2
// 信息 ID: 9051f9eecbc667c80ed56882
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry; module=190, inline_record=29068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$3
// 信息 ID: 2753589db0d79258ecbece11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry; module=190, inline_record=29732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$4
// 信息 ID: f6a8ee44ee03bf82c63f8bb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,9>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,9>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,9>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4> >; module=104, inline_record=96600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$5
// 信息 ID: 783153b5912cd111390f9d89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5 <- alloc::vec::Vec::retain<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,alloc::alloc::Global,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5 <- alloc::vec::impl$1::retain::closure$0<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,alloc::alloc::Global,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,alloc::alloc::Global,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5>; module=104, inline_record=24576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$6
// 信息 ID: 4d442773ea8f10fa5dba177e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6 <- sentry_core::hub::Hub::with_scope<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6 <- std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>,uuid::Uuid>,uuid::Uuid>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6 <- core::ptr::drop_in_place<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] sentry_core::hub::Hub::with_scope<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>; module=20, inline_record=1364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$7
// 信息 ID: 095438733a2ea0f2b183f5c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7 <- sentry_core::hub::Hub::with_scope<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7 <- std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>,uuid::Uuid>,uuid::Uuid>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7 <- core::ptr::drop_in_place<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] sentry_core::hub::Hub::with_scope<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>; module=20, inline_record=3352; 不是独立调用边。
// [PDB 内联宿主] std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>,uuid::Uuid>,uuid::Uuid>; module=21, inline_record=142548; 不是独立调用边。
// [PDB 内联宿主] std::thread::local::LocalKey::with<tuple$<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global> >,core::cell::Cell<bool> >,sentry_core::hub_impl::impl$3::with::closure_env$0<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$7,uuid::Uuid>,uuid::Uuid>,uuid::Uuid>; module=21, inline_record=142732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

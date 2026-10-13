// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 8e48442bb3928e3e31a8e251bd7186bef794039719fe7c2341651eb9417cbf5e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sentry_reporter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 21, 49, 73, 134, 201
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use raw_cpuid; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.4.2"},{"name":"raw-cpuid","source":"PDB_registry_path_only","version":"11.1.0"}]
// use sentry_core; // namespace candidate
//   package identities: [{"name":"sentry-core","source":"PDB_registry_path_only","version":"0.31.7"}]
// use sentry_types; // namespace candidate
//   package identities: [{"name":"sentry-types","source":"PDB_registry_path_only","version":"0.31.7"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":49,"record_offset":9836,"rva":450656,"va":5369159776}
// 编译实例 RVAs: 0x6e060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140263ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2054714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e83a5a1f00","size":5,"soft_fail":false,"target_va":5371215584,"va":5369160865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402643b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2055826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e8925e1f00","size":5,"soft_fail":false,"target_va":5371216816,"va":5369160985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3589485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e86dc53600","size":5,"soft_fail":false,"target_va":5372749696,"va":5369160206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3589754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e87ac63600","size":5,"soft_fail":false,"target_va":5372749712,"va":5369159953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=4
//     target_aliases=28 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6011054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e8aeb85b00","size":5,"soft_fail":false,"target_va":5375171056,"va":5369159997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8244978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e8f2ce7d00","size":5,"soft_fail":false,"target_va":5377405008,"va":5369160025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34433108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e854680d02","size":5,"soft_fail":false,"target_va":5403593360,"va":5369160247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33816995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e8a3010402","size":5,"soft_fail":false,"target_va":5402978144,"va":5369161144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8365986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e8a2a77f00","size":5,"soft_fail":false,"target_va":5377527120,"va":5369161129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t33745717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e835eb0202","size":5,"soft_fail":false,"target_va":5402905648,"va":5369159926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h17e9edfa19878bd5 (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t33786038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e8b6880302","size":5,"soft_fail":false,"target_va":5402946160,"va":5369160117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8235451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e8bba97d00","size":5,"soft_fail":false,"target_va":5377395552,"va":5369160096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8244099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e883cb7d00","size":5,"soft_fail":false,"target_va":5377405072,"va":5369160968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8279395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e863557e00","size":5,"soft_fail":false,"target_va":5377439664,"va":5369160264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8277270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e8164d7e00","size":5,"soft_fail":false,"target_va":5377437280,"va":5369160005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8312484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369159776,"raw_hex":"e8a4d67e00","size":5,"soft_fail":false,"target_va":5377473264,"va":5369160775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":49,"record_offset":11260,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":49,"record_offset":10720,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":7558,"source_line":91,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":49,"record_offset":13892,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":49,"record_offset":10844,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":7550,"source_line":1231,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":10960,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":49,"record_offset":10040,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":49,"record_offset":13860,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":49,"record_offset":10552,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":49,"record_offset":10784,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":49,"record_offset":13920,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":7565,"source_line":259,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":49,"record_offset":13760,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":7564,"source_line":2750,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":49,"record_offset":10580,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::sentry_reporter::extension_to_content_type
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":49,"record_offset":12260,"parent_record_offset":9836,"ancestor_record_offset":9836,"inlinee":7563,"source_line":157,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":11692,"parent_record_offset":11668,"ancestor_record_offset":9836,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":9836,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":10292,"parent_record_offset":10268,"ancestor_record_offset":9836,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":11668,"parent_record_offset":11616,"ancestor_record_offset":9836,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":10268,"parent_record_offset":10212,"ancestor_record_offset":9836,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":10240,"parent_record_offset":10212,"ancestor_record_offset":9836,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":11616,"parent_record_offset":11572,"ancestor_record_offset":9836,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":11572,"parent_record_offset":11528,"ancestor_record_offset":9836,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":11788,"parent_record_offset":11572,"ancestor_record_offset":9836,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":10212,"parent_record_offset":10184,"ancestor_record_offset":9836,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":11480,"parent_record_offset":11436,"ancestor_record_offset":9836,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":11436,"parent_record_offset":11392,"ancestor_record_offset":9836,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":11348,"parent_record_offset":11304,"ancestor_record_offset":9836,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":11392,"parent_record_offset":11348,"ancestor_record_offset":9836,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":11304,"parent_record_offset":11260,"ancestor_record_offset":9836,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":49,"record_offset":11528,"parent_record_offset":11480,"ancestor_record_offset":9836,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_display
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":49,"record_offset":10752,"parent_record_offset":10720,"ancestor_record_offset":9836,"inlinee":7560,"source_line":143,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":49,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":9836,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":49,"record_offset":13944,"parent_record_offset":13920,"ancestor_record_offset":9836,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":49,"record_offset":10896,"parent_record_offset":10872,"ancestor_record_offset":9836,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":49,"record_offset":10108,"parent_record_offset":10080,"ancestor_record_offset":9836,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":49,"record_offset":11852,"parent_record_offset":11828,"ancestor_record_offset":9836,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":49,"record_offset":11828,"parent_record_offset":11480,"ancestor_record_offset":9836,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":10136,"parent_record_offset":10080,"ancestor_record_offset":9836,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":11744,"parent_record_offset":11716,"ancestor_record_offset":9836,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":49,"record_offset":13816,"parent_record_offset":13788,"ancestor_record_offset":9836,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":49,"record_offset":10812,"parent_record_offset":10784,"ancestor_record_offset":9836,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":49,"record_offset":12348,"parent_record_offset":12324,"ancestor_record_offset":9836,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":49,"record_offset":12372,"parent_record_offset":12348,"ancestor_record_offset":9836,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":49,"record_offset":12300,"parent_record_offset":12260,"ancestor_record_offset":9836,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":49,"record_offset":10628,"parent_record_offset":10604,"ancestor_record_offset":9836,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":49,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":9836,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":49,"record_offset":10652,"parent_record_offset":10628,"ancestor_record_offset":9836,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":49,"record_offset":13788,"parent_record_offset":13760,"ancestor_record_offset":9836,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":49,"record_offset":10604,"parent_record_offset":10580,"ancestor_record_offset":9836,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":49,"record_offset":13992,"parent_record_offset":13968,"ancestor_record_offset":9836,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":49,"record_offset":13968,"parent_record_offset":13944,"ancestor_record_offset":9836,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":49,"record_offset":10080,"parent_record_offset":10040,"ancestor_record_offset":9836,"inlinee":7556,"source_line":118,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":49,"record_offset":10872,"parent_record_offset":10844,"ancestor_record_offset":9836,"inlinee":7559,"source_line":124,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":49,"record_offset":12200,"parent_record_offset":12172,"ancestor_record_offset":9836,"inlinee":7561,"source_line":131,"archive":"_pdb/A/modules/00049.jsonl.gz"}
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
// 首条位置: {"module":49,"record_offset":6616,"rva":447872,"va":5369156992}
// 编译实例 RVAs: 0x6d580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e00e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t468417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e8c1250700","size":5,"soft_fail":false,"target_va":5369626848,"va":5369158426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e44f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t485867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e8eb690700","size":5,"soft_fail":false,"target_va":5369644272,"va":5369158400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14030a1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2739005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e83dcb2900","size":5,"soft_fail":false,"target_va":5371896272,"va":5369157262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14030aba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2740257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e821d02900","size":5,"soft_fail":false,"target_va":5371898784,"va":5369158522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140367690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3118904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e838972f00","size":5,"soft_fail":false,"target_va":5372278416,"va":5369159507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140379490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3191966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e89eb43000","size":5,"soft_fail":false,"target_va":5372351632,"va":5369159661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037e9d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3216077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e8cd123100","size":5,"soft_fail":false,"target_va":5372373456,"va":5369157374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140396ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3313208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e8388e3200","size":5,"soft_fail":false,"target_va":5372472224,"va":5369159011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3591797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e875ce3600","size":5,"soft_fail":false,"target_va":5372749696,"va":5369157894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3592294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e866d03600","size":5,"soft_fail":false,"target_va":5372749712,"va":5369157413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34434876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e83c6f0d02","size":5,"soft_fail":false,"target_va":5403593360,"va":5369158479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33819503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e86f0b0402","size":5,"soft_fail":false,"target_va":5402978144,"va":5369158636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8372209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e8f1bf7f00","size":5,"soft_fail":false,"target_va":5377529888,"va":5369157674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-426381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e8737ef9ff","size":5,"soft_fail":false,"target_va":5368732400,"va":5369158776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tiny_glade::crash_pickup::crash_payload::RecoveredSave,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-427007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e8017cf9ff","size":5,"soft_fail":false,"target_va":5368731856,"va":5369158858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369156992,"raw_hex":"e838f5ffff","size":5,"soft_fail":false,"target_va":5369156672,"va":5369159427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":49,"record_offset":6808,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":49,"record_offset":6864,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":49,"record_offset":9272,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":49,"record_offset":8708,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":7550,"source_line":1231,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":6996,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":49,"record_offset":7740,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":49,"record_offset":8120,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::add_breadcrumb
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":49,"record_offset":6940,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":7547,"source_line":109,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::with_scope
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":49,"record_offset":9356,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":7554,"source_line":183,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":6616,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":6616,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":6616,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":9028,"parent_record_offset":9000,"ancestor_record_offset":6616,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":7424,"parent_record_offset":7332,"ancestor_record_offset":6616,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":49,"record_offset":8188,"parent_record_offset":8160,"ancestor_record_offset":6616,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":49,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":6616,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":7360,"parent_record_offset":7332,"ancestor_record_offset":6616,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":9000,"parent_record_offset":8972,"ancestor_record_offset":6616,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":8972,"parent_record_offset":8944,"ancestor_record_offset":6616,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":9148,"parent_record_offset":8972,"ancestor_record_offset":6616,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":7332,"parent_record_offset":7308,"ancestor_record_offset":6616,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":8908,"parent_record_offset":8880,"ancestor_record_offset":6616,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":8880,"parent_record_offset":8852,"ancestor_record_offset":6616,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":6616,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":8852,"parent_record_offset":8824,"ancestor_record_offset":6616,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":8796,"parent_record_offset":8768,"ancestor_record_offset":6616,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":49,"record_offset":8768,"parent_record_offset":8740,"ancestor_record_offset":6616,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":49,"record_offset":8944,"parent_record_offset":8908,"ancestor_record_offset":6616,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":49,"record_offset":9296,"parent_record_offset":9272,"ancestor_record_offset":6616,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":49,"record_offset":7060,"parent_record_offset":7024,"ancestor_record_offset":6616,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":49,"record_offset":7024,"parent_record_offset":6996,"ancestor_record_offset":6616,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":49,"record_offset":7284,"parent_record_offset":7260,"ancestor_record_offset":6616,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":49,"record_offset":7132,"parent_record_offset":7108,"ancestor_record_offset":6616,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":49,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":6616,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":49,"record_offset":9320,"parent_record_offset":9296,"ancestor_record_offset":6616,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":49,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":6616,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":49,"record_offset":7084,"parent_record_offset":7060,"ancestor_record_offset":6616,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":49,"record_offset":7388,"parent_record_offset":7360,"ancestor_record_offset":6616,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":49,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":6616,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":49,"record_offset":9184,"parent_record_offset":8908,"ancestor_record_offset":6616,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":49,"record_offset":7180,"parent_record_offset":7156,"ancestor_record_offset":6616,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":7228,"parent_record_offset":7024,"ancestor_record_offset":6616,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":49,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":6616,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":49,"record_offset":7156,"parent_record_offset":7132,"ancestor_record_offset":6616,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":49,"record_offset":6904,"parent_record_offset":6864,"ancestor_record_offset":6616,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":9104,"parent_record_offset":9076,"ancestor_record_offset":6616,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":49,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":6616,"inlinee":5309,"source_line":51,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":49,"record_offset":7772,"parent_record_offset":7740,"ancestor_record_offset":6616,"inlinee":7549,"source_line":36,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":49,"record_offset":8160,"parent_record_offset":8120,"ancestor_record_offset":6616,"inlinee":7548,"source_line":37,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/sentry_reporter.rs
//     evidence={"module":49,"record_offset":8740,"parent_record_offset":8708,"ancestor_record_offset":6616,"inlinee":7551,"source_line":39,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D7D
// 首条位置: {"module":49,"inlinee":7549}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":7800,"parent_record_offset":7772,"ancestor_record_offset":7772,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":8044,"parent_record_offset":8020,"ancestor_record_offset":7772,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":7772,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":7996,"parent_record_offset":7972,"ancestor_record_offset":7772,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":7828,"parent_record_offset":7800,"ancestor_record_offset":7772,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D7C
// 首条位置: {"module":49,"inlinee":7548}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":49,"record_offset":8188,"parent_record_offset":8160,"ancestor_record_offset":8160,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":8272,"parent_record_offset":8160,"ancestor_record_offset":8160,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":8452,"parent_record_offset":8428,"ancestor_record_offset":8160,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":8428,"parent_record_offset":8372,"ancestor_record_offset":8160,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":49,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":8160,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":8400,"parent_record_offset":8372,"ancestor_record_offset":8160,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":8372,"parent_record_offset":8344,"ancestor_record_offset":8160,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":49,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":8160,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":8296,"parent_record_offset":8272,"ancestor_record_offset":8160,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D7F
// 首条位置: {"module":49,"inlinee":7551}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":49,"record_offset":8768,"parent_record_offset":8740,"ancestor_record_offset":8740,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":8740,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":8740,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":9028,"parent_record_offset":9000,"ancestor_record_offset":8740,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":9000,"parent_record_offset":8972,"ancestor_record_offset":8740,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":8972,"parent_record_offset":8944,"ancestor_record_offset":8740,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":9148,"parent_record_offset":8972,"ancestor_record_offset":8740,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":8908,"parent_record_offset":8880,"ancestor_record_offset":8740,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":8880,"parent_record_offset":8852,"ancestor_record_offset":8740,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":8740,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":8852,"parent_record_offset":8824,"ancestor_record_offset":8740,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":8796,"parent_record_offset":8768,"ancestor_record_offset":8740,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":49,"record_offset":8944,"parent_record_offset":8908,"ancestor_record_offset":8740,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":49,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":8740,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":49,"record_offset":9184,"parent_record_offset":8908,"ancestor_record_offset":8740,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":9104,"parent_record_offset":9076,"ancestor_record_offset":8740,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x17CF
// 首条位置: {"module":21,"inlinee":6095}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x214E
// 首条位置: {"module":73,"inlinee":8526}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":73,"record_offset":14560,"parent_record_offset":13944,"ancestor_record_offset":13944,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":73,"record_offset":14004,"parent_record_offset":13944,"ancestor_record_offset":13944,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":73,"record_offset":14708,"parent_record_offset":13944,"ancestor_record_offset":13944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::add_breadcrumb
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":73,"record_offset":14644,"parent_record_offset":13944,"ancestor_record_offset":13944,"inlinee":7547,"source_line":109,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":73,"record_offset":14344,"parent_record_offset":14320,"ancestor_record_offset":13944,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":73,"record_offset":14368,"parent_record_offset":14344,"ancestor_record_offset":13944,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":73,"record_offset":14888,"parent_record_offset":14864,"ancestor_record_offset":13944,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":73,"record_offset":14320,"parent_record_offset":14292,"ancestor_record_offset":13944,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":73,"record_offset":14864,"parent_record_offset":14808,"ancestor_record_offset":13944,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":73,"record_offset":14608,"parent_record_offset":14584,"ancestor_record_offset":13944,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":73,"record_offset":14836,"parent_record_offset":14808,"ancestor_record_offset":13944,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":73,"record_offset":14292,"parent_record_offset":14264,"ancestor_record_offset":13944,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":73,"record_offset":14264,"parent_record_offset":14236,"ancestor_record_offset":13944,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":73,"record_offset":14436,"parent_record_offset":14264,"ancestor_record_offset":13944,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":73,"record_offset":14808,"parent_record_offset":14780,"ancestor_record_offset":13944,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":73,"record_offset":14200,"parent_record_offset":14172,"ancestor_record_offset":13944,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":73,"record_offset":14172,"parent_record_offset":14144,"ancestor_record_offset":13944,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":73,"record_offset":14116,"parent_record_offset":14088,"ancestor_record_offset":13944,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":73,"record_offset":14144,"parent_record_offset":14116,"ancestor_record_offset":13944,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":73,"record_offset":14088,"parent_record_offset":14060,"ancestor_record_offset":13944,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":73,"record_offset":14060,"parent_record_offset":14032,"ancestor_record_offset":13944,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":73,"record_offset":14032,"parent_record_offset":14004,"ancestor_record_offset":13944,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":73,"record_offset":14236,"parent_record_offset":14200,"ancestor_record_offset":13944,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":73,"record_offset":14496,"parent_record_offset":14472,"ancestor_record_offset":13944,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":73,"record_offset":14584,"parent_record_offset":14560,"ancestor_record_offset":13944,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":73,"record_offset":14472,"parent_record_offset":14200,"ancestor_record_offset":13944,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":73,"record_offset":14732,"parent_record_offset":14708,"ancestor_record_offset":13944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":73,"record_offset":14392,"parent_record_offset":14368,"ancestor_record_offset":13944,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":73,"record_offset":14672,"parent_record_offset":14644,"ancestor_record_offset":13944,"inlinee":5309,"source_line":51,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31DA
// 首条位置: {"module":201,"inlinee":12762}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":201,"record_offset":3236,"parent_record_offset":1416,"ancestor_record_offset":1416,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":201,"record_offset":1524,"parent_record_offset":1416,"ancestor_record_offset":1416,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":201,"record_offset":1580,"parent_record_offset":1416,"ancestor_record_offset":1416,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":201,"record_offset":1752,"parent_record_offset":1416,"ancestor_record_offset":1416,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":201,"record_offset":1680,"parent_record_offset":1416,"ancestor_record_offset":1416,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_direct] raw_cpuid::CpuId::get_feature_info
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":201,"record_offset":2976,"parent_record_offset":1416,"ancestor_record_offset":1416,"inlinee":11878,"source_line":391,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_direct] raw_cpuid::CpuId::get_processor_brand_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":201,"record_offset":2600,"parent_record_offset":1416,"ancestor_record_offset":1416,"inlinee":11879,"source_line":820,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_direct] raw_cpuid::CpuId::get_vendor_info
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":201,"record_offset":2460,"parent_record_offset":1416,"ancestor_record_offset":1416,"inlinee":11877,"source_line":374,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":201,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1416,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":201,"record_offset":2144,"parent_record_offset":2052,"ancestor_record_offset":1416,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":201,"record_offset":3284,"parent_record_offset":3260,"ancestor_record_offset":1416,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":201,"record_offset":2080,"parent_record_offset":2052,"ancestor_record_offset":1416,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":201,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":1416,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":201,"record_offset":1812,"parent_record_offset":1780,"ancestor_record_offset":1416,"inlinee":6040,"source_line":112,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":201,"record_offset":1780,"parent_record_offset":1752,"ancestor_record_offset":1416,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":201,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":1416,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":201,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":1416,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::cpuid::__cpuid_count
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\cpuid.rs
//     evidence={"module":201,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":1416,"inlinee":5914,"source_line":54,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":201,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":1416,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":201,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1416,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":201,"record_offset":2108,"parent_record_offset":2080,"ancestor_record_offset":1416,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":201,"record_offset":3260,"parent_record_offset":3236,"ancestor_record_offset":1416,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":201,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":1416,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":201,"record_offset":1720,"parent_record_offset":1680,"ancestor_record_offset":1416,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":201,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":1416,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":201,"record_offset":1648,"parent_record_offset":1580,"ancestor_record_offset":1416,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":201,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":1416,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":201,"record_offset":1616,"parent_record_offset":1580,"ancestor_record_offset":1416,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::CpuId::leaf_is_supported
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":201,"record_offset":3012,"parent_record_offset":2976,"ancestor_record_offset":1416,"inlinee":11872,"source_line":353,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::CpuIdReader::cpuid1
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":201,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":1416,"inlinee":5938,"source_line":175,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::FeatureInfoFlags::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.4.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-2.4.2/src/lib.rs
//     evidence={"module":201,"record_offset":3168,"parent_record_offset":2976,"ancestor_record_offset":1416,"inlinee":11874,"source_line":652,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::_::InternalBitFlags::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.4.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bitflags-2.4.2/src/lib.rs
//     evidence={"module":201,"record_offset":3196,"parent_record_offset":3168,"ancestor_record_offset":1416,"inlinee":11876,"source_line":652,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::native_cpuid::cpuid_count
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":201,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":1416,"inlinee":5912,"source_line":85,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   [pdb_inline_transitive] raw_cpuid::native_cpuid::impl$0::cpuid2
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"raw-cpuid","version":"11.1.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/raw-cpuid-11.1.0/src/lib.rs
//     evidence={"module":201,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":1416,"inlinee":5910,"source_line":103,"archive":"_pdb/A/modules/00201.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31DC
// 首条位置: {"module":201,"inlinee":12764}
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
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D84
// 首条位置: {"module":49,"inlinee":7556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":49,"record_offset":10108,"parent_record_offset":10080,"ancestor_record_offset":10080,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":10136,"parent_record_offset":10080,"ancestor_record_offset":10080,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":10292,"parent_record_offset":10268,"ancestor_record_offset":10080,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":10268,"parent_record_offset":10212,"ancestor_record_offset":10080,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":10240,"parent_record_offset":10212,"ancestor_record_offset":10080,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":10212,"parent_record_offset":10184,"ancestor_record_offset":10080,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":10160,"parent_record_offset":10136,"ancestor_record_offset":10080,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D87
// 首条位置: {"module":49,"inlinee":7559}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":49,"record_offset":10896,"parent_record_offset":10872,"ancestor_record_offset":10872,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D89
// 首条位置: {"module":49,"inlinee":7561}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":49,"record_offset":12224,"parent_record_offset":12200,"ancestor_record_offset":12200,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 137
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2ADC
// 首条位置: {"module":134,"inlinee":10972}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":134,"record_offset":12996,"parent_record_offset":12976,"ancestor_record_offset":12976,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2AE0
// 首条位置: {"module":134,"inlinee":10976}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":134,"record_offset":14828,"parent_record_offset":14808,"ancestor_record_offset":14808,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00134.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::extension_to_content_type
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D8B
// 首条位置: {"module":49,"inlinee":7563}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":49,"record_offset":12864,"parent_record_offset":12260,"ancestor_record_offset":12260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":49,"record_offset":12300,"parent_record_offset":12260,"ancestor_record_offset":12260,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":49,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":12260,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":49,"record_offset":12348,"parent_record_offset":12324,"ancestor_record_offset":12260,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":49,"record_offset":12372,"parent_record_offset":12348,"ancestor_record_offset":12260,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_message
// 记录形式: compiled_procedure; instances=1
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":49,"record_offset":15020,"rva":452320,"va":5369161440}
// 编译实例 RVAs: 0x6e6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5369161440,"raw_hex":"ff5718","size":3,"soft_fail":false,"va":5369161456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::any::impl$5::downcast_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":49,"record_offset":15172,"parent_record_offset":15020,"ancestor_record_offset":15020,"inlinee":7567,"source_line":378,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::downcast_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":49,"record_offset":15192,"parent_record_offset":15172,"ancestor_record_offset":15020,"inlinee":5890,"source_line":227,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$4::is
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":49,"record_offset":15212,"parent_record_offset":15192,"ancestor_record_offset":15020,"inlinee":5891,"source_line":195,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":49,"record_offset":15240,"parent_record_offset":15212,"ancestor_record_offset":15020,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":49,"record_offset":15288,"parent_record_offset":15264,"ancestor_record_offset":15020,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":49,"record_offset":15264,"parent_record_offset":15240,"ancestor_record_offset":15020,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_sentry_event
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":49,"record_offset":15512,"rva":452464,"va":5369161584}
// 编译实例 RVAs: 0x6e770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$sentry_types..protocol..v7..Event$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3935353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369161584,"raw_hex":"e8790c3c00","size":5,"soft_fail":false,"target_va":5373097520,"va":5369162162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3071176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369161584,"raw_hex":"e8c8dc2e00","size":5,"soft_fail":false,"target_va":5372234848,"va":5369163667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3203130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369161584,"raw_hex":"e83ae03000","size":5,"soft_fail":false,"target_va":5372366272,"va":5369163137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3203259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369161584,"raw_hex":"e8bbe03000","size":5,"soft_fail":false,"target_va":5372366736,"va":5369163472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3588040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369161584,"raw_hex":"e8c8bf3600","size":5,"soft_fail":false,"target_va":5372749696,"va":5369161651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3586528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369161584,"raw_hex":"e8e0b93600","size":5,"soft_fail":false,"target_va":5372749712,"va":5369163179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sentry_reporter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34431432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369161584,"raw_hex":"e8c8610d02","size":5,"soft_fail":false,"target_va":5403593360,"va":5369161923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33814922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369161584,"raw_hex":"e88af90302","size":5,"soft_fail":false,"target_va":5402978144,"va":5369163217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8363859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369161584,"raw_hex":"e8539f7f00","size":5,"soft_fail":false,"target_va":5377527120,"va":5369163256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] sentry_backtrace::process::backtrace_to_stacktrace
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3861506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369161584,"raw_hex":"e802ec3a00","size":5,"soft_fail":false,"target_va":5373023456,"va":5369161945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":15684,"parent_record_offset":15512,"ancestor_record_offset":15512,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":49,"record_offset":18068,"parent_record_offset":15512,"ancestor_record_offset":15512,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":49,"record_offset":16948,"parent_record_offset":15512,"ancestor_record_offset":15512,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":49,"record_offset":15832,"parent_record_offset":15512,"ancestor_record_offset":15512,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":17580,"parent_record_offset":15512,"ancestor_record_offset":15512,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":15740,"parent_record_offset":15716,"ancestor_record_offset":15512,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":15764,"parent_record_offset":15740,"ancestor_record_offset":15512,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":17840,"parent_record_offset":17816,"ancestor_record_offset":15512,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":15716,"parent_record_offset":15684,"ancestor_record_offset":15512,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":49,"record_offset":17816,"parent_record_offset":17724,"ancestor_record_offset":15512,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":17752,"parent_record_offset":17724,"ancestor_record_offset":15512,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":16120,"parent_record_offset":16092,"ancestor_record_offset":15512,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":16092,"parent_record_offset":16064,"ancestor_record_offset":15512,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":16264,"parent_record_offset":16092,"ancestor_record_offset":15512,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":49,"record_offset":17724,"parent_record_offset":17700,"ancestor_record_offset":15512,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":16028,"parent_record_offset":16000,"ancestor_record_offset":15512,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":16000,"parent_record_offset":15972,"ancestor_record_offset":15512,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":15944,"parent_record_offset":15916,"ancestor_record_offset":15512,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":15972,"parent_record_offset":15944,"ancestor_record_offset":15512,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":49,"record_offset":15916,"parent_record_offset":15888,"ancestor_record_offset":15512,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":49,"record_offset":15888,"parent_record_offset":15860,"ancestor_record_offset":15512,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":49,"record_offset":15860,"parent_record_offset":15832,"ancestor_record_offset":15512,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":49,"record_offset":16064,"parent_record_offset":16028,"ancestor_record_offset":15512,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":49,"record_offset":17636,"parent_record_offset":17608,"ancestor_record_offset":15512,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":49,"record_offset":16324,"parent_record_offset":16300,"ancestor_record_offset":15512,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":49,"record_offset":17780,"parent_record_offset":17752,"ancestor_record_offset":15512,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":49,"record_offset":16300,"parent_record_offset":16028,"ancestor_record_offset":15512,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":17608,"parent_record_offset":17580,"ancestor_record_offset":15512,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":49,"record_offset":15788,"parent_record_offset":15764,"ancestor_record_offset":15512,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00049.jsonl.gz"}
//   [pdb_inline_transitive] sentry_types::protocol::v7::impl$2::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":49,"record_offset":17548,"parent_record_offset":17524,"ancestor_record_offset":15512,"inlinee":5581,"source_line":84,"archive":"_pdb/A/modules/00049.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

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
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment
// 信息 ID: 83cf4a738866c943093558f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,7>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$3> > @ 0xe4683 -> 0x6e060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$0
// 信息 ID: 5be9a3b8a51480689769c195
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment; module=49, inline_record=10080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$1
// 信息 ID: fdbbf8f02ff4fbc59626069b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment; module=49, inline_record=10872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$2
// 信息 ID: 8fa5ab30d2afa02287636f90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment; module=49, inline_record=12200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$3
// 信息 ID: 93948a028015a5d56c48dea3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$3 <- anyhow::context::impl$0::with_context<std::fs::Metadata,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::Metadata,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$3>; module=134, inline_record=12976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure$4
// 信息 ID: 9870e9674b36de11dc4082f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$4 <- anyhow::context::impl$0::with_context<alloc::vec::Vec<u8,alloc::alloc::Global>,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::vec::Vec<u8,alloc::alloc::Global>,std::io::error::Error,anyhow::Error,tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment::closure_env$4>; module=134, inline_record=14808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::extension_to_content_type
// 信息 ID: 509be4f87f9fcad8d49a36e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::create_sentry_attachment; module=49, inline_record=12260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_message
// 信息 ID: b014ef439ebc356a7adc19aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0x2788df -> 0x6e6e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::panic_info_to_sentry_event
// 信息 ID: 873ace6e31b2f94f5ca0fc53
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > @ 0x1cc81 -> 0x6e770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry
// 信息 ID: b4e518fbb1048a6c786575ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > @ 0x1ce65 -> 0x6d580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$0
// 信息 ID: 85a9ea6572c22db432bb191c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry; module=49, inline_record=7772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$1
// 信息 ID: f2223932db4551aa118252ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry; module=49, inline_record=8160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$2
// 信息 ID: b5c3fdbdfe332d1153d2c31d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry; module=49, inline_record=8740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$3
// 信息 ID: 83d271a6822ba7b974f6cd03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,7>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$3 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,7>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >,7>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$3> >; module=73, inline_record=62716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$4
// 信息 ID: 2bf3cbf0539e8659cf4e60ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4 <- alloc::vec::Vec::retain<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,alloc::alloc::Global,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4 <- alloc::vec::impl$1::retain::closure$0<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,alloc::alloc::Global,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<tuple$<ref$<str$>,enum2$<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error> > >,alloc::alloc::Global,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$4>; module=73, inline_record=13944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$5
// 信息 ID: 2fe06a68d5a23d91202bd6f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5 <- sentry_core::api::with_scope::closure$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,uuid::Uuid>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5 <- sentry_core::hub::Hub::with<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,uuid::Uuid>,uuid::Uuid>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5 <- core::ptr::drop_in_place<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,uuid::Uuid> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] sentry_core::api::with_scope::closure$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,uuid::Uuid>; module=201, inline_record=1416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure$6
// 信息 ID: 4a262774a761865a9adcbfe6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6 <- sentry_core::api::with_scope::closure$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,uuid::Uuid>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6 <- sentry_core::hub::Hub::with<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,uuid::Uuid>,uuid::Uuid>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6 <- core::ptr::drop_in_place<sentry_core::api::with_scope::closure_env$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,uuid::Uuid> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] sentry_core::api::with_scope::closure$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,uuid::Uuid>; module=201, inline_record=3636; 不是独立调用边。
// [PDB 内联宿主] sentry_core::api::with_scope::closure$0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$5,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env$6,uuid::Uuid>; module=201, inline_record=4172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

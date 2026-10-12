// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/save/save_file_identity.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: cda56610d24164fac39dd2c67bae6f5b10b37dd3fa8b25a7a25a5f7b065dfffa
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: save_file_identity.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 521, 1253, 1307, 1666, 2894
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>
// 记录形式: compiled_procedure; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":521,"record_offset":41636,"rva":10334704,"va":5379043824}
// 编译实例 RVAs: 0x9db1f0, 0x2349eb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::str::pattern::StrSearcher>::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5780476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e8fc335800","size":5,"soft_fail":false,"target_va":5384824608,"va":5379044127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3365197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e8b3a6ccff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379043880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-3365304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e848a6ccff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379044003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10321678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e8f28062ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379043865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c75340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2728141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e8cda02900","size":5,"soft_fail":false,"target_va":5381772096,"va":5379043950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b05b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8211124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e8b44a7d00","size":5,"soft_fail":false,"target_va":5387257264,"va":5379046135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411e7ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18229293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405712048,"raw_hex":"e8d3d7e9fe","size":5,"soft_fail":false,"target_va":5387484384,"va":5405713672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416513f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13064449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e80159c700","size":5,"soft_fail":false,"target_va":5392110576,"va":5379046122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142999ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6617584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405712048,"raw_hex":"e8f0f96400","size":5,"soft_fail":false,"target_va":5412331248,"va":5405713659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77b70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37340240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e850c43902","size":5,"soft_fail":false,"target_va":5416385392,"va":5379045147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd56b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37725230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e82ea43f02","size":5,"soft_fail":false,"target_va":5416769200,"va":5379043965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37574435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e823573d02","size":5,"soft_fail":false,"target_va":5416620794,"va":5379046354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11243347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405712048,"raw_hex":"e8538fab00","size":5,"soft_fail":false,"target_va":5416956960,"va":5405713608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10944065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405712048,"raw_hex":"e841fea600","size":5,"soft_fail":false,"target_va":5416656256,"va":5405712186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t37575551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e87f5b3d02","size":5,"soft_fail":false,"target_va":5416621762,"va":5379046206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37576086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e8965d3d02","size":5,"soft_fail":false,"target_va":5416622320,"va":5379046229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-20882714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405712048,"raw_hex":"e8e65ac1fe","size":5,"soft_fail":false,"target_va":5384830608,"va":5405713317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::_$LT$impl$u20$str$GT$::trim_start_matches
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-345395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405712048,"raw_hex":"e8cdbafaff","size":5,"soft_fail":false,"target_va":5405368032,"va":5405713422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5786017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e8a1495800","size":5,"soft_fail":false,"target_va":5384830080,"va":5379044058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-14606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e8f2c6ffff","size":5,"soft_fail":false,"target_va":5379030448,"va":5379045049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t37576465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e8115f3d02","size":5,"soft_fail":false,"target_va":5416621376,"va":5379044906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$0<std::fs::File>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t37340581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379043824,"raw_hex":"e8a5c53902","size":5,"soft_fail":false,"target_va":5416385296,"va":5379044710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::Read::read_to_end
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6477360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405712048,"raw_hex":"e830d66200","size":5,"soft_fail":false,"target_va":5412189536,"va":5405712171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t*47117192(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379043824,"raw_hex":"ff1588f3ce02","size":6,"soft_fail":false,"va":5379043978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":521,"record_offset":41852,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":521,"record_offset":42816,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":44580,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":6793,"source_line":1363,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":521,"record_offset":42204,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":521,"record_offset":42176,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::find
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":521,"record_offset":42948,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":22162,"source_line":1481,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::trim_start
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":521,"record_offset":45236,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":30744,"source_line":2210,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":521,"record_offset":44612,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_direct] std::io::Read::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\mod.rs
//     evidence={"module":521,"record_offset":42148,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":19864,"source_line":1239,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":42040,"parent_record_offset":42016,"ancestor_record_offset":41636,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":42064,"parent_record_offset":42040,"ancestor_record_offset":41636,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":42544,"parent_record_offset":42520,"ancestor_record_offset":41636,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":42568,"parent_record_offset":42544,"ancestor_record_offset":41636,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":42088,"parent_record_offset":42064,"ancestor_record_offset":41636,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":42592,"parent_record_offset":42568,"ancestor_record_offset":41636,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":42016,"parent_record_offset":41984,"ancestor_record_offset":41636,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":42520,"parent_record_offset":42456,"ancestor_record_offset":41636,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":41916,"parent_record_offset":41884,"ancestor_record_offset":41636,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":42488,"parent_record_offset":42456,"ancestor_record_offset":41636,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":42456,"parent_record_offset":42428,"ancestor_record_offset":41636,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":41984,"parent_record_offset":41948,"ancestor_record_offset":41636,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":41948,"parent_record_offset":41916,"ancestor_record_offset":41636,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":42428,"parent_record_offset":42400,"ancestor_record_offset":41636,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":521,"record_offset":41884,"parent_record_offset":41852,"ancestor_record_offset":41636,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_whitespace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":521,"record_offset":45812,"parent_record_offset":45780,"ancestor_record_offset":41636,"inlinee":8307,"source_line":893,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::len_utf8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":521,"record_offset":44140,"parent_record_offset":43600,"ancestor_record_offset":41636,"inlinee":6762,"source_line":645,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::len_utf8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\char\methods.rs
//     evidence={"module":521,"record_offset":44164,"parent_record_offset":44140,"ancestor_record_offset":41636,"inlinee":6763,"source_line":1843,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":521,"record_offset":43332,"parent_record_offset":43304,"ancestor_record_offset":41636,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2894,"record_offset":3752,"parent_record_offset":3512,"ancestor_record_offset":264,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02894.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":521,"record_offset":43424,"parent_record_offset":43132,"ancestor_record_offset":41636,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":521,"record_offset":43528,"parent_record_offset":43472,"ancestor_record_offset":41636,"inlinee":10953,"source_line":217,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":521,"record_offset":43360,"parent_record_offset":43304,"ancestor_record_offset":41636,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":521,"record_offset":43304,"parent_record_offset":43276,"ancestor_record_offset":41636,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":521,"record_offset":43472,"parent_record_offset":43448,"ancestor_record_offset":41636,"inlinee":9305,"source_line":813,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":521,"record_offset":43276,"parent_record_offset":43132,"ancestor_record_offset":41636,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":521,"record_offset":43448,"parent_record_offset":43424,"ancestor_record_offset":41636,"inlinee":9304,"source_line":978,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2894,"record_offset":3608,"parent_record_offset":3580,"ancestor_record_offset":264,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02894.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":521,"record_offset":43384,"parent_record_offset":43360,"ancestor_record_offset":41636,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":521,"record_offset":43552,"parent_record_offset":43528,"ancestor_record_offset":41636,"inlinee":10954,"source_line":1011,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":521,"record_offset":43836,"parent_record_offset":43788,"ancestor_record_offset":41636,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":521,"record_offset":45780,"parent_record_offset":45748,"ancestor_record_offset":41636,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":521,"record_offset":44816,"parent_record_offset":44788,"ancestor_record_offset":41636,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":521,"record_offset":42228,"parent_record_offset":42204,"ancestor_record_offset":41636,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":521,"record_offset":46520,"parent_record_offset":46496,"ancestor_record_offset":41636,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":521,"record_offset":45676,"parent_record_offset":45652,"ancestor_record_offset":41636,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":521,"record_offset":43984,"parent_record_offset":43956,"ancestor_record_offset":41636,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2894,"record_offset":3780,"parent_record_offset":3752,"ancestor_record_offset":264,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02894.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2894,"record_offset":3808,"parent_record_offset":3780,"ancestor_record_offset":264,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02894.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":521,"record_offset":43212,"parent_record_offset":43132,"ancestor_record_offset":41636,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":521,"record_offset":46472,"parent_record_offset":46448,"ancestor_record_offset":41636,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2894,"record_offset":3640,"parent_record_offset":3580,"ancestor_record_offset":264,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/02894.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":521,"record_offset":42844,"parent_record_offset":42816,"ancestor_record_offset":41636,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":521,"record_offset":43240,"parent_record_offset":43212,"ancestor_record_offset":41636,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2894,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":264,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/02894.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":521,"record_offset":42900,"parent_record_offset":42872,"ancestor_record_offset":41636,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":521,"record_offset":42872,"parent_record_offset":42844,"ancestor_record_offset":41636,"inlinee":6155,"source_line":528,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":521,"record_offset":46496,"parent_record_offset":46472,"ancestor_record_offset":41636,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":521,"record_offset":43956,"parent_record_offset":43880,"ancestor_record_offset":41636,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":2894,"record_offset":3676,"parent_record_offset":3512,"ancestor_record_offset":264,"inlinee":6283,"source_line":25,"archive":"_pdb/C/modules/02894.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":2894,"record_offset":3708,"parent_record_offset":3676,"ancestor_record_offset":264,"inlinee":6284,"source_line":35,"archive":"_pdb/C/modules/02894.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":521,"record_offset":46424,"parent_record_offset":46400,"ancestor_record_offset":41636,"inlinee":8292,"source_line":1123,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":521,"record_offset":46448,"parent_record_offset":46424,"ancestor_record_offset":41636,"inlinee":8293,"source_line":1066,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":521,"record_offset":45928,"parent_record_offset":45260,"ancestor_record_offset":41636,"inlinee":8313,"source_line":687,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":521,"record_offset":43788,"parent_record_offset":43748,"ancestor_record_offset":41636,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::trim_start_matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":521,"record_offset":45260,"parent_record_offset":45236,"ancestor_record_offset":41636,"inlinee":23497,"source_line":2409,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":521,"record_offset":45412,"parent_record_offset":45380,"ancestor_record_offset":41636,"inlinee":8303,"source_line":39,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":521,"record_offset":45380,"parent_record_offset":45336,"ancestor_record_offset":41636,"inlinee":8301,"source_line":182,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::Searcher::next_reject
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":521,"record_offset":45312,"parent_record_offset":45288,"ancestor_record_offset":41636,"inlinee":8297,"source_line":263,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::TwoWaySearcher::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":521,"record_offset":43104,"parent_record_offset":43016,"ancestor_record_offset":41636,"inlinee":18836,"source_line":1461,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$10::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":521,"record_offset":45336,"parent_record_offset":45312,"ancestor_record_offset":41636,"inlinee":8299,"source_line":689,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2894,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":264,"inlinee":12692,"source_line":432,"archive":"_pdb/C/modules/02894.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$26::next_reject
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":521,"record_offset":45288,"parent_record_offset":45260,"ancestor_record_offset":41636,"inlinee":8295,"source_line":788,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$29::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":521,"record_offset":46376,"parent_record_offset":46344,"ancestor_record_offset":41636,"inlinee":8289,"source_line":736,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":521,"record_offset":42988,"parent_record_offset":42948,"ancestor_record_offset":41636,"inlinee":9296,"source_line":975,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":521,"record_offset":43600,"parent_record_offset":43016,"ancestor_record_offset":41636,"inlinee":18705,"source_line":1119,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":521,"record_offset":43016,"parent_record_offset":42948,"ancestor_record_offset":41636,"inlinee":18827,"source_line":1171,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::into_searcher
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":521,"record_offset":44888,"parent_record_offset":44864,"ancestor_record_offset":41636,"inlinee":30741,"source_line":564,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$5::matches
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":521,"record_offset":45748,"parent_record_offset":45336,"ancestor_record_offset":41636,"inlinee":8306,"source_line":638,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$9::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":521,"record_offset":46400,"parent_record_offset":46376,"ancestor_record_offset":41636,"inlinee":8291,"source_line":677,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":521,"record_offset":46208,"parent_record_offset":46172,"ancestor_record_offset":41636,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":521,"record_offset":46172,"parent_record_offset":46144,"ancestor_record_offset":41636,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":521,"record_offset":43748,"parent_record_offset":43704,"ancestor_record_offset":41636,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":521,"record_offset":44764,"parent_record_offset":44668,"ancestor_record_offset":41636,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":521,"record_offset":43704,"parent_record_offset":43648,"ancestor_record_offset":41636,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":521,"record_offset":43648,"parent_record_offset":43600,"ancestor_record_offset":41636,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":521,"record_offset":44788,"parent_record_offset":44764,"ancestor_record_offset":41636,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":521,"record_offset":43880,"parent_record_offset":43648,"ancestor_record_offset":41636,"inlinee":8304,"source_line":35,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":521,"record_offset":44048,"parent_record_offset":43880,"ancestor_record_offset":41636,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":521,"record_offset":44020,"parent_record_offset":43880,"ancestor_record_offset":41636,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::unicode::unicode_data::white_space::lookup
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\unicode\unicode_data.rs
//     evidence={"module":521,"record_offset":45852,"parent_record_offset":45812,"ancestor_record_offset":41636,"inlinee":8309,"source_line":747,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_file_identity.rs
//     evidence={"module":521,"record_offset":44952,"parent_record_offset":44920,"ancestor_record_offset":41636,"inlinee":30743,"source_line":34,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_file_identity.rs
//     evidence={"module":521,"record_offset":46112,"parent_record_offset":46080,"ancestor_record_offset":41636,"inlinee":30745,"source_line":39,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":521,"record_offset":42324,"parent_record_offset":42300,"ancestor_record_offset":41636,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$0<std::fs::File>
// 记录形式: compiled_procedure; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":521,"record_offset":48108,"rva":47676176,"va":5416385296}
// 编译实例 RVAs: 0x2d77b10, 0x2e033c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t376234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416385296,"raw_hex":"e8aabd0500","size":5,"soft_fail":false,"target_va":5416761616,"va":5416385377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-31590510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416385296,"raw_hex":"e892f71dfe","size":5,"soft_fail":false,"target_va":5384794864,"va":5416385369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-302838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416956864,"raw_hex":"e80a61fbff","size":5,"soft_fail":false,"target_va":5416654112,"va":5416956945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":48304,"parent_record_offset":48108,"ancestor_record_offset":48108,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":521,"record_offset":48352,"parent_record_offset":48328,"ancestor_record_offset":48108,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":521,"record_offset":48328,"parent_record_offset":48304,"ancestor_record_offset":48108,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7817
// 首条位置: {"module":521,"inlinee":30743}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7819
// 首条位置: {"module":521,"inlinee":30745}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112DD
// 首条位置: {"module":1253,"inlinee":70365}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":40336,"parent_record_offset":40316,"ancestor_record_offset":40316,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":40384,"parent_record_offset":40360,"ancestor_record_offset":40316,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1253,"record_offset":40360,"parent_record_offset":40336,"ancestor_record_offset":40316,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_file_identity::derive_fallback_save_file_identity
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1666,"record_offset":59676,"rva":24961840,"va":5393670960}
// 编译实例 RVAs: 0x17ce330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9013957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393670960,"raw_hex":"e83b7576ff","size":5,"soft_fail":false,"target_va":5384657024,"va":5393670976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24332308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393670960,"raw_hex":"e8ecb78cfe","size":5,"soft_fail":false,"target_va":5369338848,"va":5393671151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc8770
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12606636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393670960,"raw_hex":"e854a33fff","size":5,"soft_fail":false,"target_va":5381064560,"va":5393671191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc8ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12605049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393670960,"raw_hex":"e887a93fff","size":5,"soft_fail":false,"target_va":5381065952,"va":5393670996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::metadata
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8946108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393670960,"raw_hex":"e8447e77ff","size":5,"soft_fail":false,"target_va":5384724912,"va":5393671015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1666,"record_offset":59936,"parent_record_offset":59676,"ancestor_record_offset":59676,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1666,"record_offset":59848,"parent_record_offset":59676,"ancestor_record_offset":59676,"inlinee":10149,"source_line":3261,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1666,"record_offset":59896,"parent_record_offset":59872,"ancestor_record_offset":59676,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1666,"record_offset":59872,"parent_record_offset":59848,"ancestor_record_offset":59676,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/01666.jsonl.gz"}
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
// 完整结构及原始类型记录: save_file_identity.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: save_file_identity.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: save_file_identity.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] bool (*)(char)
// 候选参数类型: ["char"]

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::derive_fallback_save_file_identity
// 信息 ID: a01909144f99c2d917ec35e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::load_game::parse_saved_game_data @ 0xb23778 -> 0x17ce330 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui::closure$9 @ 0x11a0611 -> 0x17ce330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$0<std::fs::File>
// 信息 ID: f1d06c9c1746f892381005aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$0 -> ["std::fs::File"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File> @ 0x9db566 -> 0x2d77b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File> @ 0x234a221 -> 0x2e033c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$1
// 信息 ID: 43256ed093abe3d19d86dbe3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>; module=521, inline_record=44952; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>; module=2894, inline_record=3888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$2
// 信息 ID: 5e7d0558d58df0a4559f024f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>; module=521, inline_record=46112; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>; module=2894, inline_record=4600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$3
// 信息 ID: 6d7be0a23fe13352a385b08c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure_env$3 <- anyhow::context::impl$0::with_context<country_core::resources::history::HistoryIdentity,serde_json::error::Error,alloc::string::String,country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure_env$3<std::fs::File> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure_env$3 <- anyhow::context::impl$0::with_context<country_core::resources::history::HistoryIdentity,serde_json::error::Error,alloc::string::String,country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure_env$3<std::fs::File> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<country_core::resources::history::HistoryIdentity,serde_json::error::Error,alloc::string::String,country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure_env$3<std::fs::File> >; module=1253, inline_record=40316; 不是独立调用边。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<country_core::resources::history::HistoryIdentity,serde_json::error::Error,alloc::string::String,country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure_env$3<std::fs::File> >; module=1307, inline_record=604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>
// 信息 ID: 1e6f06d837c67ae47416ca05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity -> ["std::fs::File"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::session::import::read_origin @ 0xa828db -> 0x9db1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui::closure$9 @ 0x11a050a -> 0x2349eb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/save/save_file_identity.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: cda56610d24164fac39dd2c67bae6f5b10b37dd3fa8b25a7a25a5f7b065dfffa
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: save_file_identity.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1644, 2800, 3953
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2800,"record_offset":256,"rva":31821600,"va":5400530720}
// 编译实例 RVAs: 0x1e58f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25449295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e8b1ac7bfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5400530778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25449409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e83fac7bfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5400530908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142379a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5375014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e826045200","size":5,"soft_fail":false,"target_va":5405907536,"va":5400532517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14237c800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5386698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e8ca315200","size":5,"soft_fail":false,"target_va":5405919232,"va":5400532529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428e5920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11060698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e8dac5a800","size":5,"soft_fail":false,"target_va":5411592480,"va":5400531777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428e60d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11063578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e81ad1a800","size":5,"soft_fail":false,"target_va":5411594448,"va":5400530865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17691565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e8530cf2fe","size":5,"soft_fail":false,"target_va":5382840208,"va":5400531768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11008166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e8a6f8a700","size":5,"soft_fail":false,"target_va":5411540963,"va":5400532792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11238549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e8957cab00","size":5,"soft_fail":false,"target_va":5411770928,"va":5400532374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t11009590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e836fea700","size":5,"soft_fail":false,"target_va":5411542296,"va":5400532701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11010428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e87c01a800","size":5,"soft_fail":false,"target_va":5411543104,"va":5400532671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17631914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e856f5f2fe","size":5,"soft_fail":false,"target_va":5382900288,"va":5400532197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::_$LT$impl$u20$str$GT$::trim_start_matches
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-383767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e8e924faff","size":5,"soft_fail":false,"target_va":5400148528,"va":5400532290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::converts::from_utf8
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17629374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e842fff2fe","size":5,"soft_fail":false,"target_va":5382901600,"va":5400530969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::pattern::StrSearcher::new
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17626920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e8d808f3fe","size":5,"soft_fail":false,"target_va":5382904128,"va":5400531043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11011261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e8bd04a800","size":5,"soft_fail":false,"target_va":5411543360,"va":5400532094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::Read::read_to_end
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5202775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400530720,"raw_hex":"e857634f00","size":5,"soft_fail":false,"target_va":5405733632,"va":5400530852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*20157485(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5400530720,"raw_hex":"ff152d943301","size":6,"soft_fail":false,"va":5400530877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2800,"record_offset":468,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2800,"record_offset":1324,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2800,"record_offset":3108,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5145,"source_line":1277,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2800,"record_offset":764,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2800,"record_offset":736,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::find
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2800,"record_offset":1456,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":20192,"source_line":1442,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::trim_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2800,"record_offset":4224,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":60139,"source_line":2164,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2800,"record_offset":3252,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2800,"record_offset":656,"parent_record_offset":632,"ancestor_record_offset":256,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2800,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":256,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2800,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2800,"record_offset":632,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2800,"record_offset":1084,"parent_record_offset":1016,"ancestor_record_offset":256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2800,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":256,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2800,"record_offset":3208,"parent_record_offset":3184,"ancestor_record_offset":256,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2800,"record_offset":532,"parent_record_offset":500,"ancestor_record_offset":256,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2800,"record_offset":1052,"parent_record_offset":1016,"ancestor_record_offset":256,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2800,"record_offset":1016,"parent_record_offset":988,"ancestor_record_offset":256,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2800,"record_offset":600,"parent_record_offset":564,"ancestor_record_offset":256,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2800,"record_offset":564,"parent_record_offset":532,"ancestor_record_offset":256,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2800,"record_offset":988,"parent_record_offset":960,"ancestor_record_offset":256,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2800,"record_offset":500,"parent_record_offset":468,"ancestor_record_offset":256,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":2800,"record_offset":2552,"parent_record_offset":2096,"ancestor_record_offset":256,"inlinee":5002,"source_line":633,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::len_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":2800,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":256,"inlinee":5004,"source_line":1765,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2800,"record_offset":1828,"parent_record_offset":1800,"ancestor_record_offset":256,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2800,"record_offset":3788,"parent_record_offset":3528,"ancestor_record_offset":256,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":2800,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":256,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2800,"record_offset":1920,"parent_record_offset":1632,"ancestor_record_offset":256,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::backward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2800,"record_offset":2024,"parent_record_offset":1968,"ancestor_record_offset":256,"inlinee":12495,"source_line":209,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2800,"record_offset":1856,"parent_record_offset":1800,"ancestor_record_offset":256,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2800,"record_offset":1800,"parent_record_offset":1772,"ancestor_record_offset":256,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2800,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":256,"inlinee":5010,"source_line":805,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2800,"record_offset":1772,"parent_record_offset":1632,"ancestor_record_offset":256,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$7::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":2800,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":256,"inlinee":5009,"source_line":970,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2800,"record_offset":3616,"parent_record_offset":3588,"ancestor_record_offset":256,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2800,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":256,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2800,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":256,"inlinee":12496,"source_line":750,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":2800,"record_offset":2284,"parent_record_offset":2252,"ancestor_record_offset":256,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2800,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":256,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2800,"record_offset":3456,"parent_record_offset":3428,"ancestor_record_offset":256,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2800,"record_offset":788,"parent_record_offset":764,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2800,"record_offset":2400,"parent_record_offset":2372,"ancestor_record_offset":256,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2800,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":256,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2800,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":256,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2800,"record_offset":1708,"parent_record_offset":1632,"ancestor_record_offset":256,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2800,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":256,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2800,"record_offset":3648,"parent_record_offset":3588,"ancestor_record_offset":256,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2800,"record_offset":1352,"parent_record_offset":1324,"ancestor_record_offset":256,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2800,"record_offset":1736,"parent_record_offset":1708,"ancestor_record_offset":256,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2800,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":256,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2800,"record_offset":1408,"parent_record_offset":1380,"ancestor_record_offset":256,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2800,"record_offset":1380,"parent_record_offset":1352,"ancestor_record_offset":256,"inlinee":9466,"source_line":521,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2800,"record_offset":2372,"parent_record_offset":2324,"ancestor_record_offset":256,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\memchr.rs
//     evidence={"module":2800,"record_offset":3712,"parent_record_offset":3528,"ancestor_record_offset":256,"inlinee":6428,"source_line":26,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\memchr.rs
//     evidence={"module":2800,"record_offset":3744,"parent_record_offset":3712,"ancestor_record_offset":256,"inlinee":6429,"source_line":36,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2800,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":256,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::TwoWaySearcher::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2800,"record_offset":1604,"parent_record_offset":1524,"ancestor_record_offset":256,"inlinee":16918,"source_line":1457,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2800,"record_offset":3528,"parent_record_offset":3504,"ancestor_record_offset":256,"inlinee":17879,"source_line":432,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2800,"record_offset":1496,"parent_record_offset":1456,"ancestor_record_offset":256,"inlinee":4997,"source_line":975,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2800,"record_offset":2096,"parent_record_offset":1524,"ancestor_record_offset":256,"inlinee":16825,"source_line":1115,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$33::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2800,"record_offset":1524,"parent_record_offset":1456,"ancestor_record_offset":256,"inlinee":16919,"source_line":1167,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2800,"record_offset":2228,"parent_record_offset":2196,"ancestor_record_offset":256,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2800,"record_offset":3404,"parent_record_offset":3308,"ancestor_record_offset":256,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2800,"record_offset":2196,"parent_record_offset":2156,"ancestor_record_offset":256,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2800,"record_offset":2156,"parent_record_offset":2096,"ancestor_record_offset":256,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2800,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":256,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2800,"record_offset":2324,"parent_record_offset":2156,"ancestor_record_offset":256,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2800,"record_offset":2460,"parent_record_offset":2324,"ancestor_record_offset":256,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2800,"record_offset":2432,"parent_record_offset":2324,"ancestor_record_offset":256,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_file_identity.rs
//     evidence={"module":2800,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":256,"inlinee":100203,"source_line":23,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_file_identity.rs
//     evidence={"module":2800,"record_offset":4164,"parent_record_offset":4132,"ancestor_record_offset":256,"inlinee":100204,"source_line":34,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/save_file_identity.rs
//     evidence={"module":2800,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":256,"inlinee":100205,"source_line":39,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":2800,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":256,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1876B
// 首条位置: {"module":2800,"inlinee":100203}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2800,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":3136,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2800,"record_offset":3208,"parent_record_offset":3184,"ancestor_record_offset":3136,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2800,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":3136,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1876C
// 首条位置: {"module":2800,"inlinee":100204}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":2800,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":4164,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1876D
// 首条位置: {"module":2800,"inlinee":100205}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":2800,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":4880,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/02800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1E08B
// 首条位置: {"module":3953,"inlinee":123019}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3953,"record_offset":616,"parent_record_offset":596,"ancestor_record_offset":596,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":3953,"record_offset":664,"parent_record_offset":640,"ancestor_record_offset":596,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/03953.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3953,"record_offset":640,"parent_record_offset":616,"ancestor_record_offset":596,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/03953.jsonl.gz"}
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
// 首条位置: {"module":1644,"record_offset":56636,"rva":23540032,"va":5392249152}
// 编译实例 RVAs: 0x1673140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17168071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392249152,"raw_hex":"e83909fafe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392249570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14145a8e0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2197828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392249152,"raw_hex":"e8bc76deff","size":5,"soft_fail":false,"target_va":5390051552,"va":5392249375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14145aa70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: save_file_identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2197258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392249152,"raw_hex":"e8f678deff","size":5,"soft_fail":false,"target_va":5390051952,"va":5392249205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::Metadata::modified
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9540564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392249152,"raw_hex":"e82c6c6eff","size":5,"soft_fail":false,"target_va":5382708832,"va":5392249391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9507689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392249152,"raw_hex":"e897ec6eff","size":5,"soft_fail":false,"target_va":5382741504,"va":5392249188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9472636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392249152,"raw_hex":"e884756fff","size":5,"soft_fail":false,"target_va":5382776592,"va":5392249223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5392249152,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5392249296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":56904,"parent_record_offset":56636,"ancestor_record_offset":56636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1644,"record_offset":56808,"parent_record_offset":56636,"ancestor_record_offset":56636,"inlinee":14362,"source_line":2913,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":57352,"parent_record_offset":57328,"ancestor_record_offset":56636,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1644,"record_offset":57328,"parent_record_offset":57244,"ancestor_record_offset":56636,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1644,"record_offset":57244,"parent_record_offset":57216,"ancestor_record_offset":56636,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1644,"record_offset":57272,"parent_record_offset":57244,"ancestor_record_offset":56636,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1644,"record_offset":57296,"parent_record_offset":57272,"ancestor_record_offset":56636,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1644,"record_offset":56932,"parent_record_offset":56904,"ancestor_record_offset":56636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1644,"record_offset":57044,"parent_record_offset":57016,"ancestor_record_offset":56636,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1644,"record_offset":57092,"parent_record_offset":57068,"ancestor_record_offset":56636,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1644,"record_offset":57068,"parent_record_offset":57044,"ancestor_record_offset":56636,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1644,"record_offset":56864,"parent_record_offset":56836,"ancestor_record_offset":56636,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1644,"record_offset":56836,"parent_record_offset":56808,"ancestor_record_offset":56636,"inlinee":12458,"source_line":2381,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1644,"record_offset":57016,"parent_record_offset":56988,"ancestor_record_offset":56636,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01644.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1644,"record_offset":56988,"parent_record_offset":56960,"ancestor_record_offset":56636,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01644.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::derive_fallback_save_file_identity
// 信息 ID: 985c56f805450c2b307b93ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::load::ui::closure$7 @ 0xfc44cb -> 0x1673140 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::load_game::parse_saved_game_data @ 0x1472bc1 -> 0x1673140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$0
// 信息 ID: 01587a7cac1bb3b6bf5979cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>; module=2800, inline_record=3136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$1
// 信息 ID: f0c8b69f49a09ce37a078f9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>; module=2800, inline_record=4164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$2
// 信息 ID: 338a8a80f72a14e3c33545ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>; module=2800, inline_record=4880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure$3
// 信息 ID: 0eb9ac323555953c4f39c065
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure_env$3 <- anyhow::context::impl$0::with_context<country_core::resources::history::HistoryIdentity,serde_json::error::Error,alloc::string::String,country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure_env$3<std::fs::File> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<country_core::resources::history::HistoryIdentity,serde_json::error::Error,alloc::string::String,country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity::closure_env$3<std::fs::File> >; module=3953, inline_record=596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity<std::fs::File>
// 信息 ID: 3c78f90217a1da74e23ea640
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::save::save_file_identity::try_approx_parse_save_file_identity -> ["std::fs::File"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::load::ui::closure$7 @ 0xfc43a0 -> 0x1e58f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/render/nani/trimesh.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5d0ed0fcb7185d05a90d1c1db604a27ca85e29cba3311c265d52c4b81bb9f9d0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: trimesh.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 503, 1553, 1554
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::trimesh::TrimeshChunk::index_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1553,"record_offset":2544,"rva":19520448,"va":5388229568}
// 编译实例 RVAs: 0x129dbc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::trimesh::Trimesh::builder
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1553,"record_offset":2684,"rva":19520464,"va":5388229584}
// 编译实例 RVAs: 0x129dbd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140149f50
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18172187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e8e5b6eafe","size":5,"soft_fail":false,"target_va":5370060624,"va":5388232806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13553703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e8d92f31ff","size":5,"soft_fail":false,"target_va":5374676688,"va":5388230386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e6a70
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13335634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e8ae8334ff","size":5,"soft_fail":false,"target_va":5374896752,"va":5388232381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13148245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e8ab5f37ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388229728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13150603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e8755637ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388232102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140876080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10651547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e865785dff","size":5,"soft_fail":false,"target_va":5377581184,"va":5388232726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428b99a0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23180608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e840b56101","size":5,"soft_fail":false,"target_va":5411412384,"va":5388231771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23617068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e82c5e6801","size":5,"soft_fail":false,"target_va":5411847200,"va":5388230127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t4236216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e8b8a34000","size":5,"soft_fail":false,"target_va":5392466848,"va":5388230627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23310978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e882b26301","size":5,"soft_fail":false,"target_va":5411540963,"va":5388229980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::Fallibility::alloc_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5392654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e8f2b6adff","size":5,"soft_fail":false,"target_va":5382837200,"va":5388229849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5571790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e832fbaaff","size":5,"soft_fail":false,"target_va":5382657856,"va":5388229641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5475884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388229584,"raw_hex":"e8d471acff","size":5,"soft_fail":false,"target_va":5382753792,"va":5388229671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":4352,"parent_record_offset":2684,"ancestor_record_offset":2684,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":4116,"parent_record_offset":2684,"ancestor_record_offset":2684,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":5736,"parent_record_offset":2684,"ancestor_record_offset":2684,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":4084,"parent_record_offset":2684,"ancestor_record_offset":2684,"inlinee":6245,"source_line":3197,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":13240,"parent_record_offset":2684,"ancestor_record_offset":2684,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1553,"record_offset":4016,"parent_record_offset":2684,"ancestor_record_offset":2684,"inlinee":30845,"source_line":1235,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1553,"record_offset":5820,"parent_record_offset":2684,"ancestor_record_offset":2684,"inlinee":17422,"source_line":1911,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::trimesh::impl$1::builder::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/trimesh.rs
//     evidence={"module":1553,"record_offset":5848,"parent_record_offset":2684,"ancestor_record_offset":2684,"inlinee":71863,"source_line":56,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":4552,"parent_record_offset":2684,"ancestor_record_offset":2684,"inlinee":19928,"source_line":727,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":13184,"parent_record_offset":2684,"ancestor_record_offset":2684,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":2864,"parent_record_offset":2684,"ancestor_record_offset":2684,"inlinee":13588,"source_line":268,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":2684,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":2684,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":13428,"parent_record_offset":13404,"ancestor_record_offset":2684,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":3728,"parent_record_offset":3704,"ancestor_record_offset":2684,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":13404,"parent_record_offset":13312,"ancestor_record_offset":2684,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":4432,"parent_record_offset":4400,"ancestor_record_offset":2684,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":2684,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":13340,"parent_record_offset":13312,"ancestor_record_offset":2684,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":13312,"parent_record_offset":13288,"ancestor_record_offset":2684,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":2684,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":2684,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":4216,"parent_record_offset":4188,"ancestor_record_offset":2684,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":2684,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":13288,"parent_record_offset":13264,"ancestor_record_offset":2684,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":4400,"parent_record_offset":4352,"ancestor_record_offset":2684,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":6892,"parent_record_offset":6864,"ancestor_record_offset":2684,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":2684,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":3372,"parent_record_offset":3344,"ancestor_record_offset":2684,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1553,"record_offset":3432,"parent_record_offset":3400,"ancestor_record_offset":2684,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":5900,"parent_record_offset":5848,"ancestor_record_offset":2684,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":2684,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":2684,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":2684,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":2684,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":6284,"parent_record_offset":6260,"ancestor_record_offset":2684,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1553,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":2684,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1553,"record_offset":6560,"parent_record_offset":6064,"ancestor_record_offset":2684,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1553,"record_offset":6780,"parent_record_offset":6748,"ancestor_record_offset":2684,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1553,"record_offset":3948,"parent_record_offset":3924,"ancestor_record_offset":2684,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1553,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":2684,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1553,"record_offset":3236,"parent_record_offset":3212,"ancestor_record_offset":2684,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":13956,"parent_record_offset":13928,"ancestor_record_offset":2684,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":13372,"parent_record_offset":13340,"ancestor_record_offset":2684,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":7980,"parent_record_offset":7952,"ancestor_record_offset":2684,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":3400,"parent_record_offset":3344,"ancestor_record_offset":2684,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1553,"record_offset":6432,"parent_record_offset":6404,"ancestor_record_offset":2684,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1553,"record_offset":6404,"parent_record_offset":6376,"ancestor_record_offset":2684,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1553,"record_offset":3000,"parent_record_offset":2968,"ancestor_record_offset":2684,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":2684,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":4684,"parent_record_offset":4600,"ancestor_record_offset":2684,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1553,"record_offset":3844,"parent_record_offset":3672,"ancestor_record_offset":2684,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1553,"record_offset":3188,"parent_record_offset":3080,"ancestor_record_offset":2684,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1553,"record_offset":8076,"parent_record_offset":8048,"ancestor_record_offset":2684,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1553,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":2684,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":3260,"parent_record_offset":3236,"ancestor_record_offset":2684,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":2684,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":2684,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":3288,"parent_record_offset":3236,"ancestor_record_offset":2684,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1553,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":2684,"inlinee":30847,"source_line":1866,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::trimesh::impl$1::builder::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/trimesh.rs
//     evidence={"module":1553,"record_offset":6864,"parent_record_offset":6824,"ancestor_record_offset":2684,"inlinee":71865,"source_line":64,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":6344,"parent_record_offset":6316,"ancestor_record_offset":2684,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":6376,"parent_record_offset":6344,"ancestor_record_offset":2684,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":6316,"parent_record_offset":6064,"ancestor_record_offset":2684,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":6176,"parent_record_offset":6064,"ancestor_record_offset":2684,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":7696,"parent_record_offset":7624,"ancestor_record_offset":2684,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":6588,"parent_record_offset":6064,"ancestor_record_offset":2684,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":7468,"parent_record_offset":7428,"ancestor_record_offset":2684,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":6260,"parent_record_offset":6064,"ancestor_record_offset":2684,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1553,"record_offset":6120,"parent_record_offset":6064,"ancestor_record_offset":2684,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1553,"record_offset":8116,"parent_record_offset":7148,"ancestor_record_offset":2684,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::TagSliceExt::fill_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1553,"record_offset":3876,"parent_record_offset":3640,"ancestor_record_offset":2684,"inlinee":6223,"source_line":73,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1553,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":2684,"inlinee":6225,"source_line":79,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":2684,"inlinee":17339,"source_line":1009,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":13208,"parent_record_offset":13184,"ancestor_record_offset":2684,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":6720,"parent_record_offset":5964,"ancestor_record_offset":2684,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":1553,"record_offset":5964,"parent_record_offset":5928,"ancestor_record_offset":2684,"inlinee":7717,"source_line":34,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_capacity_and_hasher
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":3536,"parent_record_offset":2888,"ancestor_record_offset":2684,"inlinee":11991,"source_line":499,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":6008,"parent_record_offset":5964,"ancestor_record_offset":2684,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":2684,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":8048,"parent_record_offset":8020,"ancestor_record_offset":2684,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":8144,"parent_record_offset":7148,"ancestor_record_offset":2684,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6680,"parent_record_offset":6064,"ancestor_record_offset":2684,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":2684,"inlinee":10622,"source_line":813,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":4756,"parent_record_offset":4732,"ancestor_record_offset":2684,"inlinee":10625,"source_line":807,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6036,"parent_record_offset":5964,"ancestor_record_offset":2684,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7148,"parent_record_offset":7116,"ancestor_record_offset":2684,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":4628,"parent_record_offset":4600,"ancestor_record_offset":2684,"inlinee":10143,"source_line":1263,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":4652,"parent_record_offset":4628,"ancestor_record_offset":2684,"inlinee":10144,"source_line":1257,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6748,"parent_record_offset":6720,"ancestor_record_offset":2684,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::with_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":3564,"parent_record_offset":3536,"ancestor_record_offset":2684,"inlinee":8476,"source_line":592,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::with_capacity_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":2684,"inlinee":6289,"source_line":641,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":13896,"parent_record_offset":13864,"ancestor_record_offset":2684,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":8020,"parent_record_offset":7148,"ancestor_record_offset":2684,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":4780,"parent_record_offset":4756,"ancestor_record_offset":2684,"inlinee":10626,"source_line":3020,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":4840,"parent_record_offset":4780,"ancestor_record_offset":2684,"inlinee":10627,"source_line":2482,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":2684,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fallible_with_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":2684,"inlinee":6237,"source_line":1465,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6064,"parent_record_offset":6036,"ancestor_record_offset":2684,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7244,"parent_record_offset":7200,"ancestor_record_offset":2684,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7428,"parent_record_offset":7244,"ancestor_record_offset":2684,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7624,"parent_record_offset":7244,"ancestor_record_offset":2684,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":13864,"parent_record_offset":13808,"ancestor_record_offset":2684,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7660,"parent_record_offset":7624,"ancestor_record_offset":2684,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":4812,"parent_record_offset":4780,"ancestor_record_offset":2684,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::new_uninitialized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":3672,"parent_record_offset":3640,"ancestor_record_offset":2684,"inlinee":6141,"source_line":1428,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":4872,"parent_record_offset":4840,"ancestor_record_offset":2684,"inlinee":6148,"source_line":2504,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7200,"parent_record_offset":7148,"ancestor_record_offset":2684,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6148,"parent_record_offset":6064,"ancestor_record_offset":2684,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":2684,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7928,"parent_record_offset":7200,"ancestor_record_offset":2684,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::with_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":3616,"parent_record_offset":3588,"ancestor_record_offset":2684,"inlinee":6278,"source_line":1506,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":13928,"parent_record_offset":13896,"ancestor_record_offset":2684,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::alloc::inner::do_alloc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\alloc.rs
//     evidence={"module":1553,"record_offset":3704,"parent_record_offset":3672,"ancestor_record_offset":2684,"inlinee":6144,"source_line":14,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":4988,"parent_record_offset":4780,"ancestor_record_offset":2684,"inlinee":6127,"source_line":128,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":13784,"parent_record_offset":13760,"ancestor_record_offset":2684,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":4732,"parent_record_offset":4708,"ancestor_record_offset":2684,"inlinee":10624,"source_line":819,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6476,"parent_record_offset":6064,"ancestor_record_offset":2684,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":1553,"record_offset":7116,"parent_record_offset":7084,"ancestor_record_offset":2684,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":1553,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":2684,"inlinee":7719,"source_line":36,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\scopeguard.rs
//     evidence={"module":1553,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":2684,"inlinee":6123,"source_line":69,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":6824,"parent_record_offset":5848,"ancestor_record_offset":2684,"inlinee":7951,"source_line":2822,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":5928,"parent_record_offset":5848,"ancestor_record_offset":2684,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":7084,"parent_record_offset":6824,"ancestor_record_offset":2684,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":2920,"parent_record_offset":2888,"ancestor_record_offset":2684,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1553,"record_offset":3148,"parent_record_offset":3128,"ancestor_record_offset":2684,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1553,"record_offset":3024,"parent_record_offset":3000,"ancestor_record_offset":2684,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":3344,"parent_record_offset":2968,"ancestor_record_offset":2684,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1553,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":2684,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1553,"record_offset":3052,"parent_record_offset":3024,"ancestor_record_offset":2684,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1553,"record_offset":3080,"parent_record_offset":3052,"ancestor_record_offset":2684,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":2684,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1553,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":2684,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::trimesh::impl$1::builder::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x118B7
// 首条位置: {"module":1553,"inlinee":71863}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":5900,"parent_record_offset":5848,"ancestor_record_offset":5848,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":6824,"parent_record_offset":5848,"ancestor_record_offset":5848,"inlinee":7951,"source_line":2822,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":5928,"parent_record_offset":5848,"ancestor_record_offset":5848,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":5848,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":5848,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":5848,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":6936,"parent_record_offset":6892,"ancestor_record_offset":5848,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":6892,"parent_record_offset":6864,"ancestor_record_offset":5848,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1553,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":5848,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":5848,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":5848,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1553,"record_offset":6284,"parent_record_offset":6260,"ancestor_record_offset":5848,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1553,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":5848,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1553,"record_offset":6560,"parent_record_offset":6064,"ancestor_record_offset":5848,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1553,"record_offset":6780,"parent_record_offset":6748,"ancestor_record_offset":5848,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":7980,"parent_record_offset":7952,"ancestor_record_offset":5848,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1553,"record_offset":6432,"parent_record_offset":6404,"ancestor_record_offset":5848,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1553,"record_offset":6404,"parent_record_offset":6376,"ancestor_record_offset":5848,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1553,"record_offset":8076,"parent_record_offset":8048,"ancestor_record_offset":5848,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":7048,"parent_record_offset":6892,"ancestor_record_offset":5848,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::trimesh::impl$1::builder::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/trimesh.rs
//     evidence={"module":1553,"record_offset":6864,"parent_record_offset":6824,"ancestor_record_offset":5848,"inlinee":71865,"source_line":64,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":6344,"parent_record_offset":6316,"ancestor_record_offset":5848,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":6376,"parent_record_offset":6344,"ancestor_record_offset":5848,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1553,"record_offset":6316,"parent_record_offset":6064,"ancestor_record_offset":5848,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":6176,"parent_record_offset":6064,"ancestor_record_offset":5848,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":7696,"parent_record_offset":7624,"ancestor_record_offset":5848,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":6588,"parent_record_offset":6064,"ancestor_record_offset":5848,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":7468,"parent_record_offset":7428,"ancestor_record_offset":5848,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1553,"record_offset":6260,"parent_record_offset":6064,"ancestor_record_offset":5848,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1553,"record_offset":6120,"parent_record_offset":6064,"ancestor_record_offset":5848,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1553,"record_offset":8116,"parent_record_offset":7148,"ancestor_record_offset":5848,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":6720,"parent_record_offset":5964,"ancestor_record_offset":5848,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":1553,"record_offset":5964,"parent_record_offset":5928,"ancestor_record_offset":5848,"inlinee":7717,"source_line":34,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1553,"record_offset":6008,"parent_record_offset":5964,"ancestor_record_offset":5848,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":5848,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":8048,"parent_record_offset":8020,"ancestor_record_offset":5848,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":8144,"parent_record_offset":7148,"ancestor_record_offset":5848,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6680,"parent_record_offset":6064,"ancestor_record_offset":5848,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6036,"parent_record_offset":5964,"ancestor_record_offset":5848,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7148,"parent_record_offset":7116,"ancestor_record_offset":5848,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6748,"parent_record_offset":6720,"ancestor_record_offset":5848,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":8020,"parent_record_offset":7148,"ancestor_record_offset":5848,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6064,"parent_record_offset":6036,"ancestor_record_offset":5848,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7244,"parent_record_offset":7200,"ancestor_record_offset":5848,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7428,"parent_record_offset":7244,"ancestor_record_offset":5848,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7624,"parent_record_offset":7244,"ancestor_record_offset":5848,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7660,"parent_record_offset":7624,"ancestor_record_offset":5848,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7200,"parent_record_offset":7148,"ancestor_record_offset":5848,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6148,"parent_record_offset":6064,"ancestor_record_offset":5848,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":5848,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":7928,"parent_record_offset":7200,"ancestor_record_offset":5848,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1553,"record_offset":6476,"parent_record_offset":6064,"ancestor_record_offset":5848,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":1553,"record_offset":7116,"parent_record_offset":7084,"ancestor_record_offset":5848,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":1553,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":5848,"inlinee":7719,"source_line":36,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1553,"record_offset":7084,"parent_record_offset":6824,"ancestor_record_offset":5848,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::trimesh::impl$1::builder::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x118B9
// 首条位置: {"module":1553,"inlinee":71865}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":6892,"parent_record_offset":6864,"ancestor_record_offset":6864,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":6864,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":6864,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":6864,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":6936,"parent_record_offset":6892,"ancestor_record_offset":6864,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":7048,"parent_record_offset":6892,"ancestor_record_offset":6864,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::trimesh::TrimeshBuilder::positions
// 记录形式: compiled_procedure; instances=1
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1553,"record_offset":14580,"rva":19523728,"va":5388232848}
// 编译实例 RVAs: 0x129e890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140905080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10066025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232848,"raw_hex":"e8976766ff","size":5,"soft_fail":false,"target_va":5378166912,"va":5388232932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::trimesh::TrimeshBuilder::attrib<glam::f32::vec3::Vec3>
// 记录形式: compiled_procedure; instances=2
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":11296,"rva":9457792,"va":5378166912}
// 编译实例 RVAs: 0x905080, 0x12a90c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14088e180
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-487630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378166912,"raw_hex":"e8328ff8ff","size":5,"soft_fail":false,"target_va":5377679744,"va":5378167369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3810195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378166912,"raw_hex":"e86ddcc5ff","size":5,"soft_fail":false,"target_va":5374356912,"va":5378167102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33375393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378166912,"raw_hex":"e8a144fd01","size":5,"soft_fail":false,"target_va":5411542422,"va":5378167024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33374817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378166912,"raw_hex":"e86142fd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5378167242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33374571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378166912,"raw_hex":"e86b41fd01","size":5,"soft_fail":false,"target_va":5411541904,"va":5378167328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":11600,"parent_record_offset":11296,"ancestor_record_offset":11296,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":503,"record_offset":11784,"parent_record_offset":11296,"ancestor_record_offset":11296,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":503,"record_offset":11896,"parent_record_offset":11296,"ancestor_record_offset":11296,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":503,"record_offset":11540,"parent_record_offset":11296,"ancestor_record_offset":11296,"inlinee":9187,"source_line":672,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":503,"record_offset":11484,"parent_record_offset":11296,"ancestor_record_offset":11296,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":503,"record_offset":11512,"parent_record_offset":11296,"ancestor_record_offset":11296,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":11664,"parent_record_offset":11640,"ancestor_record_offset":11296,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":11712,"parent_record_offset":11688,"ancestor_record_offset":11296,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":503,"record_offset":11688,"parent_record_offset":11664,"ancestor_record_offset":11296,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":11640,"parent_record_offset":11600,"ancestor_record_offset":11296,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":503,"record_offset":11808,"parent_record_offset":11784,"ancestor_record_offset":11296,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":503,"record_offset":11832,"parent_record_offset":11808,"ancestor_record_offset":11296,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":503,"record_offset":11856,"parent_record_offset":11832,"ancestor_record_offset":11296,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":11752,"parent_record_offset":11600,"ancestor_record_offset":11296,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::trimesh::impl$2::attrib::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/trimesh.rs
//     evidence={"module":503,"record_offset":11568,"parent_record_offset":11540,"ancestor_record_offset":11296,"inlinee":28394,"source_line":127,"archive":"_pdb/B/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::trimesh::impl$2::attrib::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6EEA
// 首条位置: {"module":503,"inlinee":28394}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::trimesh::TrimeshBuilder::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1553,"record_offset":14720,"rva":19523840,"va":5388232960}
// 编译实例 RVAs: 0x129e900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13151598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e8925237ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388233081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13153446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e85a4b37ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388234945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13152385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e87f4f37ff","size":5,"soft_fail":false,"target_va":5375081536,"va":5388233916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088e180
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10555255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e889f05eff","size":5,"soft_fail":false,"target_va":5377679744,"va":5388234994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7065402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e8c63094ff","size":5,"soft_fail":false,"target_va":5381170352,"va":5388235749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dc0a90
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5103846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e81a1fb2ff","size":5,"soft_fail":false,"target_va":5383129744,"va":5388233585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416a2960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4209221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e8453a4000","size":5,"soft_fail":false,"target_va":5392443744,"va":5388234518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: trimesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23613111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e8b74e6801","size":5,"soft_fail":false,"target_va":5411847168,"va":5388234052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13877104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e890402cff","size":5,"soft_fail":false,"target_va":5374356528,"va":5388233627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23305557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e8559d6301","size":5,"soft_fail":false,"target_va":5411540963,"va":5388235401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bytemuck::internal::something_went_wrong
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23243696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e8b0ab6201","size":5,"soft_fail":false,"target_va":5411479040,"va":5388235339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23306424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e8b8a06301","size":5,"soft_fail":false,"target_va":5411541808,"va":5388235379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23307566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e82ea56301","size":5,"soft_fail":false,"target_va":5411542422,"va":5388234851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23307087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e84fa36301","size":5,"soft_fail":false,"target_va":5411542296,"va":5388235204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23308358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e846a86301","size":5,"soft_fail":false,"target_va":5411543584,"va":5388235221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23308032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e800a76301","size":5,"soft_fail":false,"target_va":5411543104,"va":5388235067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23308075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e82ba76301","size":5,"soft_fail":false,"target_va":5411543120,"va":5388235040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::Buffer::device_address
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7119736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e8885c93ff","size":5,"soft_fail":false,"target_va":5381114880,"va":5388234611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7119046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388232960,"raw_hex":"e83a5f93ff","size":5,"soft_fail":false,"target_va":5381115424,"va":5388234465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":16444,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":14988,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":16860,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":15608,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":17524,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":15248,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":18528,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":1553,"record_offset":15704,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1553,"record_offset":16388,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::next_multiple_of
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":15580,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":27473,"source_line":3233,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1553,"record_offset":14952,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":18692,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1553,"record_offset":18492,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1553,"record_offset":18892,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1553,"record_offset":17716,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":18276,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1553,"record_offset":15520,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1553,"record_offset":18612,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1553,"record_offset":16332,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1553,"record_offset":16080,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":13663,"source_line":1280,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1553,"record_offset":15992,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1553,"record_offset":16244,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1553,"record_offset":16416,"parent_record_offset":14720,"ancestor_record_offset":14720,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":15168,"parent_record_offset":15144,"ancestor_record_offset":14720,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":15192,"parent_record_offset":15168,"ancestor_record_offset":14720,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":17196,"parent_record_offset":17172,"ancestor_record_offset":14720,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":18844,"parent_record_offset":18820,"ancestor_record_offset":14720,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":15144,"parent_record_offset":15112,"ancestor_record_offset":14720,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":17148,"parent_record_offset":17040,"ancestor_record_offset":14720,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1553,"record_offset":18820,"parent_record_offset":18764,"ancestor_record_offset":14720,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":15364,"parent_record_offset":15340,"ancestor_record_offset":14720,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":15052,"parent_record_offset":15020,"ancestor_record_offset":14720,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":16948,"parent_record_offset":16904,"ancestor_record_offset":14720,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":18792,"parent_record_offset":18764,"ancestor_record_offset":14720,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":18764,"parent_record_offset":18740,"ancestor_record_offset":14720,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":15412,"parent_record_offset":15388,"ancestor_record_offset":14720,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":15388,"parent_record_offset":15364,"ancestor_record_offset":14720,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":15112,"parent_record_offset":15080,"ancestor_record_offset":14720,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":15080,"parent_record_offset":15052,"ancestor_record_offset":14720,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":16992,"parent_record_offset":16948,"ancestor_record_offset":14720,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":18740,"parent_record_offset":18716,"ancestor_record_offset":14720,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1553,"record_offset":17092,"parent_record_offset":17040,"ancestor_record_offset":14720,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":16484,"parent_record_offset":16444,"ancestor_record_offset":14720,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":18576,"parent_record_offset":18552,"ancestor_record_offset":14720,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":15340,"parent_record_offset":15312,"ancestor_record_offset":14720,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":15312,"parent_record_offset":15276,"ancestor_record_offset":14720,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":15020,"parent_record_offset":14988,"ancestor_record_offset":14720,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":15276,"parent_record_offset":15248,"ancestor_record_offset":14720,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1553,"record_offset":18552,"parent_record_offset":18528,"ancestor_record_offset":14720,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1553,"record_offset":16904,"parent_record_offset":16860,"ancestor_record_offset":14720,"inlinee":5369,"source_line":53,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1553,"record_offset":15736,"parent_record_offset":15704,"ancestor_record_offset":14720,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1553,"record_offset":15772,"parent_record_offset":15736,"ancestor_record_offset":14720,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1553,"record_offset":17116,"parent_record_offset":17092,"ancestor_record_offset":14720,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1553,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":14720,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1553,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":14720,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":17620,"parent_record_offset":17580,"ancestor_record_offset":14720,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1553,"record_offset":19252,"parent_record_offset":19220,"ancestor_record_offset":14720,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1553,"record_offset":16020,"parent_record_offset":15992,"ancestor_record_offset":14720,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1553,"record_offset":16112,"parent_record_offset":16080,"ancestor_record_offset":14720,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1553,"record_offset":16272,"parent_record_offset":16244,"ancestor_record_offset":14720,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":18716,"parent_record_offset":18692,"ancestor_record_offset":14720,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1553,"record_offset":15480,"parent_record_offset":15456,"ancestor_record_offset":14720,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1553,"record_offset":15548,"parent_record_offset":15520,"ancestor_record_offset":14720,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1553,"record_offset":16604,"parent_record_offset":16444,"ancestor_record_offset":14720,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":17672,"parent_record_offset":17648,"ancestor_record_offset":14720,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":18368,"parent_record_offset":18344,"ancestor_record_offset":14720,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":17648,"parent_record_offset":17580,"ancestor_record_offset":14720,"inlinee":5242,"source_line":112,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":18344,"parent_record_offset":18308,"ancestor_record_offset":14720,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":15636,"parent_record_offset":15608,"ancestor_record_offset":14720,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":17552,"parent_record_offset":17524,"ancestor_record_offset":14720,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":15664,"parent_record_offset":15636,"ancestor_record_offset":14720,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":18308,"parent_record_offset":18276,"ancestor_record_offset":14720,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1553,"record_offset":17580,"parent_record_offset":17552,"ancestor_record_offset":14720,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1553,"record_offset":15456,"parent_record_offset":15276,"ancestor_record_offset":14720,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01553.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1553,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":14720,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01553.jsonl.gz"}
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
// 完整结构及原始类型记录: trimesh.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::trimesh::Trimesh
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::trimesh::TrimeshBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::nani::trimesh::TrimeshBuilder>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::nani::trimesh::TrimeshBuilder"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::nani::trimesh::TrimeshBuilder::attrib<glam::f32::vec3::Vec3>", "constructor": "country_core::render::nani::trimesh::TrimeshBuilder::attrib", "arguments": ["glam::f32::vec3::Vec3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::trimesh::TrimeshChunk
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::trimesh::TrimeshChunkBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::render::nani::trimesh::TrimeshChunkBuilder,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::render::nani::trimesh::TrimeshChunkBuilder", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::render::nani::trimesh::TrimeshChunkBuilder,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::render::nani::trimesh::TrimeshChunkBuilder,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::nani::trimesh::TrimeshChunkBuilder>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::nani::trimesh::TrimeshChunkBuilder"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: trimesh.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: trimesh.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::nani::trimesh::Trimesh::builder
// 信息 ID: 24c0db4ca22086fe3c03b707
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::gen_stair_floors @ 0x7dd66b -> 0x129dbd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::trimesh::TrimeshBuilder::attrib<glam::f32::vec3::Vec3>
// 信息 ID: 089a12de8b0cc4e87deb3b24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::nani::trimesh::TrimeshBuilder::attrib -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::trimesh::TrimeshBuilder::positions @ 0x129e8e4 -> 0x905080 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::gen_stair_floors @ 0x7dd6b0 -> 0x12a90c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::trimesh::TrimeshBuilder::build
// 信息 ID: 8552406127cc6593bb0a2e3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::gen_stair_floors @ 0x7dd6d4 -> 0x129e900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::trimesh::TrimeshBuilder::positions
// 信息 ID: 569291f7a1266adefac998b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::gen_stair_floors @ 0x7dd68c -> 0x129e890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::trimesh::TrimeshChunk::index_count
// 信息 ID: b66c6295d3239443fb13a6f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::gen_stair_floors @ 0x7dd7f8 -> 0x129dbc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::trimesh::impl$1::builder::closure$0
// 信息 ID: cf2d660507f239af3ad6c950
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::nani::trimesh::impl$1::builder", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::trimesh::Trimesh::builder; module=1553, inline_record=5848; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::trimesh::Trimesh::builder; module=1553, inline_record=8284; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::trimesh::Trimesh::builder; module=1553, inline_record=10744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::trimesh::impl$1::builder::closure$0::closure$0
// 信息 ID: 993dcec04709c8fd94e701b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::nani::trimesh::impl$1::builder", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::trimesh::Trimesh::builder; module=1553, inline_record=6864; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::trimesh::Trimesh::builder; module=1553, inline_record=9308; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::trimesh::Trimesh::builder; module=1553, inline_record=11760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::trimesh::impl$2::attrib::closure$0
// 信息 ID: 85fb5a8fa4f383953164ce57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::nani::trimesh::impl$2::attrib", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::trimesh::TrimeshBuilder::attrib<glam::f32::vec3::Vec3>; module=503, inline_record=11568; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::trimesh::TrimeshBuilder::attrib<glam::f32::vec3::Vec3>; module=1554, inline_record=536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

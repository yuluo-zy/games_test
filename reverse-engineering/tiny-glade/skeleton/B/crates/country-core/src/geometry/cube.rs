// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/geometry/cube.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 61bdc1d6d07608d4707165c4c2af26ed7e2166ba425d13071b26c554b7f2eb29
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cube.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 486, 491, 500, 537, 540, 1674
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::geometry::cube::impl$2::from
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6E0D
// 首条位置: {"module":500,"inlinee":28173}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":500,"record_offset":39460,"parent_record_offset":39408,"ancestor_record_offset":39408,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] country_core::geometry::cube::Box::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/geometry/cube.rs
//     evidence={"module":500,"record_offset":39432,"parent_record_offset":39408,"ancestor_record_offset":39408,"inlinee":26955,"source_line":43,"archive":"_pdb/B/modules/00500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::geometry::cube::Box::new
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x694B
// 首条位置: {"module":486,"inlinee":26955}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::geometry::cube::impl$5::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":537,"record_offset":56364,"rva":10507568,"va":5379216688}
// 编译实例 RVAs: 0xa05530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: cube.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4136353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379216688,"raw_hex":"e85fe2c0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379217836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: cube.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4137673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379216688,"raw_hex":"e837ddc0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379219172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cube.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1562630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379216688,"raw_hex":"e8fa27e8ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5379220273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34c90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cube.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t192307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379216688,"raw_hex":"e833ef0200","size":5,"soft_fail":false,"target_va":5379411088,"va":5379218776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a355f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cube.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t194559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379216688,"raw_hex":"e8fff70200","size":5,"soft_fail":false,"target_va":5379413488,"va":5379218924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32321505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379216688,"raw_hex":"e8e12fed01","size":5,"soft_fail":false,"target_va":5411540992,"va":5379219482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4861433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379216688,"raw_hex":"e807d2b5ff","size":5,"soft_fail":false,"target_va":5374356720,"va":5379218148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t32321581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379216688,"raw_hex":"e82d30ed01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379219377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3439346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379216688,"raw_hex":"e8f27a3400","size":5,"soft_fail":false,"target_va":5382657856,"va":5379218505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3535231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379216688,"raw_hex":"e87ff13500","size":5,"soft_fail":false,"target_va":5382753792,"va":5379218556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":57892,"parent_record_offset":56364,"ancestor_record_offset":56364,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":57264,"parent_record_offset":56364,"ancestor_record_offset":56364,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":56520,"parent_record_offset":56364,"ancestor_record_offset":56364,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":58776,"parent_record_offset":56364,"ancestor_record_offset":56364,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":59388,"parent_record_offset":56364,"ancestor_record_offset":56364,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":537,"record_offset":57448,"parent_record_offset":56364,"ancestor_record_offset":56364,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":537,"record_offset":58044,"parent_record_offset":56364,"ancestor_record_offset":56364,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":537,"record_offset":59124,"parent_record_offset":56364,"ancestor_record_offset":56364,"inlinee":30837,"source_line":55,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":56688,"parent_record_offset":56664,"ancestor_record_offset":56364,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":56712,"parent_record_offset":56688,"ancestor_record_offset":56364,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":59336,"parent_record_offset":59312,"ancestor_record_offset":56364,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":56664,"parent_record_offset":56636,"ancestor_record_offset":56364,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":59312,"parent_record_offset":59220,"ancestor_record_offset":56364,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":57328,"parent_record_offset":57304,"ancestor_record_offset":56364,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":56576,"parent_record_offset":56548,"ancestor_record_offset":56364,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":59248,"parent_record_offset":59220,"ancestor_record_offset":56364,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":59220,"parent_record_offset":59196,"ancestor_record_offset":56364,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":57376,"parent_record_offset":57352,"ancestor_record_offset":56364,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":57352,"parent_record_offset":57328,"ancestor_record_offset":56364,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":56636,"parent_record_offset":56604,"ancestor_record_offset":56364,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":56604,"parent_record_offset":56576,"ancestor_record_offset":56364,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":59196,"parent_record_offset":59172,"ancestor_record_offset":56364,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":57304,"parent_record_offset":57264,"ancestor_record_offset":56364,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":59060,"parent_record_offset":58808,"ancestor_record_offset":56364,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":56548,"parent_record_offset":56520,"ancestor_record_offset":56364,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":537,"record_offset":58808,"parent_record_offset":58776,"ancestor_record_offset":56364,"inlinee":4768,"source_line":24,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":537,"record_offset":58600,"parent_record_offset":58568,"ancestor_record_offset":56364,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":537,"record_offset":58660,"parent_record_offset":58628,"ancestor_record_offset":56364,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":537,"record_offset":58456,"parent_record_offset":58432,"ancestor_record_offset":56364,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":537,"record_offset":59280,"parent_record_offset":59248,"ancestor_record_offset":56364,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":537,"record_offset":58628,"parent_record_offset":58568,"ancestor_record_offset":56364,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":537,"record_offset":58204,"parent_record_offset":58168,"ancestor_record_offset":56364,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":537,"record_offset":58320,"parent_record_offset":58292,"ancestor_record_offset":56364,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":59148,"parent_record_offset":59124,"ancestor_record_offset":56364,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":537,"record_offset":58408,"parent_record_offset":58292,"ancestor_record_offset":56364,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":537,"record_offset":57476,"parent_record_offset":57448,"ancestor_record_offset":56364,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":58484,"parent_record_offset":58456,"ancestor_record_offset":56364,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":58000,"parent_record_offset":57972,"ancestor_record_offset":56364,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":58432,"parent_record_offset":58408,"ancestor_record_offset":56364,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":57416,"parent_record_offset":57264,"ancestor_record_offset":56364,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":537,"record_offset":58072,"parent_record_offset":58044,"ancestor_record_offset":56364,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":58120,"parent_record_offset":58096,"ancestor_record_offset":56364,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":537,"record_offset":58368,"parent_record_offset":58344,"ancestor_record_offset":56364,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":537,"record_offset":58232,"parent_record_offset":58204,"ancestor_record_offset":56364,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":58568,"parent_record_offset":58168,"ancestor_record_offset":56364,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":58096,"parent_record_offset":58072,"ancestor_record_offset":56364,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":537,"record_offset":58260,"parent_record_offset":58232,"ancestor_record_offset":56364,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":537,"record_offset":58292,"parent_record_offset":58260,"ancestor_record_offset":56364,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":537,"record_offset":58168,"parent_record_offset":58144,"ancestor_record_offset":56364,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":537,"record_offset":58144,"parent_record_offset":58120,"ancestor_record_offset":56364,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00537.jsonl.gz"}
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
// 完整结构及原始类型记录: cube.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::geometry::cube::Box
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::geometry::cube::Cube
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cube.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cube.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >,ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >)
// 候选参数类型: ["ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >", "ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::event::writer::EventWriter<country_core::resources::prefab::PrefabsModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::event::writer::EventWriter<country_core::resources::prefab::PrefabsModifiedCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::prefab::PrefabsModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>,bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>", "bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::prefab::PrefabsModifiedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::render::mesh_atlas_library::MeshAtlasesModifiedCmd>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::geometry::cube::Box::new
// 信息 ID: 24f883689697974c19cb5488
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::spawn_human_ref::spawn_human_ref; module=486, inline_record=38264; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::debug::debug; module=500, inline_record=39432; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::debug::debug; module=500, inline_record=39516; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_ivy::startup_ivy; module=540, inline_record=104072; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::debug::sheep_controls::SheepControl::new; module=1674, inline_record=44840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::geometry::cube::impl$2::from
// 信息 ID: 3dd644d5ba2c45c53c677ec3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::debug::debug; module=500, inline_record=39408; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::debug::debug; module=500, inline_record=39492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::geometry::cube::impl$5::from
// 信息 ID: fededc3a99e7abe278db7e3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::create_prefab_rt @ 0x84170a -> 0xa05530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::spawn_human_ref::spawn_human_ref @ 0x89d5e1 -> 0xa05530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0770 -> 0xa05530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f07aa -> 0xa05530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_ivy::startup_ivy @ 0xa21a8c -> 0xa05530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::debug::sheep_controls::SheepControl::new @ 0x16f0ef2 -> 0xa05530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/geometry/sphere.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e4f55ca435d655a65cc3498d2de3f6843c08dba872af9ad9b60727620c4463aa
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sphere.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 537, 589, 1585, 1610, 1620
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"}]
// use hexasphere; // namespace candidate
//   package identities: [{"name":"hexasphere","source":"registry+https://github.com/rust-lang/crates.io-index","version":"12.0.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::geometry::sphere::Sphere::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":589,"record_offset":118440,"rva":11841376,"va":5380550496}
// 编译实例 RVAs: 0xb4af60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::geometry::sphere::impl$2::try_from
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":537,"record_offset":63272,"rva":10513152,"va":5379222272}
// 编译实例 RVAs: 0xa06b00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4141048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e808d0c0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379222531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4142453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e88bcac0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379223952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140878b40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1631466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e8161be7ff","size":5,"soft_fail":false,"target_va":5377592128,"va":5379223589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1566179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e81d1ae8ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5379223822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t187712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e840dd0200","size":5,"soft_fail":false,"target_va":5379411088,"va":5379223371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a355f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t189969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e811e60200","size":5,"soft_fail":false,"target_va":5379413488,"va":5379223514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413096c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9447910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e8e6299000","size":5,"soft_fail":false,"target_va":5388670656,"va":5379222741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32624288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e8a0cef101","size":5,"soft_fail":false,"target_va":5411847168,"va":5379222875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3617751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e8d7333700","size":5,"soft_fail":false,"target_va":5382840208,"va":5379222452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32317959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e80722ed01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379222999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32318643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e8b324ed01","size":5,"soft_fail":false,"target_va":5411542296,"va":5379223648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hexasphere::Subdivided$LT$T$C$S$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hexasphere","version":"12.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14044108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e8cc4bd600","size":5,"soft_fail":false,"target_va":5393266592,"va":5379222479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hexasphere::Triangle::add_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hexasphere","version":"12.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14224109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e8ed0ad900","size":5,"soft_fail":false,"target_va":5393447232,"va":5379223118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3434726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e8e6683400","size":5,"soft_fail":false,"target_va":5382657856,"va":5379223125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3530611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379222272,"raw_hex":"e873df3500","size":5,"soft_fail":false,"target_va":5382753792,"va":5379223176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":537,"record_offset":63436,"parent_record_offset":63272,"ancestor_record_offset":63272,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":64676,"parent_record_offset":63272,"ancestor_record_offset":63272,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":65308,"parent_record_offset":63272,"ancestor_record_offset":63272,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":537,"record_offset":65688,"parent_record_offset":63272,"ancestor_record_offset":63272,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":537,"record_offset":63532,"parent_record_offset":63272,"ancestor_record_offset":63272,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":66608,"parent_record_offset":63272,"ancestor_record_offset":63272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":537,"record_offset":65876,"parent_record_offset":63272,"ancestor_record_offset":63272,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] hexasphere::Subdivided::get_indices
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hexasphere","version":"12.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hexasphere-12.0.0/src/lib.rs
//     evidence={"module":537,"record_offset":65840,"parent_record_offset":63272,"ancestor_record_offset":63272,"inlinee":30844,"source_line":1138,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_direct] hexasphere::Subdivided::indices_per_main_triangle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hexasphere","version":"12.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hexasphere-12.0.0/src/lib.rs
//     evidence={"module":537,"record_offset":65280,"parent_record_offset":63272,"ancestor_record_offset":63272,"inlinee":30843,"source_line":1240,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":64012,"parent_record_offset":63988,"ancestor_record_offset":63272,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":64036,"parent_record_offset":64012,"ancestor_record_offset":63272,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":66796,"parent_record_offset":66772,"ancestor_record_offset":63272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":63988,"parent_record_offset":63804,"ancestor_record_offset":63272,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":537,"record_offset":66772,"parent_record_offset":66680,"ancestor_record_offset":63272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":537,"record_offset":63492,"parent_record_offset":63464,"ancestor_record_offset":63272,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":63736,"parent_record_offset":63704,"ancestor_record_offset":63272,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":66708,"parent_record_offset":66680,"ancestor_record_offset":63272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":66680,"parent_record_offset":66656,"ancestor_record_offset":63272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":63804,"parent_record_offset":63768,"ancestor_record_offset":63272,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":63768,"parent_record_offset":63736,"ancestor_record_offset":63272,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":66656,"parent_record_offset":66632,"ancestor_record_offset":63272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":537,"record_offset":63848,"parent_record_offset":63804,"ancestor_record_offset":63272,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":64804,"parent_record_offset":64772,"ancestor_record_offset":63272,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":64772,"parent_record_offset":64740,"ancestor_record_offset":63272,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":64708,"parent_record_offset":64676,"ancestor_record_offset":63272,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":537,"record_offset":64740,"parent_record_offset":64708,"ancestor_record_offset":63272,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":64120,"parent_record_offset":64092,"ancestor_record_offset":63272,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":63672,"parent_record_offset":63628,"ancestor_record_offset":63272,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":63704,"parent_record_offset":63672,"ancestor_record_offset":63272,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":63564,"parent_record_offset":63532,"ancestor_record_offset":63272,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":537,"record_offset":64420,"parent_record_offset":64384,"ancestor_record_offset":63272,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":537,"record_offset":64516,"parent_record_offset":64420,"ancestor_record_offset":63272,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":537,"record_offset":64092,"parent_record_offset":63628,"ancestor_record_offset":63272,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":537,"record_offset":63596,"parent_record_offset":63564,"ancestor_record_offset":63272,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":537,"record_offset":63628,"parent_record_offset":63596,"ancestor_record_offset":63272,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":537,"record_offset":63872,"parent_record_offset":63848,"ancestor_record_offset":63272,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":537,"record_offset":63896,"parent_record_offset":63872,"ancestor_record_offset":63272,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":537,"record_offset":66432,"parent_record_offset":66400,"ancestor_record_offset":63272,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":537,"record_offset":66492,"parent_record_offset":66460,"ancestor_record_offset":63272,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":537,"record_offset":65748,"parent_record_offset":65716,"ancestor_record_offset":63272,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":537,"record_offset":64340,"parent_record_offset":64308,"ancestor_record_offset":63272,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":537,"record_offset":65228,"parent_record_offset":65204,"ancestor_record_offset":63272,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":537,"record_offset":64176,"parent_record_offset":64148,"ancestor_record_offset":63272,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":537,"record_offset":64244,"parent_record_offset":64204,"ancestor_record_offset":63272,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":537,"record_offset":65776,"parent_record_offset":65716,"ancestor_record_offset":63272,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":537,"record_offset":65716,"parent_record_offset":65688,"ancestor_record_offset":63272,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":537,"record_offset":64148,"parent_record_offset":64120,"ancestor_record_offset":63272,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":537,"record_offset":64384,"parent_record_offset":64244,"ancestor_record_offset":63272,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":537,"record_offset":66288,"parent_record_offset":66264,"ancestor_record_offset":63272,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":537,"record_offset":63920,"parent_record_offset":63896,"ancestor_record_offset":63272,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":537,"record_offset":63944,"parent_record_offset":63920,"ancestor_record_offset":63272,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":537,"record_offset":65800,"parent_record_offset":65776,"ancestor_record_offset":63272,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":537,"record_offset":66740,"parent_record_offset":66708,"ancestor_record_offset":63272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":537,"record_offset":66460,"parent_record_offset":66400,"ancestor_record_offset":63272,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":537,"record_offset":66036,"parent_record_offset":66000,"ancestor_record_offset":63272,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":537,"record_offset":63464,"parent_record_offset":63436,"ancestor_record_offset":63272,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":537,"record_offset":66152,"parent_record_offset":66124,"ancestor_record_offset":63272,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":537,"record_offset":65204,"parent_record_offset":64804,"ancestor_record_offset":63272,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":66632,"parent_record_offset":66608,"ancestor_record_offset":63272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":537,"record_offset":66240,"parent_record_offset":66124,"ancestor_record_offset":63272,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":537,"record_offset":64556,"parent_record_offset":64204,"ancestor_record_offset":63272,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":66316,"parent_record_offset":66288,"ancestor_record_offset":63272,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":66264,"parent_record_offset":66240,"ancestor_record_offset":63272,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":537,"record_offset":64464,"parent_record_offset":64420,"ancestor_record_offset":63272,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":537,"record_offset":64204,"parent_record_offset":64176,"ancestor_record_offset":63272,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] country_core::geometry::sphere::impl$2::try_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/geometry/sphere.rs
//     evidence={"module":537,"record_offset":64276,"parent_record_offset":64244,"ancestor_record_offset":63272,"inlinee":30839,"source_line":82,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":537,"record_offset":64308,"parent_record_offset":64276,"ancestor_record_offset":63272,"inlinee":30841,"source_line":1054,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":537,"record_offset":65904,"parent_record_offset":65876,"ancestor_record_offset":63272,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":65952,"parent_record_offset":65928,"ancestor_record_offset":63272,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":537,"record_offset":66200,"parent_record_offset":66176,"ancestor_record_offset":63272,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":537,"record_offset":66064,"parent_record_offset":66036,"ancestor_record_offset":63272,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":66400,"parent_record_offset":66000,"ancestor_record_offset":63272,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":537,"record_offset":65928,"parent_record_offset":65904,"ancestor_record_offset":63272,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":537,"record_offset":66092,"parent_record_offset":66064,"ancestor_record_offset":63272,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":537,"record_offset":66124,"parent_record_offset":66092,"ancestor_record_offset":63272,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":537,"record_offset":66000,"parent_record_offset":65976,"ancestor_record_offset":63272,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":537,"record_offset":65976,"parent_record_offset":65952,"ancestor_record_offset":63272,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::geometry::sphere::impl$2::try_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AC9
// 首条位置: {"module":1585,"inlinee":72393}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::acos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1585,"record_offset":352904,"parent_record_offset":352868,"ancestor_record_offset":352868,"inlinee":31906,"source_line":760,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1585,"record_offset":352932,"parent_record_offset":352868,"ancestor_record_offset":352868,"inlinee":14897,"source_line":830,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::geometry::sphere::impl$2::try_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7877
// 首条位置: {"module":537,"inlinee":30839}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":537,"record_offset":64308,"parent_record_offset":64276,"ancestor_record_offset":64276,"inlinee":30841,"source_line":1054,"archive":"_pdb/B/modules/00537.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":537,"record_offset":64340,"parent_record_offset":64308,"ancestor_record_offset":64276,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00537.jsonl.gz"}
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
// 完整结构及原始类型记录: sphere.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::geometry::sphere::Sphere
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sphere.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sphere.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 17; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd> >,country_core::systems::paths::draw_path::PathRastersResources)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "country_core::systems::world_raster::RasterDrawResources", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd> >", "country_core::systems::paths::draw_path::PathRastersResources"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<utils::components::transform::Transform>,bevy_ecs::query::filter::With<country_core::systems::paths::path_brush_preview_size::SetSizeBasedOnAppModePath> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::Sphere::new
// 信息 ID: 6b2404c5bec054e8a704cf9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x7b6c0e -> 0xb4af60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::impl$2::try_from
// 信息 ID: 335b5a04b6aab68fc0ecc68d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x7b6c1c -> 0xa06b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0b76 -> 0xa06b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0c4c -> 0xa06b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0d1e -> 0xa06b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0df5 -> 0xa06b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0x8f0ecc -> 0xa06b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::spawn_brush_previews::spawn_brush_previews @ 0x925ca0 -> 0xa06b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0x9d50b5 -> 0xa06b00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::impl$2::try_from::closure$0
// 信息 ID: b0fb8cae04eddcdf23c7f12f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::geometry::sphere::impl$2::try_from", "captures": "unknown"}
// [闭包环境类型引用] country_core::geometry::sphere::impl$2::try_from::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,2>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec3a::Vec3A> >,country_core::geometry::sphere::impl$2::try_from::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::geometry::sphere::impl$2::try_from::closure_env$0 <- hexasphere::Subdivided::new<array$<f32,2>,hexasphere::shapes::IcoSphereBase,country_core::geometry::sphere::impl$2::try_from::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,2>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec3a::Vec3A> >,country_core::geometry::sphere::impl$2::try_from::closure_env$0> >; module=1585, inline_record=352868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::impl$2::try_from::closure$1
// 信息 ID: f9ce92f81af95032de123b50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::geometry::sphere::impl$2::try_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::geometry::sphere::impl$2::try_from; module=537, inline_record=64276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

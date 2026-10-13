// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/geometry/sphere.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e4f55ca435d655a65cc3498d2de3f6843c08dba872af9ad9b60727620c4463aa
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sphere.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 528, 582, 596, 1643, 1682, 1687
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
// 首条位置: {"module":528,"record_offset":4940,"rva":10505744,"va":5379214864}
// 编译实例 RVAs: 0xa04e10
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
// 首条位置: {"module":596,"record_offset":57048,"rva":12775152,"va":5381484272}
// 编译实例 RVAs: 0xc2eef0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3159632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e850363000","size":5,"soft_fail":false,"target_va":5384644912,"va":5381485275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-5805828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e8fc68a7ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381484511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-5807349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e80b63a7ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381486048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12762311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e839433dff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381484498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140971930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2874196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e8ac24d4ff","size":5,"soft_fail":false,"target_va":5378611504,"va":5381485695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2750210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e8fe08d6ff","size":5,"soft_fail":false,"target_va":5378736416,"va":5381486621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac7970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1473227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e83585e9ff","size":5,"soft_fail":false,"target_va":5380012400,"va":5381485622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac8270
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1470767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e8d18ee9ff","size":5,"soft_fail":false,"target_va":5380014704,"va":5381485466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d79360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34907083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e8cba31402","size":5,"soft_fail":false,"target_va":5416391520,"va":5381484432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35572075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e86bc91e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5381484976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3310454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e876833200","size":5,"soft_fail":false,"target_va":5384794864,"va":5381484405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35135708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e8dc201802","size":5,"soft_fail":false,"target_va":5416620794,"va":5381485081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35135951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e8cf211802","size":5,"soft_fail":false,"target_va":5416621762,"va":5381485806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hexasphere::Triangle::add_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hexasphere","version":"12.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17009837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e8ad8c0301","size":5,"soft_fail":false,"target_va":5398495088,"va":5381485246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3240936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381484272,"raw_hex":"e8e8733100","size":5,"soft_fail":false,"target_va":5384726688,"va":5381485747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":596,"record_offset":57212,"parent_record_offset":57048,"ancestor_record_offset":57048,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":596,"record_offset":59240,"parent_record_offset":57048,"ancestor_record_offset":57048,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":59912,"parent_record_offset":57048,"ancestor_record_offset":57048,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":596,"record_offset":60332,"parent_record_offset":57048,"ancestor_record_offset":57048,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":596,"record_offset":57292,"parent_record_offset":57048,"ancestor_record_offset":57048,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":61244,"parent_record_offset":57048,"ancestor_record_offset":57048,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":596,"record_offset":60520,"parent_record_offset":57048,"ancestor_record_offset":57048,"inlinee":30455,"source_line":40,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] hexasphere::Subdivided::get_indices
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hexasphere","version":"12.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hexasphere-12.0.0/src/lib.rs
//     evidence={"module":596,"record_offset":60484,"parent_record_offset":57048,"ancestor_record_offset":57048,"inlinee":37128,"source_line":1138,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] hexasphere::Subdivided::indices_per_main_triangle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hexasphere","version":"12.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hexasphere-12.0.0/src/lib.rs
//     evidence={"module":596,"record_offset":59884,"parent_record_offset":57048,"ancestor_record_offset":57048,"inlinee":37971,"source_line":1240,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":57712,"parent_record_offset":57688,"ancestor_record_offset":57048,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":57736,"parent_record_offset":57712,"ancestor_record_offset":57048,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":61432,"parent_record_offset":61408,"ancestor_record_offset":57048,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":61456,"parent_record_offset":61432,"ancestor_record_offset":57048,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":57760,"parent_record_offset":57736,"ancestor_record_offset":57048,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":61480,"parent_record_offset":61456,"ancestor_record_offset":57048,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":57688,"parent_record_offset":57540,"ancestor_record_offset":57048,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":596,"record_offset":61408,"parent_record_offset":61316,"ancestor_record_offset":57048,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":596,"record_offset":57260,"parent_record_offset":57236,"ancestor_record_offset":57048,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":57480,"parent_record_offset":57452,"ancestor_record_offset":57048,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":61344,"parent_record_offset":61316,"ancestor_record_offset":57048,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":61316,"parent_record_offset":61292,"ancestor_record_offset":57048,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":57540,"parent_record_offset":57508,"ancestor_record_offset":57048,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":57508,"parent_record_offset":57480,"ancestor_record_offset":57048,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":61292,"parent_record_offset":61268,"ancestor_record_offset":57048,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":57576,"parent_record_offset":57540,"ancestor_record_offset":57048,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":596,"record_offset":59280,"parent_record_offset":59240,"ancestor_record_offset":57048,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":596,"record_offset":59320,"parent_record_offset":59280,"ancestor_record_offset":57048,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":596,"record_offset":59360,"parent_record_offset":59320,"ancestor_record_offset":57048,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":57844,"parent_record_offset":57820,"ancestor_record_offset":57048,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":57424,"parent_record_offset":57388,"ancestor_record_offset":57048,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":57452,"parent_record_offset":57424,"ancestor_record_offset":57048,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":57324,"parent_record_offset":57292,"ancestor_record_offset":57048,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":58236,"parent_record_offset":58200,"ancestor_record_offset":57048,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":596,"record_offset":58332,"parent_record_offset":58236,"ancestor_record_offset":57048,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":596,"record_offset":57820,"parent_record_offset":57388,"ancestor_record_offset":57048,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":596,"record_offset":57356,"parent_record_offset":57324,"ancestor_record_offset":57048,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":596,"record_offset":57388,"parent_record_offset":57356,"ancestor_record_offset":57048,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":596,"record_offset":57600,"parent_record_offset":57576,"ancestor_record_offset":57048,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":596,"record_offset":61096,"parent_record_offset":61068,"ancestor_record_offset":57048,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":596,"record_offset":60996,"parent_record_offset":60792,"ancestor_record_offset":57048,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":596,"record_offset":60392,"parent_record_offset":60360,"ancestor_record_offset":57048,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":596,"record_offset":58016,"parent_record_offset":57984,"ancestor_record_offset":57048,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":596,"record_offset":60160,"parent_record_offset":60104,"ancestor_record_offset":57048,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":596,"record_offset":59004,"parent_record_offset":58964,"ancestor_record_offset":57048,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":596,"record_offset":58940,"parent_record_offset":58916,"ancestor_record_offset":57048,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":596,"record_offset":57892,"parent_record_offset":57868,"ancestor_record_offset":57048,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":596,"record_offset":57952,"parent_record_offset":57916,"ancestor_record_offset":57048,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":596,"record_offset":60420,"parent_record_offset":60360,"ancestor_record_offset":57048,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":596,"record_offset":60360,"parent_record_offset":60332,"ancestor_record_offset":57048,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":596,"record_offset":57868,"parent_record_offset":57844,"ancestor_record_offset":57048,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":596,"record_offset":58200,"parent_record_offset":57952,"ancestor_record_offset":57048,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":596,"record_offset":58080,"parent_record_offset":58048,"ancestor_record_offset":57048,"inlinee":12957,"source_line":703,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":596,"record_offset":60960,"parent_record_offset":60936,"ancestor_record_offset":57048,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":596,"record_offset":57624,"parent_record_offset":57600,"ancestor_record_offset":57048,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":596,"record_offset":57648,"parent_record_offset":57624,"ancestor_record_offset":57048,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":596,"record_offset":60444,"parent_record_offset":60420,"ancestor_record_offset":57048,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":596,"record_offset":61376,"parent_record_offset":61344,"ancestor_record_offset":57048,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":596,"record_offset":61124,"parent_record_offset":61068,"ancestor_record_offset":57048,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":596,"record_offset":60704,"parent_record_offset":60668,"ancestor_record_offset":57048,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":596,"record_offset":57236,"parent_record_offset":57212,"ancestor_record_offset":57048,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":596,"record_offset":60824,"parent_record_offset":60792,"ancestor_record_offset":57048,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":596,"record_offset":59812,"parent_record_offset":59360,"ancestor_record_offset":57048,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":596,"record_offset":58112,"parent_record_offset":58080,"ancestor_record_offset":57048,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":59836,"parent_record_offset":59812,"ancestor_record_offset":57048,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":61268,"parent_record_offset":61244,"ancestor_record_offset":57048,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":60912,"parent_record_offset":60792,"ancestor_record_offset":57048,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":596,"record_offset":58372,"parent_record_offset":57916,"ancestor_record_offset":57048,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":58144,"parent_record_offset":58112,"ancestor_record_offset":57048,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":60936,"parent_record_offset":60912,"ancestor_record_offset":57048,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":58280,"parent_record_offset":58236,"ancestor_record_offset":57048,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":596,"record_offset":57916,"parent_record_offset":57892,"ancestor_record_offset":57048,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] country_core::geometry::sphere::impl$2::try_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/geometry/sphere.rs
//     evidence={"module":596,"record_offset":57984,"parent_record_offset":57952,"ancestor_record_offset":57048,"inlinee":37968,"source_line":82,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$42::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":596,"record_offset":58048,"parent_record_offset":58016,"ancestor_record_offset":57048,"inlinee":37970,"source_line":1325,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":596,"record_offset":60556,"parent_record_offset":60520,"ancestor_record_offset":57048,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":596,"record_offset":60612,"parent_record_offset":60584,"ancestor_record_offset":57048,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":596,"record_offset":60872,"parent_record_offset":60848,"ancestor_record_offset":57048,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":596,"record_offset":60732,"parent_record_offset":60704,"ancestor_record_offset":57048,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":596,"record_offset":61068,"parent_record_offset":60668,"ancestor_record_offset":57048,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":596,"record_offset":60584,"parent_record_offset":60556,"ancestor_record_offset":57048,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":596,"record_offset":60760,"parent_record_offset":60732,"ancestor_record_offset":57048,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":596,"record_offset":60792,"parent_record_offset":60760,"ancestor_record_offset":57048,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":596,"record_offset":60668,"parent_record_offset":60640,"ancestor_record_offset":57048,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":596,"record_offset":60640,"parent_record_offset":60612,"ancestor_record_offset":57048,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::geometry::sphere::impl$2::try_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x90F1
// 首条位置: {"module":582,"inlinee":37105}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::acos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":582,"record_offset":9008,"parent_record_offset":8972,"ancestor_record_offset":8972,"inlinee":34828,"source_line":808,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":582,"record_offset":9036,"parent_record_offset":8972,"ancestor_record_offset":8972,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/00582.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::geometry::sphere::impl$2::try_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9450
// 首条位置: {"module":596,"inlinee":37968}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":596,"record_offset":58016,"parent_record_offset":57984,"ancestor_record_offset":57984,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":596,"record_offset":58080,"parent_record_offset":58048,"ancestor_record_offset":57984,"inlinee":12957,"source_line":703,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":596,"record_offset":58112,"parent_record_offset":58080,"ancestor_record_offset":57984,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":58144,"parent_record_offset":58112,"ancestor_record_offset":57984,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$42::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":596,"record_offset":58048,"parent_record_offset":58016,"ancestor_record_offset":57984,"inlinee":37970,"source_line":1325,"archive":"_pdb/C/modules/00596.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 37; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] relative_path::relative_path_buf::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::Sphere::new
// 信息 ID: b7b69d21e8a3a8eef03b5d36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x872a5c -> 0xa04e10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::impl$2::try_from
// 信息 ID: ed7e63db539a22b0479f9e9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x872a6a -> 0xc2eef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0xa8415f -> 0xc2eef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0xba60ec -> 0xc2eef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0xba6278 -> 0xc2eef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0xba63f8 -> 0xc2eef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0xba6578 -> 0xc2eef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0xba66f8 -> 0xc2eef0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::spawn_brush_previews::spawn_brush_previews @ 0x15148e2 -> 0xc2eef0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::impl$2::try_from::closure$0
// 信息 ID: b9b46f41ee0047b989f19aaf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::geometry::sphere::impl$2::try_from", "captures": "unknown"}
// [闭包环境类型引用] country_core::geometry::sphere::impl$2::try_from::closure_env$0 <- hexasphere::Subdivided::new<array$<f32,2>,hexasphere::shapes::IcoSphereBase,country_core::geometry::sphere::impl$2::try_from::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] hexasphere::Subdivided::new<array$<f32,2>,hexasphere::shapes::IcoSphereBase,country_core::geometry::sphere::impl$2::try_from::closure_env$0>; module=582, inline_record=8972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::impl$2::try_from::closure$1
// 信息 ID: 6e9e5979a98bae0af9d1890b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::geometry::sphere::impl$2::try_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::geometry::sphere::impl$2::try_from; module=596, inline_record=57984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

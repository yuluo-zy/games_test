// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/walls/wall_path_segment.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d40ae4b030fb5d2eac50a767cbe25adf95aff8ae8e4e46900913af3e95ca6569
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_path_segment.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 396, 1451
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FEC
// 首条位置: {"module":396,"inlinee":24556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":396,"record_offset":16344,"parent_record_offset":16280,"ancestor_record_offset":16280,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":396,"record_offset":18172,"parent_record_offset":16280,"ancestor_record_offset":16280,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$13::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":396,"record_offset":18144,"parent_record_offset":16280,"ancestor_record_offset":16280,"inlinee":24558,"source_line":49,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":16800,"parent_record_offset":16776,"ancestor_record_offset":16280,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":16824,"parent_record_offset":16800,"ancestor_record_offset":16280,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":18376,"parent_record_offset":18348,"ancestor_record_offset":16280,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":16776,"parent_record_offset":16600,"ancestor_record_offset":16280,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":396,"record_offset":18348,"parent_record_offset":18248,"ancestor_record_offset":16280,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":16540,"parent_record_offset":16512,"ancestor_record_offset":16280,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":18280,"parent_record_offset":18248,"ancestor_record_offset":16280,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":18248,"parent_record_offset":18220,"ancestor_record_offset":16280,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":16600,"parent_record_offset":16568,"ancestor_record_offset":16280,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":16568,"parent_record_offset":16540,"ancestor_record_offset":16280,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":18220,"parent_record_offset":18196,"ancestor_record_offset":16280,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":396,"record_offset":16636,"parent_record_offset":16600,"ancestor_record_offset":16280,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":16476,"parent_record_offset":16444,"ancestor_record_offset":16280,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":396,"record_offset":16444,"parent_record_offset":16376,"ancestor_record_offset":16280,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":396,"record_offset":16416,"parent_record_offset":16376,"ancestor_record_offset":16280,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":396,"record_offset":16512,"parent_record_offset":16476,"ancestor_record_offset":16280,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":396,"record_offset":16376,"parent_record_offset":16344,"ancestor_record_offset":16280,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":396,"record_offset":16660,"parent_record_offset":16636,"ancestor_record_offset":16280,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":396,"record_offset":16684,"parent_record_offset":16660,"ancestor_record_offset":16280,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":396,"record_offset":16900,"parent_record_offset":16876,"ancestor_record_offset":16280,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":396,"record_offset":16708,"parent_record_offset":16684,"ancestor_record_offset":16280,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":396,"record_offset":16732,"parent_record_offset":16708,"ancestor_record_offset":16280,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":396,"record_offset":18312,"parent_record_offset":18280,"ancestor_record_offset":16280,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":396,"record_offset":16876,"parent_record_offset":16476,"ancestor_record_offset":16280,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":396,"record_offset":18196,"parent_record_offset":18172,"ancestor_record_offset":16280,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::WallPathSegment::pt_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1451,"record_offset":5368,"rva":19089776,"va":5387798896}
// 编译实例 RVAs: 0x1234970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::loop_range::LoopRange::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t-214569","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387798896,"raw_hex":"e9d7b9fcff","size":5,"soft_fail":false,"target_va":5387584336,"va":5387798900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1451,"record_offset":5520,"rva":19089792,"va":5387798912}
// 编译实例 RVAs: 0x1234980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":5920,"parent_record_offset":5520,"ancestor_record_offset":5520,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1451,"record_offset":5688,"parent_record_offset":5520,"ancestor_record_offset":5520,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1451,"record_offset":6048,"parent_record_offset":5964,"ancestor_record_offset":5520,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":5844,"parent_record_offset":5816,"ancestor_record_offset":5520,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":5884,"parent_record_offset":5728,"ancestor_record_offset":5520,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":5816,"parent_record_offset":5788,"ancestor_record_offset":5520,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":5728,"parent_record_offset":5688,"ancestor_record_offset":5520,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":5788,"parent_record_offset":5728,"ancestor_record_offset":5520,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":6072,"parent_record_offset":5964,"ancestor_record_offset":5520,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1451,"record_offset":5964,"parent_record_offset":5920,"ancestor_record_offset":5520,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1451,"record_offset":6156,"parent_record_offset":6072,"ancestor_record_offset":5520,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws_2d
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1451,"record_offset":6284,"rva":19090128,"va":5387799248}
// 编译实例 RVAs: 0x1234ad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1451,"record_offset":6660,"parent_record_offset":6284,"ancestor_record_offset":6284,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1451,"record_offset":6452,"parent_record_offset":6284,"ancestor_record_offset":6284,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1451,"record_offset":6768,"parent_record_offset":6696,"ancestor_record_offset":6284,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":6588,"parent_record_offset":6564,"ancestor_record_offset":6284,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1451,"record_offset":6624,"parent_record_offset":6488,"ancestor_record_offset":6284,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":6284,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":6488,"parent_record_offset":6452,"ancestor_record_offset":6284,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1451,"record_offset":6540,"parent_record_offset":6488,"ancestor_record_offset":6284,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1451,"record_offset":6804,"parent_record_offset":6696,"ancestor_record_offset":6284,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1451,"record_offset":6696,"parent_record_offset":6660,"ancestor_record_offset":6284,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1451,"record_offset":6884,"parent_record_offset":6804,"ancestor_record_offset":6284,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01451.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_path_segment.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::wall_path_segment::WallPathSegment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::WallId", "alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::wall_path_segment::WallPathSegment>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_path_segment.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/walls/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/walls/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/walls/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_path_segment.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws
// 信息 ID: 088931f8e136bf9d17fc67be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x760f3a -> 0x1234980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x775515 -> 0x1234980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7755b8 -> 0x1234980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws_2d
// 信息 ID: 0fc3afc4027020c7047fb617
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7755c4 -> 0x1234ad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x777808 -> 0x1234ad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::WallPathSegment::pt_count
// 信息 ID: b330236c3c56f26e6ecbb952
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7613ba -> 0x1234970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x760f02 -> 0x1234970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x760f88 -> 0x1234970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x760fc4 -> 0x1234970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x760fef -> 0x1234970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x761023 -> 0x1234970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x761058 -> 0x1234970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x76ed79 -> 0x1234970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x76edb7 -> 0x1234970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x76eded -> 0x1234970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x76ee21 -> 0x1234970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x76ee56 -> 0x1234970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::impl$2::clone
// 信息 ID: ab15f24460d4dc6d0b5f60d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=396, inline_record=16280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

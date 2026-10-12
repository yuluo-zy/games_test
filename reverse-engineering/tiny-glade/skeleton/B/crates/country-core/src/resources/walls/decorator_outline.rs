// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/walls/decorator_outline.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 08404d80f8a320f42f6dee48e901b156c6b314b42a48449ce5f797e22913f434
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_outline.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 585, 1585, 1610
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":585,"record_offset":17340,"rva":11745568,"va":5380454688}
// 编译实例 RVAs: 0xb33920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5373427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380454688,"raw_hex":"e80d02aeff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380454910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31085800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380454688,"raw_hex":"e8e854da01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380455158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-191325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380454688,"raw_hex":"e8a314fdff","size":5,"soft_fail":false,"target_va":5380263792,"va":5380455112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-201868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380454688,"raw_hex":"e874ebfcff","size":5,"soft_fail":false,"target_va":5380252960,"va":5380454823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":585,"record_offset":17548,"parent_record_offset":17340,"ancestor_record_offset":17340,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::line_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":585,"record_offset":18976,"parent_record_offset":17340,"ancestor_record_offset":17340,"inlinee":30066,"source_line":407,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugSphere::radius
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":585,"record_offset":17520,"parent_record_offset":17340,"ancestor_record_offset":17340,"inlinee":29721,"source_line":100,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":18128,"parent_record_offset":18104,"ancestor_record_offset":17340,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":18152,"parent_record_offset":18128,"ancestor_record_offset":17340,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":18104,"parent_record_offset":17928,"ancestor_record_offset":17340,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":17868,"parent_record_offset":17840,"ancestor_record_offset":17340,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":17928,"parent_record_offset":17896,"ancestor_record_offset":17340,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":17896,"parent_record_offset":17868,"ancestor_record_offset":17340,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":17964,"parent_record_offset":17928,"ancestor_record_offset":17340,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":18232,"parent_record_offset":18208,"ancestor_record_offset":17340,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":17812,"parent_record_offset":17632,"ancestor_record_offset":17340,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":17840,"parent_record_offset":17812,"ancestor_record_offset":17340,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":17576,"parent_record_offset":17548,"ancestor_record_offset":17340,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":18876,"parent_record_offset":18852,"ancestor_record_offset":17340,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":585,"record_offset":18208,"parent_record_offset":17632,"ancestor_record_offset":17340,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":585,"record_offset":17604,"parent_record_offset":17576,"ancestor_record_offset":17340,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":585,"record_offset":17632,"parent_record_offset":17604,"ancestor_record_offset":17340,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":585,"record_offset":17988,"parent_record_offset":17964,"ancestor_record_offset":17340,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":585,"record_offset":18012,"parent_record_offset":17988,"ancestor_record_offset":17340,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":585,"record_offset":18516,"parent_record_offset":18420,"ancestor_record_offset":17340,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":585,"record_offset":18488,"parent_record_offset":18420,"ancestor_record_offset":17340,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":585,"record_offset":18456,"parent_record_offset":18420,"ancestor_record_offset":17340,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":585,"record_offset":18060,"parent_record_offset":18036,"ancestor_record_offset":17340,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":585,"record_offset":18280,"parent_record_offset":18256,"ancestor_record_offset":17340,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":585,"record_offset":17672,"parent_record_offset":17632,"ancestor_record_offset":17340,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":585,"record_offset":18332,"parent_record_offset":18304,"ancestor_record_offset":17340,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":585,"record_offset":18256,"parent_record_offset":18232,"ancestor_record_offset":17340,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":585,"record_offset":18852,"parent_record_offset":18332,"ancestor_record_offset":17340,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":585,"record_offset":18036,"parent_record_offset":18012,"ancestor_record_offset":17340,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":585,"record_offset":17768,"parent_record_offset":17744,"ancestor_record_offset":17340,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":585,"record_offset":17744,"parent_record_offset":17720,"ancestor_record_offset":17340,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":585,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":17340,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":585,"record_offset":18900,"parent_record_offset":18876,"ancestor_record_offset":17340,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":585,"record_offset":18304,"parent_record_offset":18280,"ancestor_record_offset":17340,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":585,"record_offset":17696,"parent_record_offset":17672,"ancestor_record_offset":17340,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_outline::impl$0::display::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_outline.rs
//     evidence={"module":585,"record_offset":18356,"parent_record_offset":18332,"ancestor_record_offset":17340,"inlinee":33334,"source_line":32,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":585,"record_offset":18380,"parent_record_offset":18356,"ancestor_record_offset":17340,"inlinee":25215,"source_line":372,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":585,"record_offset":18420,"parent_record_offset":18380,"ancestor_record_offset":17340,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":585,"record_offset":18732,"parent_record_offset":18380,"ancestor_record_offset":17340,"inlinee":24484,"source_line":1323,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$0::display::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8236
// 首条位置: {"module":585,"inlinee":33334}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":585,"record_offset":18380,"parent_record_offset":18356,"ancestor_record_offset":18356,"inlinee":25215,"source_line":372,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":585,"record_offset":18516,"parent_record_offset":18420,"ancestor_record_offset":18356,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":585,"record_offset":18488,"parent_record_offset":18420,"ancestor_record_offset":18356,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":585,"record_offset":18456,"parent_record_offset":18420,"ancestor_record_offset":18356,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":585,"record_offset":18420,"parent_record_offset":18380,"ancestor_record_offset":18356,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":585,"record_offset":18732,"parent_record_offset":18380,"ancestor_record_offset":18356,"inlinee":24484,"source_line":1323,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::DecoratorOutline::display
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":585,"record_offset":19008,"rva":11746048,"va":5380455168}
// 编译实例 RVAs: 0xb33b00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2391944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380455168,"raw_hex":"e8887f2400","size":5,"soft_fail":false,"target_va":5382847888,"va":5380455939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5374512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380455168,"raw_hex":"e8d0fdadff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380455995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5375292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380455168,"raw_hex":"e8c4faadff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380456791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064f9d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5130813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380455168,"raw_hex":"e8c3b5b1ff","size":5,"soft_fail":false,"target_va":5375326672,"va":5380457480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146c880
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9668519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380455168,"raw_hex":"e8a7879300","size":5,"soft_fail":false,"target_va":5390125184,"va":5380456660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31390727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380455168,"raw_hex":"e807fcde01","size":5,"soft_fail":false,"target_va":5411847168,"va":5380456436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31084093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380455168,"raw_hex":"e83d4eda01","size":5,"soft_fail":false,"target_va":5411540992,"va":5380456894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2384298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380455168,"raw_hex":"e8aa612400","size":5,"soft_fail":false,"target_va":5382840208,"va":5380455905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31084084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380455168,"raw_hex":"e8344eda01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380456874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":585,"record_offset":20200,"parent_record_offset":19008,"ancestor_record_offset":19008,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":585,"record_offset":20284,"parent_record_offset":19008,"ancestor_record_offset":19008,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":21704,"parent_record_offset":19008,"ancestor_record_offset":19008,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":585,"record_offset":19208,"parent_record_offset":19008,"ancestor_record_offset":19008,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":585,"record_offset":24916,"parent_record_offset":19008,"ancestor_record_offset":19008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorId::get_inner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":585,"record_offset":20172,"parent_record_offset":19008,"ancestor_record_offset":19008,"inlinee":25032,"source_line":166,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":585,"record_offset":20800,"parent_record_offset":19008,"ancestor_record_offset":19008,"inlinee":26169,"source_line":112,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":585,"record_offset":24832,"parent_record_offset":19008,"ancestor_record_offset":19008,"inlinee":26171,"source_line":24,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_affine3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":585,"record_offset":19292,"parent_record_offset":19008,"ancestor_record_offset":19008,"inlinee":33335,"source_line":776,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":585,"record_offset":19996,"parent_record_offset":19008,"ancestor_record_offset":19008,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":585,"record_offset":20084,"parent_record_offset":19008,"ancestor_record_offset":19008,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":20412,"parent_record_offset":20388,"ancestor_record_offset":19008,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":20436,"parent_record_offset":20412,"ancestor_record_offset":19008,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":20732,"parent_record_offset":20708,"ancestor_record_offset":19008,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":20356,"parent_record_offset":20320,"ancestor_record_offset":19008,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":20388,"parent_record_offset":20356,"ancestor_record_offset":19008,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":20708,"parent_record_offset":20652,"ancestor_record_offset":19008,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":585,"record_offset":20320,"parent_record_offset":20284,"ancestor_record_offset":19008,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":585,"record_offset":20248,"parent_record_offset":20224,"ancestor_record_offset":19008,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":21128,"parent_record_offset":21100,"ancestor_record_offset":19008,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":20680,"parent_record_offset":20652,"ancestor_record_offset":19008,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":20652,"parent_record_offset":20628,"ancestor_record_offset":19008,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":21188,"parent_record_offset":21156,"ancestor_record_offset":19008,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":21156,"parent_record_offset":21128,"ancestor_record_offset":19008,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":20628,"parent_record_offset":20604,"ancestor_record_offset":19008,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":21224,"parent_record_offset":21188,"ancestor_record_offset":19008,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":21060,"parent_record_offset":21024,"ancestor_record_offset":19008,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":21024,"parent_record_offset":20988,"ancestor_record_offset":19008,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":20952,"parent_record_offset":20916,"ancestor_record_offset":19008,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":20988,"parent_record_offset":20952,"ancestor_record_offset":19008,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":20916,"parent_record_offset":20880,"ancestor_record_offset":19008,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":585,"record_offset":20880,"parent_record_offset":20844,"ancestor_record_offset":19008,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":585,"record_offset":20844,"parent_record_offset":20800,"ancestor_record_offset":19008,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":585,"record_offset":21572,"parent_record_offset":21548,"ancestor_record_offset":19008,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":21784,"parent_record_offset":21740,"ancestor_record_offset":19008,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":21100,"parent_record_offset":21060,"ancestor_record_offset":19008,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":21740,"parent_record_offset":21704,"ancestor_record_offset":19008,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":585,"record_offset":21248,"parent_record_offset":21224,"ancestor_record_offset":19008,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":585,"record_offset":21272,"parent_record_offset":21248,"ancestor_record_offset":19008,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":585,"record_offset":19340,"parent_record_offset":19292,"ancestor_record_offset":19008,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":585,"record_offset":21488,"parent_record_offset":21464,"ancestor_record_offset":19008,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":585,"record_offset":21320,"parent_record_offset":21296,"ancestor_record_offset":19008,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":585,"record_offset":19256,"parent_record_offset":19232,"ancestor_record_offset":19008,"inlinee":11332,"source_line":650,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":585,"record_offset":21296,"parent_record_offset":21272,"ancestor_record_offset":19008,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":585,"record_offset":22108,"parent_record_offset":22084,"ancestor_record_offset":19008,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":585,"record_offset":23840,"parent_record_offset":23812,"ancestor_record_offset":19008,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":585,"record_offset":20024,"parent_record_offset":19996,"ancestor_record_offset":19008,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":585,"record_offset":20112,"parent_record_offset":20084,"ancestor_record_offset":19008,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":585,"record_offset":20224,"parent_record_offset":20200,"ancestor_record_offset":19008,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":585,"record_offset":21464,"parent_record_offset":21060,"ancestor_record_offset":19008,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":585,"record_offset":20508,"parent_record_offset":20320,"ancestor_record_offset":19008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":585,"record_offset":20464,"parent_record_offset":20436,"ancestor_record_offset":19008,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":585,"record_offset":21608,"parent_record_offset":21572,"ancestor_record_offset":19008,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":585,"record_offset":21632,"parent_record_offset":21608,"ancestor_record_offset":19008,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":585,"record_offset":21664,"parent_record_offset":21572,"ancestor_record_offset":19008,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":585,"record_offset":19724,"parent_record_offset":19592,"ancestor_record_offset":19008,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":585,"record_offset":19592,"parent_record_offset":19292,"ancestor_record_offset":19008,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":585,"record_offset":19780,"parent_record_offset":19592,"ancestor_record_offset":19008,"inlinee":12706,"source_line":71,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":585,"record_offset":19232,"parent_record_offset":19208,"ancestor_record_offset":19008,"inlinee":11331,"source_line":1875,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":585,"record_offset":19748,"parent_record_offset":19724,"ancestor_record_offset":19008,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$1::display::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":585,"record_offset":25404,"rva":11748928,"va":5380458048}
// 编译实例 RVAs: 0xb34640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2389745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458048,"raw_hex":"e8f1762400","size":5,"soft_fail":false,"target_va":5382847888,"va":5380458138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5376626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458048,"raw_hex":"e88ef5adff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380458109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5377197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458048,"raw_hex":"e853f3adff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380458696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089ada0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2726639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458048,"raw_hex":"e81165d6ff","size":5,"soft_fail":false,"target_va":5377732000,"va":5380458634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31082415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458048,"raw_hex":"e8af47da01","size":5,"soft_fail":false,"target_va":5411540992,"va":5380458572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30888483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458048,"raw_hex":"e82352d701","size":5,"soft_fail":false,"target_va":5411347040,"va":5380458552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31082364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458048,"raw_hex":"e87c47da01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380458594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2714178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458048,"raw_hex":"e8be95d6ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5380458381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":25588,"parent_record_offset":25404,"ancestor_record_offset":25404,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":26616,"parent_record_offset":25404,"ancestor_record_offset":25404,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":585,"record_offset":26312,"parent_record_offset":25404,"ancestor_record_offset":25404,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":585,"record_offset":26700,"parent_record_offset":25404,"ancestor_record_offset":25404,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":585,"record_offset":25736,"parent_record_offset":25404,"ancestor_record_offset":25404,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":585,"record_offset":26284,"parent_record_offset":25404,"ancestor_record_offset":25404,"inlinee":27002,"source_line":194,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":25644,"parent_record_offset":25620,"ancestor_record_offset":25404,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":25668,"parent_record_offset":25644,"ancestor_record_offset":25404,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":26772,"parent_record_offset":26748,"ancestor_record_offset":25404,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":25620,"parent_record_offset":25588,"ancestor_record_offset":25404,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":26748,"parent_record_offset":26724,"ancestor_record_offset":25404,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":585,"record_offset":26724,"parent_record_offset":26700,"ancestor_record_offset":25404,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":26464,"parent_record_offset":26436,"ancestor_record_offset":25404,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":26008,"parent_record_offset":25980,"ancestor_record_offset":25404,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":26548,"parent_record_offset":26524,"ancestor_record_offset":25404,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":26524,"parent_record_offset":26492,"ancestor_record_offset":25404,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":26492,"parent_record_offset":26464,"ancestor_record_offset":25404,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":26068,"parent_record_offset":26036,"ancestor_record_offset":25404,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":26036,"parent_record_offset":26008,"ancestor_record_offset":25404,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":25944,"parent_record_offset":25916,"ancestor_record_offset":25404,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":26640,"parent_record_offset":26616,"ancestor_record_offset":25404,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":25916,"parent_record_offset":25888,"ancestor_record_offset":25404,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":25860,"parent_record_offset":25832,"ancestor_record_offset":25404,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":25888,"parent_record_offset":25860,"ancestor_record_offset":25404,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":25832,"parent_record_offset":25804,"ancestor_record_offset":25404,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":585,"record_offset":25804,"parent_record_offset":25776,"ancestor_record_offset":25404,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":585,"record_offset":25776,"parent_record_offset":25736,"ancestor_record_offset":25404,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":26664,"parent_record_offset":26640,"ancestor_record_offset":25404,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":26436,"parent_record_offset":26400,"ancestor_record_offset":25404,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":25980,"parent_record_offset":25944,"ancestor_record_offset":25404,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":585,"record_offset":26368,"parent_record_offset":26340,"ancestor_record_offset":25404,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":585,"record_offset":26400,"parent_record_offset":26368,"ancestor_record_offset":25404,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":585,"record_offset":26340,"parent_record_offset":26312,"ancestor_record_offset":25404,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":585,"record_offset":26220,"parent_record_offset":26196,"ancestor_record_offset":25404,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":585,"record_offset":26196,"parent_record_offset":25944,"ancestor_record_offset":25404,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":585,"record_offset":25692,"parent_record_offset":25668,"ancestor_record_offset":25404,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$1::display::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":585,"record_offset":26816,"rva":11749600,"va":5380458720}
// 编译实例 RVAs: 0xb348e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5377304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458720,"raw_hex":"e8e8f2adff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380458787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5377987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458720,"raw_hex":"e83df0adff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380459486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089ada0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2727557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458720,"raw_hex":"e87b61d6ff","size":5,"soft_fail":false,"target_va":5377732000,"va":5380459552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31388232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458720,"raw_hex":"e848f2de01","size":5,"soft_fail":false,"target_va":5411847168,"va":5380458931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31081583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458720,"raw_hex":"e86f44da01","size":5,"soft_fail":false,"target_va":5411540992,"va":5380459404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30887671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458720,"raw_hex":"e8f74ed701","size":5,"soft_fail":false,"target_va":5411347040,"va":5380459364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31081571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458720,"raw_hex":"e86344da01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380459387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2714984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380458720,"raw_hex":"e89892d6ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5380459187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":27004,"parent_record_offset":26816,"ancestor_record_offset":26816,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":28624,"parent_record_offset":26816,"ancestor_record_offset":26816,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":27152,"parent_record_offset":26816,"ancestor_record_offset":26816,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":585,"record_offset":28320,"parent_record_offset":26816,"ancestor_record_offset":26816,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":585,"record_offset":28708,"parent_record_offset":26816,"ancestor_record_offset":26816,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":585,"record_offset":27744,"parent_record_offset":26816,"ancestor_record_offset":26816,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":585,"record_offset":28292,"parent_record_offset":26816,"ancestor_record_offset":26816,"inlinee":27002,"source_line":194,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":27060,"parent_record_offset":27036,"ancestor_record_offset":26816,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":27084,"parent_record_offset":27060,"ancestor_record_offset":26816,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":28780,"parent_record_offset":28756,"ancestor_record_offset":26816,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":27036,"parent_record_offset":27004,"ancestor_record_offset":26816,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":28756,"parent_record_offset":28732,"ancestor_record_offset":26816,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":585,"record_offset":28732,"parent_record_offset":28708,"ancestor_record_offset":26816,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":28472,"parent_record_offset":28444,"ancestor_record_offset":26816,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":27336,"parent_record_offset":27308,"ancestor_record_offset":26816,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":28556,"parent_record_offset":28532,"ancestor_record_offset":26816,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":28532,"parent_record_offset":28500,"ancestor_record_offset":26816,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":28500,"parent_record_offset":28472,"ancestor_record_offset":26816,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":27396,"parent_record_offset":27364,"ancestor_record_offset":26816,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":27364,"parent_record_offset":27336,"ancestor_record_offset":26816,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":27432,"parent_record_offset":27396,"ancestor_record_offset":26816,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":27272,"parent_record_offset":27244,"ancestor_record_offset":26816,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":28648,"parent_record_offset":28624,"ancestor_record_offset":26816,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":27244,"parent_record_offset":27180,"ancestor_record_offset":26816,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":27868,"parent_record_offset":27840,"ancestor_record_offset":26816,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":27896,"parent_record_offset":27868,"ancestor_record_offset":26816,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":27840,"parent_record_offset":27812,"ancestor_record_offset":26816,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":585,"record_offset":27812,"parent_record_offset":27784,"ancestor_record_offset":26816,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":585,"record_offset":27784,"parent_record_offset":27744,"ancestor_record_offset":26816,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":27216,"parent_record_offset":27180,"ancestor_record_offset":26816,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":28672,"parent_record_offset":28648,"ancestor_record_offset":26816,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":28444,"parent_record_offset":28408,"ancestor_record_offset":26816,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":27308,"parent_record_offset":27272,"ancestor_record_offset":26816,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":27180,"parent_record_offset":27152,"ancestor_record_offset":26816,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":585,"record_offset":28376,"parent_record_offset":28348,"ancestor_record_offset":26816,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":585,"record_offset":28408,"parent_record_offset":28376,"ancestor_record_offset":26816,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":585,"record_offset":28348,"parent_record_offset":28320,"ancestor_record_offset":26816,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":585,"record_offset":27456,"parent_record_offset":27432,"ancestor_record_offset":26816,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":585,"record_offset":27480,"parent_record_offset":27456,"ancestor_record_offset":26816,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":585,"record_offset":27696,"parent_record_offset":27672,"ancestor_record_offset":26816,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":585,"record_offset":27504,"parent_record_offset":27480,"ancestor_record_offset":26816,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":585,"record_offset":27528,"parent_record_offset":27504,"ancestor_record_offset":26816,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":585,"record_offset":27672,"parent_record_offset":27272,"ancestor_record_offset":26816,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":585,"record_offset":27108,"parent_record_offset":27084,"ancestor_record_offset":26816,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$1::display::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":585,"record_offset":28824,"rva":11750464,"va":5380459584}
// 编译实例 RVAs: 0xb34c40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5378164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380459584,"raw_hex":"e88cefadff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380459647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5378652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380459584,"raw_hex":"e8a4edadff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380460151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089ada0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2728209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380459584,"raw_hex":"e8ef5ed6ff","size":5,"soft_fail":false,"target_va":5377732000,"va":5380460204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31080931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380459584,"raw_hex":"e8e341da01","size":5,"soft_fail":false,"target_va":5411540992,"va":5380460056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31080880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380459584,"raw_hex":"e8b041da01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380460078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2715789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380459584,"raw_hex":"e8738fd6ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5380459992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":29008,"parent_record_offset":28824,"ancestor_record_offset":28824,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":30268,"parent_record_offset":28824,"ancestor_record_offset":28824,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":585,"record_offset":29156,"parent_record_offset":28824,"ancestor_record_offset":28824,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":585,"record_offset":30352,"parent_record_offset":28824,"ancestor_record_offset":28824,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":585,"record_offset":29692,"parent_record_offset":28824,"ancestor_record_offset":28824,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":585,"record_offset":30240,"parent_record_offset":28824,"ancestor_record_offset":28824,"inlinee":26175,"source_line":189,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":29064,"parent_record_offset":29040,"ancestor_record_offset":28824,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":29088,"parent_record_offset":29064,"ancestor_record_offset":28824,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":30424,"parent_record_offset":30400,"ancestor_record_offset":28824,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":29040,"parent_record_offset":29008,"ancestor_record_offset":28824,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":30400,"parent_record_offset":30376,"ancestor_record_offset":28824,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":585,"record_offset":30376,"parent_record_offset":30352,"ancestor_record_offset":28824,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":29416,"parent_record_offset":29388,"ancestor_record_offset":28824,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":29476,"parent_record_offset":29444,"ancestor_record_offset":28824,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":585,"record_offset":29444,"parent_record_offset":29416,"ancestor_record_offset":28824,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":29352,"parent_record_offset":29324,"ancestor_record_offset":28824,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":30292,"parent_record_offset":30268,"ancestor_record_offset":28824,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":29324,"parent_record_offset":29296,"ancestor_record_offset":28824,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":29268,"parent_record_offset":29240,"ancestor_record_offset":28824,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":29296,"parent_record_offset":29268,"ancestor_record_offset":28824,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":585,"record_offset":29240,"parent_record_offset":29212,"ancestor_record_offset":28824,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":585,"record_offset":29212,"parent_record_offset":29184,"ancestor_record_offset":28824,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":585,"record_offset":29184,"parent_record_offset":29156,"ancestor_record_offset":28824,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":30316,"parent_record_offset":30292,"ancestor_record_offset":28824,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":29388,"parent_record_offset":29352,"ancestor_record_offset":28824,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":585,"record_offset":29628,"parent_record_offset":29604,"ancestor_record_offset":28824,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":585,"record_offset":29604,"parent_record_offset":29352,"ancestor_record_offset":28824,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":585,"record_offset":29112,"parent_record_offset":29088,"ancestor_record_offset":28824,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00585.jsonl.gz"}
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
// 完整结构及原始类型记录: decorator_outline.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>", "constructor": "enum2$", "arguments": ["country_core::resources::walls::decorator_outline::DecoratorCollisionOutline"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline> >", "constructor": "tuple$", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "enum2$<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::decorator_outline::DecoratorOutline
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>", "constructor": "enum2$", "arguments": ["country_core::resources::walls::decorator_outline::DecoratorOutline"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >", "constructor": "tuple$", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>", "enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorator_outline.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorator_outline.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display
// 信息 ID: aca92976b8e65b94032bac4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x12b8b5d -> 0xb33920 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x12b9b6e -> 0xb33920 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x12b8d0e -> 0xb33920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::DecoratorOutline::display
// 信息 ID: 91a886136c4d95dc5b0eb062
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a57b1 -> 0xb33b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter @ 0x99dd74 -> 0xb33b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_color_decorator::ui_color_windows_and_flags @ 0x22b1628 -> 0xb33b00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$0::display::closure$0
// 信息 ID: cd944436d6f6bc4a849a6953
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$0::display", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display; module=585, inline_record=18356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$1::display::closure$0
// 信息 ID: b4c0f50196a79b3378d7afad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$1::display", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display::closure_env$0 <- core::ptr::drop_in_place<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xb30e93 -> 0xb34640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$1::display::closure$1
// 信息 ID: c85a92529ecb63284d07d2ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$1::display", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display::closure_env$1 <- core::ptr::drop_in_place<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xb30df3 -> 0xb348e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$1::display::closure$2
// 信息 ID: dfa9cb1de7bbcb902ed8a405
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$1::display", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display::closure_env$2 <- core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xb30dc2 -> 0xb34c40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

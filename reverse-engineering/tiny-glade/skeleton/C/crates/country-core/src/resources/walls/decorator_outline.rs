// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/walls/decorator_outline.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4c02bd5da38d5653772c9dbde0cab50ce22ad1647acaceeca89686ea7cdafd83
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_outline.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 503, 551, 577, 1643, 1682
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display
// 记录形式: compiled_procedure; instances=1
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":106776,"rva":9483920,"va":5378193040}
// 编译实例 RVAs: 0x90b690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2514536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378193040,"raw_hex":"e898a1d9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378193219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9471019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378193040,"raw_hex":"e8d57b6fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378193206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140904120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378193040,"raw_hex":"e8158affff","size":5,"soft_fail":false,"target_va":5378162976,"va":5378193158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140906710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378193040,"raw_hex":"e806afffff","size":5,"soft_fail":false,"target_va":5378172688,"va":5378193413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38427339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378193040,"raw_hex":"e8cb5a4a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378193450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":503,"record_offset":107020,"parent_record_offset":106776,"ancestor_record_offset":106776,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::line_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":108320,"parent_record_offset":106776,"ancestor_record_offset":106776,"inlinee":28668,"source_line":441,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::show
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":108348,"parent_record_offset":106776,"ancestor_record_offset":106776,"inlinee":28669,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugSphere::radius
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":106964,"parent_record_offset":106776,"ancestor_record_offset":106776,"inlinee":28663,"source_line":125,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugSphere::show
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":503,"record_offset":106992,"parent_record_offset":106776,"ancestor_record_offset":106776,"inlinee":28664,"source_line":141,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":107348,"parent_record_offset":107324,"ancestor_record_offset":106776,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":107372,"parent_record_offset":107348,"ancestor_record_offset":106776,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":107396,"parent_record_offset":107372,"ancestor_record_offset":106776,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":107324,"parent_record_offset":107288,"ancestor_record_offset":106776,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":107220,"parent_record_offset":107188,"ancestor_record_offset":106776,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":107288,"parent_record_offset":107252,"ancestor_record_offset":106776,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":107252,"parent_record_offset":107220,"ancestor_record_offset":106776,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":107480,"parent_record_offset":107456,"ancestor_record_offset":106776,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":107156,"parent_record_offset":107116,"ancestor_record_offset":106776,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":107188,"parent_record_offset":107156,"ancestor_record_offset":106776,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":107052,"parent_record_offset":107020,"ancestor_record_offset":106776,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":108220,"parent_record_offset":108196,"ancestor_record_offset":106776,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":503,"record_offset":107456,"parent_record_offset":107116,"ancestor_record_offset":106776,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":503,"record_offset":107084,"parent_record_offset":107052,"ancestor_record_offset":106776,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":503,"record_offset":107116,"parent_record_offset":107084,"ancestor_record_offset":106776,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":503,"record_offset":107732,"parent_record_offset":107704,"ancestor_record_offset":106776,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":503,"record_offset":107796,"parent_record_offset":107668,"ancestor_record_offset":106776,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":503,"record_offset":107768,"parent_record_offset":107668,"ancestor_record_offset":106776,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":503,"record_offset":107704,"parent_record_offset":107668,"ancestor_record_offset":106776,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":503,"record_offset":107528,"parent_record_offset":107504,"ancestor_record_offset":106776,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":503,"record_offset":107580,"parent_record_offset":107552,"ancestor_record_offset":106776,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":503,"record_offset":107504,"parent_record_offset":107480,"ancestor_record_offset":106776,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":503,"record_offset":108196,"parent_record_offset":107580,"ancestor_record_offset":106776,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":108244,"parent_record_offset":108220,"ancestor_record_offset":106776,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":503,"record_offset":107552,"parent_record_offset":107528,"ancestor_record_offset":106776,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_outline::impl$0::display::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_outline.rs
//     evidence={"module":503,"record_offset":107604,"parent_record_offset":107580,"ancestor_record_offset":106776,"inlinee":28666,"source_line":31,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":503,"record_offset":107628,"parent_record_offset":107604,"ancestor_record_offset":106776,"inlinee":27157,"source_line":392,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":503,"record_offset":107668,"parent_record_offset":107628,"ancestor_record_offset":106776,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":503,"record_offset":108076,"parent_record_offset":107628,"ancestor_record_offset":106776,"inlinee":27159,"source_line":1577,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$0::display::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FFA
// 首条位置: {"module":503,"inlinee":28666}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::transform_point3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":503,"record_offset":107628,"parent_record_offset":107604,"ancestor_record_offset":107604,"inlinee":27157,"source_line":392,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":503,"record_offset":107732,"parent_record_offset":107704,"ancestor_record_offset":107604,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":503,"record_offset":107796,"parent_record_offset":107668,"ancestor_record_offset":107604,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":503,"record_offset":107768,"parent_record_offset":107668,"ancestor_record_offset":107604,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":503,"record_offset":107704,"parent_record_offset":107668,"ancestor_record_offset":107604,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":503,"record_offset":107668,"parent_record_offset":107628,"ancestor_record_offset":107604,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":503,"record_offset":108076,"parent_record_offset":107628,"ancestor_record_offset":107604,"inlinee":27159,"source_line":1577,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::DecoratorOutline::display
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7057
// 首条位置: {"module":503,"record_offset":106624,"rva":9483856,"va":5378192976}
// 编译实例 RVAs: 0x90b650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_outline::DecoratorOutline::display_with_opacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192976,"raw_hex":"e8bdeeffff","size":5,"soft_fail":false,"target_va":5378188608,"va":5378193022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::DecoratorOutline::display_with_opacity
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":96700,"rva":9479488,"va":5378188608}
// 编译实例 RVAs: 0x90a540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6606662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378188608,"raw_hex":"e846cf6400","size":5,"soft_fail":false,"target_va":5384795984,"va":5378189317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-2510689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378188608,"raw_hex":"e89fb0d9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378189372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-2511580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378188608,"raw_hex":"e824add9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378190279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=7
//     target_aliases=1699 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9467170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378188608,"raw_hex":"e8de8a6fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378189357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3699866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378188608,"raw_hex":"e89a743800","size":5,"soft_fail":false,"target_va":5381890784,"va":5378190913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15706775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378188608,"raw_hex":"e897aaef00","size":5,"soft_fail":false,"target_va":5393896944,"va":5378190164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38867286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378188608,"raw_hex":"e856115102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378189765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38430400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378188608,"raw_hex":"e8c0664a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378190364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6605576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378188608,"raw_hex":"e808cb6400","size":5,"soft_fail":false,"target_va":5384794864,"va":5378189283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t38431311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378188608,"raw_hex":"e84f6a4a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378189478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":98192,"parent_record_offset":96700,"ancestor_record_offset":96700,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":98276,"parent_record_offset":96700,"ancestor_record_offset":96700,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":99672,"parent_record_offset":96700,"ancestor_record_offset":96700,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":503,"record_offset":96916,"parent_record_offset":96700,"ancestor_record_offset":96700,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":102820,"parent_record_offset":96700,"ancestor_record_offset":96700,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorId::get_inner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":503,"record_offset":98164,"parent_record_offset":96700,"ancestor_record_offset":96700,"inlinee":26975,"source_line":166,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":98856,"parent_record_offset":96700,"ancestor_record_offset":96700,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":503,"record_offset":102736,"parent_record_offset":96700,"ancestor_record_offset":96700,"inlinee":28609,"source_line":61,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_affine3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":503,"record_offset":97056,"parent_record_offset":96700,"ancestor_record_offset":96700,"inlinee":28659,"source_line":853,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":97988,"parent_record_offset":96700,"ancestor_record_offset":96700,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":503,"record_offset":98076,"parent_record_offset":96700,"ancestor_record_offset":96700,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":98412,"parent_record_offset":98388,"ancestor_record_offset":96700,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":98436,"parent_record_offset":98412,"ancestor_record_offset":96700,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":98732,"parent_record_offset":98708,"ancestor_record_offset":96700,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":98756,"parent_record_offset":98732,"ancestor_record_offset":96700,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":98460,"parent_record_offset":98436,"ancestor_record_offset":96700,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":98780,"parent_record_offset":98756,"ancestor_record_offset":96700,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":98388,"parent_record_offset":98356,"ancestor_record_offset":96700,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":98708,"parent_record_offset":98652,"ancestor_record_offset":96700,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":98356,"parent_record_offset":98312,"ancestor_record_offset":96700,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":98312,"parent_record_offset":98276,"ancestor_record_offset":96700,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":503,"record_offset":98240,"parent_record_offset":98216,"ancestor_record_offset":96700,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":99092,"parent_record_offset":99064,"ancestor_record_offset":96700,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":98680,"parent_record_offset":98652,"ancestor_record_offset":96700,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":98652,"parent_record_offset":98628,"ancestor_record_offset":96700,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":99156,"parent_record_offset":99120,"ancestor_record_offset":96700,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":99120,"parent_record_offset":99092,"ancestor_record_offset":96700,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":98628,"parent_record_offset":98604,"ancestor_record_offset":96700,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":99196,"parent_record_offset":99156,"ancestor_record_offset":96700,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":98976,"parent_record_offset":98948,"ancestor_record_offset":96700,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":99004,"parent_record_offset":98976,"ancestor_record_offset":96700,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":99032,"parent_record_offset":99004,"ancestor_record_offset":96700,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":98948,"parent_record_offset":98920,"ancestor_record_offset":96700,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":503,"record_offset":98920,"parent_record_offset":98892,"ancestor_record_offset":96700,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":503,"record_offset":98892,"parent_record_offset":98856,"ancestor_record_offset":96700,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":503,"record_offset":99540,"parent_record_offset":99516,"ancestor_record_offset":96700,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":99752,"parent_record_offset":99708,"ancestor_record_offset":96700,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":99064,"parent_record_offset":99032,"ancestor_record_offset":96700,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":99708,"parent_record_offset":99672,"ancestor_record_offset":96700,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":503,"record_offset":99220,"parent_record_offset":99196,"ancestor_record_offset":96700,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":503,"record_offset":97104,"parent_record_offset":97056,"ancestor_record_offset":96700,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":503,"record_offset":97692,"parent_record_offset":97668,"ancestor_record_offset":96700,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":99268,"parent_record_offset":99244,"ancestor_record_offset":96700,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":503,"record_offset":96964,"parent_record_offset":96940,"ancestor_record_offset":96700,"inlinee":12957,"source_line":703,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":99244,"parent_record_offset":99220,"ancestor_record_offset":96700,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":100032,"parent_record_offset":100004,"ancestor_record_offset":96700,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":101744,"parent_record_offset":101716,"ancestor_record_offset":96700,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":503,"record_offset":98016,"parent_record_offset":97988,"ancestor_record_offset":96700,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":503,"record_offset":98104,"parent_record_offset":98076,"ancestor_record_offset":96700,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":503,"record_offset":98216,"parent_record_offset":98192,"ancestor_record_offset":96700,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":99436,"parent_record_offset":99032,"ancestor_record_offset":96700,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":96988,"parent_record_offset":96964,"ancestor_record_offset":96700,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":99460,"parent_record_offset":99436,"ancestor_record_offset":96700,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":98508,"parent_record_offset":98312,"ancestor_record_offset":96700,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":97012,"parent_record_offset":96988,"ancestor_record_offset":96700,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":99576,"parent_record_offset":99540,"ancestor_record_offset":96700,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":99600,"parent_record_offset":99576,"ancestor_record_offset":96700,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":503,"record_offset":99632,"parent_record_offset":99540,"ancestor_record_offset":96700,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":503,"record_offset":97644,"parent_record_offset":97524,"ancestor_record_offset":96700,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":503,"record_offset":97524,"parent_record_offset":97056,"ancestor_record_offset":96700,"inlinee":16668,"source_line":208,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$102::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":503,"record_offset":96940,"parent_record_offset":96916,"ancestor_record_offset":96700,"inlinee":13903,"source_line":2121,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":503,"record_offset":97668,"parent_record_offset":97644,"ancestor_record_offset":96700,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":103468,"rva":9482416,"va":5378191536}
// 编译实例 RVAs: 0x90b0b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6604361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191536,"raw_hex":"e849c66400","size":5,"soft_fail":false,"target_va":5384795984,"va":5378191618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2512909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191536,"raw_hex":"e8f3a7d9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378191592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2513210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191536,"raw_hex":"e8c6a6d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378191909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9469390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191536,"raw_hex":"e832826fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378191577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38428902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191536,"raw_hex":"e8e6604a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378191862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t746932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191536,"raw_hex":"e8b4650b00","size":5,"soft_fail":false,"target_va":5378938736,"va":5378191799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":103900,"parent_record_offset":103468,"ancestor_record_offset":103468,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":103656,"parent_record_offset":103468,"ancestor_record_offset":103468,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":103984,"parent_record_offset":103468,"ancestor_record_offset":103468,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::opacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":103836,"parent_record_offset":103468,"ancestor_record_offset":103468,"inlinee":28660,"source_line":273,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":103740,"parent_record_offset":103716,"ancestor_record_offset":103468,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":103764,"parent_record_offset":103740,"ancestor_record_offset":103468,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":104056,"parent_record_offset":104032,"ancestor_record_offset":103468,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":104080,"parent_record_offset":104056,"ancestor_record_offset":103468,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":103788,"parent_record_offset":103764,"ancestor_record_offset":103468,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":104104,"parent_record_offset":104080,"ancestor_record_offset":103468,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":103716,"parent_record_offset":103684,"ancestor_record_offset":103468,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":104032,"parent_record_offset":104008,"ancestor_record_offset":103468,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":103684,"parent_record_offset":103656,"ancestor_record_offset":103468,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":104008,"parent_record_offset":103984,"ancestor_record_offset":103468,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":103924,"parent_record_offset":103900,"ancestor_record_offset":103468,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":103948,"parent_record_offset":103924,"ancestor_record_offset":103468,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":503,"record_offset":103864,"parent_record_offset":103836,"ancestor_record_offset":103468,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":105344,"rva":9483280,"va":5378192400}
// 编译实例 RVAs: 0x90b410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2513782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192400,"raw_hex":"e88aa4d9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378192465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2514240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192400,"raw_hex":"e8c0a2d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378192939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9470263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192400,"raw_hex":"e8c97e6fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378192450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38864445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192400,"raw_hex":"e83d065102","size":5,"soft_fail":false,"target_va":5417057056,"va":5378192606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38427882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192400,"raw_hex":"e8ea5c4a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378192882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38428267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192400,"raw_hex":"e86b5e4a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378192522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t745947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378192400,"raw_hex":"e8db610b00","size":5,"soft_fail":false,"target_va":5378938736,"va":5378192784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":106368,"parent_record_offset":105344,"ancestor_record_offset":105344,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":105536,"parent_record_offset":105344,"ancestor_record_offset":105344,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":105716,"parent_record_offset":105344,"ancestor_record_offset":105344,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":106452,"parent_record_offset":105344,"ancestor_record_offset":105344,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::opacity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":106304,"parent_record_offset":105344,"ancestor_record_offset":105344,"inlinee":28660,"source_line":273,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":105620,"parent_record_offset":105596,"ancestor_record_offset":105344,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":105644,"parent_record_offset":105620,"ancestor_record_offset":105344,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":106524,"parent_record_offset":106500,"ancestor_record_offset":105344,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":106548,"parent_record_offset":106524,"ancestor_record_offset":105344,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":105668,"parent_record_offset":105644,"ancestor_record_offset":105344,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":106572,"parent_record_offset":106548,"ancestor_record_offset":105344,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":105596,"parent_record_offset":105564,"ancestor_record_offset":105344,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":106500,"parent_record_offset":106476,"ancestor_record_offset":105344,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":105564,"parent_record_offset":105536,"ancestor_record_offset":105344,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":106476,"parent_record_offset":106452,"ancestor_record_offset":105344,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":105872,"parent_record_offset":105848,"ancestor_record_offset":105344,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":105928,"parent_record_offset":105896,"ancestor_record_offset":105344,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":105896,"parent_record_offset":105872,"ancestor_record_offset":105344,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":105964,"parent_record_offset":105928,"ancestor_record_offset":105344,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":106392,"parent_record_offset":106368,"ancestor_record_offset":105344,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":105808,"parent_record_offset":105744,"ancestor_record_offset":105344,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":105780,"parent_record_offset":105744,"ancestor_record_offset":105344,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":106416,"parent_record_offset":106392,"ancestor_record_offset":105344,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":105848,"parent_record_offset":105808,"ancestor_record_offset":105344,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":105744,"parent_record_offset":105716,"ancestor_record_offset":105344,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":503,"record_offset":105988,"parent_record_offset":105964,"ancestor_record_offset":105344,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":503,"record_offset":106332,"parent_record_offset":106304,"ancestor_record_offset":105344,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":503,"record_offset":106068,"parent_record_offset":106012,"ancestor_record_offset":105344,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":106012,"parent_record_offset":105988,"ancestor_record_offset":105344,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":503,"record_offset":106040,"parent_record_offset":106012,"ancestor_record_offset":105344,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":106236,"parent_record_offset":105808,"ancestor_record_offset":105344,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":106260,"parent_record_offset":106236,"ancestor_record_offset":105344,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":503,"record_offset":104156,"rva":9482816,"va":5378191936}
// 编译实例 RVAs: 0x90b240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2513309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191936,"raw_hex":"e863a6d9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5378191992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2513674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191936,"raw_hex":"e8f6a4d9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378192373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: decorator_outline.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9469790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191936,"raw_hex":"e8a2806fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5378191977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38428456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191936,"raw_hex":"e8285f4a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5378192308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38428465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191936,"raw_hex":"e8315f4a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5378192324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t746486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378191936,"raw_hex":"e8f6630b00","size":5,"soft_fail":false,"target_va":5378938736,"va":5378192245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":105088,"parent_record_offset":104156,"ancestor_record_offset":104156,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":104348,"parent_record_offset":104156,"ancestor_record_offset":104156,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":503,"record_offset":104528,"parent_record_offset":104156,"ancestor_record_offset":104156,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":105172,"parent_record_offset":104156,"ancestor_record_offset":104156,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":503,"record_offset":105060,"parent_record_offset":104156,"ancestor_record_offset":104156,"inlinee":28661,"source_line":296,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":104432,"parent_record_offset":104408,"ancestor_record_offset":104156,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":104456,"parent_record_offset":104432,"ancestor_record_offset":104156,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":105244,"parent_record_offset":105220,"ancestor_record_offset":104156,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":105268,"parent_record_offset":105244,"ancestor_record_offset":104156,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":104480,"parent_record_offset":104456,"ancestor_record_offset":104156,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":105292,"parent_record_offset":105268,"ancestor_record_offset":104156,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":104408,"parent_record_offset":104376,"ancestor_record_offset":104156,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":503,"record_offset":105220,"parent_record_offset":105196,"ancestor_record_offset":104156,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":104376,"parent_record_offset":104348,"ancestor_record_offset":104156,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":503,"record_offset":105196,"parent_record_offset":105172,"ancestor_record_offset":104156,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":104760,"parent_record_offset":104732,"ancestor_record_offset":104156,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":104820,"parent_record_offset":104788,"ancestor_record_offset":104156,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":503,"record_offset":104788,"parent_record_offset":104760,"ancestor_record_offset":104156,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":105112,"parent_record_offset":105088,"ancestor_record_offset":104156,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":104640,"parent_record_offset":104612,"ancestor_record_offset":104156,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":104668,"parent_record_offset":104640,"ancestor_record_offset":104156,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":104696,"parent_record_offset":104668,"ancestor_record_offset":104156,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":503,"record_offset":104612,"parent_record_offset":104584,"ancestor_record_offset":104156,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":503,"record_offset":104584,"parent_record_offset":104556,"ancestor_record_offset":104156,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":503,"record_offset":104556,"parent_record_offset":104528,"ancestor_record_offset":104156,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":105136,"parent_record_offset":105112,"ancestor_record_offset":104156,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":503,"record_offset":104732,"parent_record_offset":104696,"ancestor_record_offset":104156,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":503,"record_offset":104976,"parent_record_offset":104696,"ancestor_record_offset":104156,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00503.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":503,"record_offset":105000,"parent_record_offset":104976,"ancestor_record_offset":104156,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00503.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 10
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 27; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

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
// 函数: country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display
// 信息 ID: 1da5e96b208126cfd8aa8c9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x15084d5 -> 0x90b690 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x1508626 -> 0x90b690 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::copied::impl$1::try_fold<std::collections::hash::set::Iter<country_core::resources::walls::decorator::DecoratorLocator>,country_core::resources::walls::decorator::DecoratorLocator,tuple$<>,core::iter::traits::iterator::Iterator::all::check::closure_env$0<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::blueprint::validate_blueprints::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<>,tuple$<> > > > @ 0x15087ea -> 0x90b690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::DecoratorOutline::display
// 信息 ID: e9a8eb4fae89bf104732a76b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x86160f -> 0x90b650 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_color_decorator::ui_color_windows_and_flags @ 0x28b09f4 -> 0x90b650 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter; module=551, inline_record=49528; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze; module=577, inline_record=23936; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::DecoratorOutline::display_with_opacity
// 信息 ID: 0a24c6fa2209cd2d1d0209ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::decorator_outline::DecoratorOutline::display @ 0x90b67e -> 0x90a540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_clutter::ui_autoclutter::ui_convert_autoclutter_to_decoclutter @ 0xac991a -> 0x90a540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::style_picker_feedback::outline_target @ 0x2892a39 -> 0x90a540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$0::display::closure$0
// 信息 ID: 158b9f58349e8af100752cc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$0::display", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::decorator_outline::DecoratorCollisionOutline::display; module=503, inline_record=107604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure$0
// 信息 ID: 548d782133bacaa516af177b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$1::display_with_opacity", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$0 <- core::ptr::drop_in_place<country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x90b9b3 -> 0x90b0b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure$1
// 信息 ID: a539edf694bbcca02eed7ed3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$1::display_with_opacity", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$1 <- core::ptr::drop_in_place<country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x90ba53 -> 0x90b410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure$2
// 信息 ID: 7be14d50f674c7ecca7176a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::walls::decorator_outline::impl$1::display_with_opacity", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$2 <- core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::resources::walls::decorator_outline::impl$1::display_with_opacity::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x90bb80 -> 0x90b240 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

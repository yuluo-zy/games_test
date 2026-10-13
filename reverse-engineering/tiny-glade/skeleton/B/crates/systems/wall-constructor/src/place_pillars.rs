// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/place_pillars.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 86754cf4818361f9ef4480ea2059c7e200d260bd0d225d386207f1c69e40ed5a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: place_pillars.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 390, 392, 396, 402, 1415, 1417, 1426
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
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1426,"record_offset":1872,"rva":18929264,"va":5387638384}
// 编译实例 RVAs: 0x120d670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12557082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387638384,"raw_hex":"e8e66440ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387638565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12557810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387638384,"raw_hex":"e80e6240ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387639309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24208947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387638384,"raw_hex":"e833667101","size":5,"soft_fail":false,"target_va":5411847776,"va":5387638824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23900992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387638384,"raw_hex":"e840b36c01","size":5,"soft_fail":false,"target_va":5411540992,"va":5387639995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12311008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387638384,"raw_hex":"e8202644ff","size":5,"soft_fail":false,"target_va":5375328336,"va":5387639339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23900980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387638384,"raw_hex":"e834b36c01","size":5,"soft_fail":false,"target_va":5411540963,"va":5387639978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5629893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387638384,"raw_hex":"e83b18aaff","size":5,"soft_fail":false,"target_va":5382009728,"va":5387639616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::place_pillars::place_pillars::closure$4
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387638384,"raw_hex":"e89df7ffff","size":5,"soft_fail":false,"target_va":5387637568,"va":5387639710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5756239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387638384,"raw_hex":"e8b12aa8ff","size":5,"soft_fail":false,"target_va":5381882704,"va":5387638938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1426,"record_offset":2180,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1426,"record_offset":4148,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1426,"record_offset":2760,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1426,"record_offset":2440,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1426,"record_offset":4408,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1426,"record_offset":2788,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1426,"record_offset":3916,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1426,"record_offset":2588,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1426,"record_offset":2672,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":5380,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":4928,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":2148,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":5308,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":5480,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1426,"record_offset":5568,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1426,"record_offset":2732,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1426,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":1872,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1426,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":1872,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1426,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":1872,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1426,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":1872,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1426,"record_offset":4076,"parent_record_offset":3988,"ancestor_record_offset":1872,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1426,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":1872,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1426,"record_offset":2992,"parent_record_offset":2960,"ancestor_record_offset":1872,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1426,"record_offset":4016,"parent_record_offset":3988,"ancestor_record_offset":1872,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1426,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":1872,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1426,"record_offset":4252,"parent_record_offset":4228,"ancestor_record_offset":1872,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1426,"record_offset":4228,"parent_record_offset":4204,"ancestor_record_offset":1872,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1426,"record_offset":3060,"parent_record_offset":3024,"ancestor_record_offset":1872,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1426,"record_offset":3024,"parent_record_offset":2992,"ancestor_record_offset":1872,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1426,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":1872,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1426,"record_offset":4180,"parent_record_offset":4148,"ancestor_record_offset":1872,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1426,"record_offset":3364,"parent_record_offset":3340,"ancestor_record_offset":1872,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1426,"record_offset":2928,"parent_record_offset":2884,"ancestor_record_offset":1872,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1426,"record_offset":2960,"parent_record_offset":2928,"ancestor_record_offset":1872,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1426,"record_offset":2820,"parent_record_offset":2788,"ancestor_record_offset":1872,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1426,"record_offset":3704,"parent_record_offset":3660,"ancestor_record_offset":1872,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1426,"record_offset":3808,"parent_record_offset":3704,"ancestor_record_offset":1872,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1426,"record_offset":3340,"parent_record_offset":2884,"ancestor_record_offset":1872,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1426,"record_offset":2852,"parent_record_offset":2820,"ancestor_record_offset":1872,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1426,"record_offset":2884,"parent_record_offset":2852,"ancestor_record_offset":1872,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1426,"record_offset":4652,"parent_record_offset":4628,"ancestor_record_offset":1872,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1426,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":1872,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1426,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":1872,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1426,"record_offset":3200,"parent_record_offset":2884,"ancestor_record_offset":1872,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1426,"record_offset":3484,"parent_record_offset":3436,"ancestor_record_offset":1872,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1426,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":1872,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1426,"record_offset":3660,"parent_record_offset":3484,"ancestor_record_offset":1872,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1426,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":1872,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1426,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":1872,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1426,"record_offset":5016,"parent_record_offset":4984,"ancestor_record_offset":1872,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1426,"record_offset":3604,"parent_record_offset":3560,"ancestor_record_offset":1872,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1426,"record_offset":3560,"parent_record_offset":3516,"ancestor_record_offset":1872,"inlinee":24913,"source_line":35,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1426,"record_offset":5508,"parent_record_offset":5480,"ancestor_record_offset":1872,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1426,"record_offset":5340,"parent_record_offset":5308,"ancestor_record_offset":1872,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1426,"record_offset":4676,"parent_record_offset":4652,"ancestor_record_offset":1872,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1426,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":1872,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1426,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":1872,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1426,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":1872,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1426,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":1872,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1426,"record_offset":3852,"parent_record_offset":3436,"ancestor_record_offset":1872,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1426,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":1872,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1426,"record_offset":2700,"parent_record_offset":2672,"ancestor_record_offset":1872,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1426,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":1872,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1426,"record_offset":3760,"parent_record_offset":3704,"ancestor_record_offset":1872,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1426,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":1872,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1426,"record_offset":4564,"parent_record_offset":4540,"ancestor_record_offset":1872,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1426,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":1872,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1426,"record_offset":4476,"parent_record_offset":4440,"ancestor_record_offset":1872,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1426,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":1872,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1426,"record_offset":4540,"parent_record_offset":4476,"ancestor_record_offset":1872,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":5408,"parent_record_offset":5380,"ancestor_record_offset":1872,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1426,"record_offset":4628,"parent_record_offset":4440,"ancestor_record_offset":1872,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1426,"record_offset":4440,"parent_record_offset":4408,"ancestor_record_offset":1872,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1426,"record_offset":5444,"parent_record_offset":5408,"ancestor_record_offset":1872,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::place_pillars::assemble_stone_pillar::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":1426,"record_offset":3516,"parent_record_offset":3484,"ancestor_record_offset":1872,"inlinee":70802,"source_line":376,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1146C
// 首条位置: {"module":1415,"inlinee":70764}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1146A
// 首条位置: {"module":1415,"inlinee":70762}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1415,"record_offset":16524,"parent_record_offset":16460,"ancestor_record_offset":16460,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1415,"record_offset":18560,"parent_record_offset":16460,"ancestor_record_offset":16460,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1415,"record_offset":17024,"parent_record_offset":17000,"ancestor_record_offset":16460,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1415,"record_offset":17048,"parent_record_offset":17024,"ancestor_record_offset":16460,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1415,"record_offset":18744,"parent_record_offset":18720,"ancestor_record_offset":16460,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1415,"record_offset":17000,"parent_record_offset":16824,"ancestor_record_offset":16460,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1415,"record_offset":18720,"parent_record_offset":18632,"ancestor_record_offset":16460,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":16764,"parent_record_offset":16736,"ancestor_record_offset":16460,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":18660,"parent_record_offset":18632,"ancestor_record_offset":16460,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":18632,"parent_record_offset":18608,"ancestor_record_offset":16460,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":16824,"parent_record_offset":16792,"ancestor_record_offset":16460,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":16792,"parent_record_offset":16764,"ancestor_record_offset":16460,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":18608,"parent_record_offset":18584,"ancestor_record_offset":16460,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":16860,"parent_record_offset":16824,"ancestor_record_offset":16460,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1415,"record_offset":17240,"parent_record_offset":17208,"ancestor_record_offset":16460,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1415,"record_offset":16644,"parent_record_offset":16608,"ancestor_record_offset":16460,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1415,"record_offset":16608,"parent_record_offset":16560,"ancestor_record_offset":16460,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1415,"record_offset":16560,"parent_record_offset":16524,"ancestor_record_offset":16460,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1415,"record_offset":17108,"parent_record_offset":16560,"ancestor_record_offset":16460,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1415,"record_offset":16736,"parent_record_offset":16644,"ancestor_record_offset":16460,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1415,"record_offset":17988,"parent_record_offset":17956,"ancestor_record_offset":16460,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1415,"record_offset":16884,"parent_record_offset":16860,"ancestor_record_offset":16460,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1415,"record_offset":16908,"parent_record_offset":16884,"ancestor_record_offset":16460,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1415,"record_offset":16704,"parent_record_offset":16680,"ancestor_record_offset":16460,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1415,"record_offset":17688,"parent_record_offset":17652,"ancestor_record_offset":16460,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":1415,"record_offset":17652,"parent_record_offset":17240,"ancestor_record_offset":16460,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1415,"record_offset":17908,"parent_record_offset":17240,"ancestor_record_offset":16460,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1415,"record_offset":16932,"parent_record_offset":16908,"ancestor_record_offset":16460,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1415,"record_offset":16956,"parent_record_offset":16932,"ancestor_record_offset":16460,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1415,"record_offset":18688,"parent_record_offset":18660,"ancestor_record_offset":16460,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1415,"record_offset":16680,"parent_record_offset":16644,"ancestor_record_offset":16460,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1415,"record_offset":18584,"parent_record_offset":18560,"ancestor_record_offset":16460,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1415,"record_offset":17828,"parent_record_offset":17804,"ancestor_record_offset":16460,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1415,"record_offset":17752,"parent_record_offset":17716,"ancestor_record_offset":16460,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1415,"record_offset":17804,"parent_record_offset":17780,"ancestor_record_offset":16460,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1415,"record_offset":17716,"parent_record_offset":17688,"ancestor_record_offset":16460,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1415,"record_offset":17780,"parent_record_offset":17716,"ancestor_record_offset":16460,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::place_pillars::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":1415,"record_offset":17876,"parent_record_offset":17240,"ancestor_record_offset":16460,"inlinee":70764,"source_line":43,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::PillarProposals::add_proposal
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FA9
// 首条位置: {"module":392,"record_offset":1660,"rva":7681232,"va":5376390352}
// 编译实例 RVAs: 0x7534d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-1062260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390352,"raw_hex":"e88ccaefff","size":5,"soft_fail":false,"target_va":5375328336,"va":5376390591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35151033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390352,"raw_hex":"e8b95c1802","size":5,"soft_fail":false,"target_va":5411541776,"va":5376390738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":2108,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":392,"record_offset":2012,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":392,"record_offset":1972,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":392,"record_offset":1864,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":392,"record_offset":1836,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":392,"record_offset":1944,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":7317,"source_line":302,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1660,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":1660,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":1660,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":2144,"parent_record_offset":2108,"ancestor_record_offset":1660,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1660,"inlinee":4913,"source_line":1740,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":1660,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":392,"record_offset":2256,"parent_record_offset":2108,"ancestor_record_offset":1660,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::PillarProposals::empty
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F72
// 首条位置: {"module":390,"inlinee":24434}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::place_pillars
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":392,"record_offset":2660,"rva":7681648,"va":5376390768}
// 编译实例 RVAs: 0x753670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7127219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e84d3f93ff","size":5,"soft_fail":false,"target_va":5369267664,"va":5376394878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2766384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8d0c9d5ff","size":5,"soft_fail":false,"target_va":5373629488,"va":5376395867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1716144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e850d0e5ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5376392475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dec30
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1534228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8ec96e8ff","size":5,"soft_fail":false,"target_va":5374864432,"va":5376398655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1314568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8f8f0ebff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376396067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-879766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e86a93f2ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5376395793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1347305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8e98e1400","size":5,"soft_fail":false,"target_va":5377742176,"va":5376394866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140958d60
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2112968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8c83d2000","size":5,"soft_fail":false,"target_va":5378510176,"va":5376397203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bece40
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4820393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8a98d4900","size":5,"soft_fail":false,"target_va":5381213760,"va":5376393362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5539840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e800885400","size":5,"soft_fail":false,"target_va":5381934528,"va":5376394683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412036d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11205870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8eefcaa00","size":5,"soft_fail":false,"target_va":5387597520,"va":5376391645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141206c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11217561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8992aab00","size":5,"soft_fail":false,"target_va":5387611216,"va":5376393650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141207a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11221115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e87b38ab00","size":5,"soft_fail":false,"target_va":5387614736,"va":5376393616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209c40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11229426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8f258ab00","size":5,"soft_fail":false,"target_va":5387623488,"va":5376394057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11238772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8747dab00","size":5,"soft_fail":false,"target_va":5387633712,"va":5376394935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120d670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11243149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e88d8eab00","size":5,"soft_fail":false,"target_va":5387638384,"va":5376395230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35453021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e85df81c02","size":5,"soft_fail":false,"target_va":5411847152,"va":5376394126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4381872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8b0dc4200","size":5,"soft_fail":false,"target_va":5380775824,"va":5376393947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34947927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e857431502","size":5,"soft_fail":false,"target_va":5411339744,"va":5376391812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35144779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e84b441802","size":5,"soft_fail":false,"target_va":5411541584,"va":5376396800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35145527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e837471802","size":5,"soft_fail":false,"target_va":5411541808,"va":5376396276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t35145478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e806471802","size":5,"soft_fail":false,"target_va":5411541776,"va":5376396293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35086052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8e45e1702","size":5,"soft_fail":false,"target_va":5411482433,"va":5376396376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35145469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8fd461802","size":5,"soft_fail":false,"target_va":5411542296,"va":5376396822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35146706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8d24b1802","size":5,"soft_fail":false,"target_va":5411543584,"va":5376396873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35146527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e81f4b1802","size":5,"soft_fail":false,"target_va":5411543456,"va":5376396924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35145323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e86b461802","size":5,"soft_fail":false,"target_va":5411541904,"va":5376396576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e888f9ffff","size":5,"soft_fail":false,"target_va":5376389280,"va":5376390931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35152347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8db611802","size":5,"soft_fail":false,"target_va":5411543360,"va":5376391008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_arch_pillars_count
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3357393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8d13a3300","size":5,"soft_fail":false,"target_va":5379753008,"va":5376395610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_lintel_pillars_count
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3357401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8d93a3300","size":5,"soft_fail":false,"target_va":5379752992,"va":5376395586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_pillar_placements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3357069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e88d393300","size":5,"soft_fail":false,"target_va":5379752736,"va":5376395662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t35097050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8da891702","size":5,"soft_fail":false,"target_va":5411492368,"va":5376395313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t35099788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e88c941702","size":5,"soft_fail":false,"target_va":5411494800,"va":5376395007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5429375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e87fd85200","size":5,"soft_fail":false,"target_va":5381823600,"va":5376394220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5429035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e82bd75200","size":5,"soft_fail":false,"target_va":5381824016,"va":5376394976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4994101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e835344c00","size":5,"soft_fail":false,"target_va":5381385536,"va":5376391430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4999290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e87a484c00","size":5,"soft_fail":false,"target_va":5381390640,"va":5376391345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4990331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e87b254c00","size":5,"soft_fail":false,"target_va":5381386480,"va":5376396144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4993794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e802334c00","size":5,"soft_fail":false,"target_va":5381384704,"va":5376390905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35244170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e88ac81902","size":5,"soft_fail":false,"target_va":5411640768,"va":5376396593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35140456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e868331802","size":5,"soft_fail":false,"target_va":5411537104,"va":5376396643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11190995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8d3c2aa00","size":5,"soft_fail":false,"target_va":5387584384,"va":5376393384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11190839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e837c2aa00","size":5,"soft_fail":false,"target_va":5387584336,"va":5376393492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11191032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376390768,"raw_hex":"e8f8c2aa00","size":5,"soft_fail":false,"target_va":5387584368,"va":5376393331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5376390768,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5376394157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":21688,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":10753,"source_line":3172,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":13112,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":15436,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":14400,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":13408,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":6872,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":392,"record_offset":5340,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":20136,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":392,"record_offset":20168,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":392,"record_offset":13224,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":392,"record_offset":6108,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":392,"record_offset":6052,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":392,"record_offset":13604,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":392,"record_offset":15860,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":392,"record_offset":18672,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":392,"record_offset":10900,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":392,"record_offset":10252,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":24453,"source_line":2972,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":10984,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":392,"record_offset":15620,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":392,"record_offset":15896,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":392,"record_offset":14620,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":24415,"source_line":118,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":392,"record_offset":20052,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":392,"record_offset":4356,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":392,"record_offset":2932,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":392,"record_offset":3380,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":12136,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":11060,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::place_pillars::emit_pillar_at_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":392,"record_offset":17044,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":24467,"source_line":276,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_for_point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":14676,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":24420,"source_line":1305,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":392,"record_offset":14876,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":392,"record_offset":14820,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":14904,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":392,"record_offset":14592,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":24417,"source_line":175,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":392,"record_offset":14648,"parent_record_offset":2660,"ancestor_record_offset":2660,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":392,"record_offset":20528,"parent_record_offset":20504,"ancestor_record_offset":2660,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":392,"record_offset":20504,"parent_record_offset":20412,"ancestor_record_offset":2660,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":7628,"parent_record_offset":7596,"ancestor_record_offset":2660,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":2660,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":20440,"parent_record_offset":20412,"ancestor_record_offset":2660,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":20412,"parent_record_offset":20388,"ancestor_record_offset":2660,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":7676,"parent_record_offset":7652,"ancestor_record_offset":2660,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":7652,"parent_record_offset":7628,"ancestor_record_offset":2660,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":2660,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":20388,"parent_record_offset":20364,"ancestor_record_offset":2660,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":7596,"parent_record_offset":6920,"ancestor_record_offset":2660,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":13480,"parent_record_offset":13456,"ancestor_record_offset":2660,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":5680,"parent_record_offset":5656,"ancestor_record_offset":2660,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":21720,"parent_record_offset":21688,"ancestor_record_offset":2660,"inlinee":10754,"source_line":2264,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":2660,"inlinee":11829,"source_line":3524,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":8168,"parent_record_offset":6920,"ancestor_record_offset":2660,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":7512,"parent_record_offset":6920,"ancestor_record_offset":2660,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":7716,"parent_record_offset":6920,"ancestor_record_offset":2660,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":2660,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":13628,"parent_record_offset":13604,"ancestor_record_offset":2660,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$5::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":21816,"parent_record_offset":21720,"ancestor_record_offset":2660,"inlinee":10756,"source_line":3173,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":5656,"parent_record_offset":5428,"ancestor_record_offset":2660,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":392,"record_offset":6896,"parent_record_offset":6872,"ancestor_record_offset":2660,"inlinee":11862,"source_line":18,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":392,"record_offset":5404,"parent_record_offset":5364,"ancestor_record_offset":2660,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":392,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":2660,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":392,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":2660,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$0::size_hint
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":392,"record_offset":6996,"parent_record_offset":6968,"ancestor_record_offset":2660,"inlinee":24451,"source_line":18,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":392,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":2660,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::size_hint
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":392,"record_offset":7024,"parent_record_offset":6996,"ancestor_record_offset":2660,"inlinee":24452,"source_line":103,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":392,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":2660,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":392,"record_offset":4692,"parent_record_offset":4664,"ancestor_record_offset":2660,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":392,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":2660,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":392,"record_offset":4720,"parent_record_offset":4692,"ancestor_record_offset":2660,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":392,"record_offset":4640,"parent_record_offset":4612,"ancestor_record_offset":2660,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":392,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":2660,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":392,"record_offset":4836,"parent_record_offset":4664,"ancestor_record_offset":2660,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":392,"record_offset":4948,"parent_record_offset":4924,"ancestor_record_offset":2660,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":13896,"parent_record_offset":13872,"ancestor_record_offset":2660,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":10628,"parent_record_offset":10604,"ancestor_record_offset":2660,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":21840,"parent_record_offset":21816,"ancestor_record_offset":2660,"inlinee":10758,"source_line":1976,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":2660,"inlinee":14516,"source_line":1820,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":11888,"parent_record_offset":11864,"ancestor_record_offset":2660,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":2660,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":392,"record_offset":17176,"parent_record_offset":17044,"ancestor_record_offset":2660,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":392,"record_offset":11396,"parent_record_offset":11372,"ancestor_record_offset":2660,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":392,"record_offset":11480,"parent_record_offset":11456,"ancestor_record_offset":2660,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":17768,"parent_record_offset":17744,"ancestor_record_offset":2660,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":17372,"parent_record_offset":17344,"ancestor_record_offset":2660,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":17400,"parent_record_offset":17344,"ancestor_record_offset":2660,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":392,"record_offset":14788,"parent_record_offset":14676,"ancestor_record_offset":2660,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":392,"record_offset":10568,"parent_record_offset":10544,"ancestor_record_offset":2660,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":392,"record_offset":10328,"parent_record_offset":10288,"ancestor_record_offset":2660,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":392,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":2660,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":392,"record_offset":7820,"parent_record_offset":7792,"ancestor_record_offset":2660,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":392,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":2660,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":392,"record_offset":7052,"parent_record_offset":7024,"ancestor_record_offset":2660,"inlinee":11429,"source_line":184,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":392,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":2660,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":392,"record_offset":7744,"parent_record_offset":6920,"ancestor_record_offset":2660,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":392,"record_offset":6968,"parent_record_offset":6920,"ancestor_record_offset":2660,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":392,"record_offset":13824,"parent_record_offset":13604,"ancestor_record_offset":2660,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":392,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":2660,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":392,"record_offset":13848,"parent_record_offset":13824,"ancestor_record_offset":2660,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":392,"record_offset":14360,"parent_record_offset":14336,"ancestor_record_offset":2660,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":392,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":2660,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":392,"record_offset":5612,"parent_record_offset":5584,"ancestor_record_offset":2660,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":392,"record_offset":7440,"parent_record_offset":7052,"ancestor_record_offset":2660,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":392,"record_offset":5472,"parent_record_offset":5428,"ancestor_record_offset":2660,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":392,"record_offset":20472,"parent_record_offset":20440,"ancestor_record_offset":2660,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":392,"record_offset":15016,"parent_record_offset":14964,"ancestor_record_offset":2660,"inlinee":24425,"source_line":3058,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":392,"record_offset":3080,"parent_record_offset":3048,"ancestor_record_offset":2660,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":392,"record_offset":11628,"parent_record_offset":11600,"ancestor_record_offset":2660,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":392,"record_offset":11600,"parent_record_offset":11572,"ancestor_record_offset":2660,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":15372,"parent_record_offset":15340,"ancestor_record_offset":2660,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":15280,"parent_record_offset":15248,"ancestor_record_offset":2660,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":15188,"parent_record_offset":15160,"ancestor_record_offset":2660,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":392,"record_offset":10604,"parent_record_offset":10480,"ancestor_record_offset":2660,"inlinee":4196,"source_line":166,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":392,"record_offset":4508,"parent_record_offset":4464,"ancestor_record_offset":2660,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":392,"record_offset":8132,"parent_record_offset":8108,"ancestor_record_offset":2660,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":392,"record_offset":7868,"parent_record_offset":7844,"ancestor_record_offset":2660,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":392,"record_offset":16164,"parent_record_offset":15896,"ancestor_record_offset":2660,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":392,"record_offset":8108,"parent_record_offset":7744,"ancestor_record_offset":2660,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":392,"record_offset":7900,"parent_record_offset":7820,"ancestor_record_offset":2660,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":392,"record_offset":14752,"parent_record_offset":14676,"ancestor_record_offset":2660,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":392,"record_offset":16200,"parent_record_offset":15896,"ancestor_record_offset":2660,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":392,"record_offset":3328,"parent_record_offset":3300,"ancestor_record_offset":2660,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":392,"record_offset":5296,"parent_record_offset":4464,"ancestor_record_offset":2660,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":392,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":2660,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":392,"record_offset":4900,"parent_record_offset":4612,"ancestor_record_offset":2660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":392,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":2660,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":392,"record_offset":11748,"parent_record_offset":11724,"ancestor_record_offset":2660,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":392,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":2660,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":392,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":2660,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":392,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":2660,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":392,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":2660,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":392,"record_offset":7128,"parent_record_offset":7104,"ancestor_record_offset":2660,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":392,"record_offset":3976,"parent_record_offset":3948,"ancestor_record_offset":2660,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":392,"record_offset":4792,"parent_record_offset":4768,"ancestor_record_offset":2660,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":392,"record_offset":15772,"parent_record_offset":15744,"ancestor_record_offset":2660,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":392,"record_offset":15800,"parent_record_offset":15772,"ancestor_record_offset":2660,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":392,"record_offset":5528,"parent_record_offset":5428,"ancestor_record_offset":2660,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":392,"record_offset":5896,"parent_record_offset":5428,"ancestor_record_offset":2660,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":392,"record_offset":16136,"parent_record_offset":15896,"ancestor_record_offset":2660,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":392,"record_offset":14428,"parent_record_offset":14400,"ancestor_record_offset":2660,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":392,"record_offset":14456,"parent_record_offset":14428,"ancestor_record_offset":2660,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":392,"record_offset":5584,"parent_record_offset":5556,"ancestor_record_offset":2660,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":392,"record_offset":5556,"parent_record_offset":5528,"ancestor_record_offset":2660,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":392,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":2660,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":3720,"parent_record_offset":3696,"ancestor_record_offset":2660,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":8024,"parent_record_offset":8000,"ancestor_record_offset":2660,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":2660,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":2660,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":7976,"parent_record_offset":7924,"ancestor_record_offset":2660,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":7104,"parent_record_offset":7052,"ancestor_record_offset":2660,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":3696,"parent_record_offset":3580,"ancestor_record_offset":2660,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":8000,"parent_record_offset":7976,"ancestor_record_offset":2660,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":392,"record_offset":10544,"parent_record_offset":10520,"ancestor_record_offset":2660,"inlinee":10771,"source_line":302,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insert_tail
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":392,"record_offset":10480,"parent_record_offset":10384,"ancestor_record_offset":2660,"inlinee":10769,"source_line":542,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":392,"record_offset":10384,"parent_record_offset":10288,"ancestor_record_offset":2660,"inlinee":7664,"source_line":580,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":392,"record_offset":10288,"parent_record_offset":10252,"ancestor_record_offset":2660,"inlinee":7632,"source_line":20,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":392,"record_offset":3048,"parent_record_offset":3020,"ancestor_record_offset":2660,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":392,"record_offset":3408,"parent_record_offset":3380,"ancestor_record_offset":2660,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":392,"record_offset":3548,"parent_record_offset":3516,"ancestor_record_offset":2660,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":392,"record_offset":3516,"parent_record_offset":3480,"ancestor_record_offset":2660,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":392,"record_offset":3456,"parent_record_offset":3432,"ancestor_record_offset":2660,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":392,"record_offset":3480,"parent_record_offset":3456,"ancestor_record_offset":2660,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":392,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":2660,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":392,"record_offset":4020,"parent_record_offset":3480,"ancestor_record_offset":2660,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":392,"record_offset":3020,"parent_record_offset":2988,"ancestor_record_offset":2660,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":392,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":2660,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":392,"record_offset":3180,"parent_record_offset":3148,"ancestor_record_offset":2660,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":392,"record_offset":3276,"parent_record_offset":3180,"ancestor_record_offset":2660,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":392,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":2660,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":392,"record_offset":15716,"parent_record_offset":15688,"ancestor_record_offset":2660,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":392,"record_offset":2964,"parent_record_offset":2932,"ancestor_record_offset":2660,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":392,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":2660,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":392,"record_offset":3580,"parent_record_offset":3548,"ancestor_record_offset":2660,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":392,"record_offset":3864,"parent_record_offset":3580,"ancestor_record_offset":2660,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":392,"record_offset":3836,"parent_record_offset":3580,"ancestor_record_offset":2660,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":392,"record_offset":3640,"parent_record_offset":3580,"ancestor_record_offset":2660,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":392,"record_offset":18780,"parent_record_offset":18720,"ancestor_record_offset":2660,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":392,"record_offset":16048,"parent_record_offset":16012,"ancestor_record_offset":2660,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":392,"record_offset":20076,"parent_record_offset":20052,"ancestor_record_offset":2660,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":392,"record_offset":18804,"parent_record_offset":18780,"ancestor_record_offset":2660,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":392,"record_offset":16072,"parent_record_offset":16048,"ancestor_record_offset":2660,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":392,"record_offset":20100,"parent_record_offset":20076,"ancestor_record_offset":2660,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":392,"record_offset":15648,"parent_record_offset":15620,"ancestor_record_offset":2660,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":392,"record_offset":16228,"parent_record_offset":16200,"ancestor_record_offset":2660,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":392,"record_offset":18696,"parent_record_offset":18672,"ancestor_record_offset":2660,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$11::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":392,"record_offset":10652,"parent_record_offset":10628,"ancestor_record_offset":2660,"inlinee":24458,"source_line":69,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":392,"record_offset":11912,"parent_record_offset":11888,"ancestor_record_offset":2660,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":392,"record_offset":17436,"parent_record_offset":17044,"ancestor_record_offset":2660,"inlinee":24469,"source_line":129,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":392,"record_offset":17548,"parent_record_offset":17044,"ancestor_record_offset":2660,"inlinee":24474,"source_line":538,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17460,"parent_record_offset":17436,"ancestor_record_offset":2660,"inlinee":24471,"source_line":152,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17604,"parent_record_offset":17580,"ancestor_record_offset":2660,"inlinee":24477,"source_line":646,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17660,"parent_record_offset":17636,"ancestor_record_offset":2660,"inlinee":24480,"source_line":636,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":2660,"inlinee":24472,"source_line":70,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17580,"parent_record_offset":17548,"ancestor_record_offset":2660,"inlinee":24476,"source_line":781,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17636,"parent_record_offset":17604,"ancestor_record_offset":2660,"inlinee":24479,"source_line":796,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":17344,"parent_record_offset":17044,"ancestor_record_offset":2660,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":2660,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":2660,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$19::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":17688,"parent_record_offset":17660,"ancestor_record_offset":2660,"inlinee":24482,"source_line":1199,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":17744,"parent_record_offset":17660,"ancestor_record_offset":2660,"inlinee":24484,"source_line":1323,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":15248,"parent_record_offset":14904,"ancestor_record_offset":2660,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":15340,"parent_record_offset":14904,"ancestor_record_offset":2660,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":15160,"parent_record_offset":14904,"ancestor_record_offset":2660,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":392,"record_offset":18004,"parent_record_offset":17920,"ancestor_record_offset":2660,"inlinee":24486,"source_line":172,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":392,"record_offset":11540,"parent_record_offset":11512,"ancestor_record_offset":2660,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":392,"record_offset":11572,"parent_record_offset":11540,"ancestor_record_offset":2660,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":392,"record_offset":11512,"parent_record_offset":11260,"ancestor_record_offset":2660,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":392,"record_offset":11372,"parent_record_offset":11260,"ancestor_record_offset":2660,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":392,"record_offset":11984,"parent_record_offset":11260,"ancestor_record_offset":2660,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":392,"record_offset":11456,"parent_record_offset":11260,"ancestor_record_offset":2660,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":392,"record_offset":11344,"parent_record_offset":11260,"ancestor_record_offset":2660,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":392,"record_offset":11864,"parent_record_offset":11840,"ancestor_record_offset":2660,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":392,"record_offset":11128,"parent_record_offset":11092,"ancestor_record_offset":2660,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":392,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":2660,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":392,"record_offset":11840,"parent_record_offset":11672,"ancestor_record_offset":2660,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":392,"record_offset":11184,"parent_record_offset":11152,"ancestor_record_offset":2660,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":392,"record_offset":11780,"parent_record_offset":11696,"ancestor_record_offset":2660,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":392,"record_offset":12736,"parent_record_offset":12712,"ancestor_record_offset":2660,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":392,"record_offset":11724,"parent_record_offset":11696,"ancestor_record_offset":2660,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":392,"record_offset":12076,"parent_record_offset":11260,"ancestor_record_offset":2660,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":392,"record_offset":11696,"parent_record_offset":11672,"ancestor_record_offset":2660,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":392,"record_offset":11236,"parent_record_offset":11212,"ancestor_record_offset":2660,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":392,"record_offset":11212,"parent_record_offset":11152,"ancestor_record_offset":2660,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":392,"record_offset":11260,"parent_record_offset":11236,"ancestor_record_offset":2660,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":392,"record_offset":11316,"parent_record_offset":11260,"ancestor_record_offset":2660,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":392,"record_offset":11672,"parent_record_offset":11260,"ancestor_record_offset":2660,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":392,"record_offset":16012,"parent_record_offset":15896,"ancestor_record_offset":2660,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":392,"record_offset":18720,"parent_record_offset":18696,"ancestor_record_offset":2660,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":392,"record_offset":16104,"parent_record_offset":16012,"ancestor_record_offset":2660,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":392,"record_offset":18752,"parent_record_offset":18720,"ancestor_record_offset":2660,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":392,"record_offset":4392,"parent_record_offset":4356,"ancestor_record_offset":2660,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":392,"record_offset":20640,"parent_record_offset":20608,"ancestor_record_offset":2660,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":392,"record_offset":21156,"parent_record_offset":20896,"ancestor_record_offset":2660,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":392,"record_offset":4536,"parent_record_offset":4508,"ancestor_record_offset":2660,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":392,"record_offset":4612,"parent_record_offset":4464,"ancestor_record_offset":2660,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":392,"record_offset":4872,"parent_record_offset":4612,"ancestor_record_offset":2660,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":11092,"parent_record_offset":11060,"ancestor_record_offset":2660,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":392,"record_offset":4564,"parent_record_offset":4536,"ancestor_record_offset":2660,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":392,"record_offset":4464,"parent_record_offset":4428,"ancestor_record_offset":2660,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":392,"record_offset":4428,"parent_record_offset":4392,"ancestor_record_offset":2660,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::place_pillars::place_pillars::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":392,"record_offset":18640,"parent_record_offset":17044,"ancestor_record_offset":2660,"inlinee":24488,"source_line":181,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":392,"record_offset":14844,"parent_record_offset":14820,"ancestor_record_offset":2660,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":392,"record_offset":14964,"parent_record_offset":14904,"ancestor_record_offset":2660,"inlinee":24424,"source_line":226,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::place_pillars::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FA8
// 首条位置: {"module":392,"inlinee":24488}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::place_pillars::closure$4
// 记录形式: compiled_procedure; instances=1
// 源码行: 205
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1426,"record_offset":272,"rva":18928448,"va":5387637568}
// 编译实例 RVAs: 0x120d340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140858790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10178017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387637568,"raw_hex":"e81fb264ff","size":5,"soft_fail":false,"target_va":5377460112,"va":5387638124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408587c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10177991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387637568,"raw_hex":"e839b264ff","size":5,"soft_fail":false,"target_va":5377460160,"va":5387638146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5703166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387637568,"raw_hex":"e802faa8ff","size":5,"soft_fail":false,"target_va":5381934528,"va":5387637689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: place_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387637568,"raw_hex":"e830ebffff","size":5,"soft_fail":false,"target_va":5387632928,"va":5387638251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_mat3_translation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9896085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387637568,"raw_hex":"e86bff68ff","size":5,"soft_fail":false,"target_va":5377742032,"va":5387638112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1426,"record_offset":1840,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1426,"record_offset":484,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70879,"source_line":150,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1426,"record_offset":512,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":14180,"source_line":706,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":440,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8865,"source_line":217,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":1576,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1426,"record_offset":1812,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1426,"record_offset":848,"parent_record_offset":772,"ancestor_record_offset":272,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1426,"record_offset":732,"parent_record_offset":700,"ancestor_record_offset":272,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1426,"record_offset":1504,"parent_record_offset":544,"ancestor_record_offset":272,"inlinee":14186,"source_line":86,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1426,"record_offset":544,"parent_record_offset":512,"ancestor_record_offset":272,"inlinee":14182,"source_line":571,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1426,"record_offset":644,"parent_record_offset":604,"ancestor_record_offset":272,"inlinee":14185,"source_line":554,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":1426,"record_offset":604,"parent_record_offset":544,"ancestor_record_offset":272,"inlinee":14184,"source_line":721,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":1676,"parent_record_offset":1640,"ancestor_record_offset":272,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":1640,"parent_record_offset":1604,"ancestor_record_offset":272,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":1604,"parent_record_offset":1576,"ancestor_record_offset":272,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":700,"parent_record_offset":644,"ancestor_record_offset":272,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1426,"record_offset":1084,"parent_record_offset":1052,"ancestor_record_offset":272,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1426,"record_offset":1712,"parent_record_offset":1676,"ancestor_record_offset":272,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::emit_pillar_at_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 276
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F93
// 首条位置: {"module":392,"inlinee":24467}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":392,"record_offset":17176,"parent_record_offset":17044,"ancestor_record_offset":17044,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::Affine3A::from_scale
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":392,"record_offset":17436,"parent_record_offset":17044,"ancestor_record_offset":17044,"inlinee":24469,"source_line":129,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine3a::impl$8::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":392,"record_offset":17548,"parent_record_offset":17044,"ancestor_record_offset":17044,"inlinee":24474,"source_line":538,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":17344,"parent_record_offset":17044,"ancestor_record_offset":17044,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":17108,"parent_record_offset":17044,"ancestor_record_offset":17044,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::place_pillars::place_pillars::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":392,"record_offset":18640,"parent_record_offset":17044,"ancestor_record_offset":17044,"inlinee":24488,"source_line":181,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":17768,"parent_record_offset":17744,"ancestor_record_offset":17044,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":17372,"parent_record_offset":17344,"ancestor_record_offset":17044,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":17400,"parent_record_offset":17344,"ancestor_record_offset":17044,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":17144,"parent_record_offset":17108,"ancestor_record_offset":17044,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_diagonal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17460,"parent_record_offset":17436,"ancestor_record_offset":17044,"inlinee":24471,"source_line":152,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17604,"parent_record_offset":17580,"ancestor_record_offset":17044,"inlinee":24477,"source_line":646,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17660,"parent_record_offset":17636,"ancestor_record_offset":17044,"inlinee":24480,"source_line":636,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":17044,"inlinee":24472,"source_line":70,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17580,"parent_record_offset":17548,"ancestor_record_offset":17044,"inlinee":24476,"source_line":781,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":392,"record_offset":17636,"parent_record_offset":17604,"ancestor_record_offset":17044,"inlinee":24479,"source_line":796,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":17044,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":17044,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$19::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":17688,"parent_record_offset":17660,"ancestor_record_offset":17044,"inlinee":24482,"source_line":1199,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$35::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":17744,"parent_record_offset":17660,"ancestor_record_offset":17044,"inlinee":24484,"source_line":1323,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec3a_impl::impl$0::yyy
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec3a_impl.rs
//     evidence={"module":392,"record_offset":18004,"parent_record_offset":17920,"ancestor_record_offset":17044,"inlinee":24486,"source_line":172,"archive":"_pdb/B/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::place_pillars::assemble_stone_pillar::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 376
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11492
// 首条位置: {"module":1415,"inlinee":70802}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$183::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1426,"record_offset":3560,"parent_record_offset":3516,"ancestor_record_offset":3516,"inlinee":24913,"source_line":35,"archive":"_pdb/B/modules/01426.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1426,"record_offset":3604,"parent_record_offset":3560,"ancestor_record_offset":3516,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01426.jsonl.gz"}
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
// 完整结构及原始类型记录: place_pillars.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::place_pillars::PillarPlacementProposal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::place_pillars::PillarPlacementProposal,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::place_pillars::PillarPlacementProposal", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::place_pillars::PillarProposals
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: place_pillars.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: place_pillars.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::PillarProposals::add_proposal
// 信息 ID: bad1d87085bc9f898bdf87cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7610f3 -> 0x7534d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=396, inline_record=19948; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::construct_lintels; module=402, inline_record=17036; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::construct_lintels; module=402, inline_record=18672; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::PillarProposals::empty
// 信息 ID: a21f92103767d8574c848077
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=390, inline_record=17240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::assemble_stone_pillar::closure$0
// 信息 ID: b47817403478a172da658b68
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::place_pillars::assemble_stone_pillar", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>; module=1426, inline_record=3516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>
// 信息 ID: 6ce1d0ec0230731de2ea9a04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::place_pillars::assemble_stone_pillar -> ["system_wall_constructor::place_pillars::place_pillars::closure_env$4"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x7547de -> 0x120d670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::emit_pillar_at_point
// 信息 ID: 42256a2865ecdcaee624016a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=392, inline_record=17044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::impl$1::clone
// 信息 ID: 3214c2e81eb4e14e9983f1ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$11::clone<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>; module=1415, inline_record=17876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::impl$2::clone
// 信息 ID: c9ff6f7cb7bdc4bf7177fe1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$11::clone<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>; module=1415, inline_record=16460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::place_pillars
// 信息 ID: 26b9224e331adb70ff45c6c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 264ecb8d94e725024c747e58: void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)> > @ 0x336f3a -> 0x753670 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_wall_constructor::WallPathSegmentationMasksMinusStairs>)> > > @ 0x469f8c -> 0x753670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::place_pillars::closure$3
// 信息 ID: b8b28ba6fadc4b383bc35c1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::place_pillars::place_pillars", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=392, inline_record=18640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::place_pillars::place_pillars::closure$4
// 信息 ID: 486ca4dbf9ef1b1cafee611f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::place_pillars::place_pillars", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::place_pillars::place_pillars::closure_env$4 <- system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4> @ 0x120db9e -> 0x120d340 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4> @ 0x120dc22 -> 0x120d340 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

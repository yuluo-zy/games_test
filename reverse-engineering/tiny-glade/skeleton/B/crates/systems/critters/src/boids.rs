// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/critters/src/boids.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d04e4692f377357466cb2473d37ef79f22acaabd8a6eb2f1858ed37e876c3a3f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: boids.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 331, 1177, 2739, 2746, 2747, 3894
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lazy_static; // namespace candidate
//   package identities: [{"name":"lazy_static","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_critters; // namespace candidate
//   package identities: [{"name":"lazy_static","source":"PDB_registry_path_only","version":"1.4.0"},{"name":"system-critters","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::boids::impl$5::clone
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF161
// 首条位置: {"module":1177,"inlinee":61793}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$101::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1177,"record_offset":5276,"parent_record_offset":5244,"ancestor_record_offset":5244,"inlinee":25047,"source_line":17,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::impl$6::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4CEA
// 首条位置: {"module":331,"inlinee":19690}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::push
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF15E
// 首条位置: {"module":1177,"inlinee":61790}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1177,"record_offset":4020,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1177,"record_offset":4088,"parent_record_offset":4056,"ancestor_record_offset":3992,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1177,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":3992,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1177,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":3992,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1177,"record_offset":4056,"parent_record_offset":4020,"ancestor_record_offset":3992,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1177,"record_offset":4176,"parent_record_offset":4020,"ancestor_record_offset":3992,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::drain
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1853B
// 首条位置: {"module":3894,"inlinee":99643}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3894,"record_offset":5892,"parent_record_offset":5864,"ancestor_record_offset":5864,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3894,"record_offset":5920,"parent_record_offset":5864,"ancestor_record_offset":5864,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3894,"record_offset":5980,"parent_record_offset":5956,"ancestor_record_offset":5864,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3894,"record_offset":6028,"parent_record_offset":6004,"ancestor_record_offset":5864,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3894,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":5864,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3894,"record_offset":5956,"parent_record_offset":5920,"ancestor_record_offset":5864,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3894,"record_offset":6068,"parent_record_offset":5920,"ancestor_record_offset":5864,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3894,"record_offset":6096,"parent_record_offset":5920,"ancestor_record_offset":5864,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3894,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":5864,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3894,"record_offset":6124,"parent_record_offset":5920,"ancestor_record_offset":5864,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3894,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":5864,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::len
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF163
// 首条位置: {"module":1177,"inlinee":61795}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::clone_state
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF15F
// 首条位置: {"module":1177,"inlinee":61791}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1177,"record_offset":4440,"parent_record_offset":4408,"ancestor_record_offset":4408,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1177,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":4408,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1177,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":4408,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1177,"record_offset":4916,"parent_record_offset":4732,"ancestor_record_offset":4408,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3894,"record_offset":16372,"parent_record_offset":16348,"ancestor_record_offset":16196,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1177,"record_offset":4660,"parent_record_offset":4624,"ancestor_record_offset":4408,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3894,"record_offset":16420,"parent_record_offset":16396,"ancestor_record_offset":16196,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3894,"record_offset":16396,"parent_record_offset":16372,"ancestor_record_offset":16196,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1177,"record_offset":4732,"parent_record_offset":4696,"ancestor_record_offset":4408,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1177,"record_offset":4696,"parent_record_offset":4660,"ancestor_record_offset":4408,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1177,"record_offset":4776,"parent_record_offset":4732,"ancestor_record_offset":4408,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1177,"record_offset":4572,"parent_record_offset":4540,"ancestor_record_offset":4408,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1177,"record_offset":4540,"parent_record_offset":4472,"ancestor_record_offset":4408,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3894,"record_offset":16348,"parent_record_offset":16324,"ancestor_record_offset":16196,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03894.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1177,"record_offset":4512,"parent_record_offset":4472,"ancestor_record_offset":4408,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1177,"record_offset":4624,"parent_record_offset":4572,"ancestor_record_offset":4408,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1177,"record_offset":4472,"parent_record_offset":4440,"ancestor_record_offset":4408,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1177,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4408,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1177,"record_offset":4824,"parent_record_offset":4800,"ancestor_record_offset":4408,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1177,"record_offset":5056,"parent_record_offset":5016,"ancestor_record_offset":4408,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":1177,"record_offset":5016,"parent_record_offset":4572,"ancestor_record_offset":4408,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1177,"record_offset":5308,"parent_record_offset":4572,"ancestor_record_offset":4408,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1177,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":4408,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1177,"record_offset":4872,"parent_record_offset":4848,"ancestor_record_offset":4408,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1177,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":4408,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1177,"record_offset":5120,"parent_record_offset":5084,"ancestor_record_offset":4408,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1177,"record_offset":5172,"parent_record_offset":5148,"ancestor_record_offset":4408,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1177,"record_offset":5084,"parent_record_offset":5056,"ancestor_record_offset":4408,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1177,"record_offset":5148,"parent_record_offset":5084,"ancestor_record_offset":4408,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$101::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1177,"record_offset":5276,"parent_record_offset":5244,"ancestor_record_offset":4408,"inlinee":25047,"source_line":17,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":1177,"record_offset":5244,"parent_record_offset":4572,"ancestor_record_offset":4408,"inlinee":61793,"source_line":61,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::iter_mut
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF162
// 首条位置: {"module":1177,"inlinee":61794}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1177,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":5384,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1177,"record_offset":5504,"parent_record_offset":5480,"ancestor_record_offset":5384,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1177,"record_offset":5480,"parent_record_offset":5456,"ancestor_record_offset":5384,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1177,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":5384,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1177,"record_offset":5456,"parent_record_offset":5432,"ancestor_record_offset":5384,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/01177.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::remove<system_critters::fish::fish::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2747,"record_offset":264,"rva":31204160,"va":5399913280}
// 编译实例 RVAs: 0x1dc2340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: boids.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24832156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399913280,"raw_hex":"e8641785fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5399913655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::swap_remove::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11627327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399913280,"raw_hex":"e83f6bb100","size":5,"soft_fail":false,"target_va":5411541017,"va":5399913685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17659075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399913280,"raw_hex":"e83d8bf2fe","size":5,"soft_fail":false,"target_va":5382254496,"va":5399913566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17489195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399913280,"raw_hex":"e8d522f5fe","size":5,"soft_fail":false,"target_va":5382424432,"va":5399913622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2747,"record_offset":776,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5532,"source_line":1927,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2747,"record_offset":660,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2747,"record_offset":1180,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2747,"record_offset":456,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":2747,"record_offset":540,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] system_critters::fish::fish::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2747,"record_offset":1120,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":99651,"source_line":283,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2747,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2747,"record_offset":1436,"parent_record_offset":1348,"ancestor_record_offset":264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2747,"record_offset":1376,"parent_record_offset":1348,"ancestor_record_offset":264,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2747,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":264,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2747,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":264,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2747,"record_offset":820,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2747,"record_offset":912,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2747,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":264,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2747,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":264,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2747,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":264,"inlinee":21009,"source_line":371,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":2747,"record_offset":1148,"parent_record_offset":1120,"ancestor_record_offset":264,"inlinee":24591,"source_line":455,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":2747,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":264,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2747,"record_offset":1404,"parent_record_offset":1376,"ancestor_record_offset":264,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2747,"record_offset":848,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2747,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2747,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2747,"record_offset":620,"parent_record_offset":588,"ancestor_record_offset":264,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2747,"record_offset":740,"parent_record_offset":684,"ancestor_record_offset":264,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2747,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":264,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::BoidFlock::remove<system_critters::wren_boids::sim_wren_boids::closure_env$6>
// 记录形式: compiled_procedure; instances=1
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2747,"record_offset":1872,"rva":31204672,"va":5399913792}
// 编译实例 RVAs: 0x1dc2540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: boids.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24832668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399913792,"raw_hex":"e8641585fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5399914167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::swap_remove::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11626815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399913792,"raw_hex":"e83f69b100","size":5,"soft_fail":false,"target_va":5411541017,"va":5399914197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17659587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399913792,"raw_hex":"e83d89f2fe","size":5,"soft_fail":false,"target_va":5382254496,"va":5399914078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17489707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399913792,"raw_hex":"e8d520f5fe","size":5,"soft_fail":false,"target_va":5382424432,"va":5399914134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2747,"record_offset":2400,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":5532,"source_line":1927,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2747,"record_offset":2284,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":5007,"source_line":33,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2747,"record_offset":2804,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2747,"record_offset":2080,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":2747,"record_offset":2164,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] system_critters::wren_boids::sim_wren_boids::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/wren_boids.rs
//     evidence={"module":2747,"record_offset":2744,"parent_record_offset":1872,"ancestor_record_offset":1872,"inlinee":99652,"source_line":464,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2747,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":1872,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2747,"record_offset":3060,"parent_record_offset":2972,"ancestor_record_offset":1872,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2747,"record_offset":3000,"parent_record_offset":2972,"ancestor_record_offset":1872,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2747,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":1872,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2747,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":1872,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2747,"record_offset":2444,"parent_record_offset":2400,"ancestor_record_offset":1872,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2747,"record_offset":2536,"parent_record_offset":2400,"ancestor_record_offset":1872,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2747,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":1872,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2747,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":1872,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2747,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":1872,"inlinee":21009,"source_line":371,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":2747,"record_offset":2772,"parent_record_offset":2744,"ancestor_record_offset":1872,"inlinee":24591,"source_line":455,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":2747,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":1872,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2747,"record_offset":3028,"parent_record_offset":3000,"ancestor_record_offset":1872,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2747,"record_offset":2472,"parent_record_offset":2400,"ancestor_record_offset":1872,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2747,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":1872,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2747,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":1872,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2747,"record_offset":2244,"parent_record_offset":2212,"ancestor_record_offset":1872,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2747,"record_offset":2364,"parent_record_offset":2308,"ancestor_record_offset":1872,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2747,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":1872,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":1872,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::new
// 记录形式: inlinee_declaration; instances=1
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18537
// 首条位置: {"module":2746,"inlinee":99639}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::unobstructed_dir
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18546
// 首条位置: {"module":2747,"inlinee":99654}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::function::impls::impl$0::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2747,"record_offset":7864,"parent_record_offset":6968,"ancestor_record_offset":6968,"inlinee":41965,"source_line":261,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":8100,"parent_record_offset":6968,"ancestor_record_offset":6968,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":2747,"record_offset":7352,"parent_record_offset":6968,"ancestor_record_offset":6968,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":7028,"parent_record_offset":6968,"ancestor_record_offset":6968,"inlinee":12697,"source_line":538,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2747,"record_offset":7060,"parent_record_offset":6968,"ancestor_record_offset":6968,"inlinee":99642,"source_line":135,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2747,"record_offset":7284,"parent_record_offset":7124,"ancestor_record_offset":6968,"inlinee":19551,"source_line":592,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2747,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":6968,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7592,"parent_record_offset":7352,"ancestor_record_offset":6968,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7500,"parent_record_offset":7444,"ancestor_record_offset":6968,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7392,"parent_record_offset":7352,"ancestor_record_offset":6968,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7680,"parent_record_offset":7620,"ancestor_record_offset":6968,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7740,"parent_record_offset":7620,"ancestor_record_offset":6968,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":8032,"parent_record_offset":8004,"ancestor_record_offset":6968,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":7944,"parent_record_offset":7916,"ancestor_record_offset":6968,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2747,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":6968,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2747,"record_offset":8216,"parent_record_offset":8100,"ancestor_record_offset":6968,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":8156,"parent_record_offset":8132,"ancestor_record_offset":6968,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":8132,"parent_record_offset":8100,"ancestor_record_offset":6968,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2747,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":6968,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2747,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":6968,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":2747,"record_offset":7620,"parent_record_offset":7352,"ancestor_record_offset":6968,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":7916,"parent_record_offset":7888,"ancestor_record_offset":6968,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":8004,"parent_record_offset":7888,"ancestor_record_offset":6968,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2747,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":6968,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2747,"record_offset":7420,"parent_record_offset":7352,"ancestor_record_offset":6968,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":2747,"record_offset":7124,"parent_record_offset":7092,"ancestor_record_offset":6968,"inlinee":19548,"source_line":26,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::once::Once::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\once.rs
//     evidence={"module":2747,"record_offset":7160,"parent_record_offset":7124,"ancestor_record_offset":6968,"inlinee":19549,"source_line":145,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\once\futex.rs
//     evidence={"module":2747,"record_offset":7196,"parent_record_offset":7160,"ancestor_record_offset":6968,"inlinee":19550,"source_line":77,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2747,"record_offset":7092,"parent_record_offset":7060,"ancestor_record_offset":6968,"inlinee":99640,"source_line":140,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2747,"record_offset":7888,"parent_record_offset":7864,"ancestor_record_offset":6968,"inlinee":99591,"source_line":196,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::apply_rules
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18545
// 首条位置: {"module":2747,"inlinee":99653}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":9304,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":6456,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":6428,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2747,"record_offset":4104,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2747,"record_offset":8252,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":9028,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":29375,"source_line":830,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":6200,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4452,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5748,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4388,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5252,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":6540,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5524,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4891,"source_line":573,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::div_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4976,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":9338,"source_line":1140,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4796,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":9420,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":3988,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4280,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4884,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4636,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2747,"record_offset":6484,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2747,"record_offset":4576,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::Boid::unobstructed_dir
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":2747,"record_offset":6968,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":99654,"source_line":156,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2747,"record_offset":6760,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":99591,"source_line":196,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2747,"record_offset":4732,"parent_record_offset":3712,"ancestor_record_offset":3712,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2747,"record_offset":7284,"parent_record_offset":7124,"ancestor_record_offset":3712,"inlinee":19551,"source_line":592,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2747,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":3712,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7592,"parent_record_offset":7352,"ancestor_record_offset":3712,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7500,"parent_record_offset":7444,"ancestor_record_offset":3712,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7392,"parent_record_offset":7352,"ancestor_record_offset":3712,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7680,"parent_record_offset":7620,"ancestor_record_offset":3712,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7740,"parent_record_offset":7620,"ancestor_record_offset":3712,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":4760,"parent_record_offset":4732,"ancestor_record_offset":3712,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":2747,"record_offset":9384,"parent_record_offset":9304,"ancestor_record_offset":3712,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2747,"record_offset":9356,"parent_record_offset":9304,"ancestor_record_offset":3712,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":5460,"parent_record_offset":5428,"ancestor_record_offset":3712,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":4824,"parent_record_offset":4796,"ancestor_record_offset":3712,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":4668,"parent_record_offset":4636,"ancestor_record_offset":3712,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":4028,"parent_record_offset":3988,"ancestor_record_offset":3712,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":4916,"parent_record_offset":4884,"ancestor_record_offset":3712,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":5008,"parent_record_offset":4976,"ancestor_record_offset":3712,"inlinee":9340,"source_line":943,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":4320,"parent_record_offset":4280,"ancestor_record_offset":3712,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$0::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2747,"record_offset":7864,"parent_record_offset":6968,"ancestor_record_offset":3712,"inlinee":41965,"source_line":261,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2747,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":3712,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2747,"record_offset":4228,"parent_record_offset":4160,"ancestor_record_offset":3712,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":8156,"parent_record_offset":8132,"ancestor_record_offset":3712,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":4160,"parent_record_offset":4104,"ancestor_record_offset":3712,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":8132,"parent_record_offset":8100,"ancestor_record_offset":3712,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2747,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":3712,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2747,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":3712,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":2747,"record_offset":9116,"parent_record_offset":9028,"ancestor_record_offset":3712,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":2747,"record_offset":7352,"parent_record_offset":6968,"ancestor_record_offset":3712,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":2747,"record_offset":7620,"parent_record_offset":7352,"ancestor_record_offset":3712,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4416,"parent_record_offset":4388,"ancestor_record_offset":3712,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5628,"parent_record_offset":5600,"ancestor_record_offset":3712,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5600,"parent_record_offset":5560,"ancestor_record_offset":3712,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":9060,"parent_record_offset":9028,"ancestor_record_offset":3712,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5560,"parent_record_offset":5524,"ancestor_record_offset":3712,"inlinee":4892,"source_line":556,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":7028,"parent_record_offset":6968,"ancestor_record_offset":3712,"inlinee":12697,"source_line":538,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5280,"parent_record_offset":5252,"ancestor_record_offset":3712,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5428,"parent_record_offset":5252,"ancestor_record_offset":3712,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":6232,"parent_record_offset":6200,"ancestor_record_offset":3712,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2747,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":3712,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2747,"record_offset":7420,"parent_record_offset":7352,"ancestor_record_offset":3712,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":2747,"record_offset":7124,"parent_record_offset":7092,"ancestor_record_offset":3712,"inlinee":19548,"source_line":26,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2747,"record_offset":5700,"parent_record_offset":5664,"ancestor_record_offset":3712,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::once::Once::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\once.rs
//     evidence={"module":2747,"record_offset":7160,"parent_record_offset":7124,"ancestor_record_offset":3712,"inlinee":19549,"source_line":145,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\once\futex.rs
//     evidence={"module":2747,"record_offset":7196,"parent_record_offset":7160,"ancestor_record_offset":3712,"inlinee":19550,"source_line":77,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2747,"record_offset":7060,"parent_record_offset":6968,"ancestor_record_offset":3712,"inlinee":99642,"source_line":135,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2747,"record_offset":7092,"parent_record_offset":7060,"ancestor_record_offset":3712,"inlinee":99640,"source_line":140,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2747,"record_offset":7888,"parent_record_offset":7864,"ancestor_record_offset":3712,"inlinee":99591,"source_line":196,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2747,"record_offset":3496,"rva":31205184,"va":5399914304}
// 编译实例 RVAs: 0x1dc2740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14df0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: boids.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20635751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399914304,"raw_hex":"e8991fc5fe","size":5,"soft_fail":false,"target_va":5379280368,"va":5399916114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::f32::_$LT$impl$u20$f32$GT$::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17006659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399914304,"raw_hex":"e8bd7ffcfe","size":5,"soft_fail":false,"target_va":5382911104,"va":5399917758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17268900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399914304,"raw_hex":"e85c7ff8fe","size":5,"soft_fail":false,"target_va":5382647344,"va":5399916239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::once::futex::Once::call
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11621149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399914304,"raw_hex":"e81d53b100","size":5,"soft_fail":false,"target_va":5411538976,"va":5399917822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_critters::boids::Boid::random_vel_change
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399914304,"raw_hex":"e887f9ffff","size":5,"soft_fail":false,"target_va":5399912816,"va":5399914468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18034077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399914304,"raw_hex":"e863d2ecfe","size":5,"soft_fail":false,"target_va":5381883088,"va":5399917160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18031403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399914304,"raw_hex":"e8d5dcecfe","size":5,"soft_fail":false,"target_va":5381883104,"va":5399914502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_vec3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18032556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399914304,"raw_hex":"e854d8ecfe","size":5,"soft_fail":false,"target_va":5381883392,"va":5399915943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2747,"record_offset":9512,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":9800,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":99655,"source_line":810,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":9544,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":10080,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":25834,"source_line":420,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":10252,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":12697,"source_line":538,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":10164,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":10896,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":10624,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":10712,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":13727,"source_line":1809,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::Boid::apply_rules
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":2747,"record_offset":3712,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":99653,"source_line":186,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2747,"record_offset":10416,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":99591,"source_line":196,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2747,"record_offset":9740,"parent_record_offset":3496,"ancestor_record_offset":3496,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2747,"record_offset":7284,"parent_record_offset":7124,"ancestor_record_offset":3496,"inlinee":19551,"source_line":592,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2747,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":3496,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7592,"parent_record_offset":7352,"ancestor_record_offset":3496,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7500,"parent_record_offset":7444,"ancestor_record_offset":3496,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7392,"parent_record_offset":7352,"ancestor_record_offset":3496,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7680,"parent_record_offset":7620,"ancestor_record_offset":3496,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":7740,"parent_record_offset":7620,"ancestor_record_offset":3496,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":3496,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":4760,"parent_record_offset":4732,"ancestor_record_offset":3496,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":2747,"record_offset":9384,"parent_record_offset":9304,"ancestor_record_offset":3496,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":10104,"parent_record_offset":10080,"ancestor_record_offset":3496,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":6456,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":6428,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2747,"record_offset":9356,"parent_record_offset":9304,"ancestor_record_offset":3496,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2747,"record_offset":4104,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":5460,"parent_record_offset":5428,"ancestor_record_offset":3496,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":4824,"parent_record_offset":4796,"ancestor_record_offset":3496,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":4668,"parent_record_offset":4636,"ancestor_record_offset":3496,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":10744,"parent_record_offset":10712,"ancestor_record_offset":3496,"inlinee":11306,"source_line":694,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":4028,"parent_record_offset":3988,"ancestor_record_offset":3496,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":4916,"parent_record_offset":4884,"ancestor_record_offset":3496,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":5008,"parent_record_offset":4976,"ancestor_record_offset":3496,"inlinee":9340,"source_line":943,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":4320,"parent_record_offset":4280,"ancestor_record_offset":3496,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$0::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2747,"record_offset":7864,"parent_record_offset":6968,"ancestor_record_offset":3496,"inlinee":41965,"source_line":261,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2747,"record_offset":8252,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2747,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":3496,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2747,"record_offset":4228,"parent_record_offset":4160,"ancestor_record_offset":3496,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":8156,"parent_record_offset":8132,"ancestor_record_offset":3496,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":4160,"parent_record_offset":4104,"ancestor_record_offset":3496,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":8132,"parent_record_offset":8100,"ancestor_record_offset":3496,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2747,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":3496,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2747,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":3496,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":2747,"record_offset":9116,"parent_record_offset":9028,"ancestor_record_offset":3496,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":2747,"record_offset":7352,"parent_record_offset":6968,"ancestor_record_offset":3496,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":2747,"record_offset":7620,"parent_record_offset":7352,"ancestor_record_offset":3496,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":9028,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":29375,"source_line":830,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":6200,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4416,"parent_record_offset":4388,"ancestor_record_offset":3496,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5628,"parent_record_offset":5600,"ancestor_record_offset":3496,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5600,"parent_record_offset":5560,"ancestor_record_offset":3496,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4388,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5252,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":6540,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5560,"parent_record_offset":5524,"ancestor_record_offset":3496,"inlinee":4892,"source_line":556,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5524,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":4891,"source_line":573,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":7028,"parent_record_offset":6968,"ancestor_record_offset":3496,"inlinee":12697,"source_line":538,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::div_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4976,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":9338,"source_line":1140,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4796,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":5428,"parent_record_offset":5252,"ancestor_record_offset":3496,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":3988,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4280,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4884,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":4636,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2747,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":3496,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2747,"record_offset":7420,"parent_record_offset":7352,"ancestor_record_offset":3496,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":2747,"record_offset":7124,"parent_record_offset":7092,"ancestor_record_offset":3496,"inlinee":19548,"source_line":26,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2747,"record_offset":6484,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2747,"record_offset":4576,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::once::Once::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\once.rs
//     evidence={"module":2747,"record_offset":7160,"parent_record_offset":7124,"ancestor_record_offset":3496,"inlinee":19549,"source_line":145,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\once\futex.rs
//     evidence={"module":2747,"record_offset":7196,"parent_record_offset":7160,"ancestor_record_offset":3496,"inlinee":19550,"source_line":77,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::Boid::unobstructed_dir
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":2747,"record_offset":6968,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":99654,"source_line":156,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2747,"record_offset":7060,"parent_record_offset":6968,"ancestor_record_offset":3496,"inlinee":99642,"source_line":135,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2747,"record_offset":7092,"parent_record_offset":7060,"ancestor_record_offset":3496,"inlinee":99640,"source_line":140,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::fish::fish::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/fish.rs
//     evidence={"module":2747,"record_offset":6760,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":99591,"source_line":196,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2747,"record_offset":4732,"parent_record_offset":3712,"ancestor_record_offset":3496,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2747,"record_offset":10988,"rva":31208720,"va":5399917840}
// 编译实例 RVAs: 0x1dc3510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141db7f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: boids.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-48858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399917840,"raw_hex":"e82641ffff","size":5,"soft_fail":false,"target_va":5399871264,"va":5399920117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::f32::_$LT$impl$u20$f32$GT$::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17010215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399917840,"raw_hex":"e8d971fcfe","size":5,"soft_fail":false,"target_va":5382911104,"va":5399921314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17272466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399917840,"raw_hex":"e86e71f8fe","size":5,"soft_fail":false,"target_va":5382647344,"va":5399919805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::once::futex::Once::call
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11617593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399917840,"raw_hex":"e83945b100","size":5,"soft_fail":false,"target_va":5411538976,"va":5399921378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_critters::boids::Boid::random_vel_change
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399917840,"raw_hex":"e8a3ebffff","size":5,"soft_fail":false,"target_va":5399912816,"va":5399918024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_critters::wren_boids::sim_wren_boids::closure$3
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399917840,"raw_hex":"e894060000","size":5,"soft_fail":false,"target_va":5399921392,"va":5399919703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18037638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399917840,"raw_hex":"e87ac4ecfe","size":5,"soft_fail":false,"target_va":5381883088,"va":5399920721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18034960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399917840,"raw_hex":"e8f0ceecfe","size":5,"soft_fail":false,"target_va":5381883104,"va":5399918059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_vec3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18036153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399917840,"raw_hex":"e847caecfe","size":5,"soft_fail":false,"target_va":5381883392,"va":5399919540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2747,"record_offset":16584,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::clamp_length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":16872,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":99655,"source_line":810,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":16616,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":17152,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":25834,"source_line":420,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":17324,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":12697,"source_line":538,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":17236,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":17764,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":17492,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":17580,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":13727,"source_line":1809,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] system_critters::boids::Boid::apply_rules
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":2747,"record_offset":11216,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":99653,"source_line":186,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2747,"record_offset":16812,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2747,"record_offset":14580,"parent_record_offset":14420,"ancestor_record_offset":10988,"inlinee":19551,"source_line":592,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2747,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":10988,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":14888,"parent_record_offset":14648,"ancestor_record_offset":10988,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":14796,"parent_record_offset":14740,"ancestor_record_offset":10988,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":14688,"parent_record_offset":14648,"ancestor_record_offset":10988,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":14976,"parent_record_offset":14916,"ancestor_record_offset":10988,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2747,"record_offset":15036,"parent_record_offset":14916,"ancestor_record_offset":10988,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":10988,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":12272,"parent_record_offset":12244,"ancestor_record_offset":10988,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic.rs
//     evidence={"module":2747,"record_offset":16456,"parent_record_offset":16376,"ancestor_record_offset":10988,"inlinee":6395,"source_line":211,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":10988,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":13968,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2747,"record_offset":13940,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2747,"record_offset":16428,"parent_record_offset":16376,"ancestor_record_offset":10988,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2747,"record_offset":11616,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":12972,"parent_record_offset":12940,"ancestor_record_offset":10988,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":12336,"parent_record_offset":12308,"ancestor_record_offset":10988,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":12180,"parent_record_offset":12148,"ancestor_record_offset":10988,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":17612,"parent_record_offset":17580,"ancestor_record_offset":10988,"inlinee":11306,"source_line":694,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":11540,"parent_record_offset":11500,"ancestor_record_offset":10988,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":12428,"parent_record_offset":12396,"ancestor_record_offset":10988,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$449::div_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":12520,"parent_record_offset":12488,"ancestor_record_offset":10988,"inlinee":9340,"source_line":943,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2747,"record_offset":11832,"parent_record_offset":11792,"ancestor_record_offset":10988,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2747,"record_offset":15312,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2747,"record_offset":15240,"parent_record_offset":15216,"ancestor_record_offset":10988,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2747,"record_offset":11740,"parent_record_offset":11672,"ancestor_record_offset":10988,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":15216,"parent_record_offset":15192,"ancestor_record_offset":10988,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":11672,"parent_record_offset":11616,"ancestor_record_offset":10988,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2747,"record_offset":15192,"parent_record_offset":15160,"ancestor_record_offset":10988,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2747,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":10988,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2747,"record_offset":14540,"parent_record_offset":14516,"ancestor_record_offset":10988,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":2747,"record_offset":16188,"parent_record_offset":16100,"ancestor_record_offset":10988,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":2747,"record_offset":14648,"parent_record_offset":14272,"ancestor_record_offset":10988,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":2747,"record_offset":14916,"parent_record_offset":14648,"ancestor_record_offset":10988,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::clamp_length_max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":16100,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":29375,"source_line":830,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":13712,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":11928,"parent_record_offset":11900,"ancestor_record_offset":10988,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":13140,"parent_record_offset":13112,"ancestor_record_offset":10988,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":13112,"parent_record_offset":13072,"ancestor_record_offset":10988,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":11900,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":12764,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":14052,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":13072,"parent_record_offset":13036,"ancestor_record_offset":10988,"inlinee":4892,"source_line":556,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":13036,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":4891,"source_line":573,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":14328,"parent_record_offset":14272,"ancestor_record_offset":10988,"inlinee":12697,"source_line":538,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::div_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":12488,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":9338,"source_line":1140,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":12308,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":12940,"parent_record_offset":12764,"ancestor_record_offset":10988,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":11500,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":11792,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":12396,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2747,"record_offset":12148,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2747,"record_offset":14740,"parent_record_offset":14716,"ancestor_record_offset":10988,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2747,"record_offset":14716,"parent_record_offset":14648,"ancestor_record_offset":10988,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] lazy_static::lazy::Lazy::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/inline_lazy.rs
//     evidence={"module":2747,"record_offset":14420,"parent_record_offset":14388,"ancestor_record_offset":10988,"inlinee":19548,"source_line":26,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2747,"record_offset":13996,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2747,"record_offset":12088,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::once::Once::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\once.rs
//     evidence={"module":2747,"record_offset":14456,"parent_record_offset":14420,"ancestor_record_offset":10988,"inlinee":19549,"source_line":145,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::futex::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\once\futex.rs
//     evidence={"module":2747,"record_offset":14492,"parent_record_offset":14456,"ancestor_record_offset":10988,"inlinee":19550,"source_line":77,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::Boid::unobstructed_dir
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/boids.rs
//     evidence={"module":2747,"record_offset":14272,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":99654,"source_line":156,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2747,"record_offset":14356,"parent_record_offset":14272,"ancestor_record_offset":10988,"inlinee":99642,"source_line":135,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::boids::impl$3::deref::__stability
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lazy_static","version":"1.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.4.0/src/lib.rs
//     evidence={"module":2747,"record_offset":14388,"parent_record_offset":14356,"ancestor_record_offset":10988,"inlinee":99640,"source_line":140,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2747,"record_offset":12244,"parent_record_offset":11216,"ancestor_record_offset":10988,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/02747.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::boids::Boid::random_vel_change
// 记录形式: compiled_procedure; instances=1
// 源码行: 389
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2746,"record_offset":2364,"rva":31203696,"va":5399912816}
// 编译实例 RVAs: 0x1dc2170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17903150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399912816,"raw_hex":"e8d2d1eefe","size":5,"soft_fail":false,"target_va":5382009728,"va":5399912873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17265763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399912816,"raw_hex":"e89d8bf8fe","size":5,"soft_fail":false,"target_va":5382647344,"va":5399913102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18029882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399912816,"raw_hex":"e8c6e2ecfe","size":5,"soft_fail":false,"target_va":5381883088,"va":5399912965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_vec3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18029646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399912816,"raw_hex":"e8b2e3ecfe","size":5,"soft_fail":false,"target_va":5381883392,"va":5399913033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::uniform_sample_cone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18028890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399912816,"raw_hex":"e8a6e6ecfe","size":5,"soft_fail":false,"target_va":5381884032,"va":5399912917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":2746,"record_offset":2496,"parent_record_offset":2364,"ancestor_record_offset":2364,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":2746,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":2364,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2746,"record_offset":3024,"parent_record_offset":2604,"ancestor_record_offset":2364,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2746,"record_offset":2932,"parent_record_offset":2876,"ancestor_record_offset":2364,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2746,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":2364,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2746,"record_offset":2732,"parent_record_offset":2648,"ancestor_record_offset":2364,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2746,"record_offset":2796,"parent_record_offset":2604,"ancestor_record_offset":2364,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":2746,"record_offset":2604,"parent_record_offset":2496,"ancestor_record_offset":2364,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":2746,"record_offset":3052,"parent_record_offset":2604,"ancestor_record_offset":2364,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":2746,"record_offset":2568,"parent_record_offset":2544,"ancestor_record_offset":2364,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":2746,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":2364,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2746,"record_offset":2676,"parent_record_offset":2648,"ancestor_record_offset":2364,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/02746.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2746,"record_offset":2648,"parent_record_offset":2604,"ancestor_record_offset":2364,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/02746.jsonl.gz"}
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
// 完整结构及原始类型记录: boids.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::boids::Boid
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_critters::boids::Boid", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>", "constructor": "system_critters::boids::Boid::frame_update", "arguments": ["system_critters::fish::fish::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>", "constructor": "system_critters::boids::Boid::frame_update", "arguments": ["system_critters::wren_boids::sim_wren_boids::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::boids::BoidAux
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_critters::boids::BoidAux,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_critters::boids::BoidAux", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["system_critters::boids::BoidAux", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>,system_critters::boids::impl$0::drain::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>", "system_critters::boids::impl$0::drain::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>,system_critters::boids::impl$0::drain::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::drain::Drain<system_critters::boids::BoidAux,alloc::alloc::Global>,system_critters::boids::impl$0::drain::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_critters::boids::BoidFlock
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "system_critters::boids::BoidFlock::remove<system_critters::fish::fish::closure_env$1>", "constructor": "system_critters::boids::BoidFlock::remove", "arguments": ["system_critters::fish::fish::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_critters::boids::BoidFlock::remove<system_critters::wren_boids::sim_wren_boids::closure_env$6>", "constructor": "system_critters::boids::BoidFlock::remove", "arguments": ["system_critters::wren_boids::sim_wren_boids::closure_env$6"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: boids.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-query/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_critters::boids::Boid", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_critters::boids::BoidFlock", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "system_critters::boids::Boid", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: boids.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] array$<glam::f32::sse2::vec3a::Vec3A,32> (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::apply_rules
// 信息 ID: 1404745840df9e29452806b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>; module=2747, inline_record=3712; 不是独立调用边。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>; module=2747, inline_record=11216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>
// 信息 ID: 0f6cd5ee526654b359be6ee8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_critters::boids::Boid::frame_update -> ["system_critters::fish::fish::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::fish::fish @ 0xf9164d -> 0x1dc2740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>
// 信息 ID: a28e5187711c2212feaaa1d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_critters::boids::Boid::frame_update -> ["system_critters::wren_boids::sim_wren_boids::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x23283e5 -> 0x1dc3510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::new
// 信息 ID: 719437de8867014f4bdf1b32
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::random_vel_change
// 信息 ID: 17c9968a718152c3ff113968
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0> @ 0x1dc27e4 -> 0x1dc2170 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3> @ 0x1dc35c8 -> 0x1dc2170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::Boid::unobstructed_dir
// 信息 ID: bfc4e951de7c5c1e85dda498
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>; module=2747, inline_record=6968; 不是独立调用边。
// [PDB 内联宿主] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>; module=2747, inline_record=14272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::clone_state
// 信息 ID: 42ac6137be847c3acc9e9679
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=1177, inline_record=4408; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3894, inline_record=16196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::drain
// 信息 ID: 51f387bc6d7b5bf4721d73c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3894, inline_record=5864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::iter_mut
// 信息 ID: c06740187a81786b7effbdb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=1177, inline_record=5384; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1177, inline_record=14904; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1177, inline_record=15332; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3894, inline_record=17308; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3894, inline_record=30032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::len
// 信息 ID: e067e363c7d765f2fc29d47a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=1177, inline_record=13452; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3894, inline_record=16168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::push
// 信息 ID: 784442e5cd4ea9c1a97e4cef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=1177, inline_record=3992; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3894, inline_record=7536; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3894, inline_record=13868; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3894, inline_record=28876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::remove<system_critters::fish::fish::closure_env$1>
// 信息 ID: 797c9aa0e693f53d5e7f5017
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_critters::boids::BoidFlock::remove -> ["system_critters::fish::fish::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::fish::fish @ 0xf92062 -> 0x1dc2340 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::BoidFlock::remove<system_critters::wren_boids::sim_wren_boids::closure_env$6>
// 信息 ID: 9901892d6720920cc30283fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_critters::boids::BoidFlock::remove -> ["system_critters::wren_boids::sim_wren_boids::closure_env$6"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x232962c -> 0x1dc2540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::impl$5::clone
// 信息 ID: 6a77e66445f1956bb40c2a26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::fish::fish; module=1177, inline_record=5244; 不是独立调用边。
// [PDB 内联宿主] system_critters::wren_boids::sim_wren_boids; module=3894, inline_record=17196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::boids::impl$6::default
// 信息 ID: 524acfe8e2851bac19cb2a74
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::impl$0::build; module=331, inline_record=1636; 不是独立调用边。
// [PDB 内联宿主] system_critters::impl$0::build; module=331, inline_record=2064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
